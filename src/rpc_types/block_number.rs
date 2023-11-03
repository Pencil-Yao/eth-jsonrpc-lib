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

use crate::rpc_types::{BlockTag, Data32, Quantity};
use cita_cloud_proto::evm::{self, block_number::Lable};
use cita_tool::H256;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum BlockNumber {
    /// Block Tag
    Tag(BlockTag),
    /// Block Hash
    Hash(Data32),
    /// Height
    Height(Quantity),
}

impl BlockNumber {
    pub fn new(height: Quantity) -> Self {
        BlockNumber::Height(height)
    }
    pub fn latest() -> Self {
        BlockNumber::Tag(BlockTag::Latest)
    }
    pub fn earliest() -> Self {
        BlockNumber::Tag(BlockTag::Earliest)
    }
    pub fn pending() -> Self {
        BlockNumber::Tag(BlockTag::Pending)
    }
    pub fn is_default(&self) -> bool {
        *self == BlockNumber::Tag(BlockTag::Latest)
    }
}

impl Default for BlockNumber {
    fn default() -> Self {
        BlockNumber::Tag(BlockTag::Latest)
    }
}

impl Display for BlockNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BlockNumber::Tag(tag) => write!(f, "{tag}"),
            BlockNumber::Height(height) => write!(f, "{height}"),
            BlockNumber::Hash(hash) => {
                let hash: H256 = hash.clone().into();
                write!(f, "{hash:?}")
            }
        }
    }
}

impl Into<evm::BlockNumber> for BlockNumber {
    fn into(self) -> evm::BlockNumber {
        match self {
            BlockNumber::Tag(tag) => match tag {
                BlockTag::Latest | BlockTag::Safe | BlockTag::Finalized => evm::BlockNumber {
                    lable: Some(Lable::Tag("latest".to_string())),
                },
                BlockTag::Earliest => evm::BlockNumber {
                    lable: Some(Lable::Tag("earliest".to_string())),
                },
                BlockTag::Pending => evm::BlockNumber {
                    lable: Some(Lable::Tag("pending".to_string())),
                },
            },
            BlockNumber::Height(height) => evm::BlockNumber {
                lable: Some(Lable::Height(height.0.low_u64())),
            },
            BlockNumber::Hash(hash) => {
                let hash: H256 = hash.into();
                evm::BlockNumber {
                    lable: Some(Lable::Hash(hash.0.to_vec())),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BlockNumber;
    use serde_json;

    #[test]
    fn serialize() {
        let testdata = vec![
            (BlockNumber::new(10u64.into()), Some(r#""0xa""#)),
            (BlockNumber::new(16u64.into()), Some(r#""0x10""#)),
            (BlockNumber::latest(), Some(r#""latest""#)),
            (BlockNumber::earliest(), Some(r#""earliest""#)),
            (BlockNumber::pending(), Some(r#""pending""#)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result = serde_json::to_string(&data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""a""#, None),
            (r#""0xg""#, None),
            (r#"0xa"#, None),
            (r#"10"#, None),
            (r#"latest"#, None),
            (r#"earliest"#, None),
            (r#""latest""#, Some(BlockNumber::latest())),
            (r#""earliest""#, Some(BlockNumber::earliest())),
            (r#""pending""#, Some(BlockNumber::pending())),
            (r#""10""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0x10""#, Some(BlockNumber::new(16u64.into()))),
            (r#""0xa""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0xA""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0Xa""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0XA""#, Some(BlockNumber::new(10u64.into()))),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<BlockNumber, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
