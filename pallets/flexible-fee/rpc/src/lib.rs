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

use std::{convert::TryInto, marker::PhantomData, sync::Arc};

pub use bifrost_flexible_fee_rpc_runtime_api::FlexibleFeeRuntimeApi as FeeRuntimeApi;
use codec::{Codec, Decode};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use node_primitives::{Balance, CurrencyId};
pub use pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi as TransactionPaymentRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_rpc::number::NumberOrHex;
use sp_runtime::traits::{Block as BlockT, Zero};

pub struct FlexibleFeeRpc<Client, Block> {
	client: Arc<Client>,
	_marker: PhantomData<Block>,
}

impl<Client, Block> FlexibleFeeRpc<Client, Block> {
	pub fn new(client: Arc<Client>) -> Self {
		Self { client, _marker: PhantomData }
	}
}

#[rpc(client, server)]
pub trait FeeRpcApi<BlockHash, AccountId> {
	/// rpc method get balances by account id
	/// useage: curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d '{"jsonrpc":"2.0","id":1,"method":"flexibleFeeFee_getFeeTokenAndAmount","params": ["0x0e0626477621754200486f323e3858cd5f28fcbe52c69b2581aecb622e384764", "0xa0040400008eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48cef70500"]}’
	#[method(name = "flexibleFee_getFeeTokenAndAmount")]
	fn get_fee_token_and_amount(
		&self,
		who: AccountId,
		encoded_xt: Bytes,
		at: Option<BlockHash>,
	) -> RpcResult<(CurrencyId, NumberOrHex)>;
}

/// Error type of this RPC api.
pub enum Error {
	/// The transaction was not decodable.
	DecodeError,
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::DecodeError => 2,
		}
	}
}

#[async_trait]
impl<C, Block, AccountId> FeeRpcApiServer<<Block as BlockT>::Hash, AccountId>
	for FlexibleFeeRpc<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: FeeRuntimeApi<Block, AccountId> + TransactionPaymentRuntimeApi<Block, Balance>,
	AccountId: Codec,
	Balance: Codec + std::fmt::Display + std::ops::Add<Output = Balance> + sp_runtime::traits::Zero,
{
	fn get_fee_token_and_amount(
		&self,
		who: AccountId,
		encoded_xt: Bytes,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<(CurrencyId, NumberOrHex)> {
		// Ok((
		//     CurrencyId::Native(TokenSymbol::BNC),
		//     sp_rpc::number::NumberOrHex::Number(1200),
		// ))

		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);
		let encoded_len = encoded_xt.len() as u32;

		let uxt: Block::Extrinsic = Decode::decode(&mut &*encoded_xt).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::DecodeError.into(),
				"Unable to query fee details.",
				Some(format!("{:?}", e)),
			))
		})?;

		let fee_details = api.query_fee_details(at, uxt.clone(), encoded_len).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query fee details.",
				Some(format!("{:?}", e)),
			))
		})?;

		let total_inclusion_fee: Balance = {
			if let Some(inclusion_fee) = fee_details.inclusion_fee {
				let base_fee = inclusion_fee.base_fee;
				let len_fee = inclusion_fee.len_fee;
				let adjusted_weight_fee = inclusion_fee.adjusted_weight_fee;

				base_fee + len_fee + adjusted_weight_fee
			} else {
				Zero::zero()
			}
		};

		let rs = api.get_fee_token_and_amount(at, who, total_inclusion_fee, uxt);

		let try_into_rpc_balance = |value: Balance| {
			value.try_into().map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					Error::RuntimeError.into(),
					format!("{} doesn't fit in NumberOrHex representation", value),
					Some(format!("{:?}", e)),
				))
			})
		};

		match rs {
			Ok((id, val)) => try_into_rpc_balance(val).map(|value| (id, value)),
			Err(e) => Err(CallError::Custom(ErrorObject::owned(
				Error::RuntimeError.into(),
				"Unable to query fee token and amount.",
				Some(format!("{:?}", e)),
			))),
		}
		.map_err(|e| jsonrpsee::core::Error::Call(e))
	}
}
