use crate::vector::Vec2;

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

    pub fn update(&mut self, delta_time: f64) {
        self.vel += delta_time * self.acc;
        self.pos += delta_time * self.vel;
    }
}

impl Default for Body {
    fn default() -> Self {
        Self {
            pos: Vec2::default(),
            vel: Vec2::default(),
            acc: Vec2::default(),
            mas: 1.0,
        }
    }
}
