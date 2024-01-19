mod impls;

#[cfg(feature = "conv-address")]
mod address;

#[cfg(feature = "conv-bytes")]
mod bytes;

pub trait Convert<T>: Sized {
    fn convert(value: T) -> Result<Self, String>;
}
