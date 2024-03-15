//! # `tangential` -- a collection of tools for working on the sphere.
//!
//! This is a collection of tools for working with elements of the set:
//! $$\mathbb{S}^2 \= \left\\{ (x, y, z) : x^2 + y^2 + z^2 = 1 \right\\}$$
//!

mod position;
mod tangent;
mod barycentrics;

pub use position::*;
pub use tangent::*;
pub use barycentrics::*;

