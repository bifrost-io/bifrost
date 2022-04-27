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

//! Autogenerated weights for `bifrost_token_issuer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// --chain=bifrost-local
// --steps=50
// --repeat=20
// --pallet=bifrost_token_issuer
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-GPL3
// --output=./runtime/bifrost-kusama/src/weights/bifrost_token_issuer.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `bifrost_token_issuer`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_token_issuer::WeightInfo for WeightInfo<T> {
	// Storage: TokenIssuer IssueWhiteList (r:1 w:1)
	fn add_to_issue_whitelist() -> Weight {
		(65_246_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TokenIssuer IssueWhiteList (r:1 w:1)
	fn remove_from_issue_whitelist() -> Weight {
		(64_545_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TokenIssuer TransferWhiteList (r:1 w:1)
	fn add_to_transfer_whitelist() -> Weight {
		(65_361_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TokenIssuer TransferWhiteList (r:1 w:1)
	fn remove_from_transfer_whitelist() -> Weight {
		(64_479_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TokenIssuer IssueWhiteList (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn issue() -> Weight {
		(111_416_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TokenIssuer TransferWhiteList (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	fn transfer() -> Weight {
		(144_058_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
