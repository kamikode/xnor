#![no_std]
extern crate alloc;

pub mod array;
pub mod shape;

pub mod prelude {
    pub use crate::array::{
        Array, BoolArray, ConvertibleToArray, F32Array, F64Array, I128Array, I16Array, I32Array,
        I64Array, I8Array, U128Array, U16Array, U32Array, U64Array, U8Array,
    };
    pub use crate::shape::Shape;
}
