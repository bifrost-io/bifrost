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

//! Autogenerated weights for bifrost_vstoken_conversion
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
// --pallet=bifrost_vstoken_conversion
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/vstoken-conversion/src/weights.rs
// --template=./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_vstoken_conversion.
pub trait WeightInfo {
	fn set_exchange_fee() -> Weight;
	fn set_exchange_rate() -> Weight;
	fn set_relaychain_lease() -> Weight;
	fn vsbond_convert_to_vstoken() -> Weight;
	fn vstoken_convert_to_vsbond() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: VstokenConversion ExchangeFee (r:1 w:1)
	/// Proof: VstokenConversion ExchangeFee (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	fn set_exchange_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180`
		//  Estimated: `1517`
		// Minimum execution time: 30_744_000 picoseconds.
		Weight::from_parts(31_166_000, 1517)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: VstokenConversion ExchangeRate (r:1 w:1)
	/// Proof: VstokenConversion ExchangeRate (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn set_exchange_rate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180`
		//  Estimated: `3479`
		// Minimum execution time: 30_215_000 picoseconds.
		Weight::from_parts(31_148_000, 3479)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: VstokenConversion RelaychainLease (r:1 w:1)
	/// Proof: VstokenConversion RelaychainLease (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_relaychain_lease() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180`
		//  Estimated: `1489`
		// Minimum execution time: 28_293_000 picoseconds.
		Weight::from_parts(29_337_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:2 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: VstokenConversion RelaychainLease (r:1 w:0)
	/// Proof: VstokenConversion RelaychainLease (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VstokenConversion ExchangeRate (r:1 w:0)
	/// Proof: VstokenConversion ExchangeRate (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: VstokenConversion ExchangeFee (r:1 w:0)
	/// Proof: VstokenConversion ExchangeFee (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	fn vsbond_convert_to_vstoken() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2336`
		//  Estimated: `11362`
		// Minimum execution time: 258_393_000 picoseconds.
		Weight::from_parts(262_031_000, 11362)
			.saturating_add(RocksDbWeight::get().reads(14_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:2 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: VstokenConversion RelaychainLease (r:1 w:0)
	/// Proof: VstokenConversion RelaychainLease (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VstokenConversion ExchangeRate (r:1 w:0)
	/// Proof: VstokenConversion ExchangeRate (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: VstokenConversion ExchangeFee (r:1 w:0)
	/// Proof: VstokenConversion ExchangeFee (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	fn vstoken_convert_to_vsbond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2327`
		//  Estimated: `11362`
		// Minimum execution time: 244_708_000 picoseconds.
		Weight::from_parts(247_427_000, 11362)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
}
