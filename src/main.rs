use std::error::Error;
use xnor::prelude::*;

#[allow(unused_variables)] // TEMPORARY.
fn main() -> Result<(), Box<dyn Error>> {
    let x = Tensor::from([0.0, 1.0]);
    let y = BoolTensor::full(shape!(), false);
    let z = BoolTensor::full(shape!(1, 2, 3, 4), false);

    println!("x ndim={}, size={}, shape={}", x.ndim(), x.size(), x.shape);
    println!("y ndim={}, size={}, shape={}", y.ndim(), y.size(), y.shape);
    println!("z ndim={}, size={}, shape={}", z.ndim(), z.size(), z.shape);
    //println!("z stride {}", z.shape.stride(-1));
    Ok(())
}
