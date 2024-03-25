/// Prints a error message to log
/// 
/// # Examples
/// ```
/// use add_macro::elog;
/// 
/// elog!("Hello, world!");             // => [2000-01-01T01:00:00.000001] (error) Hello, world!
/// elog!("My name is {}.", "Tomas");   // => [2000-01-01T01:00:00.000001] (error) My name is Tomas.
/// ```
/// 
/// # Dependencies
/// - [chrono](<https://docs.rs/chrono>)
#[macro_export]
macro_rules! elog {
    ($($arg:tt)*) => { eprintln!("[{}] (error) {}", chrono::Utc::now().format("%Y-%m-%dT%T%.6f"), format_args!($($arg)*).to_string()) };
}
