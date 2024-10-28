use eframe::egui::{ViewportBuilder, Pos2, CentralPanel, Color32, Stroke};
use eframe::{NativeOptions, run_simple_native};

pub mod simulation;
pub mod body;
pub mod mymath;
pub mod timemode;
// pub mod painter;

pub mod app;
use app::App;
// use painter::Painter;
use mymath::Vec2;


fn main() -> eframe::Result {
    let mut myapp: app::App = App::new();

    let options: NativeOptions = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([myapp.win_width, myapp.win_height])
            .with_position(Pos2 { x: 0.0, y: 0.0 }),
        ..Default::default()
    };

    let zoom = 150f32;
    let translate = 0.5 * Vec2 {
        x: myapp.win_width as f64,
        y: myapp.win_height as f64,
    };

    run_simple_native("Name", options, move |ctx, _frame| {
        CentralPanel::default().show(
            ctx,
            |ui| {
                ctx.request_repaint();
                let painter = ui.painter();

                myapp.handle_events(ctx);
                myapp.update();

                // render
                for a_body in myapp.simulation.bodies.iter_mut() {
                    let x = translate.x as f32 + zoom * a_body.pos.x as f32;
                    let y = translate.y as f32 + zoom * a_body.pos.y as f32;

                    let color: Color32;
                    color = Color32::from_rgb(
                        (255.0 * (1.0 - a_body.acc.mag_sqrd())) as u8,
                        (255.0 * (1.0 - a_body.vel.mag_sqrd())) as u8,
                        (255.0 * (1.0 - a_body.pos.mag_sqrd())) as u8,
                    );

                    let my_stroke = Stroke { width: 0.5, color: Color32::WHITE };
                    painter.circle(
                        Pos2 { x, y },
                        2.0,
                        color,
                        my_stroke,
                    );
                }
            },
        );
    })
}