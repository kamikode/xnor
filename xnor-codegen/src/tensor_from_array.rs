use crate::common::{format_token_stream, DIM, RANK};
use quote::{format_ident, quote};

pub fn generate_code(max_ndim: usize) -> syn::Result<String> {
    let rank_name = format_ident!("{RANK}0");
    let mut from_implementations = quote! {
        // The primitive type trait is necessary for the type system
        // to be able to infer the type unambiguously.
        trait PrimitiveType: Copy {}
        impl PrimitiveType for bool {}
        impl PrimitiveType for f32 {}
        impl PrimitiveType for f64 {}
        impl PrimitiveType for i8 {}
        impl PrimitiveType for i16 {}
        impl PrimitiveType for i32 {}
        impl PrimitiveType for i64 {}
        impl PrimitiveType for i128 {}
        impl PrimitiveType for u8 {}
        impl PrimitiveType for u16 {}
        impl PrimitiveType for u32 {}
        impl PrimitiveType for u64 {}
        impl PrimitiveType for u128 {}

        impl<T: PrimitiveType> From<T> for Tensor<T, #rank_name> {
            fn from(value: T) -> Self {
                Self {
                    data: alloc::vec![value; 1].into(),
                    shape: shape!(),
                }
            }
        }
    };

    for ndim in 1..=max_ndim {
        let rank_name = format_ident!("{RANK}{ndim}");

        // Build dimension generics and dimension names.
        let mut const_dims = quote! {};
        let mut dims = quote! {};
        for d in 0..ndim {
            let dim_name = format_ident!("{DIM}{d}");
            const_dims.extend(quote! {
                const #dim_name: usize,
            });
            if !dims.is_empty() {
                dims.extend(quote! { , })
            }
            dims.extend(quote! {
                #dim_name
            });
        }

        // Build data creator with correct number of flatten() invocations.
        let mut data_creator = quote! {value.iter().copied().};
        for _ in 1..ndim {
            data_creator.extend(quote! {flatten().});
        }
        data_creator.extend(quote! {collect::<Vec<T>>().into()});

        // Build array type.
        let mut array_type = quote! { T };
        for d in (0..ndim).rev() {
            let dim_name = format_ident!("{DIM}{d}");
            array_type = quote! {
                [#array_type; #dim_name]
            };
        }

        // Generate "From" trait implementation.
        from_implementations.extend(quote! {
            impl<T: PrimitiveType, #const_dims> From<#array_type> for Tensor<T, #rank_name<#dims>> {
                fn from(value: #array_type) -> Self {
                    Self {
                        data: #data_creator,
                        shape: shape!(#dims),
                    }
                }
            }
        });
    }

    format_token_stream(from_implementations)
}
