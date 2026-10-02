use crate::dimensions::SCROLL_INDICATOR_CORNER_RADIUS;
use crate::theme::colors::MaterialColorScheme;
use cherenkov::kurbo::Rect;
use cherenkov::kurbo::{RoundedRect, RoundedRectRadii};
use cherenkov::{Draw as _, Recorder};

pub fn draw_indicator(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    draw.fill(
        RoundedRect::from_rect(
            bounds,
            RoundedRectRadii::from_single_radius(SCROLL_INDICATOR_CORNER_RADIUS),
        ),
        colors.on_surface_variant.working().with_alpha(0.55),
    );
}
