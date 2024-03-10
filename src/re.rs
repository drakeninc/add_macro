/// Macros for [regex::Regex](<https://docs.rs/regex>)
/// 
/// # Examples
/// ```
/// use regex::Regex;
/// use add_macro::re;
/// 
/// let re: Regex = re!(r"^https?://[\w\-\.]+(/.*)?$");
/// assert!(re.is_match("https://example.loc/"));
///
/// let re: Regex = re!(r"^{}$", "/home");
/// assert!(re.is_match("/home"));
/// 
/// let r = r"world";
/// assert!(re!(r).is_match("Hello world!"));
/// ```
/// 
/// # Dependencies
/// - [regex](<https://docs.rs/regex>)
#[macro_export]
macro_rules! re {
    ($arg:expr) => { regex::Regex::new( &$arg ).unwrap() };
    ($($arg:tt)*) => { regex::Regex::new( &format_args!($($arg)*).to_string() ).unwrap() }
}
