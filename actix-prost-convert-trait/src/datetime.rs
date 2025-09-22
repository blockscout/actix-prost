use crate::{impl_try_convert_to_string, TryConvert};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

// DateTime<Utc> conversions
impl TryConvert<String> for DateTime<Utc> {
    fn try_convert(value: String) -> Result<Self, String> {
        if let Ok(value) = value.parse::<i64>() {
            DateTime::from_timestamp(value, 0).ok_or_else(|| {
                crate::failed_to_parse_error_message_with_description(
                    value,
                    "DateTime<Utc>",
                    "invalid timestamp",
                )
            })
        } else {
            value.parse().map_err(|e| {
                crate::failed_to_parse_error_message_with_description(value, "DateTime<Utc>", e)
            })
        }
    }
}

impl_try_convert_to_string!(DateTime<Utc>);

// DateTime<FixedOffset> conversions
impl TryConvert<String> for DateTime<FixedOffset> {
    fn try_convert(value: String) -> Result<Self, String> {
        if let Ok(value) = value.parse::<i64>() {
            let datetime_utc = DateTime::from_timestamp(value, 0).ok_or_else(|| {
                crate::failed_to_parse_error_message_with_description(
                    value,
                    "DateTime<FixedOffset>",
                    "invalid timestamp",
                )
            })?;
            Ok(datetime_utc.with_timezone(&FixedOffset::east_opt(0).unwrap()))
        } else {
            value.parse().map_err(|e| {
                crate::failed_to_parse_error_message_with_description(
                    value,
                    "DateTime<FixedOffset>",
                    e,
                )
            })
        }
    }
}

impl_try_convert_to_string!(DateTime<FixedOffset>);

// NaiveDateTime conversions
impl TryConvert<String> for NaiveDateTime {
    fn try_convert(value: String) -> Result<Self, String> {
        if let Ok(value) = value.parse::<i64>() {
            let datetime_utc = DateTime::from_timestamp(value, 0).ok_or_else(|| {
                crate::failed_to_parse_error_message_with_description(
                    value,
                    "NaiveDateTime",
                    "invalid timestamp",
                )
            })?;
            Ok(datetime_utc.naive_utc())
        } else {
            value.parse().map_err(|e| {
                crate::failed_to_parse_error_message_with_description(value, "NaiveDateTime", e)
            })
        }
    }
}

impl_try_convert_to_string!(NaiveDateTime);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_conversion_datetime() {
        let datetime = DateTime::<Utc>::try_convert("1645491600".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2022-02-22T01:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );

        let datetime = DateTime::<FixedOffset>::try_convert("1645491600".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2022-02-22T01:00:00+00:00"
                .parse::<DateTime<FixedOffset>>()
                .unwrap()
        );

        let datetime = NaiveDateTime::try_convert("1645491600".to_string()).unwrap();
        assert_eq!(
            datetime,
            "2022-02-22T01:00:00".parse::<NaiveDateTime>().unwrap()
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
