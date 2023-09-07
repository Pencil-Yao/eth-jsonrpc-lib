// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::rpc_types::Data;
use ethereum_types::{Address, H256, U256};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessList {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransaction {
    pub block_hash: H256,
    pub block_number: U256,
    pub from: Address,
    pub gas: u64,
    pub gas_price: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<U256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_priority_fee_per_gas: Option<U256>,
    pub hash: H256,
    pub input: Data,
    pub nonce: u64,
    pub to: Address,
    pub transaction_index: u64,
    pub value: U256,
    #[serde(rename = "type")]
    pub type_: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_list: Option<Vec<AccessList>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<U256>,
    pub v: U256,
    pub r: U256,
    pub s: U256,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockTransaction {
    Full(RpcTransaction),
    Hash(H256),
}
