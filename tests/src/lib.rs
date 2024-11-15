#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto;

#[cfg(test)]
mod test;

#[cfg(test)]
mod macros;

#[cfg(test)]
mod rest;

#[cfg(test)]
mod types;

#[cfg(test)]
mod errors;

#[cfg(test)]
mod conversions;

#[cfg(test)]
mod snake_case_types;
