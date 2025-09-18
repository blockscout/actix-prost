use crate::{impl_try_convert_from_string, impl_try_convert_to_string, TryConvert};
use uuid::Uuid;

impl_try_convert_from_string!(Uuid);
impl_try_convert_to_string!(Uuid);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_uuid() {
        let uuid = Uuid::try_convert("123e4567-e89b-12d3-a456-426614174000".to_string()).unwrap();
        assert_eq!(
            uuid,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap()
        );
    }
}
