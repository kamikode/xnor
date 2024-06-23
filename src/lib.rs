#![no_std]
extern crate alloc;

pub mod shapes;
pub mod tensor;
pub mod tensor_ops;

pub mod prelude {
    pub use crate::shape;
    pub use crate::shapes::{Rank0, Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8, Rank9};
    pub use crate::tensor::{
        BoolTensor, F32Tensor, F64Tensor, I128Tensor, I16Tensor, I32Tensor, I64Tensor, I8Tensor,
        Tensor, U128Tensor, U16Tensor, U32Tensor, U64Tensor, U8Tensor,
    };
}
