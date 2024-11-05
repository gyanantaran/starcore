use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f64::consts::TAU;

use crate::body::Body;
use crate::vector::Vec2;

pub struct Initializer {
    pub rng: StdRng,
}

impl Initializer {
    pub fn init(self: &mut Self, num_bodies: usize) -> Vec<Body> {
        (0..num_bodies).map(|_| self.generate_body()).collect()
    }

    fn generate_body(self: &mut Self) -> Body {
        let theta: f64 = self.rng.gen_range(0.0..TAU);
        let radius: f64 = self.rng.gen_range(0.0..1.0);
        let unit_vec: Vec2 = Vec2 {
            x: theta.cos(),
            y: theta.sin(),
        };

        let pos: Vec2 = radius.sqrt() * unit_vec.clone();
        let vel = Vec2::zero();
        let acc: Vec2 = Vec2::zero(); // uniform: zero
        let mas: f64 = 1.0 - radius.sqrt();

        Body { pos, vel, acc, mas }
    }
}

impl Default for Initializer {
    fn default() -> Self {
        let seed: u64 = 0;
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}
