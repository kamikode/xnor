use xnor::prelude::*;

fn main() {
    let x = BoolArray::full(Shape([2, 3]), false);
    let x = I64Array::ones(Shape([12]));
    let x = F64Array::new([[0.0, 1.0], [2.0, 3.0]]);
}
