use std::str::FromStr;

use crate::Convert;
use ethers_core::{types::Address, utils::to_checksum};

impl Convert<Address> for String {
    fn convert(input: Address) -> Result<Self, String> {
        Ok(to_checksum(&input, None))
    }
}

impl Convert<String> for Address {
    fn convert(input: String) -> Result<Self, String> {
        Address::from_str(&input).map_err(|e| format!("Invalid address: {e:?}"))
    }
}
