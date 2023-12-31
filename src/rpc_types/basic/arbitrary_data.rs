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

use crate::rpc_types::basic::utils::LowerHex;
use rustc_serialize::hex::FromHex;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Arbitrary length bytes (wrapper structure around vector of bytes).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Data(Vec<u8>);

impl Data {
    pub fn new(data: Vec<u8>) -> Data {
        Data(data)
    }
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.lower_hex_with_0x().as_ref())
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = Data;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("Data")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.is_empty() {
            Ok(Data::new(Vec::new()))
        } else if value.len() >= 2
            && (&value[0..2] == "0x" || &value[0..2] == "0X")
            && value.len() & 1 == 0
        {
            let data = FromHex::from_hex(&value[2..]).map_err(|_| {
                if value.len() > 12 {
                    E::custom(format!(
                        "invalid hexadecimal string: [{}..(omit {})..{}]",
                        &value[..6],
                        value.len() - 12,
                        &value[value.len() - 6..value.len()]
                    ))
                } else {
                    E::custom(format!("invalid hexadecimal string: [{}]", value))
                }
            })?;
            Ok(Data::new(data))
        } else if value.len() > 12 {
            Err(E::custom(format!(
                "invalid format: [{}..(omit {})..{}]",
                &value[..6],
                value.len() - 12,
                &value[value.len() - 6..value.len()]
            )))
        } else {
            Err(E::custom(format!("invalid format: [{}]", value)))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Data {
        Data::new(data)
    }
}

impl Into<Vec<u8>> for Data {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Data;
    use rustc_serialize::hex::FromHex;
    use serde_json;

    #[test]
    fn serialize() {
        let data = Data::new("0123456789abcdef".from_hex().unwrap());
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#""0x0123456789abcdef""#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""12""#, None),
            (r#""ab""#, None),
            (r#""0x123""#, None),
            (r#""0xabcdefgh""#, None),
            (r#""""#, Some(Data::new(vec![]))),
            (r#""0x""#, Some(Data::new(vec![]))),
            (r#""0x12""#, Some(Data::new(vec![0x12]))),
            (r#""0X12""#, Some(Data::new(vec![0x12]))),
            (r#""0x0123""#, Some(Data::new(vec![0x1, 0x23]))),
            (r#""0xabcdef""#, Some(Data::new(vec![0xab, 0xcd, 0xef]))),
            (r#""0XABCDEF""#, Some(Data::new(vec![0xab, 0xcd, 0xef]))),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<Data, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
