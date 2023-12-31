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

use std::collections::HashMap;

use ethereum_types::{Address, H256};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Proof {
    Raft,
    Bft(BftProof),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BftProof {
    pub proposal: H256,
    pub height: usize,
    pub round: usize,
    pub commits: HashMap<Address, String>,
}
