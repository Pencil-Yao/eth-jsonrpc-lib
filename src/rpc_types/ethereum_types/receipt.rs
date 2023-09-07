use crate::rpc_types::Data;
use ethereum_types::{Address, Bloom, H256};

/// Receipt
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub block_hash: H256,
    pub block_number: u64,
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub from: Address,
    pub to: Address,
    pub gas_used: u64,
    pub cumulative_gas_used: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<Address>,
    pub logs: Vec<Log>,
    pub logs_bloom: Bloom,
    #[serde(rename = "type")]
    pub type_: u64,
    pub effective_gas_price: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<H256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u64>,
}

/// Log
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Data,
    pub block_hash: H256,
    pub block_number: u64,
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub log_index: u64,
    pub removed: bool,
}
