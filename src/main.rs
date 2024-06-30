use std::error::Error;
use xnor::prelude::*;

#[allow(unused_variables)] // TEMPORARY.
fn main() -> Result<(), Box<dyn Error>> {
    let x = Tensor::from([0.0, 1.0]);
    let y = BoolTensor::full(shape!(), false);
    let z = BoolTensor::full(shape!(1, 2, 3), false);
    println!("x ndim={}, size={}, shape={}", x.ndim(), x.size(), x.shape);
    println!("y ndim={}, size={}, shape={}", y.ndim(), y.size(), y.shape);
    println!("z ndim={}, size={}, shape={}", z.ndim(), z.size(), z.shape);

    let a = Tensor::from([[3.0, 2.0, 1.0], [1.0, 0.0, 2.0]]);
    let b = Tensor::from([[1.0, 2.0], [0.0, 1.0], [4.0, 0.0]]);
    // This is the result for the matmul of a and b.
    //let c = Tensor::from([[7.0, 8.0], [9.0, 2.0]]);
    //let c = matmul(a, b);
    //println!("c ndim={}, size={}, shape={}", c.ndim(), c.size(), c.shape);
    Ok(())
}
