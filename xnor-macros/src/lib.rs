mod generate_rank_structs;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generate_rank_structs(item: TokenStream) -> TokenStream {
    crate::generate_rank_structs::generate_rank_structs_impl(item)
}
