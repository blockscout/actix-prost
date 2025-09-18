use crate::{impl_try_convert_from_string, impl_try_convert_to_string, TryConvert};
use url::Url;

impl_try_convert_from_string!(Url);
impl_try_convert_to_string!(Url);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_url() {
        let url = Url::try_convert("https://www.google.com".to_string()).unwrap();
        assert_eq!(url, Url::parse("https://www.google.com").unwrap());
    }
}
