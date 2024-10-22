// This file is part of Bifrost.// Copyright (C) Liebi Technologies PTE. LTD.// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0// This program is free software: you can redistribute it and/or modify// it under the terms of the GNU General Public License as published by// the Free Software Foundation, either version 3 of the License, or// (at your option) any later version.// This program is distributed in the hope that it will be useful,// but WITHOUT ANY WARRANTY; without even the implied warranty of// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the// GNU General Public License for more details.// You should have received a copy of the GNU General Public License// along with this program. If not, see <https://www.gnu.org/licenses/>.use crate::*;use bifrost_primitives::BNC;use frame_support::traits::OnRuntimeUpgrade;#[cfg(feature = "try-runtime")]use sp_runtime::TryRuntimeError;const LOG_TARGET: &str = "flexible-fee::migration";pub struct MigrateToV3<T>(sp_std::marker::PhantomData<T>);impl<T: Config> OnRuntimeUpgrade for MigrateToV3<T> {	fn on_runtime_upgrade() -> frame_support::weights::Weight {		// Check the storage version		let onchain_version = Pallet::<T>::on_chain_storage_version();		if onchain_version < 3 {			log::info!(target: LOG_TARGET, "Start to migrate flexible-fee storage...");			let mut count: u64 = 0;			// Traversal UserDefaultFeeCurrency storage			UserDefaultFeeCurrency::<T>::iter().for_each(|(account_id, currency_id)| {				// If currency_id is vbnc, change it to bnc				if currency_id == VBNC {					count += 1;					UserDefaultFeeCurrency::<T>::insert(account_id, BNC);				}			});			// Update the storage version			StorageVersion::new(3).put::<Pallet<T>>();			// Return the consumed weight			Weight::from(T::DbWeight::get().reads_writes(count + 1, count + 1))		} else {			// We don't do anything here.			Weight::zero()		}	}	#[cfg(feature = "try-runtime")]	fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {		let total_count = UserDefaultFeeCurrency::<T>::iter().count();		let mut vbnc_count: u64 = 0;		UserDefaultFeeCurrency::<T>::iter().for_each(|(_, currency_id)| {			if currency_id == VBNC {				vbnc_count += 1;			}		});		// print out the pre-migrate storage count		log::info!(			target: LOG_TARGET,			"UserDefaultFeeCurrency pre-migrate storage total count: {:?}",			total_count		);		log::info!(			target: LOG_TARGET,			"UserDefaultFeeCurrency pre-migrate storage vbnc count: {:?}",			vbnc_count		);		Ok((total_count as u64).encode())	}	#[cfg(feature = "try-runtime")]	fn post_upgrade(cnt: Vec<u8>) -> Result<(), TryRuntimeError> {		let old_total_count: u64 = Decode::decode(&mut cnt.as_slice())			.expect("the state parameter should be something that was generated by pre_upgrade");		let new_total_count = UserDefaultFeeCurrency::<T>::iter().count();		let mut new_vbnc_count: u64 = 0;		UserDefaultFeeCurrency::<T>::iter().for_each(|(_, currency_id)| {			if currency_id == VBNC {				new_vbnc_count += 1;			}		});		// print out the post-migrate storage count		log::info!(			target: LOG_TARGET,			"UserDefaultFeeCurrency post-migrate storage total count: {:?}",			new_total_count		);		log::info!(			target: LOG_TARGET,			"UserDefaultFeeCurrency post-migrate storage vbnc count: {:?}",			new_vbnc_count		);		ensure!(			new_total_count as u64 == old_total_count,			"Post-migration storage total count does not match pre-migration total count"		);		ensure!(			new_vbnc_count == 0,			"Post-migration storage vbnc count does not match pre-migration vbnc count"		);		Ok(())	}}