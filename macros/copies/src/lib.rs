use proc_macro::TokenStream;

#[proc_macro]
pub fn def_it_works(_item: TokenStream) -> TokenStream {
    r#"pub fn it_works() -> bool { true }"#.parse().unwrap()
}

#[proc_macro_derive(AutoFrom, attributes(from))]
pub fn derive_rename_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
