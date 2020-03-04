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
#![cfg_attr(not(feature = "std"), no_std)]

mod mock;
mod tests;

use frame_support::{Parameter, decl_event, decl_module, decl_storage, ensure};
use frame_system::{self as system, ensure_root, ensure_signed};
use node_primitives::TokenType;
use sp_runtime::traits::{Member, Saturating, SimpleArithmetic};

pub trait Trait: assets::Trait {
	/// exchange rate
	type ExchangeRate: Member + Parameter + SimpleArithmetic + Default + Copy + Into<<Self as assets::Trait>::Balance> + Into<<Self as assets::Trait>::Balance>;
	type RatePerBlock: Member + Parameter + SimpleArithmetic + Default + Copy + Into<<Self as assets::Trait>::Balance> + Into<<Self as assets::Trait>::Balance> + Into<Self::ExchangeRate>;

	/// event
	type Event: From<Event> + Into<<Self as frame_system::Trait>::Event>;
}

decl_event! {
	pub enum Event {
		UpdateExchangeSuccess,
		UpdatezRatePerBlockSuccess,
		ExchangeTokenToVTokenSuccess,
		ExchangerVTokenToTokenSuccess,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Exchange {
		/// exchange rate between two tokens, vtoken => (token, exchange_rate)
		ExchangeRate get(fn exchange_rate): linked_map hasher(blake2_256) <T as assets::Trait>::AssetId => (<T as assets::Trait>::AssetId, T::ExchangeRate);
		/// change rate per block, vtoken => (token, rate_per_block)
		RatePerBlock get(fn rate_per_block): linked_map hasher(blake2_256) <T as assets::Trait>::AssetId => (<T as assets::Trait>::AssetId, T::RatePerBlock);
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		fn set_exchange_rate(
			origin,
			vtoken_id: <T as assets::Trait>::AssetId,
			exchange_rate: T::ExchangeRate
		) {
			ensure_root(origin)?;

			ensure!(<assets::Tokens<T>>::exists(vtoken_id), "this vtoken id doesn't exist.");

			let token_id = vtoken_id; // token id is equal to vtoken id
			<ExchangeRate<T>>::insert(vtoken_id, (token_id, exchange_rate));

			Self::deposit_event(Event::UpdateExchangeSuccess);
		}

		fn set_rate_per_block(
			origin,
			vtoken_id: <T as assets::Trait>::AssetId,
			rate_per_block: T::RatePerBlock
		) {
			ensure_root(origin)?;

			ensure!(<assets::Tokens<T>>::exists(vtoken_id), "this vtoken id doesn't exist.");

			let token_id = vtoken_id; // token id is equal to vtoken id
			<RatePerBlock<T>>::insert(vtoken_id, (token_id, rate_per_block));

			Self::deposit_event(Event::UpdatezRatePerBlockSuccess);
		}

		fn exchange_token_to_vtoken(
			origin,
			token_amount: T::Balance,
			vtoken_id: <T as assets::Trait>::AssetId
		) {
			let exchanger = ensure_signed(origin)?;

			// check asset_id exist or not
			ensure!(<assets::Tokens<T>>::exists(vtoken_id), "this vtoken id is doesn't exist.");

			let token_id = vtoken_id; // token id is equal to vtoken id
			let token_balances = <assets::Balances<T>>::get((&token_id, TokenType::Token, &exchanger));
			ensure!(token_balances >= token_amount, "amount should be less than or equal to origin balance");

			// check exchange rate has been set
			ensure!(<ExchangeRate<T>>::exists(vtoken_id), "exchange rate doesn't be set.");

			let rate = <ExchangeRate<T>>::get(vtoken_id);
			ensure!(rate.0 == token_id, "token id must be equal.");

			let vtokens_buy = token_amount * rate.1.into();

			// transfer
			let to_vtoken_asset = (&vtoken_id, TokenType::VToken, &exchanger);
			<assets::Balances<T>>::mutate(to_vtoken_asset, |balances| {
				*balances = balances.saturating_add(vtokens_buy);
			});

			let to_token_asset = (&token_id, TokenType::Token, &exchanger);
			<assets::Balances<T>>::mutate(to_token_asset, |balances| {
				*balances = balances.saturating_sub(token_amount);
			});

			Self::deposit_event(Event::ExchangeTokenToVTokenSuccess);
		}

		fn exchange_vtoken_to_token(
			origin,
			vtoken_amount: T::Balance,
			vtoken_id: <T as assets::Trait>::AssetId,
		) {
			let exchanger = ensure_signed(origin)?;

			// check asset_id exist or not
			ensure!(<assets::Tokens<T>>::exists(vtoken_id), "this vtoken id is doesn't exist.");

			let vtoken_balances = <assets::Balances<T>>::get((&vtoken_id, TokenType::VToken, &exchanger));
			ensure!(vtoken_balances >= vtoken_amount, "amount should be less than or equal to origin balance");

			// check exchange rate has been set
			ensure!(<ExchangeRate<T>>::exists(vtoken_id), "exchange rate doesn't be set.");

			let token_id = vtoken_id; // token id is equal to vtoken id
			let rate = <ExchangeRate<T>>::get(vtoken_id);
			ensure!(rate.0 == token_id, "token id must be equal.");

			let tokens_buy = vtoken_amount / rate.1.into();

			// transfer
			let to_token_asset = (&token_id, TokenType::Token, &exchanger);
			<assets::Balances<T>>::mutate(to_token_asset, |balances| {
				*balances = balances.saturating_add(tokens_buy);
			});

			let to_vtoken_asset = (&vtoken_id, TokenType::VToken, &exchanger);
			<assets::Balances<T>>::mutate(to_vtoken_asset, |balances| {
				*balances = balances.saturating_sub(vtoken_amount);
			});

			Self::deposit_event(Event::ExchangerVTokenToTokenSuccess);
		}

		fn on_finalize() {
			for rate_per_block in <RatePerBlock<T>>::iter() {
				let vtoken_id = rate_per_block.0;
				if !<ExchangeRate<T>>::exists(vtoken_id) {
					continue;
				}
				<ExchangeRate<T>>::mutate(vtoken_id, |exchange_rate| {
					exchange_rate.1  = exchange_rate.1.saturating_sub(rate_per_block.1.into());
				});
			}
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn get_exchange(vtoken_id: <T as assets::Trait>::AssetId) -> T::ExchangeRate {
		let rate = <ExchangeRate<T>>::get(vtoken_id);

		rate.1
	}
}
