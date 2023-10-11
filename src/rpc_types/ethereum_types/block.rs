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

use crate::rpc_types::{ethereum_types::EthBlockTransaction, Block, BlockHeader, Data};
use ethereum_types::{Address, Bloom, H256, U256};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthBlockHeader {
    pub parent_hash: H256,
    pub sha3_uncles: H256,
    pub miner: Address,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
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

impl From<BlockHeader> for EthBlockHeader {
    fn from(origin: BlockHeader) -> Self {
        Self {
            parent_hash: origin.prev_hash,
            sha3_uncles: Default::default(),
            miner: origin.proposer,
            state_root: origin.state_root,
            transactions_root: origin.transactions_root,
            receipts_root: origin.receipts_root,
            logs_bloom: Default::default(),
            difficulty: Default::default(),
            number: origin.number,
            gas_limit: 0,
            gas_used: origin.quota_used.low_u64(),
            timestamp: origin.timestamp / 1000,
            extra_data: Data::new(serde_json::to_vec(&origin.proof).unwrap()),
            mix_hash: Default::default(),
            nonce: 0,
            base_fee_per_gas: Default::default(),
            hash: Default::default(),
            total_difficulty: Default::default(),
        }
    }
}

impl From<cita_cloud_proto::blockchain::BlockHeader> for EthBlockHeader {
    fn from(mut origin: cita_cloud_proto::blockchain::BlockHeader) -> Self {
        if origin.proposer.len() == 32 && origin.height == 0 {
            origin.proposer = vec![0; 20];
        }
        Self {
            parent_hash: H256::from_slice(origin.prevhash.as_slice()),
            sha3_uncles: Default::default(),
            miner: Address::from_slice(origin.proposer.as_slice()),
            state_root: Default::default(),
            transactions_root: H256::from_slice(origin.transactions_root.as_slice()),
            receipts_root: Default::default(),
            logs_bloom: Default::default(),
            difficulty: Default::default(),
            number: U256::from(origin.height),
            gas_limit: 0,
            gas_used: Default::default(),
            timestamp: origin.timestamp / 1000,
            extra_data: Default::default(),
            mix_hash: Default::default(),
            nonce: 0,
            base_fee_per_gas: Default::default(),
            hash: Default::default(),
            total_difficulty: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EthBlock {
    #[serde(flatten)]
    pub header: EthBlockHeader,
    pub size: u64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<EthBlockTransaction>,
    pub uncles: Vec<H256>,
}

impl From<Block> for EthBlock {
    fn from(origin: Block) -> Self {
        Self {
            header: origin.header.into(),
            size: 0,
            transactions: origin
                .body
                .transactions
                .into_iter()
                .map(EthBlockTransaction::from)
                .collect(),
            uncles: vec![],
        }
    }
}

impl From<cita_cloud_proto::blockchain::Block> for EthBlock {
    fn from(origin: cita_cloud_proto::blockchain::Block) -> Self {
        Self {
            header: origin.header.unwrap().into(),
            size: 0,
            transactions: origin
                .body
                .unwrap()
                .body
                .into_iter()
                .map(EthBlockTransaction::from)
                .collect(),
            uncles: vec![],
        }
    }
}

impl From<cita_cloud_proto::blockchain::CompactBlock> for EthBlock {
    fn from(origin: cita_cloud_proto::blockchain::CompactBlock) -> Self {
        Self {
            header: origin.header.unwrap().into(),
            size: 0,
            transactions: origin
                .body
                .unwrap()
                .tx_hashes
                .into_iter()
                .map(|hash_bz| EthBlockTransaction::Hash(H256::from_slice(hash_bz.as_slice())))
                .collect(),
            uncles: vec![],
        }
    }
}
