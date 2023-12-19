// This file is part of Bifrost.

// Copyright (C) Liebi Technologies PTE. LTD.
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

#![cfg(feature = "runtime-benchmarks")]

use bifrost_primitives::CurrencyId;
use frame_benchmarking::v1::{account, benchmarks, whitelisted_caller, BenchmarkError};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_runtime::traits::UniqueSaturatedFrom;

use super::*;
#[allow(unused_imports)]
use crate::Pallet as ChannelCommission;

benchmarks! {
	register_channel {
		// assume we have 30 vtoken at most
		let x in 1 .. 30;

		let origin = T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let channel_name =  b"Bifrost".to_vec();
		let receiver = whitelisted_caller();

		// set_commission_tokens
		for i in 0 .. x {
			let i: u8 = i.try_into().unwrap();
			let vtoken = CurrencyId::VToken2(i);
			let commission_token = CurrencyId::Token2(i);
			assert_ok!(ChannelCommission::<T>::set_commission_tokens(
				origin.clone(),
				vtoken, commission_token
			));
		}

	}: _<T::RuntimeOrigin>(origin, channel_name, receiver)

	remove_channel {
		let origin = T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let channel_name = b"Bifrost".to_vec();
		let receiver = whitelisted_caller();
		let channel_id = 0;

		assert_ok!(ChannelCommission::<T>::register_channel(
			T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
			channel_name, receiver
		));
	}: _<T::RuntimeOrigin>(origin,channel_id)

	update_channel_receive_account {
		let origin = T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let channel_name = b"Bifrost".to_vec();
		let receiver = whitelisted_caller();
		let new_receiver = account("new_receiver", 0, 0);
		let channel_id = 0;

		assert_ok!(ChannelCommission::<T>::register_channel(
			T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
			channel_name, receiver
		));
	}: _<T::RuntimeOrigin>(origin,channel_id, new_receiver)

	set_channel_commission_token {
		let origin = T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let channel_name = b"Bifrost".to_vec();
		let receiver = whitelisted_caller();
		let commission_rate = 0;
		let channel_id = 0;
		let vtoken = CurrencyId::VToken2(0);

		assert_ok!(ChannelCommission::<T>::register_channel(
			T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
			channel_name, receiver
		));
	}: _<T::RuntimeOrigin>(origin,channel_id, vtoken, Some(commission_rate))

	set_commission_tokens {
		let origin = T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let commission_token = CurrencyId::VToken2(0);
		let vtoken = CurrencyId::VToken2(0);

	}: _<T::RuntimeOrigin>(origin, vtoken, commission_token)

	claim_commissions {
		let origin = T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let channel_name = b"Bifrost".to_vec();
		let receiver = whitelisted_caller();
		let channel_id = 0;
		let vtoken = CurrencyId::VToken2(0);
		let commission_rate = 0;
		let commission_token = CurrencyId::VToken2(0);
		let commission_account = T::CommissionPalletId::get().into_account_truncating();

		assert_ok!(ChannelCommission::<T>::set_commission_tokens(
			T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
			vtoken, commission_token
		));

		assert_ok!(ChannelCommission::<T>::register_channel(
			T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?,
			channel_name, receiver
		));

		// set some amount into ChannelClaimableCommissions storage
		let amount = BalanceOf::<T>::unique_saturated_from(1000u32);
		ChannelClaimableCommissions::<T>::insert(channel_id, commission_token, amount);
		// deposit some amount into the commission pool
		assert_ok!(T::MultiCurrency::deposit(commission_token, &commission_account, 10000u32.into()));

	}: _<T::RuntimeOrigin>(origin, channel_id)

	on_initialize {
		// assume we have 30 vtoken at most
		let x in 1 .. 30;

		let origin = T::ControlOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let channel_name =  b"Bifrost".to_vec();
		let receiver = whitelisted_caller();
		let share = Permill::from_percent(20);
		let commission_account: T::AccountId = T::CommissionPalletId::get().into_account_truncating();

		// token_id
		for i in 0 .. x {
			let i: u8 = i.try_into().unwrap();
			let vtoken = CurrencyId::VToken2(i);
			let commission_token = CurrencyId::Token2(i);

			// set_commission_tokens
			assert_ok!(ChannelCommission::<T>::set_commission_tokens(
				origin.clone(),
				vtoken, commission_token
			));

			VtokenIssuanceSnapshots::<T>::insert(vtoken, (9000, 10000));
			PeriodVtokenTotalMint::<T>::insert(vtoken, (10000, 2000));
			PeriodVtokenTotalRedeem::<T>::insert(vtoken, (0, 1000));
			PeriodTotalCommissions::<T>::insert(vtoken, (0, 100));

			// set vtoken issuance to 11000
			Currencies::update_balance(
				origin.clone(),
				commission_account.clone(),
				vtoken,
				11000,
			);

			// set ksm token issuance to 11000
			Currencies::update_balance(
				origin.clone(),
				commission_account.clone(),
				commission_token,
				11000,
			);

			// register_channel
			assert_ok!(ChannelCommission::<T>::register_channel(
				origin.clone(),
				channel_name, receiver
			));

			// set channel share
			ChannelVtokenShares::<T>::insert(0, vtoken, share.clone());
			PeriodChannelVtokenMint::<T>::insert(0, vtoken, (2000, 500));
		}

		let block_num =BlockNumberFor::<T>::from(101u32);
	}: _<T::RuntimeOrigin>(block_num)


	impl_benchmark_test_suite!(ChannelCommission,crate::mock::ExtBuilder::default().build(),crate::mock::Runtime);
}
