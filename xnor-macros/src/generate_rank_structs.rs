use proc_macro::TokenStream;
use quote::quote;

pub fn generate_rank_structs_impl(_item: TokenStream) -> TokenStream {
    quote!(
        pub fn hello_world() {
            println!("Hello, world!")
        }
    )
    .into()
}
