//! Small helper functions and structs which aren't appropriate in other folders.

pub mod config;
pub mod crypto;
pub mod guards;

/// This function is used to prevent optimization attempts from the compiler to drop the variable.
pub fn no_operation<T>(_var: &T) {}
