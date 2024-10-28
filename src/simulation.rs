use crate::body::Body;
use crate::mymath::Vec2;
use crate::timemode::TimeMode;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f64::consts::TAU;

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub time_mode: TimeMode,
    pub delta_time: f64,
}

impl Simulation {
    pub fn new(num_bodies: usize) -> Self {
        let seed: u64 = 0;
        let mut rng = StdRng::seed_from_u64(seed);

        let mut bodies: Vec<Body> = Vec::<Body>::with_capacity(num_bodies);


        // another way of initialisation
        for _ in 0..(num_bodies - 1) {
            let theta: f64 = rng.gen_range(0.0..TAU);
            let radius: f64 = rng.gen_range(0.0..1.0);
            let unit_vec: Vec2 = Vec2 { x: theta.cos(), y: theta.sin() };

            let pos: Vec2 = radius * unit_vec.clone();   // parabolic: denser in center
            let vel: Vec2 = radius.sqrt() * unit_vec.clone();   // uniform: disk
            let acc: Vec2 = Vec2::zero();                       // uniform: zero
            let mas: f64 = radius;                             // linear: massive outside

            bodies.push(Body { pos, vel, acc, mas });
        }

        let time_mode = TimeMode::Pause;
        let delta_time = 1.0 / 60.0;

        Self { bodies, time_mode, delta_time }
    }

    pub fn update(self: &mut Self) {
        for a_body in self.bodies.iter_mut() {
            // update acceleration due to ~gravity~
            let center_pull: Vec2;
            let origin: Vec2;
            origin = Vec2 { x: 0.0, y: 0.0 };
            center_pull = origin - a_body.pos;
            a_body.acc = 0.6 * center_pull;

            // update body
            let delta_time = match self.time_mode {
                TimeMode::Forward => self.delta_time as f64,
                TimeMode::Pause => 0f64,
                TimeMode::Rewind => -1.0 * self.delta_time as f64,
            };
            a_body.update(delta_time);
        }
    }
}