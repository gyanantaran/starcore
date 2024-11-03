use eframe::egui;

use crate::flow::Flow;
use crate::simulation::Simulation;

pub struct App {
    pub simulation: Simulation,

    pub time_mode: Flow,
    pub delta_time_factor: f64,
    pub delta_time: f64,
}

impl App {
    pub fn name() -> &'static str {
        "Star Core"
    }

    pub fn handle_events(self: &mut Self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.time_mode.update();
            self.delta_time_factor = self.time_mode.return_time_factor().into();
        }

        // if (ctx.input(|i| i.)) // window resized
    }

    pub fn update(self: &mut Self) {
        let delta_time = self.delta_time_factor * self.delta_time;
        self.simulation.update(delta_time);
    }
}

impl Default for App {
    fn default() -> Self {
        let simulation: Simulation = Simulation::default();

        let time_mode = Flow::default();
        let delta_time = 1.0 / 60.0;
        let delta_time_factor = 0.0; // kind of pointless(gets updated in App::handle_events)??

        Self {
            simulation,
            time_mode,
            delta_time,
            delta_time_factor,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.handle_events(ctx);
            self.update();

            // render
            let window_size = ctx.input(|i| i.viewport().outer_rect);
            let zoom = 200f32;
            let translate = 0.5
                * egui::Vec2 {
                    x: window_size.unwrap().width(),
                    y: window_size.unwrap().height(),
                };

            ctx.request_repaint();
            let painter = ui.painter();
            for a_body in self.simulation.bodies.iter_mut() {
                let x = translate.x + zoom * a_body.pos.x as f32;
                let y = translate.y + zoom * a_body.pos.y as f32;

                let color: egui::Color32;
                // color = egui::Color32::from_rgb(
                //     (255.0 * (1.0 - a_body.acc.mag_squared())) as u8,
                //     (255.0 * (1.0 - a_body.vel.mag_squared())) as u8,
                //     (255.0 * (1.0 - a_body.pos.mag_squared())) as u8,
                // ); // interesting overflow
                color = egui::Color32::from_rgba_unmultiplied(
                    (255.0) as u8,
                    (255.0) as u8,
                    (255.0) as u8,
                    (255.0 * a_body.mas) as u8,
                ); // interesting overflow

                let my_stroke = egui::Stroke {
                    width: 0.,
                    color: egui::Color32::WHITE,
                };
                painter.circle(
                    egui::Pos2 { x, y },
                    a_body.mas.sqrt() as f32,
                    color,
                    my_stroke,
                );
            }
        });
    }
}
