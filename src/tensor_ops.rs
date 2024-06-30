use crate::shape::{AxisAtIndexHasSize, Rank2, Shape};
use crate::tensor::Tensor;

pub fn matmul<T, S1, S2, const D: usize>(
    _x: Tensor<T, S1>,
    _y: Tensor<T, S2>,
) -> Tensor<f32, Rank2<1, 1>>
where
    S1: Shape + AxisAtIndexHasSize<-1, D>,
    S2: Shape + AxisAtIndexHasSize<-2, D>,
{
    todo!();
    //Tensor::from([[0.0]])
}
