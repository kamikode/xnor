#![no_std]
extern crate alloc;

pub mod shape;
pub mod tensor;
pub mod tensor_ops;

pub mod prelude {
    pub use crate::shape::shape;
    include!(concat!(env!("OUT_DIR"), "/ranks_import.rs"));
    pub use crate::tensor::{
        BoolTensor, F32Tensor, F64Tensor, I128Tensor, I16Tensor, I32Tensor, I64Tensor, I8Tensor,
        Tensor, U128Tensor, U16Tensor, U32Tensor, U64Tensor, U8Tensor,
    };
    pub use crate::tensor_ops::matmul;
}
