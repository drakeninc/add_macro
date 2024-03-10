/// Macros for [std::string::String]
/// 
/// # Examples
/// ```
/// use add_macro::str;
/// 
/// assert_eq!(str!(), String::new());
/// assert_eq!(str!("Hello, world!"), String::from("Hello, world!"));
/// assert_eq!(str!("My name is {}.", "Tomas"), String::from("My name is Tomas."));
/// let num = 10;
/// assert_eq!(str!(num), num.to_string());
/// ```
#[macro_export]
macro_rules! str {
    () => { String::new() };
    ($arg:literal) => { $arg.to_owned() };
    ($arg:expr) => { $arg.to_string() };
    ($($arg:tt)*) => { format_args!($($arg)*).to_string() };
}
