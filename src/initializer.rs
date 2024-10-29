use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f64::consts::TAU;

use crate::body::Body;
use crate::model::Model;
use crate::mymath::Vec2;

pub struct Initializer {
    pub rng: StdRng,
    pub pos_init: Model,
    pub vel_init: Model,
    pub acc_init: Model,
    pub mas_init: Model,
}

impl Initializer {
    pub fn init(self: &mut Self, num_bodies: usize) -> Vec<Body> {
        let mut bodies: Vec<Body> = Vec::<Body>::new();
        for _ in 0..(num_bodies - 1) {
            let theta: f64 = self.rng.gen_range(0.0..TAU);
            let radius: f64 = self.rng.gen_range(0.0..1.0);
            let unit_vec: Vec2 = Vec2 {
                x: theta.cos(),
                y: theta.sin(),
            };

            let pos: Vec2 = radius * unit_vec.clone(); // parabolic: denser in center
            let vel: Vec2 = radius.sqrt() * unit_vec.clone(); // uniform: disk
            let acc: Vec2 = Vec2::zero(); // uniform: zero
            let mas: f64 = radius; // linear: massive outside

            bodies.push(Body { pos, vel, acc, mas });
        }

        bodies
    }
}

impl Default for Initializer {
    fn default() -> Self {
        let seed: u64 = 0;
        Self {
            rng: StdRng::seed_from_u64(seed),
            pos_init: Model::Polynomial,
            vel_init: Model::Uniform,
            acc_init: Model::Uniform,
            mas_init: Model::Uniform,
        }
    }
}
