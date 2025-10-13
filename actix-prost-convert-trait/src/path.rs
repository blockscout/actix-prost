use crate::{impl_try_convert_from_string, TryConvert};
use std::path::PathBuf;

impl_try_convert_from_string!(PathBuf, "path");

impl TryConvert<PathBuf> for String {
    fn try_convert(input: PathBuf) -> Result<Self, String> {
        input
            .into_os_string()
            .into_string()
            .map_err(|_| "Path contains invalid UTF-8".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_path() {
        let path = PathBuf::try_convert("test.txt".to_string()).unwrap();
        assert_eq!(path, PathBuf::from("test.txt"));

        let path = PathBuf::try_convert("/test.txt".to_string()).unwrap();
        assert_eq!(path, PathBuf::from("/test.txt"));
    }
}
