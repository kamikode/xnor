mod generate_rank_structs;
mod generate_tensor_from_implementations;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_rank_structs(item: TokenStream) -> TokenStream {
    crate::generate_rank_structs::generate_rank_structs_impl(item)
}

#[proc_macro]
pub fn generate_tensor_from_implementations(item: TokenStream) -> TokenStream {
    crate::generate_tensor_from_implementations::generate_tensor_from_implementations_impl(item)
}
