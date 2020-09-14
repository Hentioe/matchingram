use proc_macro::TokenStream;

#[proc_macro]
pub fn def_hello_copies(_item: TokenStream) -> TokenStream {
    r#"pub fn hello_copies() -> i32 { 0 }"#.parse().unwrap()
}
