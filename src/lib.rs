#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

// TODO: Add example on main page
// //! # Sample Code

mod check;
mod error;

/// Type alias for Result<T, [`VSCError`]>
pub type VCSResult<T> = Result<T, VSCError>;
pub use check::{check_version_control, CheckOptions};
pub use error::VSCError;
