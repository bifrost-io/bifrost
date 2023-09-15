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
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bifrost_vsbond_auction
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bifrost-jenkins`, CPU: `Intel(R) Xeon(R) CPU E5-26xx v4`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_vsbond_auction
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/bifrost-kusama/src/weights/bifrost_vsbond_auction.rs
// --template=./weight-template/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for bifrost_vsbond_auction.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_vsbond_auction::WeightInfo for BifrostWeight<T> {
	// Storage: VSBondAuction TransactionFee (r:1 w:0)
	// Proof Skipped: VSBondAuction TransactionFee (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: VSBondAuction UserOrderIds (r:1 w:1)
	// Proof Skipped: VSBondAuction UserOrderIds (max_values: None, max_size: None, mode: Measured)
	// Storage: VSBondAuction NextOrderId (r:1 w:1)
	// Proof Skipped: VSBondAuction NextOrderId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: VSBondAuction TotalOrderInfos (r:0 w:1)
	// Proof Skipped: VSBondAuction TotalOrderInfos (max_values: None, max_size: None, mode: Measured)
	fn create_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1789`
		//  Estimated: `6176`
		// Minimum execution time: 175_259 nanoseconds.
		Weight::from_parts(177_403_000, 6176)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: VSBondAuction TotalOrderInfos (r:1 w:1)
	// Proof Skipped: VSBondAuction TotalOrderInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: VSBondAuction UserOrderIds (r:1 w:1)
	// Proof Skipped: VSBondAuction UserOrderIds (max_values: None, max_size: None, mode: Measured)
	fn revoke_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2023`
		//  Estimated: `6176`
		// Minimum execution time: 161_381 nanoseconds.
		Weight::from_parts(162_871_000, 6176)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: VSBondAuction TotalOrderInfos (r:1 w:1)
	// Proof Skipped: VSBondAuction TotalOrderInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: VSBondAuction TransactionFee (r:1 w:0)
	// Proof Skipped: VSBondAuction TransactionFee (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: VSBondAuction UserOrderIds (r:1 w:1)
	// Proof Skipped: VSBondAuction UserOrderIds (max_values: None, max_size: None, mode: Measured)
	fn clinch_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2383`
		//  Estimated: `11362`
		// Minimum execution time: 236_772 nanoseconds.
		Weight::from_parts(238_487_000, 11362)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: VSBondAuction TotalOrderInfos (r:1 w:1)
	// Proof Skipped: VSBondAuction TotalOrderInfos (max_values: None, max_size: None, mode: Measured)
	// Storage: VSBondAuction TransactionFee (r:1 w:0)
	// Proof Skipped: VSBondAuction TransactionFee (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn partial_clinch_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2291`
		//  Estimated: `11362`
		// Minimum execution time: 206_572 nanoseconds.
		Weight::from_parts(214_687_000, 11362)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: VSBondAuction TransactionFee (r:1 w:1)
	// Proof Skipped: VSBondAuction TransactionFee (max_values: Some(1), max_size: None, mode: Measured)
	fn set_buy_and_sell_transaction_fee_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1489`
		// Minimum execution time: 26_870 nanoseconds.
		Weight::from_parts(27_527_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
