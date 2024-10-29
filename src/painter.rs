use crate::app::App;
use crate::mymath::Vec2;
use eframe::egui;

pub struct Painter {
    pub zoom: f32,
    pub translate: Vec2,
}

impl Painter {
    pub fn render(self: &Self, app: &App) {
        // render
        for a_body in myapp.simulation.bodies.iter_mut() {
            let x: f32 = self.translate.x + self.zoom * a_body.pos.x as f32;
            let y: f32 = self.translate.y + self.zoom * a_body.pos.y as f32;

            let color: egui::Color32;
            color = egui::Color32::from_rgb(
                (255.0 * (1.0 - a_body.acc.mag_sqrd())) as u8,
                (255.0 * (1.0 - a_body.vel.mag_sqrd())) as u8,
                (255.0 * (1.0 - a_body.pos.mag_sqrd())) as u8,
            );

            let my_stroke = egui::Stroke {
                width: 0.5,
                color: egui::Color32::WHITE,
            };
            self.painter
                .circle(egui::Pos2 { x, y }, 2.0, color, my_stroke);
        }
    }
}

// #[derive(Clone, Copy)]
// pub struct Painter<'frame> {
//     pub raw: &'frame egui::Painter,
//     pub transform: TSTransform,
// }

// impl<'frame> Painter<'frame> {
//     pub fn draw(&self, shape: impl Into<Shape>) {
//         let mut shape = shape.into();

//         shape.transform(self.transform);

//         self.raw.add(shape);
//     }
// }

// // egui::painter
// // size = 56 (0x38), align = 0x8
// pub struct Painter {
//     ctx: Context,
//     layer_id: LayerId,
//     clip_rect: Rect,
//     fade_to_color: Option<Color32>,
//     opacity_factor: f32,
// }
