use crate::common::{format_token_stream, RANK};
use quote::{format_ident, quote};

pub fn generate_code(max_ndim: usize) -> String {
    let mut all_ranks = quote! {};
    for ndim in 0..=max_ndim {
        if !all_ranks.is_empty() {
            all_ranks.extend(quote! { , });
        }
        let struct_name = format_ident!("{RANK}{ndim}");
        all_ranks.extend(quote! {#struct_name});
    }
    format_token_stream(quote! {
        pub use crate::shape::{#all_ranks};
    })
}
