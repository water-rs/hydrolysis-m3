//! The Material 3 scroll indicator look.
//!
//! Material has no scroll indicator component: the thumb is a pill in
//! `on-surface-variant` at 55%, 2.5dp thick and widening to 6.5dp while it
//! is dragged.

use hydrolysis::draw::kurbo::{RoundedRect, RoundedRectRadii};
use hydrolysis::draw::{Draw as _, Paint};
use hydrolysis::theme::{Custom, Driven, ScrollIndicatorLook};
use waterui_core::Environment;

use crate::color::OnSurfaceVariant;
use crate::controls::button::solid_paint;
use crate::dimensions::{
    SCROLL_INDICATOR_CORNER_RADIUS, SCROLL_INDICATOR_DRAG_THICKNESS, SCROLL_INDICATOR_EDGE_INSET,
    SCROLL_INDICATOR_GUTTER, SCROLL_INDICATOR_MIN_EXTENT, SCROLL_INDICATOR_OPACITY,
    SCROLL_INDICATOR_THICKNESS,
};

/// The look of a scroll indicator.
pub fn look(env: &Environment) -> ScrollIndicatorLook {
    ScrollIndicatorLook::new(
        SCROLL_INDICATOR_THICKNESS,
        SCROLL_INDICATOR_DRAG_THICKNESS,
        thumb(env),
    )
    .edge_inset(SCROLL_INDICATOR_EDGE_INSET)
    .min_extent(SCROLL_INDICATOR_MIN_EXTENT)
    .gutter(SCROLL_INDICATOR_GUTTER)
}

fn thumb(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (solid_paint(
            OnSurfaceVariant.with_opacity(SCROLL_INDICATOR_OPACITY),
            env,
        ),),
        |recorder, frame, (paint,)| {
            recorder.fill(
                RoundedRect::from_rect(
                    frame,
                    RoundedRectRadii::from_single_radius(SCROLL_INDICATOR_CORNER_RADIUS),
                ),
                paint,
            );
        },
    )
}

#[cfg(test)]
mod tests {
    use super::look;
    use hydrolysis::Style as _;
    use waterui_core::Environment;

    #[test]
    fn thumb_keeps_the_geometry_the_renderer_drew() {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        let look = look(&env);

        assert_eq!(look.get_thickness(), 2.5);
        assert_eq!(look.get_drag_thickness(), 6.5);
        assert_eq!(look.get_edge_inset(), 1.5);
        assert_eq!(look.get_min_extent(), 12.0);
        assert_eq!(look.get_gutter(), 12.0);
    }
}
