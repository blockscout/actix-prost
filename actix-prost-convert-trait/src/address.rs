use crate::{impl_try_convert_from_string, TryConvert};

#[cfg(feature = "conv-address")]
mod alloy_address {
    use super::*;
    use alloy::primitives::Address;

    impl_try_convert_from_string!(Address);

    impl TryConvert<Address> for String {
        fn try_convert(input: Address) -> Result<Self, String> {
            Ok(input.to_checksum(None))
        }
    }
}

#[cfg(feature = "conv-address-ethers")]
mod ethers_address {
    use super::*;
    use ethers_core::{types::Address, utils::to_checksum};

    impl_try_convert_from_string!(Address);

    impl TryConvert<Address> for String {
        fn try_convert(input: Address) -> Result<Self, String> {
            Ok(to_checksum(&input, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[cfg(feature = "conv-address")]
    #[test]
    fn test_conversion_address() {
        use alloy::primitives::Address;

        let expected = "0x1234567890123456789012345678901234567890"
            .parse::<Address>()
            .unwrap();
        let address =
            Address::try_convert("0x1234567890123456789012345678901234567890".to_string()).unwrap();
        assert_eq!(address, expected);

        let address =
            Address::try_convert("1234567890123456789012345678901234567890".to_string()).unwrap();
        assert_eq!(address, expected);
    }

    #[cfg(feature = "conv-address-ethers")]
    #[test]
    fn test_conversion_ethers_address() {
        use ethers_core::types::Address;

        let expected = "0x1234567890123456789012345678901234567890"
            .parse::<Address>()
            .unwrap();
        let address =
            Address::try_convert("0x1234567890123456789012345678901234567890".to_string()).unwrap();
        assert_eq!(address, expected);

        let address =
            Address::try_convert("1234567890123456789012345678901234567890".to_string()).unwrap();
        assert_eq!(address, expected);
    }
}
