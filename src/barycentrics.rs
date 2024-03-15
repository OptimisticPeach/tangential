use crate::Pos;

pub struct Bary(pub [f32; 3]);

impl Bary {
    pub fn new(center: Pos, around: [Pos; 3]) -> Bary {
        let [a, b, c] = around.map(|x| center.tangent_towards(x).0);

        let v0 = b - a;
        let v1 = c - a;
        let v2 = -a;

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let denom = (d00 * d11 - d01 * d01).recip();

        let v = (d11 * d20 - d01 * d21) * denom;
        let w = (d00 * d21 - d01 * d20) * denom;

        let u = 1.0 - v - w;

        Self([u, v, w])
    }
}
