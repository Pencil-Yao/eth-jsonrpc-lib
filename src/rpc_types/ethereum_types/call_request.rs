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

use crate::rpc_types::{Data, Data20, Integer, Quantity};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EthCallRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<Quantity>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<Integer>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EthTransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Data20>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<Integer>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<Integer>,
}
