//! The Material 3 badge look from the `md.comp.badge.*` tokens.

use hydrolysis::draw::kurbo::{Circle, Rect, RoundedRect, RoundedRectRadii};
use hydrolysis::draw::{Draw as _, Paint};
use hydrolysis::theme::{BadgeLook, BadgeMetrics, Custom, Driven};
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{Error, OnError};
use crate::controls::button::solid_paint;
use crate::theme::typography;

pub const fn metrics() -> BadgeMetrics {
    BadgeMetrics::new(6.0, 16.0, 4.0, 6.0, 6.0, 12.0, 14.0)
}

/// The look of a badge: the `error` dot (`md.comp.badge.size` 6dp) and count
/// pill (`md.comp.badge.large.size` 16dp), whose count is `label-small` in
/// `on-error` (`md.comp.badge.large.label-text`).
pub fn look(env: &Environment) -> BadgeLook {
    BadgeLook::new(
        metrics(),
        typography::label_small(),
        Color::from(OnError),
        small(env),
        large(env),
    )
}

/// The dot: a circle across its rect's shorter side.
fn small(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (solid_paint(Error, env),),
        |recorder, frame: Rect, (paint,)| {
            let radius = frame.height().min(frame.width()) * 0.5;
            recorder.fill(Circle::new(frame.center(), radius), paint);
        },
    )
}

/// The count pill: fully round ends.
fn large(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (solid_paint(Error, env),),
        |recorder, frame: Rect, (paint,)| {
            let radius = frame.height() * 0.5;
            recorder.fill(
                RoundedRect::from_rect(frame, RoundedRectRadii::from_single_radius(radius)),
                paint,
            );
        },
    )
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
