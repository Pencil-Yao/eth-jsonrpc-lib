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

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Error;

/// A unsigned integer (wrapper structure around u64).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Integer(pub u64);

impl Integer {
    pub fn new(data: u64) -> Integer {
        Integer(data)
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IntegerVisitor)
    }
}

struct IntegerVisitor;

impl<'de> Visitor<'de> for IntegerVisitor {
    type Value = Integer;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("Integer")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.len() > 2 && (&value[0..2] == "0x" || &value[0..2] == "0X") {
            let data = u64::from_str_radix(&value[2..], 16).map_err(|_| {
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
            Ok(Integer::new(data))
        } else if !value.is_empty() {
            let data = u64::from_str_radix(&value, 10).map_err(|_| {
                if value.len() > 12 {
                    E::custom(format!(
                        "invalid decimal string: [{}..(omit {})..{}]",
                        &value[..6],
                        value.len() - 12,
                        &value[value.len() - 6..value.len()]
                    ))
                } else {
                    E::custom(format!("invalid decimal string: [{}]", value))
                }
            })?;
            Ok(Integer::new(data))
        } else {
            Err(E::custom("invalid input: string is empty".to_string()))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl From<u64> for Integer {
    fn from(data: u64) -> Integer {
        Integer::new(data)
    }
}

impl Into<u64> for Integer {
    fn into(self) -> u64 {
        self.0
    }
}

pub trait LowerUint {
    fn max_value() -> u64;
    fn from(i: Integer) -> Self;
}

macro_rules! impl_convert_with_uint {
    ($uint:ident) => {
        impl From<$uint> for Integer {
            fn from(data: $uint) -> Integer {
                Integer::new(u64::from(data))
            }
        }

        impl LowerUint for $uint {
            fn max_value() -> u64 {
                u64::from($uint::max_value())
            }

            fn from(i: Integer) -> Self {
                i.0 as $uint
            }
        }
    };
}

impl Integer {
    pub fn try_into_uint<T: LowerUint>(self) -> Result<T, Error> {
        if self.0 > T::max_value() {
            Err(Error::parse_error_with_message("Integer truncated"))
        } else {
            Ok(T::from(self))
        }
    }
}

impl_convert_with_uint!(u32);
impl_convert_with_uint!(u16);
impl_convert_with_uint!(u8);

#[cfg(test)]
mod tests {
    use super::Integer;
    use serde_json;
    use std::convert::Into;

    #[test]
    fn serialize() {
        let data = Integer::new(1_311_768_467_463_790_320u64);
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"1311768467463790320"#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""""#, None),
            (r#""0""#, None),
            (r#""10""#, None),
            (r#""#, None),
            (r#"a"#, None),
            (r#"0"#, Some(0u64)),
            (r#"10"#, Some(10u64)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<Integer, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), Integer::new(expected));
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn convert() {
        let auint: u8 = 123;
        let data = Integer::new(auint as u64);
        assert_eq!(data, (auint as u32).into());
        assert_eq!(data, (auint as u16).into());
        assert_eq!(data, (auint as u8).into());
        let expected: u32 = data.clone().try_into_uint::<u32>().unwrap();
        assert_eq!(expected, 123);
        let expected: u16 = data.clone().try_into_uint::<u16>().unwrap();
        assert_eq!(expected, 123);
        let expected: u8 = data.clone().try_into_uint::<u8>().unwrap();
        assert_eq!(expected, 123);
    }
}
