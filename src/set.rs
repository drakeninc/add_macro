/// Macros for [std::collections::HashSet]
/// 
/// # Examples
/// ```
/// use std::collections::HashSet;
/// use add_macro::set;
/// 
/// assert_eq!(set![1, 5, 3, 4, 2], HashSet::from([1, 5, 3, 4, 2]));
/// ```
#[macro_export]
macro_rules! set {
    ($($v:expr),* $(,)*) => { std::collections::HashSet::from([$(($v),)*]) };
}
