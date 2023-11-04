#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

// TODO: Add example on main page
// //! # Sample Code

mod check;
mod error;

pub type VCSResult<T> = Result<T, error::VSCError>;
pub use check::{check_version_control, CheckOptions};
pub use error::VSCError;
