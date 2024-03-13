/// Reads the input buffer
/// 
/// # Examples
/// ```
/// use add_macro::inputln;
/// 
/// let buf: String = inputln!("Print something: ");    // => Print something: ...
/// 
/// let buf2: String = inputln!();                      // => ...
/// ```
#[macro_export]
macro_rules! inputln {
    () => {{
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
        buf
    }};
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = std::io::stdout().write_all( format_args!($($arg)*).to_string().as_bytes() );
        let _ = std::io::stdout().flush();
        inputln!()
    }};
}
