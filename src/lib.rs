#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod check;
mod error;

type VCSResult = Result<(), error::VSCError>;
pub use check::{check_version_control, CheckOptions};
pub use error::VSCError;
