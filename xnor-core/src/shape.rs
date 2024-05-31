pub trait Shape {}
pub trait HasAxis<const IDX: isize>: Shape {}
pub trait AxisAtIdxHasSize<const IDX: isize, const SIZE: usize>: Shape {}
