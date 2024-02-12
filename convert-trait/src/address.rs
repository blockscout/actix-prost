use std::str::FromStr;

use crate::TryConvert;
use ethers_core::{types::Address, utils::to_checksum};

impl TryConvert<Address> for String {
    fn try_convert(input: Address) -> Result<Self, String> {
        Ok(to_checksum(&input, None))
    }
}

impl TryConvert<String> for Address {
    fn try_convert(input: String) -> Result<Self, String> {
        Address::from_str(&input).map_err(|e| format!("Invalid address: {e:?}"))
    }
}
