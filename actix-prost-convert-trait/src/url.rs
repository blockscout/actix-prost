use crate::TryConvert;
use url::Url;

impl TryConvert<String> for Url {
    fn try_convert(input: String) -> Result<Self, String> {
        Url::parse(&input).map_err(|e| format!("Invalid URL: {e}"))
    }
}

impl TryConvert<Url> for String {
    fn try_convert(input: Url) -> Result<Self, String> {
        Ok(input.to_string())
    }
}
