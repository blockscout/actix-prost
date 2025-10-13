use crate::{impl_try_convert_from_string, impl_try_convert_to_string, TryConvert};
use rust_decimal::{prelude::ToPrimitive, Decimal};

impl_try_convert_from_string!(Decimal);
impl_try_convert_to_string!(Decimal);

// Decimal from numeric types
impl TryConvert<i32> for Decimal {
    fn try_convert(input: i32) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for i32 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        if input.is_integer() {
            input.to_i32().ok_or_else(|| {
                crate::failed_to_parse_error_message_with_description(
                    input,
                    "i32",
                    "too big or negative",
                )
            })
        } else {
            Err(crate::failed_to_parse_error_message_with_description(
                input,
                "i32",
                "not an integer",
            ))
        }
    }
}

impl TryConvert<i64> for Decimal {
    fn try_convert(input: i64) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for i64 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        if input.is_integer() {
            input.to_i64().ok_or_else(|| {
                crate::failed_to_parse_error_message_with_description(
                    input,
                    "i64",
                    "too big or negative",
                )
            })
        } else {
            Err(crate::failed_to_parse_error_message_with_description(
                input,
                "i64",
                "not an integer",
            ))
        }
    }
}

impl TryConvert<u32> for Decimal {
    fn try_convert(input: u32) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for u32 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        if input.is_integer() {
            input.to_u32().ok_or_else(|| {
                crate::failed_to_parse_error_message_with_description(input, "u32", "too big")
            })
        } else {
            Err(crate::failed_to_parse_error_message_with_description(
                input,
                "u32",
                "not an integer",
            ))
        }
    }
}

impl TryConvert<u64> for Decimal {
    fn try_convert(input: u64) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for u64 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        if input.is_integer() {
            input.to_u64().ok_or_else(|| {
                crate::failed_to_parse_error_message_with_description(input, "u64", "too big")
            })
        } else {
            Err(crate::failed_to_parse_error_message_with_description(
                input,
                "u64",
                "not an integer",
            ))
        }
    }
}

impl TryConvert<f32> for Decimal {
    fn try_convert(input: f32) -> Result<Self, String> {
        Decimal::try_from(input)
            .map_err(|e| crate::failed_to_parse_error_message_with_description(input, "f32", e))
    }
}

impl TryConvert<Decimal> for f32 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input.to_f32().ok_or_else(|| {
            crate::failed_to_parse_error_message_with_description(input, "f32", "too big")
        })
    }
}

impl TryConvert<f64> for Decimal {
    fn try_convert(input: f64) -> Result<Self, String> {
        Decimal::try_from(input)
            .map_err(|e| crate::failed_to_parse_error_message_with_description(input, "f64", e))
    }
}

impl TryConvert<Decimal> for f64 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input.to_f64().ok_or_else(|| {
            crate::failed_to_parse_error_message_with_description(input, "f64", "too big")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conversion_decimal() {
        let decimal = Decimal::try_convert("123.45".to_string()).unwrap();
        assert_eq!(decimal, "123.45".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123.45f32).unwrap();
        assert_eq!(decimal, "123.45".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123.45f64).unwrap();
        assert_eq!(decimal, "123.45".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123i32).unwrap();
        assert_eq!(decimal, "123".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123i64).unwrap();
        assert_eq!(decimal, "123".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123u32).unwrap();
        assert_eq!(decimal, "123".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123u64).unwrap();
        assert_eq!(decimal, "123".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123f32).unwrap();
        assert_eq!(decimal, "123".parse::<Decimal>().unwrap());

        let decimal = Decimal::try_convert(123f64).unwrap();
        assert_eq!(decimal, "123".parse::<Decimal>().unwrap());

        let error =
            i32::try_convert("123.45".parse::<Decimal>().unwrap()).expect_err("Invalid decimal");
        assert_eq!(error, "failed to parse '123.45' as i32: not an integer");

        let error =
            i64::try_convert("-123.45".parse::<Decimal>().unwrap()).expect_err("Invalid decimal");
        assert_eq!(error, "failed to parse '-123.45' as i64: not an integer");

        let error =
            u32::try_convert("-1".parse::<Decimal>().unwrap()).expect_err("Invalid decimal");
        assert_eq!(error, "failed to parse '-1' as u32: too big");
    }
}
