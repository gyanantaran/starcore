use crate::mymath::Vec2;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mas: f64,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, acc: Vec2, mas: f64) -> Self {
        Self { pos, vel, acc, mas }
    }

    pub fn zero() -> Self {
        Self { pos: Vec2::zero(), vel: Vec2::zero(), acc: Vec2::zero(), mas: 0.0 }
    }

    pub fn update(self: &mut Self, delta_time: f64) {
        self.vel += delta_time * self.acc;
        self.pos += delta_time * self.vel;
    }
}