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

use crate::rpc_types::{ethereum_types::BlockTransaction, Data};
use ethereum_types::{Address, Bloom, H256, U256};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub parent_hash: H256,
    pub sha3_uncles: H256,
    pub miner: Address,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub prev_hash: H256,
    pub logs_bloom: Bloom,
    pub difficulty: U256,
    pub number: U256,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: Data,
    pub mix_hash: H256,
    pub nonce: u64,
    pub base_fee_per_gas: U256,
    pub hash: H256,
    pub total_difficulty: U256,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
    #[serde(flatten)]
    pub header: BlockHeader,
    pub size: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<BlockTransaction>,
    pub uncles: Vec<H256>,
}
