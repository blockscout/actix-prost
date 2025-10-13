use crate::TryConvert;
use bytes::Bytes;

impl TryConvert<String> for Bytes {
    fn try_convert(input: String) -> Result<Self, String> {
        if let Some(input) = input.strip_prefix("0x") {
            hex::decode(input)
        } else {
            hex::decode(input)
        }
        .map(Into::into)
        .map_err(|e| format!("Invalid hex: {e}"))
    }
}

impl TryConvert<Bytes> for String {
    fn try_convert(input: Bytes) -> Result<Self, String> {
        Ok(format!("0x{}", hex::encode(input)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conversion_bytes() {
        let bytes = Bytes::try_convert("0x1234".to_string()).unwrap();
        assert_eq!(bytes, Bytes::from_iter([0x12, 0x34]));

        let string = String::try_convert(bytes).unwrap();
        assert_eq!(string, "0x1234");
    }
}
