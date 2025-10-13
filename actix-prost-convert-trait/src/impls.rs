use crate::TryConvert;
use std::collections::{BTreeMap, HashMap, HashSet};

impl<T, R: TryConvert<T>> TryConvert<Option<T>> for Option<R> {
    fn try_convert(input: Option<T>) -> Result<Self, String> {
        match input {
            Some(input) => Ok(Some(TryConvert::try_convert(input)?)),
            None => Ok(None),
        }
    }
}

impl<T, R: TryConvert<T>> TryConvert<Vec<T>> for Vec<R> {
    fn try_convert(input: Vec<T>) -> Result<Self, String> {
        input.into_iter().map(TryConvert::try_convert).collect()
    }
}

impl<T, R: TryConvert<T> + std::hash::Hash + Eq> TryConvert<Vec<T>> for HashSet<R> {
    fn try_convert(input: Vec<T>) -> Result<Self, String> {
        input.into_iter().map(TryConvert::try_convert).collect()
    }
}

impl<K: Eq + std::hash::Hash, T, R: TryConvert<T>> TryConvert<HashMap<K, T>> for HashMap<K, R> {
    fn try_convert(input: HashMap<K, T>) -> Result<Self, String> {
        input
            .into_iter()
            .map(|(k, v)| Ok((k, TryConvert::try_convert(v)?)))
            .collect()
    }
}

impl<K: std::cmp::Ord, T, R: TryConvert<T>> TryConvert<BTreeMap<K, T>> for BTreeMap<K, R> {
    fn try_convert(input: BTreeMap<K, T>) -> Result<Self, String> {
        input
            .into_iter()
            .map(|(k, v)| Ok((k, TryConvert::try_convert(v)?)))
            .collect()
    }
}

#[macro_export]
macro_rules! impl_try_convert_from_string {
    ($type:ty) => {
        impl_try_convert_from_string!($type, stringify!($type));
    };
    ($type:ty, $type_name:expr) => {
        impl TryConvert<String> for $type {
            fn try_convert(value: String) -> Result<Self, String> {
                value.parse().map_err(|e| {
                    $crate::failed_to_parse_error_message_with_description(&value, $type_name, e)
                })
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
