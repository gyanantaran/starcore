use crate::body::Body;
use crate::mymath::Vec2;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::f64::consts::TAU;

pub struct Simulation {
    pub bodies: Vec<Body>,
}

pub enum TimeMode {
    Forward,
    Pause,
    Revind,
}

impl Simulation {
    pub fn new(num_bodies: usize) -> Self {
        let seed: u64 = 0;
        let mut rng = StdRng::seed_from_u64(seed);

        let mut bodies: Vec<Body> = Vec::<Body>::with_capacity(num_bodies);


        // one way of initialisation
        for _ in 0..(num_bodies-1) {
            let theta: f64     = rng.gen_range(0.0..TAU);
            let radius:f64     = rng.gen_range(0.0..1.0);
            let unit_vec: Vec2 = Vec2 { x: theta.cos(), y: theta.sin() };
            
            let pos: Vec2 = radius        * unit_vec.clone();   // disk: denser in center
            let vel: Vec2 = radius.sqrt() * unit_vec.clone();   // disk: uniform
            let acc: Vec2 = Vec2::zero();                       // constant: zero
            let mas: f64  = radius;                             // linear: massive outside

            bodies.push( Body { pos, vel, acc, mas } );
        }

        Self { bodies }
    }
}