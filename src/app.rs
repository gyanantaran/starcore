use crate::simulation::Simulation;
use crate::simulation::TimeMode;

use eframe::egui;

pub struct App {
    pub win_width : f32,
    pub win_height: f32,
    pub options   : eframe::NativeOptions,

    pub num_bodies: usize,
    pub simulation: Simulation,
    pub time_mode : TimeMode,
}

impl App {
    pub fn new() -> Self {
        let win_width: f32  = 960f32;
        let win_height: f32 = 540f32;
        let options: eframe::NativeOptions = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([win_width, win_height])
                .with_position(egui::Pos2{x:0.0, y:0.0}),
            ..Default::default()
        };

        let num_bodies: usize      = 3000;
        let simulation: Simulation = Simulation::new(num_bodies);
        let time_mode: TimeMode    = TimeMode::Pause;

        Self {
            win_width,
            win_height,
            options,
            num_bodies,
            simulation,
            time_mode,
        }
    }
}