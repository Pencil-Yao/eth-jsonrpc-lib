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

pub use ethereum_types::{Address, H128, H160, H256, H264, H32, H512, H520, H64};
pub use ethereum_types::{Bloom, BloomInput, BloomRef};
pub use ethereum_types::{U128, U256, U512, U64};
use std::str::FromStr;

#[inline]
pub fn pad_left0(hexstr: &str, width: usize) -> String {
    format!("{:0>width$}", hexstr, width = width)
}

#[inline]
pub fn delete_left0(s: &str) -> &str {
    let mut cnt = 0;
    for i in s.chars() {
        if i == '0' {
            cnt += 1;
        } else {
            break;
        }
    }
    &s[cnt..]
}

#[inline]
pub fn clean_0x(s: &str) -> &str {
    if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    }
}

pub trait LowerHex {
    fn lower_hex(&self) -> String;
    fn lower_hex_with_0x(&self) -> String;
}

pub trait ConvertType
where
    Self: ::std::marker::Sized,
{
    fn from_unaligned(_: &str) -> Option<Self>;
}

fn convert_u8_to_char(u: u8) -> char {
    match u {
        u if u < 10 => (b'0' + u) as char,
        u if 10 <= u && u < 16 => (b'a' + (u - 10)) as char,
        _ => panic!("{} not bwtween 0 and 15", u),
    }
}

impl<'a> LowerHex for &'a [u8] {
    fn lower_hex(&self) -> String {
        let sz = self.len();
        let mut s = vec![0u8 as char; sz * 2];
        for i in 0..sz {
            s[i * 2] = convert_u8_to_char(self[i] / 16);
            s[i * 2 + 1] = convert_u8_to_char(self[i] % 16);
        }
        s.into_iter().collect()
    }

    fn lower_hex_with_0x(&self) -> String {
        let sz = self.len();
        let mut s = vec![0u8 as char; sz * 2 + 2];
        s[0] = '0';
        s[1] = 'x';
        for i in 0..sz {
            s[i * 2 + 2] = convert_u8_to_char(self[i] / 16);
            s[i * 2 + 3] = convert_u8_to_char(self[i] % 16);
        }
        s.into_iter().collect()
    }
}

impl LowerHex for Vec<u8> {
    fn lower_hex(&self) -> String {
        let s = self as &[u8];
        s.lower_hex()
    }
    fn lower_hex_with_0x(&self) -> String {
        let s = self as &[u8];
        s.lower_hex_with_0x()
    }
}

macro_rules! add_cita_funcs {
    ([$( ($name:ident, $bits:expr) ),+ ,]) => {
        add_cita_funcs!([ $( ($name, $bits) ),+ ]);
    };
    ([$( ($name:ident, $bits:expr) ),+]) => {
        $( add_cita_funcs!($name, $bits); )+
    };
    ($name:ident, $bits:expr) => {
        impl LowerHex for $name {
            #[inline]
            fn lower_hex(&self) -> String {
                format!("{:x}", self)
            }

            #[inline]
            fn lower_hex_with_0x(&self) -> String {
                // TODO: https://github.com/paritytech/primitives/pull/37
                format!("0x{:x}", self)
            }
        }

        impl ConvertType for $name {
            #[inline]
            fn from_unaligned(s: &str) -> Option<Self> {
                $name::from_str(
                    pad_left0(
                        delete_left0(clean_0x(s)), $bits/4usize).as_str())
                    .ok()
            }
        }
    }
}

add_cita_funcs!([
    (Bloom, 2048),
    (H32, 32),
    (H64, 64),
    (H128, 128),
    (H160, 160),
    (H256, 256),
    (H264, 264),
    (H512, 512),
    (H520, 520),
    (U64, 64),
    (U128, 128),
    (U256, 256),
    (U512, 512),
]);
