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

//! Autogenerated weights for bifrost_channel_commission
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bifrost-build-machine`, CPU: `Intel(R) Xeon(R) CPU E5-26xx v4`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_channel_commission
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/bifrost-kusama/src/weights/bifrost_channel_commission.rs
// --template=./weight-template/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for bifrost_channel_commission.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_channel_commission::WeightInfo for BifrostWeight<T> {
	// Storage: `ChannelCommission::ChannelNextId` (r:1 w:1)
	// Proof: `ChannelCommission::ChannelNextId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ChannelCommission::CommissionTokens` (r:31 w:0)
	// Proof: `ChannelCommission::CommissionTokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ChannelCommission::Channels` (r:0 w:1)
	// Proof: `ChannelCommission::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ChannelCommission::ChannelCommissionTokenRates` (r:0 w:30)
	// Proof: `ChannelCommission::ChannelCommissionTokenRates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 30]`.
	fn register_channel(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `132 + x * (24 ±0)`
		//  Estimated: `3597 + x * (2499 ±0)`
		// Minimum execution time: 0 nanoseconds.
		Weight::from_parts(42_083_817, 3597)
			// Standard Error: 14_354
			.saturating_add(Weight::from_parts(5_330_371, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2499).saturating_mul(x.into()))
	}
	// Storage: `ChannelCommission::Channels` (r:1 w:1)
	// Proof: `ChannelCommission::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ChannelCommission::ChannelClaimableCommissions` (r:1 w:0)
	// Proof: `ChannelCommission::ChannelClaimableCommissions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_channel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3684`
		// Minimum execution time: 63_875 nanoseconds.
		Weight::from_parts(65_275_000, 3684)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `ChannelCommission::Channels` (r:1 w:1)
	// Proof: `ChannelCommission::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_channel_receive_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `219`
		//  Estimated: `3684`
		// Minimum execution time: 34_773 nanoseconds.
		Weight::from_parts(35_599_000, 3684)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `ChannelCommission::Channels` (r:1 w:0)
	// Proof: `ChannelCommission::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ChannelCommission::CommissionTokens` (r:1 w:0)
	// Proof: `ChannelCommission::CommissionTokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ChannelCommission::ChannelCommissionTokenRates` (r:0 w:1)
	// Proof: `ChannelCommission::ChannelCommissionTokenRates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_channel_commission_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `284`
		//  Estimated: `3749`
		// Minimum execution time: 38_767 nanoseconds.
		Weight::from_parts(40_191_000, 3749)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `ChannelCommission::CommissionTokens` (r:1 w:1)
	// Proof: `ChannelCommission::CommissionTokens` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_commission_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3574`
		// Minimum execution time: 28_525 nanoseconds.
		Weight::from_parts(29_093_000, 3574)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `ChannelCommission::Channels` (r:1 w:0)
	// Proof: `ChannelCommission::Channels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ChannelCommission::ChannelClaimableCommissions` (r:2 w:1)
	// Proof: `ChannelCommission::ChannelClaimableCommissions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim_commissions() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2055`
		//  Estimated: `7995`
		// Minimum execution time: 133_326 nanoseconds.
		Weight::from_parts(136_142_000, 7995)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `ChannelCommission::ChannelNextId` (r:1 w:0)
	// Proof: `ChannelCommission::ChannelNextId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `x` is `[1, 30]`.
	fn on_initialize(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `347`
		//  Estimated: `1832`
		// Minimum execution time: 0 nanoseconds.
		Weight::from_parts(10_055_431, 1832)
			// Standard Error: 1_983
			.saturating_add(Weight::from_parts(69_114, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}

	fn set_channel_vtoken_shares(x: u32, ) -> Weight {
		Weight::from_parts(43_239_575, 3597)
			.saturating_add(Weight::from_parts(5_355_920, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2499).saturating_mul(x.into()))
	}
}