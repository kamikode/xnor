use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitInt};

pub fn generate_rank_structs_impl(item: TokenStream) -> TokenStream {
    // Extract maximum desired rank from macro input.
    let input = parse_macro_input!(item as LitInt);
    let max_rank = input.base10_parse::<usize>().unwrap();

    // Generate struct definitions.
    let mut struct_definitions = quote! {};
    for rank in 0..=max_rank {
        let struct_name = format_ident!("Rank{}", rank);
        let mut const_generic_dimensions = quote! {};
        for d in 0..rank {
            let const_name = format_ident!("D{d}");
            const_generic_dimensions.extend(quote! {
                const #const_name: usize,
            });
        }
        struct_definitions.extend(quote! {
            struct #struct_name<#const_generic_dimensions> {}
        });
    }

    quote! {
        #struct_definitions
    }
    .into()
}
