//! This crate adds the some useful macros for easily work

pub use display_derive::Display;

pub mod log;
pub mod elog;
pub mod str;
pub mod re;
pub mod map;
pub mod set;


/// Derive the [std::error::Error] trait (from crate [thiserror])
/// 
/// # Examples
/// ```
/// use add_macro::Error;
/// 
/// #[derive(Debug, Error)]
/// pub enum FileError {
///     #[error("The file was not found")]
///     NotFound,
///     #[error("Expected to get the file, but the folder was found")]
///     ExpectedFile,
/// }
/// ```
pub use thiserror::Error;
