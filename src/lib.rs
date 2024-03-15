//! # `tangential` -- a collection of tools for working on the sphere.
//!
//! This is a collection of tools for working with elements of the set:
//! $$\mathbb{S}^2 \= \left\\{ (x, y, z) : x^2 + y^2 + z^2 = 1 \right\\}$$
//!

use glam::Vec3A;

#[inline]
pub fn project_point_to_tangent(center_of_projection: Vec3A, x: Vec3A) -> Vec3A {
    x * center_of_projection.dot(x).recip() - center_of_projection
}
