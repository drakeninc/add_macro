/// Prints a message to log
/// 
/// # Examples
/// ```
/// use add_macro::log;
/// 
/// log!("Hello, world!");              // => [2000-01-01T01:00:00.0] Hello, world!
/// log!("My name is {}.", "Tomas");    // => [2000-01-01T01:00:00.0] My name is Tomas.
/// ```
/// 
/// # Dependencies
/// - [chrono](<https://docs.rs/chrono>)
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => { println!("[{}] {}", chrono::Utc::now().format("%Y-%m-%dT%T.%s"), format_args!($($arg)*).to_string()) };
}
