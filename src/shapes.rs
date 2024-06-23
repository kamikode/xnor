extern crate xnor_macros;
use core::fmt::Display;
use core::ops::Index;
use xnor_macros::generate_rank_structs;

pub trait Shape {
    const NDIM: usize;
    const SIZE: usize;

    #[inline]
    fn ndim(&self) -> usize {
        Self::NDIM
    }

    #[inline]
    fn size(&self) -> usize {
        Self::SIZE
    }
}
pub trait HasAxis<const INDEX: isize>: Shape {
    const STRIDE: usize;
}
pub trait AxisAtIndexHasSize<const INDEX: isize, const SIZE: usize>: HasAxis<INDEX> {}

// Generates types from Rank0 to Rank9 implementing the shape traits.
generate_rank_structs!(9);

#[macro_export]
macro_rules! shape {
    () => {
        Rank0 {}
    };
    ($d0:expr) => {
        Rank1::<$d0> {}
    };
    ($d0:expr, $d1:expr) => {
        Rank2::<$d0, $d1> {}
    };
    ($d0:expr, $d1:expr, $d2:expr) => {
        Rank3::<$d0, $d1, $d2> {}
    };
    ($d0:expr, $d1:expr, $d2:expr, $d3:expr) => {
        Rank4::<$d0, $d1, $d2, $d3> {}
    };
    ($d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr) => {
        Rank5::<$d0, $d1, $d2, $d3, $d4> {}
    };
    ($d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr) => {
        Rank6::<$d0, $d1, $d2, $d3, $d4, $d5> {}
    };
    ($d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr, $d6:expr) => {
        Rank7::<$d0, $d1, $d2, $d3, $d4, $d5, $d6> {}
    };
    ($d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr, $d6:expr,  $d7:expr) => {
        Rank8::<$d0, $d1, $d2, $d3, $d4, $d5, $d6, $d7> {}
    };
    ($d0:expr, $d1:expr, $d2:expr, $d3:expr, $d4:expr, $d5:expr, $d6:expr,  $d7:expr, $d8:expr) => {
        Rank9::<$d0, $d1, $d2, $d3, $d4, $d5, $d6, $d7, $d8> {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn ndim() {
        assert_eq!(0, shape!().ndim());
        assert_eq!(1, shape!(0).ndim());
        assert_eq!(2, shape!(0, 1).ndim());
        assert_eq!(3, shape!(0, 1, 2).ndim());
        assert_eq!(4, shape!(0, 1, 2, 3).ndim());
        assert_eq!(5, shape!(0, 1, 2, 3, 4).ndim());
        assert_eq!(6, shape!(0, 1, 2, 3, 4, 5).ndim());
        assert_eq!(7, shape!(0, 1, 2, 3, 4, 5, 6).ndim());
        assert_eq!(8, shape!(0, 1, 2, 3, 4, 5, 6, 7).ndim());
        assert_eq!(9, shape!(0, 1, 2, 3, 4, 5, 6, 7, 8).ndim());
    }

    #[test]
    fn size() {
        assert_eq!(2usize.pow(0), shape!().size());
        assert_eq!(2usize.pow(1), shape!(2).size());
        assert_eq!(2usize.pow(2), shape!(2, 2).size());
        assert_eq!(2usize.pow(3), shape!(2, 2, 2).size());
        assert_eq!(2usize.pow(4), shape!(2, 2, 2, 2).size());
        assert_eq!(2usize.pow(5), shape!(2, 2, 2, 2, 2).size());
        assert_eq!(2usize.pow(6), shape!(2, 2, 2, 2, 2, 2).size());
        assert_eq!(2usize.pow(7), shape!(2, 2, 2, 2, 2, 2, 2).size());
        assert_eq!(2usize.pow(8), shape!(2, 2, 2, 2, 2, 2, 2, 2).size());
        assert_eq!(2usize.pow(9), shape!(2, 2, 2, 2, 2, 2, 2, 2, 2).size());
        assert_eq!(0, shape!(0, 2).size());
    }

    #[test]
    fn strides() {
        assert_eq!(<Rank3<6, 2, 4> as HasAxis::<0>>::STRIDE, 8);
        assert_eq!(<Rank3<6, 2, 4> as HasAxis::<1>>::STRIDE, 4);
        assert_eq!(<Rank3<6, 2, 4> as HasAxis::<2>>::STRIDE, 1);
        assert_eq!(<Rank3<6, 2, 4> as HasAxis::<-3>>::STRIDE, 8);
        assert_eq!(<Rank3<6, 2, 4> as HasAxis::<-2>>::STRIDE, 4);
        assert_eq!(<Rank3<6, 2, 4> as HasAxis::<-1>>::STRIDE, 1);
    }

    #[test]
    fn index() {
        let s1 = shape!(7);
        assert_eq!(s1[0], 7);
        assert_eq!(s1[-1], 7);
        let s2 = shape!(3, 8);
        assert_eq!(s2[0], 3);
        assert_eq!(s2[1], 8);
        assert_eq!(s2[-1], 8);
        assert_eq!(s2[-2], 3);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let _ = shape!(2, 3)[-3];
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds_0d() {
        let _ = shape!()[0];
    }

    #[test]
    fn to_string() {
        assert_eq!(shape!().to_string(), "()");
        assert_eq!(shape!(2).to_string(), "(2)");
        assert_eq!(shape!(2, 5).to_string(), "(2, 5)");
        assert_eq!(shape!(2, 5, 8).to_string(), "(2, 5, 8)");
    }
}
