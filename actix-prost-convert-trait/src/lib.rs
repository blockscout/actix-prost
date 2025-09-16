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

#[macro_export]
macro_rules! impl_try_convert_from_string {
    ($type:ty, $type_name:expr) => {
        impl TryConvert<String> for $type {
            fn try_convert(value: String) -> Result<Self, String> {
                value
                    .parse()
                    .map_err(|e| format!("failed to parse '{}' as {}: {}", value, $type_name, e))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_convert_to_string {
    ($type:ty) => {
        impl TryConvert<$type> for String {
            fn try_convert(value: $type) -> Result<Self, String> {
                Ok(value.to_string())
            }
        }
    };
}
