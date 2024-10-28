use crate::simulation::Simulation;
use eframe::egui::{Context, Key::Space};

pub struct App {
    pub win_width: f32,
    pub win_height: f32,

    pub num_bodies: usize,
    pub simulation: Simulation,
}

impl App {
    pub fn new() -> Self {
        let win_width: f32 = 960f32;
        let win_height: f32 = 540f32;

        let num_bodies: usize = 20000;
        let simulation: Simulation = Simulation::new(num_bodies);

        Self {
            win_width,
            win_height,
            num_bodies,
            simulation,
        }
    }

    pub fn handle_events(self: &mut Self, ctx: &Context) {
        if ctx.input(|i| i.key_pressed(Space)) {
            self.simulation.time_mode.update();
        }
    }

    pub fn update(self: &mut Self) {
        self.simulation.update();
    }
}