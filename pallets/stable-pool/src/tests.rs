use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use nutsfinance_stable_asset::{StableAsset as StableAssetInterface, StableAssetPoolInfo};
use orml_traits::MultiCurrency;

pub const BALANCE_OFF: u128 = 0;

#[test]
fn it_works_for_default_value() {
	ExtBuilder::default().new_test_ext().build().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(StablePool::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(StablePool::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

fn create_pool() -> (CurrencyId, CurrencyId, CurrencyId, u64) {
	let coin0 = DOT;
	let coin1 = vDOT;
	let pool_asset = LP_KSM_ETH;
	let amount: Balance = 100_000_000;
	// assert_ok!(Tokens::set_balance(RuntimeOrigin::root(), 1, coin0, amount, 0));
	// assert_ok!(Tokens::set_balance(RuntimeOrigin::root(), 1, coin1, amount, 0));

	assert_ok!(StableAsset::create_pool(
		RuntimeOrigin::signed(1),
		pool_asset,
		vec![coin0, coin1],
		vec![10000000000u128, 10000000000u128],
		10000000u128,
		20000000u128,
		50000000u128,
		10000u128,
		2,
		1,
		1000000000000000000u128,
	));
	(coin0, coin1, pool_asset, 8319403528785522541u64)
}

// #[test]
// fn create_pool_successful() {
// 	ExtBuilder::default().new_test_ext().build().execute_with(|| {
// 		assert_eq!(StableAsset::pool_count(), 0);
// 		let pool_tokens: (i64, i64, i64, u64) = create_pool();
// 		assert_eq!(
// 			StableAsset::pools(0),
// 			Some(StableAssetPoolInfo {
// 				pool_asset: 1,
// 				assets: vec![1, 2],
// 				precisions: vec![1u128, 1u128],
// 				mint_fee: 1u128,
// 				swap_fee: 1u128,
// 				redeem_fee: 1u128,
// 				total_supply: 0u128,
// 				a: 1u128,
// 				a_block: 0,
// 				future_a: 1u128,
// 				future_a_block: 0,
// 				balances: vec![0, 0],
// 				fee_recipient: 1,
// 				account_id: 8319403528785522541u64,
// 				yield_recipient: 1,
// 				precision: 1000000000000000000u128,
// 			})
// 		);
// 	});
// }

#[test]
fn modify_a_argument_error_failed() {
	env_logger::try_init().unwrap_or(());
	ExtBuilder::default().new_test_ext().build().execute_with(|| {
		let pool_tokens = create_pool();
		match pool_tokens {
			(_coin0, _coin1, _pool_asset, _swap_id) => {
				assert_noop!(
					StableAsset::modify_a(RuntimeOrigin::signed(1), 0, 100, 0),
					nutsfinance_stable_asset::Error::<Test>::ArgumentsError
				);
			},
		}
		log::debug!("fdsf{:?}", pool_tokens);
	});
}

#[test]
fn calc() {
	env_logger::try_init().unwrap_or(());
	ExtBuilder::default().new_test_ext().build().execute_with(|| {
		let pool_tokens = create_pool();
		match pool_tokens {
			(_coin0, _coin1, _pool_asset, _swap_id) => {
				assert_noop!(
					StableAsset::modify_a(RuntimeOrigin::signed(1), 0, 100, 0),
					nutsfinance_stable_asset::Error::<Test>::ArgumentsError
				);
			},
		}
		log::debug!("fdsf{:?}", pool_tokens);
	});
}

#[test]
fn create_pool_successful() {
	ExtBuilder::default().new_test_ext().build().execute_with(|| {
		let coin0 = DOT;
		let coin1 = vDOT;
		assert_eq!(StableAsset::pool_count(), 0);
		assert_ok!(StableAsset::create_pool(
			RuntimeOrigin::signed(1),
			LP_KSM_ETH,
			vec![coin0, coin1],
			vec![1u128, 1u128],
			1u128,
			1u128,
			1u128,
			1u128,
			1,
			1,
			1000000000000000000u128,
		));
		assert_eq!(
			StableAsset::pools(0),
			Some(StableAssetPoolInfo {
				pool_asset: LP_KSM_ETH,
				assets: vec![coin0, coin1],
				precisions: vec![1u128, 1u128],
				mint_fee: 1u128,
				swap_fee: 1u128,
				redeem_fee: 1u128,
				total_supply: 0u128,
				a: 1u128,
				a_block: 0,
				future_a: 1u128,
				future_a_block: 0,
				balances: vec![0, 0],
				fee_recipient: 1,
				account_id: 8319403528785522541u64,
				yield_recipient: 1,
				precision: 1000000000000000000u128,
			})
		);
	});
}

#[test]
fn mint_successful_equal_amounts() {
	ExtBuilder::default().new_test_ext().build().execute_with(|| {
		let pool_tokens = create_pool();
		System::set_block_number(2);
		match pool_tokens {
			(coin0, coin1, pool_asset, swap_id) => {
				let amounts = vec![10000000u128, 10000000u128];
				// let amounts = vec![9999999u128, 9999999u128];
				// assert_noop!(
				// 	StableAsset::mint(
				// 		RuntimeOrigin::signed(1),
				// 		0,
				// 		amounts,
				// 		2000000000000000000000000u128
				// 	),
				// 	Error::<Test>::MintUnderMin
				// );
				assert_ok!(StableAsset::mint(RuntimeOrigin::signed(1), 0, amounts.clone(), 0));
				// assert_ok!(StableAsset::mint(RuntimeOrigin::signed(1), 0, amounts, 0));
				assert_eq!(
					StableAsset::pools(0),
					Some(StableAssetPoolInfo {
						pool_asset,
						assets: vec![coin0, coin1],
						precisions: vec![10000000000u128, 10000000000u128],
						mint_fee: 10000000u128,
						swap_fee: 20000000u128,
						redeem_fee: 50000000u128,
						total_supply: 200000000000000000u128,
						a: 10000u128,
						a_block: 0,
						future_a: 10000u128,
						future_a_block: 0,
						balances: vec![100000000000000000u128, 100000000000000000u128],
						fee_recipient: 2,
						account_id: swap_id,
						yield_recipient: 1,
						precision: 1000000000000000000u128,
					})
				);

				assert_eq!(Tokens::free_balance(coin0, &1), 90000000u128 + BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin1, &1), 90000000u128 + BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin0, &swap_id), 10000000u128 - BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin1, &swap_id), 10000000u128 - BALANCE_OFF);
				assert_eq!(
					Tokens::free_balance(pool_asset, &1),
					199800000000000000u128 - BALANCE_OFF
				);
				assert_eq!(Tokens::free_balance(pool_asset, &2), 200000000000000u128 - BALANCE_OFF);
			},
		}
	});
}

#[test]
fn swap_successful() {
	ExtBuilder::default().new_test_ext().build().execute_with(|| {
		let pool_tokens = create_pool();
		System::set_block_number(2);
		match pool_tokens {
			(coin0, coin1, pool_asset, swap_id) => {
				let amounts = vec![10000000u128, 20000000u128];
				assert_ok!(StableAsset::mint(RuntimeOrigin::signed(1), 0, amounts, 0));
				assert_ok!(StableAsset::swap(RuntimeOrigin::signed(1), 0, 0, 1, 5000000u128, 0, 2));
				assert_eq!(
					StableAsset::pools(0),
					Some(StableAssetPoolInfo {
						pool_asset,
						assets: vec![coin0, coin1],
						precisions: vec![10000000000u128, 10000000000u128],
						mint_fee: 10000000u128,
						swap_fee: 20000000u128,
						redeem_fee: 50000000u128,
						total_supply: 300006989999594867u128,
						a: 10000u128,
						a_block: 0,
						future_a: 10000u128,
						future_a_block: 0,
						balances: vec![150000000000000000u128, 150006990000000000u128],
						fee_recipient: 2,
						account_id: swap_id,
						yield_recipient: 1,
						precision: 1000000000000000000u128,
					})
				);
				assert_eq!(Tokens::free_balance(coin0, &1), 85000000u128 - BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin1, &1), 84999301u128 - BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin0, &swap_id), 15000000u128 - BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin1, &swap_id), 15000699u128 - BALANCE_OFF);
				// if let RuntimeEvent::StableAsset(crate::pallet::Event::TokenSwapped {
				// 	swapper: _,
				// 	pool_id: _,
				// 	input_asset: _,
				// 	output_asset: _,
				// 	input_amount: dx,
				// 	output_amount: dy,
				// 	a: _,
				// 	balances: _,
				// 	total_supply: _,
				// 	min_output_amount: _,
				// }) = last_event()
				// {
				// 	assert_eq!(dx, 5000000u128);
				// 	assert_eq!(dy, 4999301u128);
				// } else {
				// 	panic!("Unexpected event");
				// }
			},
		}
	});
}

#[test]
fn get_swap_output_amount() {
	ExtBuilder::default().new_test_ext().build().execute_with(|| {
		let pool_tokens = create_pool();
		System::set_block_number(2);
		match pool_tokens {
			(coin0, coin1, pool_asset, swap_id) => {
				let amounts = vec![10000000u128, 20000000u128];
				assert_ok!(StableAsset::mint(RuntimeOrigin::signed(1), 0, amounts, 0));
				let swap_out = StableAsset::get_swap_output_amount(0, 0, 1, 5000000u128);
				log::debug!("{:?}", swap_out);
				assert_ok!(StableAsset::swap(RuntimeOrigin::signed(1), 0, 0, 1, 5000000u128, 0, 2));
				assert_eq!(
					StableAsset::pools(0),
					Some(StableAssetPoolInfo {
						pool_asset,
						assets: vec![coin0, coin1],
						precisions: vec![10000000000u128, 10000000000u128],
						mint_fee: 10000000u128,
						swap_fee: 20000000u128,
						redeem_fee: 50000000u128,
						total_supply: 300006989999594867u128,
						a: 10000u128,
						a_block: 0,
						future_a: 10000u128,
						future_a_block: 0,
						balances: vec![150000000000000000u128, 150006990000000000u128],
						fee_recipient: 2,
						account_id: swap_id,
						yield_recipient: 1,
						precision: 1000000000000000000u128,
					})
				);
				assert_eq!(Tokens::free_balance(coin0, &1), 85000000u128 - BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin1, &1), 84999301u128 - BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin0, &swap_id), 15000000u128 - BALANCE_OFF);
				assert_eq!(Tokens::free_balance(coin1, &swap_id), 15000699u128 - BALANCE_OFF);
			},
		}
	});
}