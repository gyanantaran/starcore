// TODO

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
//
// let background = containers::Frame {
//     inner_margin: epaint::Margin {
//         left: 10.,
//         right: 10.,
//         top: 10.,
//         bottom: 10.,
//     },
//     outer_margin: epaint::Margin {
//         left: 10.,
//         right: 10.,
//         top: 10.,
//         bottom: 10.,
//     },
//     rounding: Rounding {
//         nw: 1.0,
//         ne: 1.0,
//         sw: 1.0,
//         se: 1.0,
//     },
//     shadow: epaint::Shadow {
//         color: Color32::YELLOW,
//         offset: eframe::egui::Vec2 { x: 0., y: 0. },
//         blur: 0.,
//         spread: 0.,
//     },
//     fill: Color32::DARK_GRAY,
//     stroke: Stroke::new(2.0, Color32::DARK_GRAY),
// };
//
// .frame(background)
