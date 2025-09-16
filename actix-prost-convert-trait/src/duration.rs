use crate::TryConvert;
use std::time::Duration;

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 3600;
const SECONDS_PER_DAY: u64 = 86400;
const SECONDS_PER_WEEK: u64 = 604800;
const SECONDS_PER_YEAR: u64 = 31536000;

// Duration from various time units
impl TryConvert<String> for Duration {
    fn try_convert(input: String) -> Result<Self, String> {
        fn err(input: &str) -> String {
            format!(
                "failed to parse '{}' as duration, try '34s', '1m', '1h', '1d' instead",
                input
            )
        }
        let input = input.trim();

        // Handle empty string
        if input.is_empty() {
            return Err(err(input));
        }

        // Try parsing with suffix first (e.g., "34s", "1m", "1h", "1d")
        if let Some(suffix) = input.chars().last() {
            if let Some(number_part) = input.get(0..input.len() - 1) {
                if let Ok(value) = number_part.parse::<f64>() {
                    match suffix {
                        's' => {
                            let secs = value as u64;
                            let nanos = ((value - secs as f64) * 1_000_000_000.0) as u32;
                            return Ok(Duration::new(secs, nanos));
                        }
                        'm' => {
                            let secs = (value * SECONDS_PER_MINUTE as f64) as u64;
                            let nanos = (((value * 60.0) - secs as f64) * 1_000_000_000.0) as u32;
                            return Ok(Duration::new(secs, nanos));
                        }
                        'h' => {
                            let secs = (value * SECONDS_PER_HOUR as f64) as u64;
                            let nanos = (((value * 3600.0) - secs as f64) * 1_000_000_000.0) as u32;
                            return Ok(Duration::new(secs, nanos));
                        }
                        'd' => {
                            let secs = (value * SECONDS_PER_DAY as f64) as u64;
                            let nanos =
                                (((value * 86400.0) - secs as f64) * 1_000_000_000.0) as u32;
                            return Ok(Duration::new(secs, nanos));
                        }
                        'w' => {
                            let secs = (value * SECONDS_PER_WEEK as f64) as u64;
                            let nanos =
                                (((value * 604800.0) - secs as f64) * 1_000_000_000.0) as u32;
                            return Ok(Duration::new(secs, nanos));
                        }
                        'y' => {
                            let secs = (value * SECONDS_PER_YEAR as f64) as u64; // 365 days
                            let nanos =
                                (((value * 31536000.0) - secs as f64) * 1_000_000_000.0) as u32;
                            return Ok(Duration::new(secs, nanos));
                        }
                        _ => {} // Fall through to try parsing as plain number
                    }
                }
            }
        }

        // Try parsing as plain number (assume seconds)
        if let Ok(secs_f64) = input.parse::<f64>() {
            if secs_f64 < 0.0 {
                return Err(err(input));
            }
            let secs = secs_f64 as u64;
            let nanos = ((secs_f64 - secs as f64) * 1_000_000_000.0) as u32;
            return Ok(Duration::new(secs, nanos));
        }

        Err(err(input))
    }
}

impl TryConvert<Duration> for String {
    fn try_convert(input: Duration) -> Result<Self, String> {
        Ok(input.as_secs().to_string())
    }
}

// Duration from numeric types
impl TryConvert<u64> for Duration {
    fn try_convert(input: u64) -> Result<Self, String> {
        Ok(Duration::from_secs(input))
    }
}

impl TryConvert<Duration> for u64 {
    fn try_convert(input: Duration) -> Result<Self, String> {
        Ok(input.as_secs())
    }
}

impl TryConvert<i64> for Duration {
    fn try_convert(input: i64) -> Result<Self, String> {
        if input < 0 {
            return Err("Duration cannot be negative".to_string());
        }
        Ok(Duration::from_secs(input as u64))
    }
}

impl TryConvert<Duration> for i64 {
    fn try_convert(input: Duration) -> Result<Self, String> {
        Ok(input.as_secs() as i64)
    }
}

impl TryConvert<f64> for Duration {
    fn try_convert(input: f64) -> Result<Self, String> {
        if input < 0.0 {
            return Err("Duration cannot be negative".to_string());
        }
        let secs = input as u64;
        let nanos = ((input - secs as f64) * 1_000_000_000.0) as u32;
        Ok(Duration::new(secs, nanos))
    }
}

impl TryConvert<Duration> for f64 {
    fn try_convert(input: Duration) -> Result<Self, String> {
        Ok(input.as_secs() as f64 + input.subsec_nanos() as f64 / 1_000_000_000.0)
    }
}

// Duration from milliseconds
impl TryConvert<u32> for Duration {
    fn try_convert(input: u32) -> Result<Self, String> {
        Ok(Duration::from_millis(input as u64))
    }
}

impl TryConvert<Duration> for u32 {
    fn try_convert(input: Duration) -> Result<Self, String> {
        Ok(input.as_millis() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_duration() {
        let duration = Duration::try_convert("34.5s".to_string()).unwrap();
        assert_eq!(duration, Duration::from_nanos(34_500_000_000));

        let duration = Duration::try_convert("1.1m".to_string()).unwrap();
        assert_eq!(duration, Duration::from_secs(SECONDS_PER_MINUTE + 6));

        let duration = Duration::try_convert("1.1h".to_string()).unwrap();
        assert_eq!(
            duration,
            Duration::from_secs(SECONDS_PER_HOUR + SECONDS_PER_MINUTE * 6)
        );

        let duration = Duration::try_convert("1h".to_string()).unwrap();
        assert_eq!(duration, Duration::from_secs(SECONDS_PER_HOUR));

        let duration = Duration::try_convert("1d".to_string()).unwrap();
        assert_eq!(duration, Duration::from_secs(SECONDS_PER_DAY));

        let duration = Duration::try_convert("1w".to_string()).unwrap();
        assert_eq!(duration, Duration::from_secs(SECONDS_PER_WEEK));

        let duration = Duration::try_convert("1y".to_string()).unwrap();
        assert_eq!(duration, Duration::from_secs(SECONDS_PER_YEAR));
    }
}
