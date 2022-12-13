// This file is part of Bifrost.

// Copyright (C) 2019-2022 Liebi Technologies (UK) Ltd.
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

use crate::{traits::VeMintingInterface, *};
use frame_system::pallet_prelude::*;
use node_primitives::currency;
pub use pallet::*;
use sp_std::collections::btree_map::BTreeMap; //{borrow::ToOwned, collections::btree_map::BTreeMap, prelude::*};

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, Default)]
pub struct IncentiveConfig<CurrencyId, Balance> {
	rewardRate: BTreeMap<CurrencyId, Balance>, // Balance,
	rewardPerTokenStored: BTreeMap<CurrencyId, Balance>,
	rewardsDuration: Timestamp,
	periodFinish: Timestamp,
	lastUpdateTime: Timestamp,
}

impl<T: Config> Pallet<T> {
	pub fn lastTimeRewardApplicable() -> Timestamp {
		let current_timestamp: Timestamp =
			sp_timestamp::InherentDataProvider::from_system_time().timestamp().as_millis();
		if current_timestamp < Self::incentive_configs().periodFinish {
			current_timestamp
		} else {
			Self::incentive_configs().periodFinish
		}
	}

	pub fn rewardPerToken() -> BTreeMap<CurrencyIdOf<T>, BalanceOf<T>> {
		let mut conf = Self::incentive_configs();
		let current_timestamp: Timestamp =
			sp_timestamp::InherentDataProvider::from_system_time().timestamp().as_millis();
		let _totalSupply = Self::totalSupply(current_timestamp);
		if _totalSupply == BalanceOf::<T>::zero() {
			return conf.rewardPerTokenStored;
		}
		conf.rewardPerTokenStored.iter_mut().for_each(|(currency, reward)| {
			*reward = reward.saturating_add(
				Self::lastTimeRewardApplicable()
					.saturated_into::<BalanceOf<T>>()
					.saturating_sub(conf.lastUpdateTime.saturated_into::<BalanceOf<T>>())
					.saturating_mul(
						*conf.rewardRate.get(currency).unwrap_or(&BalanceOf::<T>::zero()),
					)
					// .mul(1e18)
					.checked_div(&_totalSupply)
					.unwrap_or_default(), // .ok_or(Error::<T>::CalculationOverflow)?,
			);
		});

		IncentiveConfigs::<T>::set(conf.clone());
		conf.rewardPerTokenStored
	}

	pub fn earned(
		addr: &AccountIdOf<T>,
	) -> Result<BTreeMap<CurrencyIdOf<T>, BalanceOf<T>>, DispatchError> {
		let current_timestamp: Timestamp =
			sp_timestamp::InherentDataProvider::from_system_time().timestamp().as_millis();
		let mut rewards: BTreeMap<CurrencyIdOf<T>, BalanceOf<T>> = Self::rewards(addr);
		let vetoken_balance = Self::balanceOf(addr, current_timestamp)?;
		rewards.iter_mut().for_each(|(currency, reward)| {
			*reward = reward.saturating_add(
				vetoken_balance.saturating_mul(
					Self::rewardPerToken()
						.get(currency)
						.unwrap_or(&BalanceOf::<T>::zero())
						.saturating_sub(
							*Self::user_reward_per_token_paid(addr)
								.get(currency)
								.unwrap_or(&BalanceOf::<T>::zero()),
						),
				),
			);
		});
		Ok(rewards)
		// Ok(Self::balanceOf(addr, current_timestamp)?
		// 	.saturating_mul(
		// 		Self::rewardPerToken().saturating_sub(Self::user_reward_per_token_paid(addr)),
		// 	)
		// 	// .div(1e18)
		// 	.saturating_add(Self::rewards(addr)))
	}

	pub fn updateReward(addr: Option<&AccountIdOf<T>>) -> DispatchResult {
		let rewardPerTokenStored = Self::rewardPerToken();
		IncentiveConfigs::<T>::mutate(|item| {
			item.rewardPerTokenStored = rewardPerTokenStored.clone();
			item.lastUpdateTime = Self::lastTimeRewardApplicable();
		});
		if let Some(address) = addr {
			Rewards::<T>::insert(address, Self::earned(&address)?);
			UserRewardPerTokenPaid::<T>::insert(address, rewardPerTokenStored);
		}
		Ok(())
	}

	// pub fn staking(addr: &AccountIdOf<T>, reward: BalanceOf<T>) -> DispatchResult {
	// 	Self::updateReward(Some(addr))
	// }

	pub fn getReward(addr: &AccountIdOf<T>) -> DispatchResult {
		Self::updateReward(Some(addr))?;
		// let rewards = Self::rewards(addr);
		if let Some(rewards) = Self::rewards(addr) {
			rewards.iter().for_each(|(currency, reward)| {
				T::MultiCurrency::transfer(
					currency,
					&T::VeMintingPalletId::get().into_account_truncating(),
					addr,
					reward,
				)?;
			});
			Rewards::<T>::remove(addr);
		}

		// if reward > BalanceOf::<T>::zero() {
		// 	T::Currency::transfer(
		// 		&T::VeMintingPalletId::get().into_account_truncating(),
		// 		addr,
		// 		reward,
		// 		ExistenceRequirement::KeepAlive,
		// 	)?;
		// 	Rewards::<T>::remove(addr);
		// }
		Ok(())
	}

	// Motion
	pub fn notifyRewardAmount(addr: &AccountIdOf<T>, reward: BalanceOf<T>) -> DispatchResult {
		Self::updateReward(None)?;
		let mut conf = Self::incentive_configs();
		let current_timestamp: Timestamp =
			sp_timestamp::InherentDataProvider::from_system_time().timestamp().as_millis();
		// let mut rewardRate;
		// if current_timestamp >= conf.periodFinish {
		// 	conf.rewardRate = reward
		// 		.checked_div(&conf.rewardsDuration.saturated_into::<BalanceOf<T>>())
		// 		.ok_or(Error::<T>::CalculationOverflow)?;
		// } else {
		// 	let remaining = conf
		// 		.periodFinish
		// 		.saturating_sub(current_timestamp)
		// 		.saturated_into::<BalanceOf<T>>();
		// 	let leftover: BalanceOf<T> = remaining.saturating_mul(conf.rewardRate);
		// 	conf.rewardRate = reward
		// 		.saturating_add(leftover)
		// 		.checked_div(&conf.rewardsDuration.saturated_into::<BalanceOf<T>>())
		// 		.ok_or(Error::<T>::CalculationOverflow)?;
		// }
		let balance = Self::balanceOf(
			&T::VeMintingPalletId::get().into_account_truncating(),
			current_timestamp,
		)?;
		// ensure!(
		// 	conf.rewardRate <=
		// 		balance
		// 			.checked_div(&conf.rewardsDuration.saturated_into::<BalanceOf<T>>())
		// 			.ok_or(Error::<T>::CalculationOverflow)?,
		// 	Error::<T>::NotExpire
		// );
		conf.lastUpdateTime = current_timestamp;
		conf.periodFinish = current_timestamp.saturating_add(conf.rewardsDuration);

		IncentiveConfigs::<T>::set(conf);
		Ok(())
	}
}
