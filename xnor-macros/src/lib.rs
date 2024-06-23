mod generate_rank_structs;
mod generate_tensor_from_array;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_rank_structs(item: TokenStream) -> TokenStream {
    crate::generate_rank_structs::generate_rank_structs_impl(item)
}

#[proc_macro]
pub fn generate_tensor_from_array(item: TokenStream) -> TokenStream {
    crate::generate_tensor_from_array::generate_tensor_from_array_impl(item)
}
