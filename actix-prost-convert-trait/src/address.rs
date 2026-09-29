use crate::{impl_try_convert_from_string, TryConvert};
use alloy::primitives::Address;

impl_try_convert_from_string!(Address);

impl TryConvert<Address> for String {
    fn try_convert(input: Address) -> Result<Self, String> {
        Ok(input.to_checksum(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conversion_address() {
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
