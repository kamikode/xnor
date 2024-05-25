#![no_std]
extern crate alloc;

pub mod shape;

pub mod prelude {
    pub use crate::shape::Shape;
}
