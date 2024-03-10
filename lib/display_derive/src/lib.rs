extern crate proc_macro;
use proc_macro as proc;

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