use glam::Vec3A;
use crate::Pos;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Tangent(pub Vec3A);

impl Tangent {
    pub fn new(x: Vec3A) -> Self {
        Self(x)
    }

    pub fn after_other(self, relative_to: Pos, tangent: Tangent) -> Self {
        let y = self.0;
        let x = tangent.0;
        let p = relative_to.0;

        let kappa = (p + x).length();

        let result = y - y.dot(p + x)/(1.0 + kappa + p.dot(x)) * (p + kappa.recip() * (p + x));
        Self(result)
    }

    pub fn after_translation(self, from: Pos, to: Pos) -> Self {
        let q = self.0;
        let from = from.0;
        let to = to.0;

        let result = q - q.dot(to)/(1.0 + from.dot(to)) * (from + to);

        Self(result)
    }
}
