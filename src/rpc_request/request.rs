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
#![allow(non_camel_case_types)]

use crate::internals::construct_params;
use crate::rpc_types::ethereum_types::{
    EthBlock, EthCallRequest, EthFilter, EthLog, EthReceipt, EthRpcTransaction,
    EthTransactionRequest,
};
use crate::rpc_types::{
    Block, BlockNumber, Boolean, CallRequest, CallResult, CensorAddrs, Data, Data20, Data32,
    Filter, FilterChanges, Id, Integer, LicenseInfo, Log, MetaData, OneItemTupleTrick, PeersInfo,
    PoolTxNum, Quantity, Receipt, RpcTransaction, SoftwareVersion, TxResponse, Version,
};
/// JSON-RPC Request.
use serde_json;

pub type Logs = Vec<Log>;
pub type EthLogs = Vec<EthLog>;
pub type Accounts = Vec<Data20>;

#[derive(Debug, Clone, PartialEq)]
pub struct RequestInfo {
    pub jsonrpc: Option<Version>,
    pub id: Id,
}

impl RequestInfo {
    pub fn new(jsonrpc: Option<Version>, id: Id) -> Self {
        RequestInfo { jsonrpc, id }
    }
    pub fn null() -> Self {
        RequestInfo {
            jsonrpc: None,
            id: Id::Null,
        }
    }
}

impl Default for RequestInfo {
    fn default() -> Self {
        RequestInfo::new(Some(Version::default()), Id::default())
    }
}

/// JSON-RPC 2.0 Request object (http://www.jsonrpc.org/specification#request_object)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jsonrpc: Option<Version>,
    #[serde(default, skip_serializing_if = "Id::is_null")]
    pub id: Id,
    /// Contain method and params.
    #[serde(flatten)]
    pub call: Call,
}

impl Request {
    pub fn new(jsonrpc: Option<Version>, id: Id, call: Call) -> Self {
        Request { jsonrpc, id, call }
    }
    pub fn get_method(&self) -> &str {
        self.call.get_method()
    }
    pub fn get_info(&self) -> RequestInfo {
        RequestInfo::new(self.jsonrpc.clone(), self.id.clone())
    }
}

impl Into<String> for Request {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PartialRequest {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    /// Contain method and params.
    #[serde(flatten)]
    pub call: Option<PartialCall>,
}

impl PartialRequest {
    pub fn get_info(&self) -> RequestInfo {
        RequestInfo::new(self.jsonrpc.clone(), self.id.clone())
    }
}

macro_rules! define_call {
    ($( ($enum_name:ident, $params_name:ident: $params_list:expr, $result_type:ident) ),+ ,) => {
        define_call!($( ($enum_name, $params_name: $params_list, $result_type) ),+);
    };
    ($( ($enum_name:ident, $params_name:ident: $params_list:expr, $result_type:ident) ),+ ) => {

        $(
            construct_params!($params_name: $params_list, $result_type);
        )+


        #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
        #[serde(untagged)]
        #[allow(clippy::large_enum_variant)]   // TODO
        pub enum ResponseResult {
            #[serde(rename = "null")]
            Null,
            $(
                $enum_name($result_type),
            )+
        }

        impl Default for ResponseResult {
            fn default() -> Self {
                ResponseResult::Null
            }
        }

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method", rename_all = "camelCase")]
        pub enum Call {
            $(
                $enum_name { params: $params_name},
            )+
        }

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method", rename_all = "camelCase")]
        pub enum PartialCall {
            $(
                $enum_name {
                    params: Option<serde_json::Value>
                },
            )+
        }

        impl Call {
            pub fn get_method(&self) -> &str {
                match self {
                    $(
                        Call::$enum_name { ref params } => params.method_name(),
                    )+
                }
            }
            pub fn into_request(self, id: u64) -> Request {
                Request::new(
                    Some(Version::default()),
                    Id::Num(id),
                    self,
                )
            }
        }

        $(
            impl Into<$params_name> for Call {
                fn into(self) -> $params_name{
                    if let Call::$enum_name{ params } = self {
                        params
                    } else {
                        // IMHO, in Rust, no static check can do this.
                        // If https://github.com/rust-lang/rfcs/pull/1450 merged,
                        // I think I can remove this panic.
                        panic!("The method and params are one to one correspondence.")
                    }
                }
            }

            impl From<$params_name> for Call {
                fn from(params: $params_name) -> Call {
                    Call::$enum_name{ params }
                }
            }

            impl $params_name {
                pub fn into_request(self, id: u64) -> Request {
                    Request::new(
                        Some(Version::default()),
                        Id::Num(id),
                        self.into(),
                    )
                }
            }
        )+
    };
}

pub trait JsonRpcRequest {
    type Response;
    fn required_len() -> usize;
    fn valid_len() -> usize;
    fn method_name(&self) -> &'static str;
    fn value_vec(self) -> Vec<serde_json::Value>;
}

// Q. How to add a JSON-RPC method?
//
// A.
//  First, add a tuple into the follow macro.
//
//    - The 1st item in tuple is a enum name used in `Call` / `PartialCall`.
//      The enum name will used to generate the JSON-RPC method name.
//      The enum name is PascalCase and JSON-RPC method name is camelCase.
//
//    - The 2st item is params type name and it's structure.
//      The params type has some methods, such as `new()` and `method_name()`.
//      More details can found in the definition of `construct_params`.
//
//    - The 3rd item is the type of result of Response object on success.
//
//  Second, implement `TryInto<ProtoRequest>` for the new params type.
//
//  DONE!
#[macro_export]
macro_rules! impl_for_each_jsonrpc_requests {
    ($macro:ident) => {
        $macro!(
            (BlockNumber, BlockNumberParams: [], Quantity),
            (PeerCount, PeerCountParams: [], Quantity),
            (SendRawTransaction, SendRawTransactionParams: [Data], TxResponse),
            (SendTransaction, SendTransactionParams: [Data], TxResponse),
            (GetBlockByHash, GetBlockByHashParams: [Data32, Boolean], Block),
            (GetBlockByNumber, GetBlockByNumberParams: [BlockNumber, Boolean], Block),
            (GetTransactionReceipt, GetTransactionReceiptParams: [Data32], Receipt),
            (GetLogs, GetLogsParams: [Filter], Logs),
            (GetTransactionCount, GetTransactionCountParams: [Data20, BlockNumber], Quantity),
            (GetCode, GetCodeParams: [Data20, BlockNumber], Data),
            (GetAbi, GetAbiParams: [Data20, BlockNumber], Data),
            (GetBalance, GetBalanceParams: [Data20, BlockNumber], Quantity),
            (NewFilter, NewFilterParams: [Filter], Quantity),
            (NewBlockFilter, NewBlockFilterParams: [], Quantity),
            (UninstallFilter, UninstallFilterParams: [Quantity], Boolean),
            (GetFilterChanges, GetFilterChangesParams: [Quantity], FilterChanges),
            (GetFilterLogs, GetFilterLogsParams: [Quantity], Logs),
            (GetTransactionProof, GetTransactionProofParams: [Data32], Data),
            (GetMetaData, GetMetaDataParams: [BlockNumber], MetaData),
            (GetStateProof, GetStateProofParams: [Data20, Data32, BlockNumber], Data),
            (GetBlockHeader, GetBlockHeaderParams: [BlockNumber], Data),
            (GetStorageAt, GetStorageKeyParams: [Data20, Data32, BlockNumber], Data),
            (GetVersion, GetVersionParams: [], SoftwareVersion),
            (EstimateQuota, EstimateQuotaParams: [CallRequest, BlockNumber], Quantity),
            (LicenseInfo, LicenseInfoParams: [], LicenseInfo),
            (GetPoolTxNum, GetPoolTxNumParams: [], PoolTxNum),
            (OpCensoredAddress, OpCensoredAddressParams: [Integer, Data20], Boolean),
            (PeersInfo, PeersInfoParams: [
                #[serde(default)]
                Boolean
            ], PeersInfo),
            (GetTransaction, GetTransactionParams: [
                Data32,
                #[serde(default)]
                Boolean
            ], RpcTransaction),
            (Call, CallParams: [
                CallRequest,
                BlockNumber,
                #[serde(default)]
                Boolean
            ], CallResult),
            (GetCensoredAddrs, GetCensoredAddrsParams: [], CensorAddrs),
            // ethereum jsonrpc
            (eth_blockNumber, eth_blockNumberParams: [], Quantity),
            (eth_chainId, eth_chainIdParams: [], Quantity),
            (eth_getBlockByHash, eth_getBlockByHashParams: [Data32, Boolean], EthBlock),
            (eth_getBlockByNumber, eth_getBlockByNumberParams: [BlockNumber, Boolean], EthBlock),
            (eth_getTransactionByHash, eth_getTransactionByHashParams: [Data32], EthRpcTransaction),
            (eth_getTransactionByBlockHashAndIndex, eth_getTransactionByBlockHashAndIndexParams: [Data32, Integer], EthRpcTransaction),
            (eth_getTransactionByBlockNumberAndIndex, eth_getTransactionByBlockNumberAndIndexParams: [BlockNumber, Integer], EthRpcTransaction),
            (eth_getBlockTransactionCountByHash, eth_getBlockTransactionCountByHashParams: [Data32], Integer),
            (eth_getBlockTransactionCountByNumber, eth_getBlockTransactionCountByNumberParams: [BlockNumber], Integer),
            (eth_getTransactionReceipt, eth_getTransactionReceiptParams: [Data32], EthReceipt),
            (eth_getBalance, eth_getBalanceParams: [Data20, BlockNumber], Quantity),
            (eth_syncing, eth_syncingParams: [], Boolean),
            (eth_getStorageAt, eth_getStorageAtParams: [Data20, Quantity, BlockNumber], Data),
            (eth_getCode, eth_getCodeParams: [Data20, BlockNumber], Data),
            (eth_getTransactionCount, eth_getTransactionCountParams: [Data20, BlockNumber], Quantity),
            (eth_getLogs, eth_getLogsParams: [EthFilter], EthLogs),
            (eth_call, eth_callParams: [EthCallRequest, BlockNumber], Data),
            (eth_estimateGas, eth_estimateGasParams: [
                EthCallRequest,
                #[serde(default)]
                BlockNumber
            ], Quantity),
            (eth_gasPrice, eth_gasPriceParams: [], Quantity),
            (eth_maxPriorityFeePerGas, eth_maxPriorityFeePerGasParams: [], Quantity),
            (eth_sendTransaction, eth_sendTransactionParams: [EthTransactionRequest], Data32),
            (eth_sendRawTransaction, eth_sendRawTransactionParams: [Data], Data32),
            (eth_accounts, eth_accountsParams: [], Accounts),
            // net jsonrpc
            (net_version, net_versionParams: [], Integer),
        );
    };
}

impl_for_each_jsonrpc_requests!(define_call);
