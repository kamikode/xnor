use crate::common::{format_token_stream, DIM, RANK};
use quote::{format_ident, quote};

pub fn generate_code(max_ndim: usize) -> syn::Result<String> {
    // Generate struct and trait definitions.
    let mut structs_and_traits = quote! {};
    let mut const_dims = quote! {};
    let mut dims = quote! {};
    let mut size = quote! {};
    for ndim in 0..=max_ndim {
        let struct_name = format_ident!("{RANK}{ndim}");

        // Rank# struct and Shape trait implementation.
        let mut current_size = quote! { #size };
        if current_size.is_empty() {
            current_size = quote! { 1 };
        }
        structs_and_traits.extend(quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name<#const_dims> {}
            impl<#const_dims> Shape for #struct_name<#dims> {
                const NDIM: usize = #ndim;
                const SIZE: usize = #current_size;
            }
        });

        // HasAxis, AxisAtIndexHasSize, and Index traits implementation.
        let indim = ndim as isize;
        let mut index_body = quote! {};
        let mut stride = quote! {};
        for d in (0..indim).rev() {
            let dim_name = format_ident!("{DIM}{d}");
            let neg_d = -indim + d;
            let mut current_stride = quote! { #stride };
            if current_stride.is_empty() {
                current_stride = quote! { 1 };
            }
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
            index_body.extend(quote! {
                #d => { &#dim_name },
                #neg_d => { &#dim_name },
            });
            if !stride.is_empty() {
                stride.extend(quote! { * });
            }
            stride.extend(quote! { #dim_name });
        }

        let panic_out_of_bounds = quote! {
            panic!("index out of bounds: the len is {} but the index is {}", #ndim, index);
        };
        if index_body.is_empty() {
            index_body = panic_out_of_bounds;
        } else {
            index_body = quote! {
                match index {
                    #index_body
                    _ => {
                        #panic_out_of_bounds
                    }
                }
            }
        }

        structs_and_traits.extend(quote! {
            impl<#const_dims> Index<isize> for #struct_name<#dims> {
                type Output = usize;
                fn index(&self, index: isize) -> &Self::Output {
                    #index_body
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

        // Append current dimension.
        let dim_name = format_ident!("{DIM}{ndim}");
        const_dims.extend(quote! { const #dim_name: usize, });
        dims.extend(quote! { #dim_name, });
        if !size.is_empty() {
            size.extend(quote! { * });
        }
        size.extend(quote! { #dim_name });
    }

    // Generate shape macro for conveniently defining Rank# structs.
    let mut shape_macro = quote! {};
    let mut macro_captures = quote! {};
    let mut struct_args = quote! {};
    for ndim in 0..=max_ndim {
        let struct_name = format_ident!("{RANK}{ndim}");
        shape_macro.extend(quote! {
            (#macro_captures) => {
                #struct_name::<#struct_args> {}
            };
        });
        if !macro_captures.is_empty() {
            macro_captures.extend(quote! { , });
        }
        if !struct_args.is_empty() {
            struct_args.extend(quote! { , });
        }
        let dim_ident = format_ident!("d{}", ndim);
        macro_captures.extend(quote! { $#dim_ident:expr });
        struct_args.extend(quote! { $#dim_ident });
    }
    shape_macro = quote! {
        #[macro_export]
        macro_rules! shape {
            #shape_macro
        }
    };
    format_token_stream(quote! {
        #structs_and_traits
        #shape_macro
    })
}
