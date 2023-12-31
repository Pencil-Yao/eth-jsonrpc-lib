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

use crate::rpc_request::{
    eth_accountsParams, eth_blockNumberParams, eth_callParams, eth_chainIdParams,
    eth_estimateGasParams, eth_gasPriceParams, eth_getBalanceParams, eth_getBlockByHashParams,
    eth_getBlockByNumberParams, eth_getBlockTransactionCountByHashParams,
    eth_getBlockTransactionCountByNumberParams, eth_getCodeParams, eth_getLogsParams,
    eth_getStorageAtParams, eth_getTransactionByBlockHashAndIndexParams,
    eth_getTransactionByBlockNumberAndIndexParams, eth_getTransactionByHashParams,
    eth_getTransactionCountParams, eth_getTransactionReceiptParams, eth_maxPriorityFeePerGasParams,
    eth_sendRawTransactionParams, eth_sendTransactionParams, eth_syncingParams, net_versionParams,
    BlockNumberParams, CallParams, EstimateQuotaParams, GetAbiParams, GetBalanceParams,
    GetBlockByHashParams, GetBlockByNumberParams, GetBlockHeaderParams, GetCensoredAddrsParams,
    GetCodeParams, GetFilterChangesParams, GetFilterLogsParams, GetLogsParams, GetMetaDataParams,
    GetPoolTxNumParams, GetStateProofParams, GetStorageKeyParams, GetTransactionCountParams,
    GetTransactionParams, GetTransactionProofParams, GetTransactionReceiptParams, GetVersionParams,
    LicenseInfoParams, NewBlockFilterParams, NewFilterParams, OpCensoredAddressParams,
    PeerCountParams, PeersInfoParams, SendRawTransactionParams, SendTransactionParams,
    UninstallFilterParams,
};
use crate::rpc_request::{Call, JsonRpcRequest, PartialCall, PartialRequest, Request};
use crate::{impl_for_each_jsonrpc_requests, rpc_types::Params as PartialParams, Error};
use serde_json;

pub trait Complete {
    type Output;
    type Error;

    fn complete(self) -> Result<Self::Output, Self::Error>;
}

impl Complete for PartialRequest {
    type Output = Request;
    type Error = Error;

    fn complete(self) -> Result<Self::Output, Self::Error> {
        let PartialRequest { jsonrpc, id, call } = self;
        if let Some(part_call) = call {
            part_call
                .complete()
                .map(|full_call| Request::new(jsonrpc, id, full_call))
        } else {
            Err(Error::method_not_found())
        }
    }
}

macro_rules! partial_call_complete {
    ($( ($enum_name:ident, $params_name:ident: $params_list:expr, $result_type:ident) ),+ ,) => {
        partial_call_complete!($( ($enum_name, $params_name) ),+);
    };
    ($( ($enum_name:ident, $params_name:ident) ),+) => {
        impl Complete for PartialCall {
            type Output = Call;
            type Error = Error;

            fn complete(self) -> Result<Self::Output, Self::Error> {
                match self {
                    $(
                        PartialCall::$enum_name { params } => {
                            if let Some(params) = params {
                                let pparams: PartialParams = serde_json::from_value(params.clone())?;
                                if pparams.len() < $params_name::required_len()
                                    && pparams.len() > $params_name::valid_len() {
                                    Err(Error::invalid_params_len())
                                } else {
                                    Ok(Call::$enum_name{ params: serde_json::from_value(params)? })
                                }
                            } else {
                                if $params_name::required_len() == 0 {
                                    Ok(Call::$enum_name{
                                        params: serde_json::from_value(
                                                    serde_json::Value::Array(Vec::new()))?})
                                } else {
                                    Err(Error::invalid_params("params is requeired"))
                                }
                            }
                        },
                    )+
                }
            }
        }
    }
}

impl_for_each_jsonrpc_requests!(partial_call_complete);

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::H256;

    #[test]
    fn test_get_transaction_receipt_params_complete() {
        let params = GetTransactionReceiptParams::new(H256::from(10).into());
        let full_req = params.into_request(1);

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(part_req.complete().unwrap(), full_req);
    }

    #[test]
    fn test_get_transaction_receipt_params_complete_error() {
        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt"
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();

        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::invalid_params("params is requeired")
        );

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": [1, 2]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::invalid_params_len()
        );
    }

    #[test]
    fn test_block_number_params_complete() {
        let params = eth_blockNumberParams::new();
        let full_req = params.into_request(2);

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": 2,
            "method": "blockNumber"
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(part_req.complete().unwrap(), full_req);
    }

    #[test]
    fn test_method_not_found_complete_error() {
        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();

        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::method_not_found()
        );

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "notAMethod",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::method_not_found()
        );
    }
}
