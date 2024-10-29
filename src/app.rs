use eframe::egui::{Context, Key::Space};

use crate::simulation::Simulation;
use crate::timemode::TimeMode;

pub struct App {
    pub win_width: f32,
    pub win_height: f32,

    pub simulation: Simulation,

    pub time_mode: TimeMode,
    pub delta_time_factor: f64,
    pub delta_time: f64,
}

impl App {
    pub fn handle_events(self: &mut Self, ctx: &Context) {
        if ctx.input(|i| i.key_pressed(Space)) {
            self.time_mode.update();
            self.delta_time_factor = self.time_mode.return_time_factor();
        }
    }

    pub fn update(self: &mut Self) {
        let delta_time = self.delta_time_factor * self.delta_time;
        self.simulation.update(delta_time);
    }
}

impl Default for App {
    fn default() -> Self {
        let win_width: f32 = 960f32;
        let win_height: f32 = 540f32;

        let simulation: Simulation = Simulation::default();

        let time_mode = TimeMode::Pause;
        let delta_time = 1.0 / 60.0;
        let delta_time_factor = 0.0; // kind of pointless(gets updated in App::handle_events??

        Self {
            win_width,
            win_height,
            simulation,
            time_mode,
            delta_time,
            delta_time_factor,
        }
    }
}
