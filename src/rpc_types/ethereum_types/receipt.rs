use crate::rpc_types::{Data, Log, Receipt};
use cita_tool::U256;
use ethereum_types::{Address, Bloom, H256};

/// Receipt
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthReceipt {
    pub block_hash: H256,
    pub block_number: u64,
    pub transaction_hash: H256,
    pub transaction_index: u64,
    pub from: Address,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    pub gas_used: u64,
    pub cumulative_gas_used: u64,
    pub contract_address: Option<Address>,
    pub logs: Vec<EthLog>,
    pub logs_bloom: Bloom,
    #[serde(rename = "type")]
    pub type_: u64,
    pub effective_gas_price: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<H256>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Data>,
}

impl From<cita_cloud_proto::evm::Receipt> for EthReceipt {
    fn from(origin: cita_cloud_proto::evm::Receipt) -> Self {
        let contract_address = if origin.contract_address.is_empty() {
            None
        } else {
            Some(Address::from_slice(origin.contract_address.as_slice()))
        };
        let block_hash = H256::from_slice(origin.block_hash.as_slice());
        let mut logs: Vec<EthLog> = origin.logs.into_iter().map(EthLog::from).collect();
        for log in logs.iter_mut() {
            if log.block_hash != block_hash {
                log.block_hash = block_hash
            }
        }
        let status = if origin.error_message.is_empty() {
            Some(Data::new(vec![1]))
        } else {
            Some(Data::new(vec![0]))
        };
        EthReceipt {
            block_hash,
            block_number: origin.block_number,
            transaction_hash: H256::from_slice(origin.transaction_hash.as_slice()),
            transaction_index: origin.transaction_index,
            from: Default::default(),
            to: Default::default(),
            gas_used: U256::from(origin.quota_used.as_slice()).low_u64(),
            cumulative_gas_used: U256::from(origin.cumulative_quota_used.as_slice()).low_u64(),
            contract_address,
            logs,
            logs_bloom: Bloom::from_slice(origin.logs_bloom.as_slice()),
            type_: 0,
            effective_gas_price: 0,
            root: None,
            status,
        }
    }
}

impl From<Receipt> for EthReceipt {
    fn from(origin: Receipt) -> Self {
        let status = if origin.error_message.is_none() {
            Some(Data::new(vec![1]))
        } else {
            Some(Data::new(vec![0]))
        };
        EthReceipt {
            block_hash: origin.block_hash.unwrap_or_default(),
            block_number: origin.block_number.unwrap_or_default().low_u64(),
            transaction_hash: origin.transaction_hash.unwrap_or_default(),
            transaction_index: origin.transaction_index.unwrap_or_default().low_u64(),
            from: Default::default(),
            to: Default::default(),
            gas_used: origin.quota_used.unwrap_or_default().low_u64(),
            cumulative_gas_used: origin.cumulative_quota_used.low_u64(),
            contract_address: origin.contract_address,
            logs: origin.logs.into_iter().map(EthLog::from).collect(),
            logs_bloom: origin.logs_bloom,
            type_: 0,
            effective_gas_price: 0,
            root: None,
            status,
        }
    }
}

/// Log
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthLog {
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

impl From<cita_cloud_proto::evm::Log> for EthLog {
    fn from(origin: cita_cloud_proto::evm::Log) -> Self {
        EthLog {
            address: Address::from_slice(origin.address.as_slice()),
            topics: origin
                .topics
                .into_iter()
                .map(|topic| H256::from_slice(topic.as_slice()))
                .collect(),
            data: Data::new(origin.data),
            block_hash: H256::from_slice(origin.block_hash.as_slice()),
            block_number: origin.block_number,
            transaction_hash: H256::from_slice(origin.transaction_hash.as_slice()),
            transaction_index: origin.transaction_index,
            log_index: origin.log_index,
            removed: false,
        }
    }
}

impl From<Log> for EthLog {
    fn from(origin: Log) -> Self {
        EthLog {
            address: origin.address,
            topics: origin.topics,
            data: origin.data,
            block_hash: origin.block_hash.unwrap_or_default(),
            block_number: origin.block_number.unwrap_or_default().low_u64(),
            transaction_hash: origin.transaction_hash.unwrap_or_default(),
            transaction_index: origin.transaction_index.unwrap_or_default().low_u64(),
            log_index: origin.log_index.unwrap_or_default().low_u64(),
            removed: false,
        }
    }
}
