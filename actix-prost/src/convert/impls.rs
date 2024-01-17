use crate::convert::Convert;

impl<T, R: Convert<T>> Convert<Option<T>> for Option<R> {
    fn convert(input: Option<T>) -> anyhow::Result<Self> {
        match input {
            Some(input) => Ok(Some(Convert::convert(input)?)),
            None => Ok(None),
        }
    }
}

