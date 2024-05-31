extern crate xnor_core;
use xnor_core::shape::{AxisAtIdxHasSize, HasAxis, Shape};
use xnor_macros::generate_rank_structs;
struct Rank0 {}
impl Shape for Rank0 {}
struct Rank1<const D0: usize> {}
impl<const D0: usize> Shape for Rank1<D0> {}
impl<const D0: usize> HasAxis<0isize> for Rank1<D0> {}
impl<const D0: usize> HasAxis<-1isize> for Rank1<D0> {}
impl<const D0: usize> AxisAtIdxHasSize<0isize, D0> for Rank1<D0> {}
impl<const D0: usize> AxisAtIdxHasSize<-1isize, D0> for Rank1<D0> {}
struct Rank2<const D0: usize, const D1: usize> {}
impl<const D0: usize, const D1: usize> Shape for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> HasAxis<0isize> for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> HasAxis<-2isize> for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> AxisAtIdxHasSize<0isize, D0> for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> AxisAtIdxHasSize<-2isize, D0> for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> HasAxis<1isize> for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> HasAxis<-1isize> for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> AxisAtIdxHasSize<1isize, D1> for Rank2<D0, D1> {}
impl<const D0: usize, const D1: usize> AxisAtIdxHasSize<-1isize, D1> for Rank2<D0, D1> {}
