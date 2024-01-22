use crate::Convert;

impl<T, R: Convert<T>> Convert<Option<T>> for Option<R> {
    fn convert(input: Option<T>) -> Result<Self, String> {
        match input {
            Some(input) => Ok(Some(Convert::convert(input)?)),
            None => Ok(None),
        }
    }
}

impl<T, R: Convert<T>> Convert<Vec<T>> for Vec<R> {
    fn convert(input: Vec<T>) -> Result<Self, String> {
        input.into_iter().map(Convert::convert).collect()
    }
}

impl<T, R: Convert<T> + std::hash::Hash + Eq> Convert<Vec<T>> for std::collections::HashSet<R> {
    fn convert(input: Vec<T>) -> Result<Self, String> {
        input.into_iter().map(Convert::convert).collect()
    }
}
