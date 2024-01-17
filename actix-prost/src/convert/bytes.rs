
use crate::convert::Convert;
use bytes::Bytes;

impl Convert<String> for Bytes {
    fn convert(input: String) -> anyhow::Result<Self> {
        if let Some(input) = input.strip_prefix("0x") {
            hex::decode(input)
        } else {
            hex::decode(input)
        }
        .map(Into::into)
        .map_err(|e| anyhow::anyhow!("Invalid hex: {e}"))
    }
}
