use crate::body::Body;
use crate::vector::Vec2;

pub struct Gravity {
    pub g_const: f64,
}

impl Gravity {
    pub fn calculate(&self, body1: &Body, body2: &Body) -> Vec2 {
        let separation = body2.pos - body1.pos;
        let mut distance_sqrd;
        distance_sqrd = separation.mag_squared();
        distance_sqrd = f64::max(distance_sqrd, 0.1);

        let force = (self.g_const * body2.mas * body1.mas / distance_sqrd) * separation.unit_vec();
        let acc = (1.0 / body1.mas) * force;

        acc
    }

    pub fn calculate_central(&self, body: &Body) -> Vec2 {
        let central_body = Body::default();
        self.calculate(body, &central_body)
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self { g_const: 1.0 }
    }
}
