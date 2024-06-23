use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitInt};

const DIM: &str = "D";

pub fn generate_rank_structs_impl(item: TokenStream) -> TokenStream {
    // Extract maximum desired rank from macro input.
    let input = parse_macro_input!(item as LitInt);
    let max_ndim = input.base10_parse::<usize>().unwrap();

    // Generate struct and trait definitions.
    let mut structs_and_traits = quote! {};
    for ndim in 0..=max_ndim {
        let indim = ndim as isize;
        let struct_name = format_ident!("Rank{}", ndim);

        // Build dimension generics, dimension names, and size.
        let mut const_dims = quote! {};
        let mut dims = quote! {};
        let mut size = quote! {};
        for d in 0..ndim {
            let dim_name = format_ident!("{DIM}{d}");
            const_dims.extend(quote! {
                const #dim_name: usize,
            });
            dims.extend(quote! {
                #dim_name,
            });
            size.extend(quote! {
                #dim_name *
            })
        }
        size.extend(quote! {1});

        // Rank# struct and Shape trait implementation.
        structs_and_traits.extend(quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name<#const_dims> {}
            impl<#const_dims> Shape for #struct_name<#dims> {
                const NDIM: usize = #ndim;
                const SIZE: usize = #size;
            }
        });

        // HasAxis, AxisAtIndexHasSize, and Index traits implementation.
        let mut match_block = quote! {};
        let mut current_stride = quote! { 1 };
        for d in (0..indim).rev() {
            let dim_name = format_ident!("{DIM}{d}");
            let neg_d = -indim + d;
            structs_and_traits.extend(quote! {
                impl<#const_dims> HasAxis<#d> for #struct_name<#dims> {
                    const STRIDE: usize = #current_stride;
                }
                impl<#const_dims> HasAxis<#neg_d> for #struct_name<#dims> {
                    const STRIDE: usize = #current_stride;
                }
                impl<#const_dims> AxisAtIndexHasSize<#d, #dim_name> for #struct_name<#dims> {}
                impl<#const_dims> AxisAtIndexHasSize<#neg_d, #dim_name> for #struct_name<#dims> {}
            });
            match_block.extend(quote! {
                #d => { &#dim_name },
                #neg_d => { &#dim_name },
            });
            current_stride = quote! { #current_stride * #dim_name };
        }
        match_block.extend(quote! {
            _ => {
                panic!("index out of bounds: the len is {} but the index is {}",
                        #ndim, index);
            }
        });
        structs_and_traits.extend(quote! {
            impl<#const_dims> Index<isize> for #struct_name<#dims> {
                type Output = usize;
                fn index(&self, index: isize) -> &Self::Output {
                    match index {
                        #match_block
                    }
                }
            }
        });

        // Display implementation.
        let format_str = match ndim {
            0 => "()".to_string(),
            1 => "({})".to_string(),
            _ => {
                let mut s = String::from("(");
                for _ in 0..ndim - 1 {
                    s += "{}, ";
                }
                s += "{})";
                s
            }
        };
        structs_and_traits.extend(quote! {
            impl<#const_dims> Display for #struct_name<#dims> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, #format_str, #dims)
                }
            }
        });
    }

    // Return the final implementation.
    quote! {
        #structs_and_traits
    }
    .into()
}
