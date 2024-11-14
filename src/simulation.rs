use crate::body::Body;
use crate::gravity::Gravity;
use crate::initializer::Initializer;
use crate::vector::Vec2;

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub gravity: Gravity,
}

impl Simulation {
    pub fn update(&mut self, delta_time: f64) {
        self.update_acc();

        for i in 0..self.bodies.len() {
            self.bodies[i].update(delta_time);
        }
    }

    pub fn update_acc(&mut self) {
        self.straightforward_acc_update();
    }

    pub fn straightforward_acc_update(&mut self) {
        let num_bodies = self.bodies.len();
        for i in 0..num_bodies {
            self.bodies[i].acc = Vec2::zero();
            for j in 0..num_bodies {
                if i == j {
                    continue;
                }
                self.bodies[i].acc =
                    self.bodies[i].acc + self.gravity.calculate(&self.bodies[i], &self.bodies[j]);
            }
        }
    }
}

impl Default for Simulation {
    fn default() -> Self {
        let num_bodies = 2000;
        let mut my_initializer = Initializer::default();
        let bodies = my_initializer.init(num_bodies);
        let gravity = Gravity::default();

        Simulation { bodies, gravity }
    }
}
