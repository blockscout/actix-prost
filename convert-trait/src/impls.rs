use crate::TryConvert;

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

impl<T, R: TryConvert<T> + std::hash::Hash + Eq> TryConvert<Vec<T>>
    for std::collections::HashSet<R>
{
    fn try_convert(input: Vec<T>) -> Result<Self, String> {
        input.into_iter().map(TryConvert::try_convert).collect()
    }
}
