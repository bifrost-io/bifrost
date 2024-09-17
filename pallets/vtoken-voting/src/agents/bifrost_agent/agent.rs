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

use crate::{agents::bifrost_agent::BifrostCall, *};
use bifrost_primitives::{CurrencyId, DerivativeIndex};
use frame_support::{ensure, pallet_prelude::*};
use xcm::v4::Location;

use crate::{pallet::Error, traits::*};

/// VotingAgent implementation for Bifrost
pub struct BifrostAgent<T: pallet::Config> {
	vtoken: CurrencyIdOf<T>,
	location: Location,
}

impl<T: pallet::Config> BifrostAgent<T> {
	pub fn new(vtoken: CurrencyId) -> Result<Self, Error<T>> {
		return if cfg!(feature = "polkadot") {
		let location = Pallet::<T>::convert_vtoken_to_dest_location(vtoken)?;
		Ok(Self { vtoken, location })
		} else {
			Err(Error::<T>::VTokenNotSupport)
		}
	}
}

impl<T: Config> VotingAgent<BalanceOf<T>, AccountIdOf<T>, Error<T>, T> for BifrostAgent<T> {
	fn vtoken(&self) -> CurrencyIdOf<T> {
		self.vtoken
	}

	fn location(&self) -> Location {
		self.location.clone()
	}

	fn delegate_vote(
		&self,
		_who: AccountIdOf<T>,
		vtoken: CurrencyIdOf<T>,
		poll_index: PollIndexOf<T>,
		_submitted: bool,
		new_delegator_votes: Vec<(DerivativeIndex, AccountVote<BalanceOf<T>>)>,
		_maybe_old_vote: Option<(AccountVote<BalanceOf<T>>, BalanceOf<T>)>,
	) -> DispatchResult {
		let derivative_index = new_delegator_votes[0].0;
		let vote_calls = new_delegator_votes
			.iter()
			.map(|(_derivative_index, vote)| {
				pallet_conviction_voting::Call::<T>::vote {
					poll_index,
					vote: Pallet::<T>::transfer(*vote),
				}
				.into()
			})
			.collect::<Vec<<T as Config>::RuntimeCall>>();

		let vote_call = if vote_calls.len() == 1 {
			vote_calls.into_iter().nth(0).ok_or(Error::<T>::NoData)?
		} else {
			return Err(Error::<T>::NoPermissionYet.into());
		};

		let token = CurrencyId::to_token(&vtoken).map_err(|_| Error::<T>::NoData)?;
		let delegator: AccountIdOf<T> =
			T::DerivativeAccount::get_account_id(token, derivative_index)
				.ok_or(Error::<T>::NoData)?;
		let origin: <T as pallet::Config>::PalletsOrigin = RawOrigin::Signed(delegator).into();
		vote_call.dispatch(origin.into()).map_err(|_| Error::<T>::InvalidCallDispatch)?;

		Ok(())
	}

	fn vote_call_encode(
		&self,
		new_delegator_votes: Vec<(DerivativeIndex, AccountVote<BalanceOf<T>>)>,
		poll_index: PollIndexOf<T>,
		derivative_index: DerivativeIndex,
	) -> Result<Vec<u8>, Error<T>> {
		let vote_calls = new_delegator_votes
			.iter()
			.map(|(_derivative_index, vote)| {
				<BifrostCall<T> as ConvictionVotingCall<T>>::vote(poll_index, *vote)
			})
			.collect::<Vec<_>>();
		let vote_call = if vote_calls.len() == 1 {
			vote_calls.into_iter().nth(0).ok_or(Error::<T>::NoData)?
		} else {
			ensure!(false, Error::<T>::NoPermissionYet);
			<BifrostCall<T> as UtilityCall<BifrostCall<T>>>::batch_all(vote_calls)
		};

		let encode_call = <BifrostCall<T> as UtilityCall<BifrostCall<T>>>::as_derivative(
			derivative_index,
			vote_call,
		)
		.encode();

		Ok(encode_call)
	}

	fn remove_delegator_vote_call_encode(
		&self,
		class: PollClass,
		poll_index: PollIndexOf<T>,
		derivative_index: DerivativeIndex,
	) -> Vec<u8> {
		let remove_vote_call =
			<BifrostCall<T> as ConvictionVotingCall<T>>::remove_vote(Some(class), poll_index);
		<BifrostCall<T> as UtilityCall<BifrostCall<T>>>::as_derivative(
			derivative_index,
			remove_vote_call,
		)
		.encode()
	}
}
