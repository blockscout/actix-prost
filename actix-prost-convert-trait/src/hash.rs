use crate::{impl_try_convert_from_string, impl_try_convert_to_string};
use alloy::primitives::B256;

impl_try_convert_from_string!(B256);
impl_try_convert_to_string!(B256);
