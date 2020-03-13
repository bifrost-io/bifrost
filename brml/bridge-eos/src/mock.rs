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

#![cfg(test)]

use codec::Decode;
use frame_support::{
	impl_outer_origin, impl_outer_dispatch, impl_outer_event, parameter_types, ConsensusEngineId,
	traits::FindAuthor,
};
use sp_core::H256;
use sp_runtime::{
	Perbill,
	testing::{Header, TestXt, UintAuthorityId},
	traits::{BlakeTwo256, IdentityLookup, OnInitialize, OnFinalize},
};
use super::*;

impl_outer_dispatch! {
	pub enum Call for Test where origin: Origin {
		bridge_eos::BridgeEos,
	}
}

/// An extrinsic type used for tests.
pub type Extrinsic = TestXt<Call, ()>;
type SubmitTransaction = frame_system::offchain::TransactionSubmitter<(), Call, Extrinsic>;

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_event! {
	pub enum TestEvent for Test {
		bridge_eos,
		assets<T>,
	}
}

mod bridge_eos {
	pub use crate::Event;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 4 * 1024 * 1024;
	pub const MaximumBlockLength: u32 = 4 * 1024 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	pub const UncleGenerations: u32 = 5;
}

impl frame_system::Trait for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = Call;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type ModuleToIndex = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
}


pub const TEST_ID: ConsensusEngineId = [1, 2, 3, 4];

pub struct AuthorGiven;

impl FindAuthor<u64> for AuthorGiven {
	fn find_author<'a, I>(digests: I) -> Option<u64>
		where I: 'a + IntoIterator<Item=(ConsensusEngineId, &'a [u8])>
	{
		for (id, data) in digests {
			if id == TEST_ID {
				return u64::decode(&mut &data[..]).ok();
			}
		}

		None
	}
}

impl pallet_authorship::Trait for Test {
	type FindAuthor = AuthorGiven;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = ();
}

impl crate::Trait for Test {
	type AuthorityId = UintAuthorityId;
	type Event = TestEvent;
	type Balance = u64;
	type AssetId = u32;
	type Cost = u64;
	type Income = u64;
	type Precision = u32;
	type BridgeAssetFrom = ();
	type Call = Call;
	type SubmitTransaction = SubmitTransaction;
	type AssetTrait = Assets;
}

impl assets::Trait for Test {
	type Event = TestEvent;
	type Balance = u64;
	type AssetId = u32;
	type Price = u64;
	type Cost = u64;
	type Income = u64;
	type Exchange = u64;
	type AssetRedeem = ();
	type FetchExchangeRate = ();
}

pub type BridgeEos = crate::Module<Test>;
pub type Authorship = pallet_authorship::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Assets = assets::Module<Test>;

// simulate block production
pub(crate) fn run_to_block(n: u64) {
	while System::block_number() < n {
		BridgeEos::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		BridgeEos::on_initialize(System::block_number());
	}
}

// mockup runtime
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	GenesisConfig::<Test> {
		producer_schedule: eos_chain::ProducerSchedule::default(),
		bridge_contract_account: (b"bifrost".to_vec(), 2),
		notary_keys: vec![1u64, 2u64],
	}.assimilate_storage(&mut t).unwrap();
	t.into()
}
