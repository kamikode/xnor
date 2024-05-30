use super::shape::Shape;
use alloc::sync::Arc;
use alloc::vec::Vec;
use num_traits::Num;

// TODO: Make the impl_convertible_to_array macro more powerful with
// procedural macros, see
// https://www.freecodecamp.org/news/procedural-macros-in-rust/#types-of-macros-in-rust

#[allow(dead_code)] // TEMPORARY
#[derive(Debug)]
pub struct Array<T, const N: usize> {
    shape: Shape<N>,
    data: Arc<Vec<T>>,
}

pub trait ConvertibleToArray<T, const N: usize> {
    fn shape(&self) -> Shape<N>;
    fn data(&self) -> Vec<T>;
}

macro_rules! nested_array {
    ($type:ty;) => { $type };
    ($type:ty; $size:expr) => { [$type; $size] };
    ($type:ty; $size:expr, $($rest:expr),+) => { [nested_array!($type; $($rest),+); $size] };
}
macro_rules! generate_data_fn {
    () => {
        fn data(&self) -> Vec<T> { alloc::vec![*self; 1] }
    };
    ($size:expr) => {
        fn data(&self) -> Vec<T> { self.iter().copied().collect() }
    };
    ($size:expr, $($rest:expr),+) => {
        fn data(&self) -> Vec<T> { self.iter().flat_map(|x| x.data()).collect() }
    };
}
macro_rules! impl_convertible_to_array {
    ($ndim:literal, {$($sizes:tt),*}, {$($generics:tt)*}) => {
        impl<T: Copy, $($generics)*> ConvertibleToArray<T, $ndim> for nested_array!(T; $($sizes),*) {
            fn shape(&self) -> Shape<$ndim> {
                Shape([$($sizes),*])
            }
            generate_data_fn!($($sizes),*);
        }
    };
}
impl_convertible_to_array!(0, {}, {});
impl_convertible_to_array!(1, {N}, {const N: usize});
impl_convertible_to_array!(2, {M, N}, {const M: usize, const N: usize});
impl_convertible_to_array!(3, {L, M, N}, 
    {const L: usize, const M: usize, const N: usize});
impl_convertible_to_array!(4, {K, L, M, N}, 
    {const K: usize, const L: usize, const M: usize, const N: usize});

impl<T: Copy, const N: usize> Array<T, N> {
    pub fn new(val: impl ConvertibleToArray<T, N>) -> Self {
        Array {
            data: Arc::new(val.data()),
            shape: val.shape(),
        }
    }

    pub fn full(shape: Shape<N>, fill_value: T) -> Self {
        Array {
            data: Arc::new(alloc::vec![fill_value; shape.size()]),
            shape,
        }
    }
}

impl<T: Copy + Num, const N: usize> Array<T, N> {
    pub fn ones(shape: Shape<N>) -> Self {
        Array {
            data: Arc::new(alloc::vec![T::one(); shape.size()]),
            shape,
        }
    }

    pub fn zeros(shape: Shape<N>) -> Self {
        Array {
            data: Arc::new(alloc::vec![T::zero(); shape.size()]),
            shape,
        }
    }
}

// Type aliases.
pub type BoolArray<const N: usize> = Array<bool, N>;
pub type F32Array<const N: usize> = Array<f32, N>;
pub type F64Array<const N: usize> = Array<f64, N>;
pub type I8Array<const N: usize> = Array<i8, N>;
pub type I16Array<const N: usize> = Array<i16, N>;
pub type I32Array<const N: usize> = Array<i32, N>;
pub type I64Array<const N: usize> = Array<i64, N>;
pub type I128Array<const N: usize> = Array<i128, N>;
pub type U8Array<const N: usize> = Array<u8, N>;
pub type U16Array<const N: usize> = Array<u16, N>;
pub type U32Array<const N: usize> = Array<u32, N>;
pub type U64Array<const N: usize> = Array<u64, N>;
pub type U128Array<const N: usize> = Array<u128, N>;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use core::ops::Deref;

    #[test]
    fn new_0d() {
        let x = Array::new(3.2);
        assert_eq!(x.shape, Shape([]));
        assert_eq!(*x.data.deref(), vec![3.2; 1])
    }

    #[test]
    fn new_1d() {
        let x = Array::new([0.3, 1.4]);
        assert_eq!(x.shape, Shape([2]));
        assert_eq!(*x.data.deref(), vec![0.3, 1.4])
    }

    #[test]
    fn new_2d() {
        let x = Array::new([[0.3, 1.3, 0.6], [3.0, 1.2, 0.5]]);
        assert_eq!(x.shape, Shape([2, 3]));
        assert_eq!(*x.data.deref(), vec![0.3, 1.3, 0.6, 3.0, 1.2, 0.5])
    }

    #[test]
    fn new_3d() {
        let x = Array::new([[[0.3], [1.3], [0.6]], [[3.0], [1.2], [0.5]]]);
        assert_eq!(x.shape, Shape([2, 3, 1]));
        assert_eq!(*x.data.deref(), vec![0.3, 1.3, 0.6, 3.0, 1.2, 0.5])
    }

    #[test]
    fn full() {
        let x = Array::full(Shape([2, 3]), 1.3);
        assert_eq!(x.shape, Shape([2, 3]));
        assert_eq!(*x.data.deref(), vec![1.3; 6]);
    }

    #[test]
    fn ones() {
        let x = F64Array::ones(Shape([2, 3]));
        assert_eq!(x.shape, Shape([2, 3]));
        assert_eq!(*x.data.deref(), vec![1.0; 6])
    }

    #[test]
    fn zeros() {
        let x = F64Array::zeros(Shape([2, 3]));
        assert_eq!(x.shape, Shape([2, 3]));
        assert_eq!(*x.data.deref(), vec![0.0; 6])
    }
}
