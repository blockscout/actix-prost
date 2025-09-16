use crate::{impl_try_convert_from_string, impl_try_convert_to_string, TryConvert};
use rust_decimal::{prelude::ToPrimitive, Decimal};

impl_try_convert_from_string!(Decimal, "decimal number");
impl_try_convert_to_string!(Decimal);

// Decimal from numeric types
impl TryConvert<i32> for Decimal {
    fn try_convert(input: i32) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for i32 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input
            .to_i32()
            .ok_or_else(|| "Decimal too large for i32".to_string())
    }
}

impl TryConvert<i64> for Decimal {
    fn try_convert(input: i64) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for i64 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input
            .to_i64()
            .ok_or_else(|| "Decimal too large for i64".to_string())
    }
}

impl TryConvert<u32> for Decimal {
    fn try_convert(input: u32) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for u32 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input
            .to_u32()
            .ok_or_else(|| "Decimal too large for u32 or negative".to_string())
    }
}

impl TryConvert<u64> for Decimal {
    fn try_convert(input: u64) -> Result<Self, String> {
        Ok(Decimal::from(input))
    }
}

impl TryConvert<Decimal> for u64 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input
            .to_u64()
            .ok_or_else(|| "Decimal too large for u64 or negative".to_string())
    }
}

impl TryConvert<f32> for Decimal {
    fn try_convert(input: f32) -> Result<Self, String> {
        Decimal::try_from(input).map_err(|e| format!("Invalid decimal from f32: {}", e))
    }
}

impl TryConvert<Decimal> for f32 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input
            .to_f32()
            .ok_or_else(|| "Decimal cannot be converted to f32".to_string())
    }
}

impl TryConvert<f64> for Decimal {
    fn try_convert(input: f64) -> Result<Self, String> {
        Decimal::try_from(input).map_err(|e| format!("Invalid decimal from f64: {}", e))
    }
}

impl TryConvert<Decimal> for f64 {
    fn try_convert(input: Decimal) -> Result<Self, String> {
        input
            .to_f64()
            .ok_or_else(|| "Decimal cannot be converted to f64".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }
}
