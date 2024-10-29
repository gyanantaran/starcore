use crate::body::Body;
use crate::gravity::Gravity;
use crate::initializer::Initializer;

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub gravity: Gravity,
}

impl Simulation {
    pub fn update(self: &mut Self, delta_time: f64) {
        for a_body in self.bodies.iter_mut() {
            self.gravity.apply_central(a_body);
            a_body.update(delta_time);
        }
    }
}

impl Default for Simulation {
    fn default() -> Self {
        let num_bodies: usize = 5000;
        let mut my_initializer = Initializer::default();
        let bodies = my_initializer.init(num_bodies);
        let gravity = Gravity::default();
        Simulation { bodies, gravity }
    }
}
