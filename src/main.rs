use eframe::egui;

pub mod simulation;
pub mod body;
pub mod mymath;

use simulation::Simulation;

fn main() -> eframe::Result {
    let win_width: f32  = 960f32;
    let win_height: f32 = 540f32;
    let options: eframe::NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([win_width, win_height]),
        ..Default::default()
    };

    let num_bodies: usize = 3000;
    let mut my_simulation: Simulation = Simulation::new(num_bodies);

    let spread: f32 = 200 as f32;
    let my_stroke = egui::Stroke{width: 1.0, color: egui::Color32::BLACK};

    eframe::run_simple_native("My title", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My app");
            
            ctx.request_repaint();

            let painter:&egui::Painter = ui.painter();

            for mybody in my_simulation.bodies.iter_mut() {
                // update
                mybody.update(1.0/60.0); // let mybody = &mut my_simulation.bodies[i];

                // draw
                let x:f32 = (win_width / 2.0) + spread * mybody.pos.x as f32;
                let y:f32 = (win_height / 2.0) + spread * mybody.pos.y as f32;
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