use crate::Convert;

impl<T, R: Convert<T>> Convert<Option<T>> for Option<R> {
    fn convert(input: Option<T>) -> Result<Self, String> {
        match input {
            Some(input) => Ok(Some(Convert::convert(input)?)),
            None => Ok(None),
        }
    }
}
