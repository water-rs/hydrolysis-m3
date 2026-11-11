//! Visual acceptance for the Material shape set the loading indicator morphs
//! through.
//!
//! Numbers only say each outline closes; whether it is the right silhouette is
//! a question for the eye. The PNG shows all seven, plus a morph mid-flight.

use hydrolysis::draw::kurbo::{self, PathEl};
use hydrolysis_m3::material_shapes::{material_shape_sequence, morph, radii_to_path};
use waterui::layout::{Point as CanvasPoint, Size as CanvasSize};
use waterui::prelude::*;
use waterui_graphics::canvas::{Canvas, Cx, Path};
use waterui_graphics::color::{Color, Srgb};
use waterui_testing::{OffscreenApp, Styled, UiBuilder};

const CELL: f32 = 72.0;
const RADIUS: f64 = 30.0;

/// Fills one shape's outline centred in its own cell.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the canvas takes f32 coordinates; the outline is a few hundred points across"
)]
fn fill_radii(cx: &mut Cx, radii: &[f64], centre: kurbo::Point) {
    let outline = radii_to_path(radii, centre, RADIUS);
    let mut path = Path::builder();
    let mut first = true;
    kurbo::flatten(outline.iter(), 0.05, |element| match element {
        PathEl::MoveTo(point) | PathEl::LineTo(point) => {
            let point = CanvasPoint::new(point.x as f32, point.y as f32);
            path = if first {
                first = false;
                path.clone().move_to(point)
            } else {
                path.clone().line_to(point)
            };
        }
        _ => {}
    });
    cx.fill(
        path.close().build(),
        Color::from(Srgb::new(0.196, 0.184, 0.208)),
    );
}

#[expect(
    clippy::cast_precision_loss,
    reason = "the grid is four cells wide and two tall"
)]
fn shapes() -> impl View {
    Canvas::new(|cx: &mut Cx| {
        let sequence = material_shape_sequence();
        for (index, radii) in sequence.iter().enumerate() {
            let column = index % 4;
            let row = index / 4;
            let centre = kurbo::Point::new(
                f64::from(CELL) * (column as f64 + 0.5),
                f64::from(CELL) * (row as f64 + 0.5),
            );
            fill_radii(cx, radii, centre);
        }
        // The eighth cell shows a morph halfway between the last two shapes.
        let halfway = morph(&sequence[5], &sequence[6], 0.5);
        fill_radii(
            cx,
            &halfway,
            kurbo::Point::new(f64::from(CELL) * 3.5, f64::from(CELL) * 1.5),
        );
    })
    .intrinsic_size(Some(CanvasSize::new(CELL * 4.0, CELL * 2.0)))
}

#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (288, 144))]
fn the_material_shape_set_renders(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let mut app: OffscreenApp = ui.mount_offscreen(shapes);
    let _ = app.capture_snapshot("material3-preview", "material-shapes", "sequence");
}
