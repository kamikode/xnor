use std::error::Error;
use xnor::prelude::*;

struct Rank0<const D0: usize>;

#[allow(unused_variables)] // TEMPORARY.
fn main() -> Result<(), Box<dyn Error>> {
    let x = BoolArray::full(Shape([2, 3]), false);
    let x = I64Array::ones(Shape([12]));
    let x = F64Array::new([[0.0, 1.0], [2.0, 3.0]]);
    println!("Hello, world!");
    let a = Rank0::<1> {};
    Ok(())
}
