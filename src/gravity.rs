use crate::body::Body;
use crate::vector::Vec2;

pub struct Gravity {
    pub g_const: f64,
}

impl Gravity {
    pub fn apply(self: &Self, body1: &mut Body, body2: &Body) {
        let separation: Vec2 = body2.pos - body1.pos;
        let distance_sqrd: f64 = separation.mag_squared();
        let force: Vec2 =
            (self.g_const * body2.mas * body1.mas / distance_sqrd) * separation.unit_vec();

        let acc: Vec2 = (1.0 / body1.mas) * force;
        body1.acc = acc;
    }

    pub fn apply_central(self: &Self, body: &mut Body) {
        let central_body: Body = Body::default();
        self.apply(body, &central_body);
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self { g_const: 1.0 }
    }
}
