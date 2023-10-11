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

use serde::ser::Serialize;
use serde::{Deserialize, Serializer};
use serde_json::Value;

use crate::rpc_types::ethereum_types::EthLog;
use crate::rpc_types::{BlockNumber, Data20, Data32, VariadicValue};

/// Filter Address
pub type FilterAddress = VariadicValue<Data20>;
/// Topic
pub type Topic = VariadicValue<Data32>;

/// Filter
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct EthFilter {
    /// From Block
    #[serde(
        rename = "fromBlock",
        default,
        skip_serializing_if = "BlockNumber::is_default"
    )]
    pub from_block: BlockNumber,
    /// To Block
    #[serde(
        rename = "toBlock",
        default,
        skip_serializing_if = "BlockNumber::is_default"
    )]
    pub to_block: BlockNumber,
    /// Address
    pub address: Option<FilterAddress>,
    /// Topics
    pub topics: Option<Vec<Topic>>,
}

impl EthFilter {
    pub fn new(
        from_block: BlockNumber,
        to_block: BlockNumber,
        address: Option<FilterAddress>,
        topics: Option<Vec<Topic>>,
    ) -> Self {
        Self {
            from_block,
            to_block,
            address,
            topics,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EthFilterChanges {
    /// New logs.
    Logs(Vec<EthLog>),
    /// New hashes (block or transactions)
    Hashes(Vec<Data32>),
    /// Empty result,
    Empty,
}

impl Serialize for EthFilterChanges {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            EthFilterChanges::Logs(ref logs) => logs.serialize(s),
            EthFilterChanges::Hashes(ref hashes) => hashes.serialize(s),
            EthFilterChanges::Empty => (&[] as &[Value]).serialize(s),
        }
    }
}
