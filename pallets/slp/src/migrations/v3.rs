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

use crate::*;
use frame_support::traits::OnRuntimeUpgrade;

const LOG_TARGET: &str = "SLP::migration";

pub struct SlpMigration3<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for SlpMigration3<T> {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
		// Check the storage version
		let onchain_version = Pallet::<T>::on_chain_storage_version();
		if onchain_version < 3 {
			log::info!(target: LOG_TARGET, "Start to migrate DelegatorLedgers storage...");

			let mut write_count = 0;
			//migrate the value type of DelegatorLedgers
			DelegatorLedgers::<T>::translate(|key1, key2, value: Ledger<BalanceOf<T>>| {
				// Only migrate the Ledger::ParachainStaking
				if key1 == MOVR || key1 == GLMR {
					write_count = write_count + 1;
					// change Ledger::Moonbeam to Ledger::ParachainStaking
					if let Ledger::Moonbeam(ledger) = value {
						let new_ledger = Ledger::ParachainStaking(ledger);

						log::info!(
							target: LOG_TARGET,
							"Migrated to Ledger for {:?} - {:?}... into {:?}",
							key1,
							key2,
							new_ledger.clone()
						);
						Some(new_ledger)
					} else {
						Some(value)
					}
				} else {
					Some(value)
				}
			});

			// Update the storage version
			StorageVersion::new(3).put::<Pallet<T>>();

			// Return the consumed weight
			let read_count = DelegatorLedgers::<T>::iter().count();
			Weight::from(T::DbWeight::get().reads_writes(read_count as u64, write_count as u64))
		} else {
			// We don't do anything here.
			Weight::zero()
		}
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
		let delegator_Ledgers_cnt = DelegatorLedgers::<T>::iter().count();
		// print out the pre-migrate storage count
		log::info!(
			target: LOG_TARGET,
			"DelegatorLedgers pre-migrate storage count: {:?}",
			delegator_Ledgers_cnt
		);

		let cnt = delegator_Ledgers_cnt as u32;
		Ok(cnt.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(cnt: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		let delegator_Ledgers_old: (u32) = Decode::decode(&mut cnt.as_slice())
			.expect("the state parameter should be something that was generated by pre_upgrade");

		let delegator_Ledgers_new = DelegatorLedgers::<T>::iter().count();
		// print out the post-migrate storage count
		log::info!(
			target: LOG_TARGET,
			"DelegatorLedgers post-migrate storage count: {:?}",
			delegator_Ledgers_new
		);
		ensure!(
			delegator_Ledgers_new as u32 == delegator_Ledgers_old,
			"DelegatorLedgers post-migrate storage count not match"
		);

		Ok(())
	}
}
