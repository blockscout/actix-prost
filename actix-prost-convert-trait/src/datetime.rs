use crate::{impl_try_convert_from_string, impl_try_convert_to_string, TryConvert};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

impl_try_convert_from_string!(DateTime<Utc>);
impl_try_convert_to_string!(DateTime<Utc>);

impl_try_convert_from_string!(DateTime<FixedOffset>);
impl_try_convert_to_string!(DateTime<FixedOffset>);

impl_try_convert_from_string!(NaiveDateTime);
impl_try_convert_to_string!(NaiveDateTime);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conversion_datetime() {
        let datetime = DateTime::<Utc>::try_convert("2021-01-01T00:00:00Z".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2021-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );

        let datetime =
            DateTime::<FixedOffset>::try_convert("2021-01-01T00:00:00+01:00".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2021-01-01T00:00:00+01:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        );

        let datetime = NaiveDateTime::try_convert("2021-01-01T00:00:00".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2021-01-01T00:00:00".parse::<NaiveDateTime>().unwrap()
        );

        let datetime = DateTime::<Utc>::try_convert("2021-01-01T00:00:00Z".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2021-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );

        let datetime =
            DateTime::<FixedOffset>::try_convert("2021-01-01T00:00:00+01:00".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2021-01-01T00:00:00+01:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        );

        let datetime = NaiveDateTime::try_convert("2021-01-01T00:00:00".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2021-01-01T00:00:00".parse::<NaiveDateTime>().unwrap()
        );

        // Convert from datetime.to_string()
        let datetime = DateTime::<Utc>::try_convert(
            "2021-01-01T00:00:00Z"
                .parse::<DateTime<Utc>>()
                .unwrap()
                .to_string(),
        )
        .unwrap();
        assert_eq!(
            datetime,
            "2021-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );

        // Error cases
        let error = DateTime::<Utc>::try_convert("2021-01-01T00:00:00".to_string())
            .expect_err("Invalid datetime");
        assert_eq!(
            error,
            "failed to parse '2021-01-01T00:00:00' as DateTime<Utc>: premature end of input"
        );
        let error = DateTime::<FixedOffset>::try_convert("2021-01-01T00:00:00".to_string())
            .expect_err("Invalid datetime");
        assert_eq!(
            error,
            "failed to parse '2021-01-01T00:00:00' as DateTime<FixedOffset>: premature end of input"
        );
        let error = NaiveDateTime::try_convert("2021-01-01-00:00:00".to_string())
            .expect_err("Invalid datetime");
        assert_eq!(
            error,
            "failed to parse '2021-01-01-00:00:00' as NaiveDateTime: input contains invalid characters"
        );
        let error = NaiveDateTime::try_convert("".to_string()).expect_err("Invalid datetime");
        assert_eq!(
            error,
            "failed to parse '' as NaiveDateTime: premature end of input"
        );
    }
}
