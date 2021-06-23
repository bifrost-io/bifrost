// This file is part of Bifrost.

// Copyright (C) 2019-2021 Liebi Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// Import various types used to declare pallet in scope.
	use frame_support::{
		pallet_prelude::{storage::child, *},
		sp_runtime::{
			traits::{AccountIdConversion, CheckedAdd, CheckedSub, Hash, Saturating, Zero},
			MultiSignature,
		},
		storage::ChildTriePrefixIterator,
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use node_primitives::{
		traits::{BancorHandler, BifrostXcmExecutor},
		CurrencyId, LeasePeriod, ParaId,
	};
	use orml_traits::{
		currency::TransferAll, LockIdentifier, MultiCurrency, MultiCurrencyExtended,
		MultiLockableCurrency, MultiReservableCurrency,
	};
	use polkadot_parachain::primitives::Id as PolkadotParaId;
	use sp_std::{convert::TryInto, fmt::Debug, prelude::*};
	use sp_arithmetic::Percent;
	use xcm::v0::{
		prelude::{XcmError, XcmResult},
		Junction, MultiLocation,
	};

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
	pub enum FundStatus {
		Ongoing,
		Retired,
		Success,
		Failed,
		Withdrew,
	}

	impl Default for FundStatus {
		fn default() -> Self {
			FundStatus::Ongoing
		}
	}

	/// Information on a funding effort for a pre-existing parachain. We assume that the parachain ID
	/// is known as it's used for the key of the storage item for which this is the value (`Funds`).
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
	#[codec(dumb_trait_bound)]
	pub struct FundInfo<AccountId, Balance, LeasePeriod> {
		/// The owning account who placed the deposit.
		depositor: AccountId,
		/// The amount of deposit placed.
		deposit: Balance,
		/// The total amount raised.
		raised: Balance,
		/// A hard-cap on the amount that may be contributed.
		cap: Balance,
		/// First slot in range to bid on; it's actually a LeasePeriod, but that's the same type as
		/// BlockNumber.
		first_slot: LeasePeriod,
		/// Last slot in range to bid on; it's actually a LeasePeriod, but that's the same type as
		/// BlockNumber.
		last_slot: LeasePeriod,
		/// Index used for the child trie of this fund
		trie_index: TrieIndex,
		/// Fund status
		status: FundStatus,
	}

	#[allow(type_alias_bounds)]
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	#[allow(type_alias_bounds)]
	type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<AccountIdOf<T>>>::Balance;

	type TrieIndex = u32;

	#[derive(Encode, Decode)]
	pub enum CrowdloanContributeCall<BalanceOf> {
		#[codec(index = 73)]
		CrowdloanContribute(ContributeCall<BalanceOf>),
	}

	#[derive(Encode, Decode)]
	pub enum CrowdloanWithdrawCall<AccountIdOf> {
		#[codec(index = 73)]
		CrowdloanWithdraw(WithdrawCall<AccountIdOf>),
	}

	#[derive(Debug, PartialEq, Encode, Decode)]
	pub struct Contribution<BalanceOf> {
		#[codec(compact)]
		index: ParaId,
		#[codec(compact)]
		value: BalanceOf,
		signature: Option<MultiSignature>,
	}

	#[derive(Encode, Decode)]
	pub enum ContributeCall<BalanceOf> {
		#[codec(index = 1)]
		Contribute(Contribution<BalanceOf>),
	}

	#[derive(Debug, PartialEq, Encode, Decode)]
	pub struct Withdraw<AccountIdOf> {
		who: AccountIdOf,
		#[codec(compact)]
		index: ParaId,
	}

	#[derive(Encode, Decode)]
	pub enum WithdrawCall<AccountIdOf> {
		#[codec(index = 2)]
		Withdraw(Withdraw<AccountIdOf>),
	}

	#[pallet::config]
	pub trait Config: frame_system::Config<BlockNumber = LeasePeriod> {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// ModuleID for the crowdloan module. An appropriate value could be ```ModuleId(*b"py/cfund")```
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// The amount to be held on deposit by the depositor of a crowdloan.
		type SubmissionDeposit: Get<BalanceOf<Self>>;

		/// The minimum amount that may be contributed into a crowdloan. Should almost certainly be at
		/// least ExistentialDeposit.
		#[pallet::constant]
		type MinContribution: Get<BalanceOf<Self>>;

		#[pallet::constant]
		type RelyChainToken: Get<CurrencyId>;

		#[pallet::constant]
		type VSBondValidPeriod: Get<LeasePeriod>;

		/// The time interval from 1:1 redeem-pool to bancor-pool to release.
		#[pallet::constant]
		type ReleaseCycle: Get<LeasePeriod>;

		/// The release ratio from the 1:1 redeem-pool to the bancor-pool per cycle.
		///
		/// **NOTE: THE RELEASE RATIO MUST BE IN [0, 1].**
		type ReleaseRatio: Get<Percent>;

		type MultiCurrency: TransferAll<AccountIdOf<Self>>
			+ MultiCurrency<AccountIdOf<Self>, CurrencyId = CurrencyId>
			+ MultiCurrencyExtended<AccountIdOf<Self>, CurrencyId = CurrencyId>
			+ MultiLockableCurrency<AccountIdOf<Self>, CurrencyId = CurrencyId>
			+ MultiReservableCurrency<AccountIdOf<Self>, CurrencyId = CurrencyId>;

		type BancorPool: BancorHandler<BalanceOf<Self>>;

		#[pallet::constant]
		type RemoveKeysLimit: Get<u32>;

		type ExecuteXcmOrigin: EnsureOrigin<
			<Self as frame_system::Config>::Origin,
			Success = MultiLocation,
		>;

		type BifrostXcmExecutor: BifrostXcmExecutor;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Create a new crowdloaning campaign. [fund_index]
		Created(ParaId),
		/// Contributing to a crowd sale. [who, fund_index, amount]
		Contributing(AccountIdOf<T>, ParaId, BalanceOf<T>),
		/// Contributed to a crowd sale. [who, fund_index, amount]
		Contributed(AccountIdOf<T>, ParaId, BalanceOf<T>),
		/// Fail on contribute to crowd sale. [who, fund_index, amount]
		ContributeFailed(AccountIdOf<T>, ParaId, BalanceOf<T>),
		/// Withdrawing full balance of a contributor. [who, fund_index, amount]
		Withdrawing(AccountIdOf<T>, ParaId, BalanceOf<T>),
		/// Withdrew full balance of a contributor. [who, fund_index, amount]
		Withdrew(AccountIdOf<T>, ParaId, BalanceOf<T>),
		/// Fail on withdraw full balance of a contributor. [who, fund_index, amount]
		WithdrawFailed(AccountIdOf<T>, ParaId, BalanceOf<T>),
		/// Redeeming token(rely-chain) by vsToken/vsBond. [who, fund_index, amount]
		Redeeming(AccountIdOf<T>, BalanceOf<T>),
		/// Redeemed token(rely-chain) by vsToken/vsBond. [who, fund_index, amount]
		Redeemed(AccountIdOf<T>, BalanceOf<T>),
		/// Fail on redeem token(rely-chain) by vsToken/vsBond. [who, fund_index, amount]
		RedeemFailed(AccountIdOf<T>, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The first slot needs to at least be less than 3 `max_value`.
		FirstSlotTooFarInFuture,
		/// Last slot must be greater than first slot.
		LastSlotBeforeFirstSlot,
		/// The last slot cannot be more then 3 slots after the first slot.
		LastSlotTooFarInFuture,
		/// The campaign ends before the current block number. The end must be in the future.
		CannotEndInPast,
		/// There was an overflow.
		Overflow,
		/// The contribution was below the minimum, `MinContribution`.
		ContributionTooSmall,
		/// Invalid fund index.
		InvalidParaId,
		/// Contributions exceed maximum amount.
		CapExceeded,
		/// The contribution period has already ended.
		ContributionPeriodOver,
		/// The origin of this call is invalid.
		InvalidOrigin,
		/// This crowdloan does not correspond to a parachain.
		NotParachain,
		/// This parachain lease is still active and retirement cannot yet begin.
		LeaseActive,
		/// This parachain's bid or lease is still active and withdraw cannot yet begin.
		BidOrLeaseActive,
		/// Funds have not yet been returned.
		FundsNotReturned,
		/// Fund has not yet retired.
		FundNotRetired,
		/// Fund has not withdrew.
		FundNotWithdrew,
		/// The crowdloan has not yet ended.
		FundNotEnded,
		/// Fund has been expired.
		FundExpired,
		/// There are no contributions stored in this crowdloan.
		NoContributions,
		/// This crowdloan has an active parachain and cannot be dissolved.
		HasActiveParachain,
		/// The crowdloan is not ready to dissolve. Potentially still has a slot or in retirement period.
		NotReadyToDissolve,
		/// Invalid signature.
		InvalidSignature,
		/// Invalid fund status.
		InvalidFundStatus,
		/// Insufficient Balance.
		InsufficientBalance,
		/// Crosschain xcm failed
		XcmFailed,
	}

	#[pallet::storage]
	#[pallet::getter(fn validators)]
	pub(super) type Validators<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, bool, ValueQuery>;

	/// Tracker for the next available trie index
	#[pallet::storage]
	#[pallet::getter(fn next_trie_index)]
	pub(super) type NextTrieIndex<T: Config> = StorageValue<_, TrieIndex, ValueQuery>;

	/// Info on all of the funds.
	#[pallet::storage]
	#[pallet::getter(fn funds)]
	pub(super) type Funds<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		ParaId,
		Option<FundInfo<AccountIdOf<T>, BalanceOf<T>, LeasePeriod>>,
		ValueQuery,
	>;

	/// The balance of the token(rely-chain) can be redeemed.
	#[pallet::storage]
	#[pallet::getter(fn redeem_pool)]
	pub(super) type RedeemPool<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub(super) fn fund_success(
			origin: OriginFor<T>,
			#[pallet::compact] index: ParaId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(
				fund.status == FundStatus::Ongoing,
				Error::<T>::InvalidFundStatus
			);
			Funds::<T>::mutate(index, |fund| {
				if let Some(fund) = fund {
					fund.status = FundStatus::Success;
				}
			});

			// Unlock vsToken/vsBond
			for (who, _) in Self::contribution_iterator(fund.trie_index) {
				T::MultiCurrency::remove_lock(vslock(index), Self::vstoken(), &who)?;
				T::MultiCurrency::remove_lock(
					vslock(index),
					Self::vsbond(index, fund.first_slot, fund.last_slot),
					&who,
				)?;
			}

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub(super) fn fund_fail(
			origin: OriginFor<T>,
			#[pallet::compact] index: ParaId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			// crownload is failed, so enable the withdrawal function of vsToken/vsBond
			let mut fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(
				fund.status == FundStatus::Ongoing,
				Error::<T>::InvalidFundStatus
			);
			fund.status = FundStatus::Failed;
			Funds::<T>::insert(index, Some(fund));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub(super) fn fund_retire(
			origin: OriginFor<T>,
			#[pallet::compact] index: ParaId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(
				fund.status == FundStatus::Success,
				Error::<T>::InvalidFundStatus
			);
			Funds::<T>::mutate(index, |fund| {
				if let Some(fund) = fund {
					fund.status = FundStatus::Retired;
				}
			});

			// Recharge into the 1:1 redeem pool.
			RedeemPool::<T>::mutate(|balance| {
				*balance = balance.saturating_add(fund.raised);
			});

			Ok(().into())
		}

		/// Create a new crowdloaning campaign for a parachain slot deposit for the current auction.
		#[pallet::weight(0)]
		pub(super) fn create(
			origin: OriginFor<T>,
			#[pallet::compact] index: ParaId,
			#[pallet::compact] cap: BalanceOf<T>,
			#[pallet::compact] first_slot: LeasePeriod,
			#[pallet::compact] last_slot: LeasePeriod,
		) -> DispatchResultWithPostInfo {
			let depositor = ensure_signed(origin)?;

			ensure!(first_slot <= last_slot, Error::<T>::LastSlotBeforeFirstSlot);
			let last_slot_limit = first_slot
				.checked_add(7u32.into())
				.ok_or(Error::<T>::FirstSlotTooFarInFuture)?;
			ensure!(
				last_slot <= last_slot_limit,
				Error::<T>::LastSlotTooFarInFuture
			);

			// There should not be an existing fund.
			ensure!(!Funds::<T>::contains_key(index), Error::<T>::FundNotEnded);

			let trie_index = Self::next_trie_index();
			let new_trie_index = trie_index.checked_add(1).ok_or(Error::<T>::Overflow)?;

			let deposit = T::SubmissionDeposit::get();

			Funds::<T>::insert(
				index,
				Some(FundInfo {
					depositor,
					deposit,
					raised: Zero::zero(),
					cap,
					first_slot,
					last_slot,
					trie_index,
					status: FundStatus::Ongoing,
				}),
			);

			NextTrieIndex::<T>::put(new_trie_index);

			Self::deposit_event(Event::<T>::Created(index));

			Ok(().into())
		}

		/// Contribute to a crowd sale. This will transfer some balance over to fund a parachain
		/// slot. It will be withdrawable in two instances: the parachain becomes retired; or the
		/// slot is unable to be purchased and the timeout expires.
		#[pallet::weight(0)]
		pub(super) fn contribute(
			origin: OriginFor<T>,
			#[pallet::compact] index: ParaId,
			#[pallet::compact] value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;

			ensure!(
				value >= T::MinContribution::get(),
				Error::<T>::ContributionTooSmall
			);

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(
				fund.status == FundStatus::Ongoing,
				Error::<T>::InvalidFundStatus
			);
			fund.raised
				.checked_add(&value)
				.ok_or(Error::<T>::Overflow)?;
			ensure!(fund.raised <= fund.cap, Error::<T>::CapExceeded);

			Self::xcm_ump_contribute(origin, index, value).map_err(|_e| Error::<T>::XcmFailed)?;

			Self::deposit_event(Event::Contributing(who, index, value));

			Ok(().into())
		}

		/// Confirm contribute
		#[pallet::weight(0)]
		pub(super) fn confirm_contribute(
			origin: OriginFor<T>,
			who: AccountIdOf<T>,
			index: ParaId,
			#[pallet::compact] value: BalanceOf<T>,
			is_success: bool,
		) -> DispatchResultWithPostInfo {
			Self::check_fund_owner(origin.clone(), index)?;
			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(
				fund.status == FundStatus::Ongoing,
				Error::<T>::InvalidFundStatus
			);
			Self::contribute_callback(who, index, value, is_success)
		}

		/// Withdraw full balance of the parachain.
		/// - `index`: The parachain to whose crowdloan the contribution was made.
		#[pallet::weight(0)]
		pub(super) fn withdraw(
			origin: OriginFor<T>,
			#[pallet::compact] index: ParaId,
		) -> DispatchResultWithPostInfo {
			Self::check_fund_owner(origin.clone(), index)?;

			let owner = ensure_signed(origin.clone())?;

			let fund: FundInfo<AccountIdOf<T>, BalanceOf<T>, LeasePeriod> =
				Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;

			ensure!(
				fund.status == FundStatus::Failed || fund.status == FundStatus::Retired,
				Error::<T>::InvalidFundStatus
			);

			Self::xcm_ump_withdraw(origin, index).map_err(|_e| Error::<T>::XcmFailed)?;

			Self::deposit_event(Event::Withdrawing(owner, index, fund.raised));

			Ok(().into())
		}

		/// Confirm withdraw by fund owner temporarily
		#[pallet::weight(0)]
		pub(super) fn confirm_withdraw(
			origin: OriginFor<T>,
			index: ParaId,
			is_success: bool,
		) -> DispatchResultWithPostInfo {
			Self::check_fund_owner(origin.clone(), index)?;

			let fund: FundInfo<AccountIdOf<T>, BalanceOf<T>, LeasePeriod> =
				Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;

			ensure!(
				fund.status == FundStatus::Failed || fund.status == FundStatus::Retired,
				Error::<T>::InvalidFundStatus
			);
			let who = ensure_signed(origin)?;
			Self::withdraw_callback(who, index, is_success)
		}

		#[pallet::weight(0)]
		pub(super) fn redeem(
			origin: OriginFor<T>,
			#[pallet::compact] index: ParaId,
			value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let block_number = <frame_system::Pallet<T>>::block_number();
			let who = ensure_signed(origin.clone())?;

			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(
				fund.status == FundStatus::Withdrew,
				Error::<T>::FundNotWithdrew
			);
			ensure!(
				(block_number - fund.last_slot) <= T::VSBondValidPeriod::get(),
				Error::<T>::FundExpired
			);

			let new_redeem_balance = Self::redeem_pool()
				.checked_sub(&value)
				.ok_or(Error::<T>::InsufficientBalance)?;

			let vstoken = Self::vstoken();
			let vsbond = Self::vsbond(index, fund.first_slot, fund.last_slot);
			T::MultiCurrency::ensure_can_withdraw(vstoken, &who, value)?;
			T::MultiCurrency::ensure_can_withdraw(vsbond, &who, value)?;

			Self::xcm_ump_redeem(origin, index, value).map_err(|_e| Error::<T>::XcmFailed)?;

			RedeemPool::<T>::put(new_redeem_balance);

			Self::deposit_event(Event::Redeeming(who, value));

			Ok(().into())
		}

		/// Confirm redeem by fund owner temporarily
		#[pallet::weight(0)]
		pub(super) fn confirm_redeem(
			origin: OriginFor<T>,
			who: AccountIdOf<T>,
			index: ParaId,
			value: BalanceOf<T>,
			is_success: bool,
		) -> DispatchResultWithPostInfo {
			Self::check_fund_owner(origin, index)?;
			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(
				fund.status == FundStatus::Withdrew,
				Error::<T>::FundNotWithdrew
			);
			Self::redeem_callback(who, index, value, is_success)
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: BlockNumberFor<T>) {
			// Release x% KSM/DOT from 1:1 redeem-pool to bancor-pool per cycle.
			if (n % T::ReleaseCycle::get()) == 0 {
				if let Ok(redeem_pool_balance) = TryInto::<u128>::try_into(Self::redeem_pool()) {
					// Calculate the release amount by `(redeem_pool_balance * T::ReleaseRatio).main_part()`.
					let release_amount = T::ReleaseRatio::get() * redeem_pool_balance;

					// Must be ok.
					if let Ok(release_amount) = TryInto::<BalanceOf<T>>::try_into(release_amount) {
						// Decrease the balance of redeem-pool by release amount.
						RedeemPool::<T>::mutate(|b| {
							*b = b.saturating_sub(release_amount);
						});

						// Increase the balance of bancor-pool by release amount.
						if let Err(err) =
							T::BancorPool::add_token(T::RelyChainToken::get().into(), release_amount)
						{
							log::warn!("Bancor: {:?} on bifrost-bancor.", err);
						}
					}
				} else {
					log::warn!("Overflow: The balance of redeem-pool exceeds u128.");
				}
			}

			// TODO: check & lock if vsBond if expired ???
		}

		fn on_initialize(_n: BlockNumberFor<T>) -> frame_support::weights::Weight {
			// TODO estimate weight
			Zero::zero()
		}
	}

	impl<T: Config> Pallet<T> {
		/// The account ID of the fund pot.
		///
		/// This actually does computation. If you need to keep using it, then make sure you cache the
		/// value and only call this once.
		pub fn fund_account_id(index: ParaId) -> AccountIdOf<T> {
			T::PalletId::get().into_sub_account(index)
		}

		pub fn id_from_index(index: TrieIndex) -> child::ChildInfo {
			let mut buf = Vec::new();
			buf.extend_from_slice(&(T::PalletId::get().0));
			buf.extend_from_slice(&index.encode()[..]);
			child::ChildInfo::new_default(T::Hashing::hash(&buf[..]).as_ref())
		}

		pub fn contribution_put(index: TrieIndex, who: &AccountIdOf<T>, balance: &BalanceOf<T>) {
			who.using_encoded(|b| child::put(&Self::id_from_index(index), b, &(balance)));
		}

		pub fn contribution_get(index: TrieIndex, who: &AccountIdOf<T>) -> BalanceOf<T> {
			who.using_encoded(|b| {
				child::get_or_default::<BalanceOf<T>>(&Self::id_from_index(index), b)
			})
		}

		pub fn contribution_kill(index: TrieIndex, who: &AccountIdOf<T>) {
			who.using_encoded(|b| child::kill(&Self::id_from_index(index), b));
		}

		pub fn crowdloan_kill(index: TrieIndex) -> child::KillChildStorageResult {
			child::kill_storage(&Self::id_from_index(index), Some(T::RemoveKeysLimit::get()))
		}

		pub fn contribution_iterator(
			index: TrieIndex,
		) -> ChildTriePrefixIterator<(AccountIdOf<T>, (BalanceOf<T>, Vec<u8>))> {
			ChildTriePrefixIterator::<_>::with_prefix_over_key::<Identity>(
				&Self::id_from_index(index),
				&[],
			)
		}

		pub fn xcm_ump_contribute(
			origin: OriginFor<T>,
			para_id: ParaId,
			value: BalanceOf<T>,
		) -> XcmResult {
			let origin_location: MultiLocation =
				T::ExecuteXcmOrigin::ensure_origin(origin).map_err(|_e| XcmError::BadOrigin)?;

			let contribution = Contribution {
				index: para_id,
				value: value.clone(),
				signature: None,
			};

			let call = CrowdloanContributeCall::CrowdloanContribute(ContributeCall::Contribute(
				contribution,
			))
			.encode()
			.into();

			let amount = TryInto::<u128>::try_into(value).map_err(|_| XcmError::Unimplemented)?;

			let _result = T::BifrostXcmExecutor::ump_transfer_asset(
				origin_location.clone(),
				MultiLocation::X1(Junction::Parachain(para_id)),
				amount,
				true,
			)?;

			T::BifrostXcmExecutor::ump_transact(origin_location, call)
		}

		pub fn xcm_ump_withdraw(origin: OriginFor<T>, para_id: ParaId) -> XcmResult {
			let origin_location: MultiLocation =
				T::ExecuteXcmOrigin::ensure_origin(origin).map_err(|_e| XcmError::BadOrigin)?;

			let who: AccountIdOf<T> = PolkadotParaId::from(para_id).into_account();

			let withdraw = Withdraw {
				who,
				index: para_id,
			};
			let call = CrowdloanWithdrawCall::CrowdloanWithdraw(WithdrawCall::Withdraw(withdraw))
				.encode()
				.into();
			T::BifrostXcmExecutor::ump_transact(origin_location, call)
		}

		pub fn xcm_ump_redeem(
			origin: OriginFor<T>,
			para_id: ParaId,
			value: BalanceOf<T>,
		) -> XcmResult {
			let origin_location: MultiLocation =
				T::ExecuteXcmOrigin::ensure_origin(origin).map_err(|_e| XcmError::BadOrigin)?;

			let amount = TryInto::<u128>::try_into(value).map_err(|_| XcmError::Unimplemented)?;

			T::BifrostXcmExecutor::ump_transfer_asset(
				MultiLocation::X1(Junction::Parachain(para_id)),
				origin_location,
				amount,
				false,
			)
		}

		fn check_fund_owner(origin: OriginFor<T>, para_id: ParaId) -> DispatchResultWithPostInfo {
			let owner = ensure_signed(origin)?;
			let fund = Self::funds(para_id).ok_or(Error::<T>::InvalidParaId)?;
			ensure!(owner == fund.depositor, Error::<T>::InvalidOrigin);
			Ok(().into())
		}

		fn vstoken() -> CurrencyId {
			T::RelyChainToken::get().to_vstoken().unwrap()
		}

		fn vsbond(index: ParaId, first_slot: LeasePeriod, last_slot: LeasePeriod) -> CurrencyId {
			CurrencyId::VSBond(*T::RelyChainToken::get(), index, first_slot, last_slot)
		}

		// FAKE-CODE: Just for demonstrating the process.
		// async safe err?
		#[allow(dead_code)]
		fn contribute_callback(
			who: AccountIdOf<T>,
			index: ParaId,
			value: BalanceOf<T>,
			is_success: bool,
		) -> DispatchResultWithPostInfo {
			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;
			let vstoken = Self::vstoken();
			let vsbond = Self::vsbond(index, fund.first_slot, fund.last_slot);

			if is_success {
				// Issue lock vsToken/vsBond to contributor.
				T::MultiCurrency::deposit(vstoken, &who, value)?;
				T::MultiCurrency::deposit(vsbond, &who, value)?;
				T::MultiCurrency::extend_lock(vslock(index), vstoken, &who, value)?;
				T::MultiCurrency::extend_lock(vslock(index), vsbond, &who, value)?;

				let new_balance = Self::contribution_get(fund.trie_index, &who)
					.checked_add(&value)
					.ok_or(Error::<T>::Overflow)?;
				// Recalculate fund raised.
				Funds::<T>::mutate(index, |fund| {
					if let Some(fund) = fund {
						fund.raised = fund.raised.saturating_add(value);
					}
				});

				// Recalculate the contribution of contributor to the fund.
				Self::contribution_put(fund.trie_index, &who, &new_balance);

				Self::deposit_event(Event::Contributed(who, index, value));
			} else {
				Self::deposit_event(Event::ContributeFailed(who, index, value));
			}

			Ok(().into())
		}

		// FAKE-CODE: Just for demonstrating the process.
		#[allow(dead_code)]
		fn withdraw_callback(
			who: AccountIdOf<T>,
			index: ParaId,
			is_success: bool,
		) -> DispatchResultWithPostInfo {
			let mut fund: FundInfo<AccountIdOf<T>, BalanceOf<T>, LeasePeriod> =
				Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;

			if is_success {
				fund.status = FundStatus::Withdrew;
				Funds::<T>::insert(index, Some(fund.clone()));
				Self::deposit_event(Event::Withdrew(who, index, fund.raised));
			} else {
				Self::deposit_event(Event::WithdrawFailed(who, index, fund.raised));
			}

			Ok(().into())
		}

		// FAKE-CODE: Just for demonstrating the process.
		#[allow(dead_code)]
		fn redeem_callback(
			who: AccountIdOf<T>,
			index: ParaId,
			value: BalanceOf<T>,
			is_success: bool,
		) -> DispatchResultWithPostInfo {
			let fund = Self::funds(index).ok_or(Error::<T>::InvalidParaId)?;

			if is_success {
				// Update contribution trie
				let old_balance = Self::contribution_get(fund.trie_index, &who);
				let balance = old_balance.saturating_sub(value);
				Self::contribution_put(fund.trie_index, &who, &balance);
				// Burn the vsToken/vsBond.
				T::MultiCurrency::withdraw(Self::vstoken(), &who, value)?;
				T::MultiCurrency::withdraw(
					Self::vsbond(index, fund.first_slot, fund.last_slot),
					&who,
					value,
				)?;

				Self::deposit_event(Event::Redeemed(who, value));
			} else {
				// Revoke the redeem pool.
				let new_redeem_balance = Self::redeem_pool().saturating_add(value);
				RedeemPool::<T>::put(new_redeem_balance);

				Self::deposit_event(Event::RedeemFailed(who, value));
			}

			Ok(().into())
		}
	}

	const fn vslock(index: ParaId) -> LockIdentifier {
		(index as u64).to_be_bytes()
	}
}
