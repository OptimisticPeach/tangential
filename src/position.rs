use glam::Vec3A;
use crate::tangent::Tangent;

/// A position on $\mathbb{S}^2$.
///
/// This is `#[repr(transparent)]` so it has the same representation as a [`Vec3A`].
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Pos(pub Vec3A);

impl Pos {
    pub fn new(pos: Vec3A) -> Self {
        Self(pos.normalize())
    }

    pub fn from_prenormalized(pos: Vec3A) -> Self {
        Self(pos)
    }

    pub fn tangent_towards(self, other: Pos) -> Tangent {
        Tangent(other.0 * other.0.dot(self.0).recip() - self.0)
    }

    pub fn apply_tangent(self, tangent: Tangent) -> Pos {
        Pos((self.0 + tangent.0).normalize())
    }
}