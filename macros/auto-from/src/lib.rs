use proc_macro::TokenStream;

#[proc_macro]
pub fn def_it_works(_item: TokenStream) -> TokenStream {
    r#"pub fn it_works() -> bool { true }"#.parse().unwrap()
}

#[proc_macro_derive(AutoFrom, attributes(from))]
pub fn auto_from(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Struct(_data) = input.data {
    } else {
        panic!("only supports deriving to struct");
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote::quote! {
        // ...
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
