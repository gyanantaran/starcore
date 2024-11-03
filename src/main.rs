use eframe;
use eframe::egui;

pub mod app;
pub mod body;
pub mod flow;
pub mod gravity;
pub mod initializer;
pub mod model;
pub mod simulation;
pub mod vector;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([725., 725.]),
        ..Default::default()
    };

    eframe::run_native(
        app::App::name(),
        native_options,
        Box::new(|_| Ok(Box::<app::App>::default())),
    )
}
