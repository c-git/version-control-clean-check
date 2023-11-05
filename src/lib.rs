#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! # Sample Code
//!
//! ```rust
//! use version_control_clean_check::{CheckOptions, check_version_control};
//! let mut opts = CheckOptions::new();
//! opts.allow_no_vcs = true; // Disable actual checks for example
//! let actual = check_version_control("path_here", &opts);
//! assert!(actual.is_ok());
//! ```

mod check;
mod error;

/// Type alias for Result<T, [`VSCError`]>
pub type VCSResult<T> = Result<T, VCSError>;
pub use check::{check_version_control, CheckOptions};
pub use error::VCSError;
