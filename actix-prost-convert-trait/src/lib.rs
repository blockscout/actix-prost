use std::fmt::Display;

mod impls;

#[cfg(feature = "conv-address")]
mod address;

#[cfg(feature = "conv-bytes")]
mod bytes;

#[cfg(feature = "conv-url")]
mod url;

#[cfg(feature = "conv-datetime")]
mod datetime;

#[cfg(feature = "conv-uuid")]
mod uuid;

#[cfg(feature = "conv-ip")]
mod ip;

#[cfg(feature = "conv-path")]
mod path;

#[cfg(feature = "conv-duration")]
mod duration;

#[cfg(feature = "conv-decimal")]
mod decimal;

pub trait TryConvert<T>: Sized {
    fn try_convert(value: T) -> Result<Self, String>;
}

pub fn failed_to_parse_error_message_with_description(
    value: impl Display,
    type_name: impl Display,
    error_description: impl Display,
) -> String {
    format!(
        "failed to parse '{}' as {}: {}",
        value, type_name, error_description
    )
}

pub fn failed_to_parse_error_message(value: impl Display, type_name: impl Display) -> String {
    format!("failed to parse '{}' as {}", value, type_name)
}
