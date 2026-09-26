use cherenkov::kurbo::{Circle, RoundedRect};
use cherenkov::kurbo::{Rect, RoundedRectRadii};
use cherenkov::{Draw as _, Recorder};
use waterui_backend_core::widget::BadgeMetrics;

use crate::theme::colors::MaterialColorScheme;

pub const fn metrics() -> BadgeMetrics {
    BadgeMetrics::new(6.0, 16.0, 4.0, 6.0, 6.0, 12.0, 14.0)
}

pub fn label_color(colors: &MaterialColorScheme) -> waterui_graphics::color::Color {
    colors.on_error.view_color()
}

pub fn draw_small(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    let radius = bounds.height().min(bounds.width()) * 0.5;
    draw.fill(Circle::new(bounds.center(), radius), colors.error.working());
}

pub fn draw_large(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    let radius = bounds.height() * 0.5;
    draw.fill(
        RoundedRect::from_rect(bounds, RoundedRectRadii::from_single_radius(radius)),
        colors.error.working(),
    );
}

#[cfg(test)]
mod tests {
    use super::metrics;

    #[test]
    fn badge_metrics_match_compose_badge_tokens() {
        let metrics = metrics();

        assert_eq!(metrics.small_size, 6.0);
        assert_eq!(metrics.large_size, 16.0);
        assert_eq!(metrics.large_horizontal_padding, 4.0);
        assert_eq!(metrics.small_offset_x, 6.0);
        assert_eq!(metrics.small_offset_y, 6.0);
        assert_eq!(metrics.large_offset_x, 12.0);
        assert_eq!(metrics.large_offset_y, 14.0);
    }
}
