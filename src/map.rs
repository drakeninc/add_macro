/// Macros for [std::collections::HashMap]
/// 
/// # Examples
/// ```
/// use std::collections::HashMap;
/// use add_macro::map;
/// 
/// let emails: HashMap<&str, &str> = map!{
///     "Tomas" => "tomas@example.loc",
///     "Bob" => "bob@example.loc",
/// };
/// assert_eq!(emails, HashMap::from([("Tomas", "tomas@example.loc"), ("Bob", "bob@example.loc")]));
/// ```
#[macro_export]
macro_rules! map {
    ($($k:expr => $v:expr),* $(,)*) => { std::collections::HashMap::from([$(($k, $v),)*]) };
}
