use std::str::FromStr;

use crate::convert::Convert;
use ethers_core::{types::Address, utils::to_checksum};

impl Convert<Address> for String {
    fn convert(input: Address) -> anyhow::Result<Self> {
        Ok(to_checksum(&input, None))
    }
}

impl Convert<String> for Address {
    fn convert(input: String) -> anyhow::Result<Self> {
        Address::from_str(&input).map_err(|e| anyhow::anyhow!("Invalid address: {e:?}"))
    }
}
