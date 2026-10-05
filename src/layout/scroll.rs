use crate::dimensions::SCROLL_INDICATOR_CORNER_RADIUS;
use crate::theme::colors::MaterialColorScheme;
use waterui_graphics::draw::kurbo::Rect;
use waterui_graphics::draw::kurbo::{RoundedRect, RoundedRectRadii};
use waterui_graphics::draw::{Draw as _, Recorder};

pub fn draw_indicator(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    draw.fill(
        RoundedRect::from_rect(
            bounds,
            RoundedRectRadii::from_single_radius(SCROLL_INDICATOR_CORNER_RADIUS),
        ),
        colors.on_surface_variant.working().with_alpha(0.55),
    );
}
