use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f64::consts::TAU;

use crate::body::Body;
use crate::vector::Vec2;

pub struct Initializer {
    pub rng: StdRng,
}

impl Initializer {
    pub fn init(&mut self, num_bodies: usize) -> Vec<Body> {
        (0..num_bodies).map(|_| self.generate_body()).collect()
    }

    fn generate_body(&mut self) -> Body {
        let theta = self.rng.gen_range(0.0..TAU);
        let radius = self.rng.gen_range(0.0..10f64);
        let unit_vec = Vec2 {
            x: theta.cos(),
            y: theta.sin(),
        };

        let pos = radius/*.sqrt()*/ * unit_vec.clone();

        let theta = theta + TAU / 4.0;
        let unit_vec = Vec2 {
            x: theta.cos(),
            y: theta.sin(),
        };
        let vel = radius * unit_vec;
        let acc = Vec2::zero();
        let mas = 1.0 - radius.sqrt();

        Body { pos, vel, acc, mas }
    }
}

impl Default for Initializer {
    fn default() -> Self {
        let seed = 0;
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}
