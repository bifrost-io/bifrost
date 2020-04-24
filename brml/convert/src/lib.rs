// Copyright 2019-2020 Liebi Technologies.
// This file is part of Bifrost.

// Bifrost is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Bifrost is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Bifrost.  If not, see <http://www.gnu.org/licenses/>.
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

mod mock;
mod tests;

use frame_support::{Parameter, decl_event, decl_error, decl_module, decl_storage, ensure, IterableStorageMap};
use frame_system::{self as system, ensure_root, ensure_signed};
use node_primitives::{AssetTrait, AssetSymbol, FetchConvertRate, TokenType};
use sp_runtime::traits::{AtLeast32Bit, Member, Saturating, Zero};

pub trait Trait: frame_system::Trait {
	/// convert rate
	type ConvertRate: Member + Parameter + AtLeast32Bit + Default + Copy + Into<Self::Balance>;
	type RatePerBlock: Member + Parameter + AtLeast32Bit + Default + Copy + Into<Self::Balance> + Into<Self::ConvertRate>;

	/// The arithmetic type of asset identifier.
	type AssetId: Member + Parameter + AtLeast32Bit + Default + Copy;

	/// The units in which we record balances.
	type Balance: Member + Parameter + AtLeast32Bit + Default + Copy;

	/// The units in which we record costs.
	type Cost: Member + Parameter + AtLeast32Bit + Default + Copy;

	/// The units in which we record incomes.
	type Income: Member + Parameter + AtLeast32Bit + Default + Copy;

	type AssetTrait: AssetTrait<Self::AssetId, Self::AccountId, Self::Balance, Self::Cost, Self::Income>;

	/// event
	type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
}

decl_event! {
	pub enum Event {
		UpdateConvertSuccess,
		UpdatezRatePerBlockSuccess,
		ConvertTokenToVTokenSuccess,
		ConvertVTokenToTokenSuccess,
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Asset id doesn't exist
		TokenNotExist,
		/// Amount of input should be less than or equal to origin balance
		InvalidBalanceForTransaction,
		/// Convert rate doesn't be set
		ConvertRateDoesNotSet,
		/// This is an invalid convert rate
		InvalidConvertRate,
		/// Vtoken id is not equal to token id
		InvalidTokenPair,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Convert {
		/// convert rate between two tokens, vtoken => (token, convert_rate)
		ConvertRate get(fn convert_rate): map hasher(blake2_128_concat) T::AssetId => T::ConvertRate;
		/// change rate per block, vtoken => (token, rate_per_block)
		RatePerBlock get(fn rate_per_block): map hasher(blake2_128_concat) T::AssetId => T::RatePerBlock;
		/// collect referrer, converter => ([(referrer1, 1000), (referrer2, 2000), ...], total_point)
		/// total_point = 1000 + 2000 + ...
		/// referrer must be unique, so check it unique while a new referrer incoming
		ReferrerChannels get(fn referrer_channels): map hasher(blake2_128_concat) T::AccountId => (Vec<(T::AccountId, T::Balance)>, T::Balance);
		/// A pool that hold the total amount of token converted to vtoken
		TokenPool get(fn token_pool): map hasher(blake2_128_concat) T::AssetId => T::Balance;
		/// A pool that hold the total amount of vtoken converted from token
		VTokenPool get(fn vtoken_pool): map hasher(blake2_128_concat) T::AssetId => T::Balance;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
		fn set_convert_rate(
			origin,
			vtoken_id: AssetSymbol,
			convert_rate: T::ConvertRate
		) {
			ensure_root(origin)?;

			let vtoken_id = T::AssetId::from(vtoken_id as u32);

			ensure!(T::AssetTrait::token_exists(vtoken_id), Error::<T>::TokenNotExist);
			<ConvertRate<T>>::insert(vtoken_id, convert_rate);

			Self::deposit_event(Event::UpdateConvertSuccess);
		}

		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
		fn set_rate_per_block(
			origin,
			vtoken_id: AssetSymbol,
			rate_per_block: T::RatePerBlock
		) {
			ensure_root(origin)?;

			let vtoken_id = T::AssetId::from(vtoken_id as u32);

			ensure!(T::AssetTrait::token_exists(vtoken_id), Error::<T>::TokenNotExist);
			<RatePerBlock<T>>::insert(vtoken_id, rate_per_block);

			Self::deposit_event(Event::UpdatezRatePerBlockSuccess);
		}

		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
		fn convert_token_to_vtoken(
			origin,
			#[compact] token_amount: T::Balance,
			vtoken_id: AssetSymbol,
			referrer: Option<T::AccountId>
		) {
			let converter = ensure_signed(origin)?;

			let vtoken_id = T::AssetId::from(vtoken_id as u32);

			// check asset_id exist or not
			ensure!(T::AssetTrait::token_exists(vtoken_id), Error::<T>::TokenNotExist);

			let token_id = vtoken_id; // token id is equal to vtoken id
			let token_balances = T::AssetTrait::get_account_asset(&token_id, TokenType::Token, &converter).balance;
			ensure!(token_balances >= token_amount, Error::<T>::InvalidBalanceForTransaction);

			// check convert rate has been set
			ensure!(<ConvertRate<T>>::contains_key(vtoken_id), Error::<T>::ConvertRateDoesNotSet);

			let rate = <ConvertRate<T>>::get(vtoken_id);

			ensure!(!rate.is_zero(), Error::<T>::InvalidConvertRate);
			let vtokens_buy = token_amount.saturating_mul(rate.into());

			// transfer
			T::AssetTrait::asset_destroy(token_id, TokenType::Token, converter.clone(), token_amount);
			T::AssetTrait::asset_issue(vtoken_id, TokenType::VToken, converter, vtokens_buy);

			// increase token/vtoken pool
			Self::increase_token_pool(token_id, token_amount);
			Self::increase_vtoken_pool(vtoken_id, vtokens_buy);

			// save
//			if let referrer = Some(referrer) {
//				// first time to referrer
//				if !<ReferrerChannels<T>>::contains_key(&converter) {
//					let value = (vec![(referrer, vtokens_buy)], vtokens_buy);
//					<ReferrerChannels<T>>::insert(&converter, value);
//				} else {
//					// existed, but new referrer incoming
//					<ReferrerChannels<T>>::mutate(&converter, |points| {
//						if points.0.iter().any(|point| point.0 == referrer) {
//							point.0[1] += vtokens_buy;
//						} else {
//							let value = (vec![(referrer, vtokens_buy)], vtokens_buy);
//							<ReferrerChannels<T>>::insert(&converter, value);
//						}
////						points.0.iter().find(|point| )
//						points.1 += vtokens_buy;
//					});
//				}
//			} else {
//				();
//			}

			Self::deposit_event(Event::ConvertTokenToVTokenSuccess);
		}

		#[weight = frame_support::weights::SimpleDispatchInfo::default()]
		fn convert_vtoken_to_token(
			origin,
			#[compact] vtoken_amount: T::Balance,
			vtoken_id: AssetSymbol,
		) {
			let converter = ensure_signed(origin)?;

			let vtoken_id = T::AssetId::from(vtoken_id as u32);

			// check asset_id exist or not
			ensure!(T::AssetTrait::token_exists(vtoken_id), Error::<T>::TokenNotExist);

			let vtoken_balances = T::AssetTrait::get_account_asset(&vtoken_id, TokenType::VToken, &converter).balance;
			ensure!(vtoken_balances >= vtoken_amount, Error::<T>::InvalidBalanceForTransaction);

			// check convert rate has been set
			ensure!(<ConvertRate<T>>::contains_key(vtoken_id), Error::<T>::ConvertRateDoesNotSet);

			let token_id = vtoken_id; // token id is equal to vtoken id
			let rate = <ConvertRate<T>>::get(vtoken_id);

			ensure!(!rate.is_zero(), Error::<T>::InvalidConvertRate);
			let tokens_buy = vtoken_amount / rate.into();

			T::AssetTrait::asset_destroy(vtoken_id, TokenType::VToken, converter.clone(), vtoken_amount);
			T::AssetTrait::asset_issue(token_id, TokenType::Token, converter, tokens_buy);

			// decrease token/vtoken pool
			Self::decrease_token_pool(token_id, tokens_buy);
			Self::decrease_vtoken_pool(vtoken_id, vtoken_amount);

			Self::deposit_event(Event::ConvertVTokenToTokenSuccess);
		}

		fn on_finalize() {
			for (vtoken_id, rate_per_block) in <RatePerBlock<T>>::iter() {
				if !<ConvertRate<T>>::contains_key(vtoken_id) {
					continue;
				}
				<ConvertRate<T>>::mutate(vtoken_id, |convert_rate| {
					*convert_rate = convert_rate.saturating_sub(rate_per_block.into());
				});
			}

//			let vtoken_balances = T::AssetTrait::get_account_asset(&vtoken_id, TokenType::VToken, &converter).balance;
//			let benefit = 5;
//			let epoch = 2000;
//
//			let curr_rate = <ConvertRate<T>>::get(vtoken_id);
//			let epoch_rate = (1 + benefit / vtoken_balances) * curr_rate;
//
//			let curr_blk_num = system::Module::<T>::block_number();
//			let specified_convert_rate = {
//				curr_rate + ((epoch_rate - curr_rate) / epoch) * (curr_blk_num % epoch)
//			};
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn get_convert(vtoken_id: T::AssetId) -> T::ConvertRate {
		let rate = <ConvertRate<T>>::get(vtoken_id);

		rate
	}

	fn increase_token_pool(token_id: T::AssetId, amount: T::Balance) {
		if <TokenPool<T>>::contains_key(&token_id) {
			<TokenPool<T>>::insert(token_id, amount);
		} else {
			<TokenPool<T>>::mutate(token_id, |pool| {
				*pool = pool.saturating_add(amount);
			});
		}
	}

	fn increase_vtoken_pool(vtoken_id: T::AssetId, amount: T::Balance) {
		if <VTokenPool<T>>::contains_key(&vtoken_id) {
			<VTokenPool<T>>::insert(vtoken_id, amount);
		} else {
			<VTokenPool<T>>::mutate(vtoken_id, |pool| {
				*pool = pool.saturating_add(amount);
			});
		}
	}

	fn decrease_token_pool(token_id: T::AssetId, amount: T::Balance) {
		if <TokenPool<T>>::contains_key(&token_id) {
			<TokenPool<T>>::insert(token_id, amount);
		} else {
			<TokenPool<T>>::mutate(token_id, |pool| {
				*pool = pool.saturating_sub(amount);
			});
		}
	}

	fn decrease_vtoken_pool(vtoken_id: T::AssetId, amount: T::Balance) {
		if <VTokenPool<T>>::contains_key(&vtoken_id) {
			<VTokenPool<T>>::insert(vtoken_id, amount);
		} else {
			<VTokenPool<T>>::mutate(vtoken_id, |pool| {
				*pool = pool.saturating_sub(amount);
			});
		}
	}
}

impl<T: Trait> FetchConvertRate<T::AssetId, T::ConvertRate> for Module<T> {
	fn fetch_convert_rate(asset_id: T::AssetId) -> T::ConvertRate {
		let rate = <ConvertRate<T>>::get(asset_id);

		rate
	}
}
