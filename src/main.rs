use eframe::egui;

pub mod simulation;
pub mod body;
pub mod mymath;

pub mod app;

use simulation::TimeMode;
use app::App;

fn main() -> eframe::Result {
    let mut myapp: app::App = App::new();



    let spread: f32 = 200 as f32;
    let my_stroke = egui::Stroke{width: 1.0, color: egui::Color32::BLACK};
    let my_delta_time: f32     = 1.0/60.0;



    eframe::run_simple_native("My title", myapp.options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My app");
            let painter:&egui::Painter = ui.painter();

            ctx.request_repaint();

            // event handling ~loop~
            if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                match myapp.time_mode {
                    TimeMode::Forward => myapp.time_mode = TimeMode::Pause,
                    TimeMode::Pause   => myapp.time_mode = TimeMode::Revind,
                    TimeMode::Revind  => myapp.time_mode = TimeMode::Forward,
                }
            }
            // update
            for a_body in myapp.simulation.bodies.iter_mut() {
                match myapp.time_mode {
                    TimeMode::Forward => a_body.update(my_delta_time as f64),
                    TimeMode::Pause   => {},
                    TimeMode::Revind  => a_body.update(-1.0 * my_delta_time as f64),
                }
            }
            // render
            for a_body in myapp.simulation.bodies.iter_mut() {
                let x:f32 = (myapp.win_width / 2.0) + spread * a_body.pos.x as f32;
                let y:f32 = (myapp.win_height / 2.0) + spread * a_body.pos.y as f32;
                painter.circle(
                    egui::Pos2{x:x, y:y}, 
                    2.0, 
                    egui::Color32::BLACK, 
                    my_stroke,
                );
            }
        });
    })
}