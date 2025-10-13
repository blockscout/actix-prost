use crate::{impl_try_convert_from_string, TryConvert};
use ethers_core::{types::Address, utils::to_checksum};

impl_try_convert_from_string!(Address);

impl TryConvert<Address> for String {
    fn try_convert(input: Address) -> Result<Self, String> {
        Ok(to_checksum(&input, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conversion_address() {
        let address =
            Address::try_convert("0x1234567890123456789012345678901234567890".to_string()).unwrap();
        assert_eq!(
            address,
            "0x1234567890123456789012345678901234567890"
                .parse::<Address>()
                .unwrap()
        );

        let address =
            Address::try_convert("1234567890123456789012345678901234567890".to_string()).unwrap();
        assert_eq!(
            address,
            "0x1234567890123456789012345678901234567890"
                .parse::<Address>()
                .unwrap()
        );
    }
}
