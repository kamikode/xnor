use core::fmt::Display;
use core::ops::Index;

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
// Not sure this is actually useful.
pub trait AxisAtIndexHasSize<const INDEX: isize, const SIZE: usize>: HasAxis<INDEX> {}

// Rank structures that implement shape traits.
include!(concat!(env!("OUT_DIR"), "/ranks.rs"));
pub use shape; // shape! macro (for defining Rank structures).

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
    }

    #[test]
    fn size() {
        assert_eq!(2usize.pow(0), shape!().size());
        assert_eq!(2usize.pow(1), shape!(2).size());
        assert_eq!(2usize.pow(2), shape!(2, 2).size());
        assert_eq!(2usize.pow(3), shape!(2, 2, 2).size());
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
