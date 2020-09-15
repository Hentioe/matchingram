use proc_macro::TokenStream;

#[proc_macro]
pub fn def_it_works(_item: TokenStream) -> TokenStream {
    r#"pub fn it_works() -> bool { true }"#.parse().unwrap()
}
