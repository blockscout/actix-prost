mod impls;

#[cfg(feature = "conv-address")]
mod address;

#[cfg(feature = "conv-bytes")]
mod bytes;

#[cfg(feature = "conv-url")]
mod url;

pub trait TryConvert<T>: Sized {
    fn try_convert(value: T) -> Result<Self, String>;
}
