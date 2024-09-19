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
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for bifrost_salp
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
// --pallet=bifrost_salp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/salp/src/weights.rs
// --template=./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bifrost_salp.
pub trait WeightInfo {
	fn contribute() -> Weight;
	fn refund() -> Weight;
	fn unlock() -> Weight;
	fn redeem() -> Weight;
	fn set_multisig_confirm_account() -> Weight;
	fn fund_success() -> Weight;
	fn fund_fail() -> Weight;
	fn continue_fund() -> Weight;
	fn fund_retire() -> Weight;
	fn fund_end() -> Weight;
	fn create() -> Weight;
	fn edit() -> Weight;
	fn confirm_contribute() -> Weight;
	fn withdraw() -> Weight;
	fn dissolve_refunded() -> Weight;
	fn dissolve() -> Weight;
	fn buyback() -> Weight;
	fn buyback_vstoken_by_stable_pool() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Salp Funds (r:1 w:0)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: XcmInterface XcmWeightAndFee (r:1 w:0)
	/// Proof Skipped: XcmInterface XcmWeightAndFee (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm QueryCounter (r:1 w:1)
	/// Proof Skipped: PolkadotXcm QueryCounter (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Salp QueryIdContributionInfo (r:0 w:1)
	/// Proof Skipped: Salp QueryIdContributionInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm Queries (r:0 w:1)
	/// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2252`
		//  Estimated: `5717`
		// Minimum execution time: 175_000_000 picoseconds.
		Weight::from_parts(177_879_000, 5717)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: Salp FailedFundsToRefund (r:1 w:0)
	/// Proof Skipped: Salp FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Salp RedeemPool (r:1 w:1)
	/// Proof Skipped: Salp RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2375`
		//  Estimated: `11362`
		// Minimum execution time: 267_454_000 picoseconds.
		Weight::from_parts(270_319_000, 11362)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: Salp Funds (r:1 w:0)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:0)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:0)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1962`
		//  Estimated: `6176`
		// Minimum execution time: 129_578_000 picoseconds.
		Weight::from_parts(130_658_000, 6176)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Salp RedeemPool (r:1 w:1)
	/// Proof Skipped: Salp RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2356`
		//  Estimated: `11362`
		// Minimum execution time: 255_416_000 picoseconds.
		Weight::from_parts(257_010_000, 11362)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: Salp MultisigConfirmAccount (r:0 w:1)
	/// Proof Skipped: Salp MultisigConfirmAccount (max_values: Some(1), max_size: None, mode: Measured)
	fn set_multisig_confirm_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_258_000 picoseconds.
		Weight::from_parts(10_624_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_success() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3757`
		// Minimum execution time: 42_399_000 picoseconds.
		Weight::from_parts(43_690_000, 3757)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3757`
		// Minimum execution time: 42_250_000 picoseconds.
		Weight::from_parts(42_892_000, 3757)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Salp FailedFundsToRefund (r:0 w:1)
	/// Proof Skipped: Salp FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	fn continue_fund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1061`
		//  Estimated: `7001`
		// Minimum execution time: 99_887_000 picoseconds.
		Weight::from_parts(101_067_000, 7001)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_retire() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3757`
		// Minimum execution time: 40_648_000 picoseconds.
		Weight::from_parts(43_146_000, 3757)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	fn fund_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `3790`
		// Minimum execution time: 41_871_000 picoseconds.
		Weight::from_parts(42_961_000, 3790)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Salp CurrentTrieIndex (r:1 w:1)
	/// Proof Skipped: Salp CurrentTrieIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AssetRegistry CurrencyMetadatas (r:2 w:1)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `833`
		//  Estimated: `6773`
		// Minimum execution time: 81_834_000 picoseconds.
		Weight::from_parts(84_230_000, 6773)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	fn edit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `3757`
		// Minimum execution time: 39_217_000 picoseconds.
		Weight::from_parts(39_954_000, 3757)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salp MultisigConfirmAccount (r:1 w:0)
	/// Proof Skipped: Salp MultisigConfirmAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Salp QueryIdContributionInfo (r:1 w:1)
	/// Proof Skipped: Salp QueryIdContributionInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn confirm_contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2556`
		//  Estimated: `11362`
		// Minimum execution time: 278_991_000 picoseconds.
		Weight::from_parts(282_446_000, 11362)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Salp RedeemPool (r:1 w:1)
	/// Proof Skipped: Salp RedeemPool (max_values: Some(1), max_size: None, mode: Measured)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `3790`
		// Minimum execution time: 40_979_000 picoseconds.
		Weight::from_parts(46_708_000, 3790)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Salp FailedFundsToRefund (r:1 w:1)
	/// Proof Skipped: Salp FailedFundsToRefund (max_values: None, max_size: None, mode: Measured)
	fn dissolve_refunded() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `403`
		//  Estimated: `3868`
		// Minimum execution time: 53_494_000 picoseconds.
		Weight::from_parts(54_236_000, 3868)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salp Funds (r:1 w:1)
	/// Proof Skipped: Salp Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: unknown `0x` (r:1 w:0)
	/// Proof Skipped: unknown `0x` (r:1 w:0)
	/// Storage: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	/// Proof Skipped: unknown `0xd861ea1ebf4800d4b89f4ff787ad79ee96d9a708c85b57da7eb8f9ddeda61291` (r:1 w:1)
	fn dissolve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2063`
		//  Estimated: `8799`
		// Minimum execution time: 253_989_000 picoseconds.
		Weight::from_parts(256_988_000, 8799)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ZenlinkProtocol PairStatuses (r:1 w:0)
	/// Proof Skipped: ZenlinkProtocol PairStatuses (max_values: None, max_size: None, mode: Measured)
	fn buyback() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2284`
		//  Estimated: `11362`
		// Minimum execution time: 241_398_000 picoseconds.
		Weight::from_parts(244_975_000, 11362)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: StableAsset Pools (r:1 w:1)
	/// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	/// Storage: StableAsset TokenRateCaches (r:2 w:0)
	/// Proof Skipped: StableAsset TokenRateCaches (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	/// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn buyback_vstoken_by_stable_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2438`
		//  Estimated: `11362`
		// Minimum execution time: 361_674_000 picoseconds.
		Weight::from_parts(370_099_000, 11362)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
}
