use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitInt};

const DIM: &str = "D";

fn build_dimension_generics(rank: usize) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut const_dims = quote! {};
    let mut dims = quote! {};
    for d in 0..rank {
        let dim_name = format_ident!("{DIM}{d}");
        const_dims.extend(quote! {
            const #dim_name: usize,
        });
        dims.extend(quote! {
            #dim_name,
        });
    }
    (const_dims, dims)
}

pub fn generate_rank_structs_impl(item: TokenStream) -> TokenStream {
    // Extract maximum desired rank from macro input.
    let input = parse_macro_input!(item as LitInt);
    let max_rank = input.base10_parse::<usize>().unwrap();

    // Generate struct and trait definitions.
    let mut structs_and_traits = quote! {};
    for rank in 0..=max_rank {
        let struct_name = format_ident!("Rank{}", rank);
        let (const_dims, dims) = build_dimension_generics(rank);
        // RankN struct and Shape trait implementation.
        structs_and_traits.extend(quote! {
            struct #struct_name<#const_dims> {}
            impl<#const_dims> Shape for #struct_name<#dims> {}
        });
        // HasAxis trait implementations.
        let irank = rank as isize;
        for d in 0..irank {
            let dim_name = format_ident!("{DIM}{d}");
            let neg_d = -irank + d;
            structs_and_traits.extend(quote! {
                impl<#const_dims> HasAxis<#d> for #struct_name<#dims> {}
                impl<#const_dims> HasAxis<#neg_d> for #struct_name<#dims> {}
                impl<#const_dims> AxisAtIdxHasSize<#d, #dim_name> for #struct_name<#dims> {}
                impl<#const_dims> AxisAtIdxHasSize<#neg_d, #dim_name> for #struct_name<#dims> {}
            });
        }
    }

    quote! {
        #structs_and_traits
    }
    .into()
}
