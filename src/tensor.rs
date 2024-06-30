use crate::shape::*;
use alloc::sync::Arc;
use alloc::vec::Vec;
use num_traits::Num;

#[allow(dead_code)] // TEMPORARY.
#[derive(Debug)]
pub struct Tensor<T, S: Shape> {
    pub shape: S,
    data: Arc<Vec<T>>,
}

impl<T, S: Shape> Tensor<T, S> {
    #[inline]
    pub fn ndim(&self) -> usize {
        S::NDIM
    }

    #[inline]
    pub fn size(&self) -> usize {
        S::SIZE
    }
}

impl<T: Copy, S: Shape> Tensor<T, S> {
    pub fn full(shape: S, fill_value: T) -> Self {
        Tensor {
            data: Arc::new(alloc::vec![fill_value; S::SIZE]),
            shape,
        }
    }
}

impl<T: Copy + Num, S: Shape> Tensor<T, S> {
    pub fn ones(shape: S) -> Self {
        Tensor {
            data: Arc::new(alloc::vec![T::one(); S::SIZE]),
            shape,
        }
    }

    pub fn zeros(shape: S) -> Self {
        Tensor {
            data: Arc::new(alloc::vec![T::zero(); S::SIZE]),
            shape,
        }
    }
}

// Type aliases.
pub type BoolTensor<S> = Tensor<bool, S>;
pub type F32Tensor<S> = Tensor<f32, S>;
pub type F64Tensor<S> = Tensor<f64, S>;
pub type I8Tensor<S> = Tensor<i8, S>;
pub type I16Tensor<S> = Tensor<i16, S>;
pub type I32Tensor<S> = Tensor<i32, S>;
pub type I64Tensor<S> = Tensor<i64, S>;
pub type I128Tensor<S> = Tensor<i128, S>;
pub type U8Tensor<S> = Tensor<u8, S>;
pub type U16Tensor<S> = Tensor<u16, S>;
pub type U32Tensor<S> = Tensor<u32, S>;
pub type U64Tensor<S> = Tensor<u64, S>;
pub type U128Tensor<S> = Tensor<u128, S>;

// "From" trait implementations for converting arrays to tensors
include!(concat!(env!("OUT_DIR"), "/tensor_from_array.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use core::ops::Deref;

    #[test]
    fn full() {
        let x = Tensor::full(shape!(2, 3), 1.3);
        assert_eq!(x.shape, shape!(2, 3));
        assert_eq!(*x.data.deref(), vec![1.3; 6]);
    }

    #[test]
    fn ones() {
        let x = F64Tensor::ones(shape!(2, 3));
        assert_eq!(x.shape, shape!(2, 3));
        assert_eq!(*x.data.deref(), vec![1.0; 6])
    }

    #[test]
    fn zeros() {
        let x = F64Tensor::zeros(shape!(2, 3));
        assert_eq!(x.shape, shape!(2, 3));
        assert_eq!(*x.data.deref(), vec![0.0; 6])
    }

    #[test]
    fn from_0d() {
        let x = Tensor::from(3.2);
        assert_eq!(x.shape, shape!());
        assert_eq!(*x.data.deref(), vec![3.2; 1])
    }

    #[test]
    fn from_1d() {
        let x = Tensor::from([0.3, 1.4]);
        assert_eq!(x.shape, shape!(2));
        assert_eq!(*x.data.deref(), vec![0.3, 1.4])
    }

    #[test]
    fn from_2d() {
        let x = Tensor::from([[0.3, 1.3, 0.6], [3.0, 1.2, 0.5]]);
        assert_eq!(x.shape, shape!(2, 3));
        assert_eq!(*x.data.deref(), vec![0.3, 1.3, 0.6, 3.0, 1.2, 0.5])
    }

    #[test]
    fn from_3d() {
        let x = Tensor::from([[[0.3], [1.3], [0.6]], [[3.0], [1.2], [0.5]]]);
        assert_eq!(x.shape, shape!(2, 3, 1));
        assert_eq!(*x.data.deref(), vec![0.3, 1.3, 0.6, 3.0, 1.2, 0.5])
    }
}
