//! Material icon views drawn straight into the scene.
//!
//! These icons are stroked outlines rather than glyphs, so they are authored as
//! [`Canvas`] drawings: the icon records its path once and the canvas restrokes
//! it whenever the colour signal it resolved against changes, which is the
//! precise update the reactive contract asks for.

use hydrolysis::draw::WorkingColor;
use waterui::{Environment, View};
use waterui_core::layout::{Point, Size};
use waterui_core::reactive::SignalExt as _;
use waterui_core::resolve::Resolvable;
use waterui_graphics::canvas::{Canvas, Cap, Join, Path, StrokeStyle};

/// The miter limit Material icon strokes are authored against.
const MITER_LIMIT: f32 = 10.0;

/// A shared Material checkmark, drawn as a stroked polyline.
#[derive(Debug, Clone, Copy)]
pub struct CheckmarkIcon<ColorToken> {
    color: ColorToken,
    size: f32,
    line_width: f32,
    container_height: f32,
}

impl<ColorToken> CheckmarkIcon<ColorToken> {
    pub(crate) const fn new(color: ColorToken, size: f32, line_width: f32) -> Self {
        Self {
            color,
            size,
            line_width,
            container_height: size,
        }
    }

    pub(crate) const fn container_height(mut self, container_height: f32) -> Self {
        self.container_height = container_height;
        self
    }
}

impl<ColorToken> View for CheckmarkIcon<ColorToken>
where
    ColorToken: Resolvable<Resolved = WorkingColor> + 'static,
{
    fn body(self, env: &Environment) -> impl View {
        let Self {
            color,
            size,
            line_width,
            container_height,
        } = self;
        let stroke = color
            .resolve(env)
            .map(|color: WorkingColor| color.with_alpha(color.components[3].clamp(0.0, 1.0)));

        Canvas::new(move |cx| {
            // The polyline is authored in the icon's own box, which is centred
            // on the width layout gives the canvas and on the container height.
            let path = cx.size().map(move |canvas: Size| {
                let x = (canvas.width - size) * 0.5;
                let y = (container_height - size) * 0.5;
                Path::builder()
                    .move_to(Point::new(x + 4.25, y + 9.25))
                    .line_to(Point::new(x + 7.25, y + 12.25))
                    .line_to(Point::new(x + 14.0, y + 5.5))
                    .build()
            });
            cx.stroke(
                path,
                StrokeStyle::new(line_width)
                    .cap(Cap::Round)
                    .join(Join::Round)
                    .miter_limit(MITER_LIMIT),
                stroke,
            );
        })
        .intrinsic_size(Some(Size::new(size, size)))
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use hydrolysis::draw::WorkingColor;
    use waterui::Environment;
    use waterui::reactive::binding;
    use waterui_core::reactive::SignalExt as _;
    use waterui_core::reactive::signal::Computed;
    use waterui_core::resolve::Resolvable;
    use waterui_testing::{Snapshot, UiBuilder};

    use super::CheckmarkIcon;
    use crate::Material3;

    /// A colour token backed by a signal the test drives.
    #[derive(Debug, Clone)]
    struct Live(Computed<WorkingColor>);

    impl Resolvable for Live {
        type Resolved = WorkingColor;

        fn resolve(&self, _env: &Environment) -> impl waterui_core::Signal<Output = WorkingColor> {
            self.0.clone()
        }
    }

    /// The checkmark's stroke colour at the middle of its first segment, where
    /// the line is fully covered.
    fn stroke_pixel(snapshot: &Snapshot) -> [u8; 3] {
        // The icon is 18x18 and fills the viewport; (5.75, 10.75) is the
        // midpoint of the first segment in icon space.
        let (x, y) = (6_usize, 11_usize);
        let index = (y * snapshot.width() as usize + x) * 4;
        let pixel = &snapshot.rgba8()[index..index + 3];
        [pixel[0], pixel[1], pixel[2]]
    }

    #[test]
    fn checkmark_recolours_when_its_signal_changes() {
        let color = binding(WorkingColor::new([1.0, 0.0, 0.0, 1.0]));
        let token = Live(color.computed());
        let mut app = UiBuilder::new()
            .viewport(18, 18)
            .theme(Material3::defaults())
            .mount_offscreen(move || CheckmarkIcon::new(token.clone(), 18.0, 2.0));

        app.pump_for(Duration::from_millis(50));
        let red = stroke_pixel(&app.snapshot());
        assert!(
            red[0] > 200 && red[1] < 60 && red[2] < 60,
            "initial stroke should be red, got {red:?}"
        );

        color.set(WorkingColor::new([0.0, 0.0, 1.0, 1.0]));
        app.pump_for(Duration::from_millis(50));
        let blue = stroke_pixel(&app.snapshot());
        assert!(
            blue[2] > 200 && blue[0] < 60 && blue[1] < 60,
            "stroke should follow the signal to blue, got {blue:?}"
        );
    }
}
