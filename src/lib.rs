//! This crate adds the some useful macros for easily work

pub mod inputln;
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
/// use add_macro::{thiserror, Error};
/// 
/// #[derive(Debug, Error)]
/// pub enum FileError {
///     #[error("The file '{0}' is not found")]
///     NotFound(String),
///     #[error("Expected to get the file, but the folder was found")]
///     ExpectedFile,
/// }
/// ```
pub use thiserror;
pub use thiserror::Error;

/// Derive the [std::fmt::Display] trait
/// 
/// # Examples
/// ```
/// use add_macro::Display;
/// 
/// #[derive(Display)]
/// struct Person {
///     pub name: String,
///     pub age: u8,
/// }
/// 
/// impl Person {
///     // !This method needs to formatting Display
///     pub fn to_string(&self) -> String {
///         format!("Hello, {}. Your age is {} years.)", &self.name, &self.age)
///     }
/// }
/// 
/// fn main() {
///     let tomas = Person {
///         name: "Tomas".to_owned(),
///         age: 25,
///     };
///     println!("{tomas}");    // => Hello, Tomas. Your age is 25 years.
/// }
/// ```
pub use display_derive::Display;
