use crate::common::{format_token_stream, DIM, RANK};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

// TODO: This would probably be cleaner/more readable if it was split into
// smaller blocks.

fn common_code_generation_loop(
    max_ndim: usize,
    loop_body: fn(ndim: usize, &mut TokenStream, Ident, &TokenStream, &TokenStream, &TokenStream),
) -> TokenStream {
    let mut code = quote! {};
    let mut dims = quote! {};
    let mut size = quote! {};
    let mut const_dims = quote! {};
    for ndim in 0..=max_ndim {
        let struct_name = format_ident!("{RANK}{ndim}");
        loop_body(ndim, &mut code, struct_name, &dims, &size, &const_dims);
        let dim_name = format_ident!("{DIM}{ndim}");
        dims.extend(quote! { #dim_name, });
        if !size.is_empty() {
            size.extend(quote! { * });
        }
        size.extend(quote! { #dim_name });
        const_dims.extend(quote! { const #dim_name: usize, });
    }
    code
}

fn generate_struct_declarations(max_ndim: usize) -> TokenStream {
    let loop_body = |_: usize,
                     code: &mut TokenStream,
                     struct_name: Ident,
                     _dims: &TokenStream,
                     _size: &TokenStream,
                     const_dims: &TokenStream| {
        code.extend(quote! {
            #[derive(Debug, PartialEq)]
            pub struct #struct_name<#const_dims> {}
        });
    };
    common_code_generation_loop(max_ndim, loop_body)
}

pub fn generate_shape_trait_implementations(max_ndim: usize) -> TokenStream {
    let loop_body = |ndim: usize,
                     code: &mut TokenStream,
                     struct_name: Ident,
                     dims: &TokenStream,
                     size: &TokenStream,
                     const_dims: &TokenStream| {
        let mut current_size = quote! { #size };
        if current_size.is_empty() {
            current_size = quote! { 1 };
        }
        code.extend(quote! {
            impl<#const_dims> Shape for #struct_name<#dims> {
                const NDIM: usize = #ndim;
                const SIZE: usize = #current_size;
            }
        });
    };
    common_code_generation_loop(max_ndim, loop_body)
}

pub fn generate_index_trait_implementations(max_ndim: usize) -> TokenStream {
    let loop_body = |ndim: usize,
                     code: &mut TokenStream,
                     struct_name: Ident,
                     dims: &TokenStream,
                     _size: &TokenStream,
                     const_dims: &TokenStream| {
        let indim = ndim as isize;
        let mut index_body = quote! {};
        for d in (0..indim).rev() {
            let dim_name = format_ident!("{DIM}{d}");
            let neg_d = -indim + d;
            index_body.extend(quote! {
                #d => { &#dim_name },
                #neg_d => { &#dim_name },
            });
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
        code.extend(quote! {
            impl<#const_dims> Index<isize> for #struct_name<#dims> {
                type Output = usize;
                fn index(&self, index: isize) -> &Self::Output {
                    #index_body
                }
            }
        });
    };
    common_code_generation_loop(max_ndim, loop_body)
}

pub fn generate_axis_trait_implementations(max_ndim: usize) -> TokenStream {
    let loop_body = |ndim: usize,
                     code: &mut TokenStream,
                     struct_name: Ident,
                     dims: &TokenStream,
                     _size: &TokenStream,
                     const_dims: &TokenStream| {
        let indim = ndim as isize;
        let mut stride = quote! {};
        for d in (0..indim).rev() {
            let dim_name = format_ident!("{DIM}{d}");
            let neg_d = -indim + d;
            let mut current_stride = quote! { #stride };
            if current_stride.is_empty() {
                current_stride = quote! { 1 };
            }
            code.extend(quote! {
                impl<#const_dims> HasAxis<#d> for #struct_name<#dims> {
                    const STRIDE: usize = #current_stride;
                }
                impl<#const_dims> HasAxis<#neg_d> for #struct_name<#dims> {
                    const STRIDE: usize = #current_stride;
                }
                impl<#const_dims> AxisAtIndexHasSize<#d, #dim_name> for #struct_name<#dims> {}
                impl<#const_dims> AxisAtIndexHasSize<#neg_d, #dim_name> for #struct_name<#dims> {}
            });
            if !stride.is_empty() {
                stride.extend(quote! { * });
            }
            stride.extend(quote! { #dim_name });
        }
    };
    common_code_generation_loop(max_ndim, loop_body)
}

pub fn generate_display_trait_implementations(max_ndim: usize) -> TokenStream {
    let loop_body = |ndim: usize,
                     code: &mut TokenStream,
                     struct_name: Ident,
                     dims: &TokenStream,
                     _size: &TokenStream,
                     const_dims: &TokenStream| {
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
        code.extend(quote! {
            impl<#const_dims> Display for #struct_name<#dims> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, #format_str, #dims)
                }
            }
        });
    };
    common_code_generation_loop(max_ndim, loop_body)
}

fn generate_shape_macro(max_ndim: usize) -> TokenStream {
    let mut code = quote! {};
    let mut captures = quote! {};
    let mut args = quote! {};
    for ndim in 0..=max_ndim {
        let struct_name = format_ident!("{RANK}{ndim}");
        code.extend(quote! {
            (#captures) => {
                #struct_name::<#args> {}
            };
        });
        if !captures.is_empty() {
            captures.extend(quote! { , });
        }
        if !args.is_empty() {
            args.extend(quote! { , });
        }
        let dim_ident = format_ident!("d{}", ndim);
        captures.extend(quote! { $#dim_ident:expr });
        args.extend(quote! { $#dim_ident });
    }
    code = quote! {
        #[macro_export]
        macro_rules! shape {
            #code
        }
    };
    code
}

pub fn generate_code(max_ndim: usize) -> syn::Result<String> {
    let structs = generate_struct_declarations(max_ndim);
    let shape_traits = generate_shape_trait_implementations(max_ndim);
    let index_traits = generate_index_trait_implementations(max_ndim);
    let axis_traits = generate_axis_trait_implementations(max_ndim);
    let display_traits = generate_display_trait_implementations(max_ndim);
    let shape_macro = generate_shape_macro(max_ndim);
    format_token_stream(quote! {
        #structs
        #shape_traits
        #index_traits
        #axis_traits
        #display_traits
        #shape_macro
    })
}
