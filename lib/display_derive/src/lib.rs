extern crate proc_macro;
use proc_macro as proc;

/// Derive the [std::fmt::Display](<https://doc.rust-lang.org/std/fmt/trait.Display.html>) trait
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
#[proc_macro_derive(Display)]
pub fn display_derive(input: proc::TokenStream) -> proc::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    quote::quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }
    }.into()
}