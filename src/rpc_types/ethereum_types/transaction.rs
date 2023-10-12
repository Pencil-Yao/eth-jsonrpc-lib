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

use crate::rpc_types::ethereum_types::EthTransactionRequest;
use crate::rpc_types::parity_types::Action;
use crate::rpc_types::{parity_types, BlockTransaction, Data, Integer, RpcTransaction};
use cita_cloud_proto::blockchain::{raw_transaction, RawTransaction, Transaction};
use cita_tool::{pubkey_to_address, Signature, UnverifiedTransaction};
use ethereum_types::{Address, H256, U256};
use protobuf::parse_from_bytes;
use web3::signing::recover;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessList {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EthRpcTransaction {
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
    pub to: Option<Address>,
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

impl From<UnverifiedTransaction> for EthRpcTransaction {
    fn from(origin: UnverifiedTransaction) -> Self {
        let pubkey = origin.public_key().unwrap();
        let raw_tx = origin.transaction.unwrap();
        let sig = Signature::from(&origin.signature);
        let (v, r, s) = match sig {
            Signature::Secp256k1(sig) => (
                U256::from(sig.v()),
                U256::from_big_endian(&sig.r()),
                U256::from_big_endian(&sig.s()),
            ),
            Signature::Sm2(sig) => (
                U256::from(2), // no use recovery id, input a wrong number
                U256::from_big_endian(&sig.r()),
                U256::from_big_endian(&sig.s()),
            ),
            Signature::Null => panic!("null signature"),
        };
        let to = if raw_tx.to_v1.len() == 20 {
            Some(Address::from_slice(&raw_tx.to_v1))
        } else {
            None
        };
        EthRpcTransaction {
            block_hash: Default::default(),
            block_number: Default::default(),
            from: pubkey_to_address(&pubkey),
            gas: raw_tx.quota,
            gas_price: U256::zero(),
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            hash: Default::default(),
            input: Data::new(raw_tx.data),
            nonce: 0,
            to,
            transaction_index: 0,
            value: U256::from_big_endian(&raw_tx.value),
            type_: 0,
            access_list: None,
            chain_id: Some(U256::from_big_endian(&raw_tx.chain_id_v1)),
            v,
            r,
            s,
        }
    }
}

impl From<RpcTransaction> for EthRpcTransaction {
    fn from(origin: RpcTransaction) -> Self {
        let content: Vec<u8> = origin.content.into();
        let tx: UnverifiedTransaction = parse_from_bytes(&content).unwrap();
        let mut eth_tx = EthRpcTransaction::from(tx);
        eth_tx.block_hash = origin.block_hash;
        eth_tx.block_number = origin.block_number;
        eth_tx.hash = origin.hash;
        eth_tx
    }
}

impl From<EthTransactionRequest> for EthRpcTransaction {
    fn from(origin: EthTransactionRequest) -> Self {
        let mut tx = EthRpcTransaction::default();
        tx.from = origin.from.unwrap_or_default().into();
        tx.to = if let Some(to_addr) = origin.to {
            Some(to_addr.into())
        } else {
            None
        };
        tx.input = origin.input.unwrap_or(origin.data.unwrap_or_default());
        tx.value = origin.value.unwrap_or_default().0;
        tx.gas = origin.gas.unwrap_or(Integer::new(1000000)).0;
        tx.gas_price = U256::from(origin.gas_price.unwrap_or_default().0);
        tx
    }
}

impl From<parity_types::UnverifiedTransaction> for EthRpcTransaction {
    fn from(origin: parity_types::UnverifiedTransaction) -> Self {
        let (sig, rec_id) = origin.as_signature();
        let from = recover(
            origin.unsigned.signature_hash(origin.chain_id).as_bytes(),
            &sig,
            rec_id,
        )
        .unwrap();
        let origin_tx = origin.tx();
        let mut tx = EthRpcTransaction::default();
        tx.from = Address::from_slice(from.as_bytes());
        tx.to = match origin_tx.action {
            Action::Call(addr) => Some(addr),
            Action::Create => None,
        };
        tx.input = Data::new(origin_tx.data.clone());
        tx.value = origin_tx.value;
        tx.gas = origin_tx.gas.low_u64();
        tx.gas_price = origin_tx.gas_price;
        tx
    }
}

impl Into<Transaction> for EthRpcTransaction {
    fn into(self) -> Transaction {
        let to = if let Some(to) = self.to {
            to.0.to_vec()
        } else {
            vec![]
        };
        let data: Vec<u8> = self.input.into();
        let mut value = vec![0; 32];
        self.value.to_big_endian(&mut value);
        Transaction {
            version: 0,
            to,
            nonce: rand::random::<u64>().to_string(),
            quota: self.gas,
            valid_until_block: 0,
            data,
            value,
            chain_id: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EthBlockTransaction {
    Full(EthRpcTransaction),
    Hash(H256),
}

impl From<BlockTransaction> for EthBlockTransaction {
    fn from(origin: BlockTransaction) -> Self {
        match origin {
            BlockTransaction::Hash(hash) => EthBlockTransaction::Hash(hash),
            BlockTransaction::Full(full_tx) => {
                let content: Vec<u8> = full_tx.content.into();
                let tx: UnverifiedTransaction = parse_from_bytes(&content).unwrap();
                let mut eth_tx = EthRpcTransaction::from(tx);
                eth_tx.hash = full_tx.hash;
                EthBlockTransaction::Full(eth_tx)
            }
        }
    }
}

impl From<RawTransaction> for EthBlockTransaction {
    fn from(origin: RawTransaction) -> Self {
        match origin.tx.unwrap() {
            raw_transaction::Tx::NormalTx(tx) => {
                let orin_tx = tx.transaction.unwrap();
                let witness = tx.witness.unwrap();
                let sig = Signature::from(&witness.signature);
                let (v, r, s) = match sig {
                    Signature::Secp256k1(sig) => (
                        U256::from(sig.v()),
                        U256::from_big_endian(&sig.r()),
                        U256::from_big_endian(&sig.s()),
                    ),
                    Signature::Sm2(sig) => (
                        U256::from(2), // no use recovery id, input a wrong number
                        U256::from_big_endian(&sig.r()),
                        U256::from_big_endian(&sig.s()),
                    ),
                    Signature::Null => panic!("null signature"),
                };
                let to = if orin_tx.to.len() == 20 {
                    Some(Address::from_slice(orin_tx.to.as_slice()))
                } else {
                    None
                };
                EthBlockTransaction::Full(EthRpcTransaction {
                    block_hash: Default::default(),
                    block_number: Default::default(),
                    from: Address::from_slice(&witness.sender),
                    gas: orin_tx.quota,
                    gas_price: U256::zero(),
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    hash: H256::from_slice(tx.transaction_hash.as_slice()),
                    input: Data::new(orin_tx.data),
                    nonce: 0,
                    to,
                    transaction_index: 0,
                    value: U256::from_big_endian(&orin_tx.value),
                    type_: 0,
                    access_list: None,
                    chain_id: Some(U256::from_big_endian(&orin_tx.chain_id)),
                    v, // no use recovery id, input a wrong number
                    r,
                    s,
                })
            }
            raw_transaction::Tx::UtxoTx(utxo) => {
                let utxo_tx = utxo.transaction.unwrap();
                let witness = utxo.witnesses[0].clone();
                let sig = Signature::from(&witness.signature);
                let (v, r, s) = match sig {
                    Signature::Secp256k1(sig) => (
                        U256::from(sig.v()),
                        U256::from_big_endian(&sig.r()),
                        U256::from_big_endian(&sig.s()),
                    ),
                    Signature::Sm2(sig) => (
                        U256::from(2), // no use recovery id, input a wrong number
                        U256::from_big_endian(&sig.r()),
                        U256::from_big_endian(&sig.s()),
                    ),
                    Signature::Null => panic!("null signature"),
                };
                EthBlockTransaction::Full(EthRpcTransaction {
                    block_hash: H256::from_slice(utxo_tx.pre_tx_hash.as_slice()),
                    block_number: U256::from(utxo_tx.lock_id),
                    from: Address::from_slice(&witness.sender),
                    gas: Default::default(),
                    gas_price: U256::zero(),
                    max_fee_per_gas: None,
                    max_priority_fee_per_gas: None,
                    hash: H256::from_slice(utxo.transaction_hash.as_slice()),
                    input: Data::new(utxo_tx.output),
                    nonce: 0,
                    to: Default::default(),
                    transaction_index: 0,
                    value: Default::default(),
                    type_: 0,
                    access_list: None,
                    chain_id: Default::default(),
                    v, // no use recovery id, input a wrong number
                    r,
                    s,
                })
            }
        }
    }
}
