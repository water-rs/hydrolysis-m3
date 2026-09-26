use crate::dimensions::{
    SLIDER_HANDLE_HEIGHT, SLIDER_HANDLE_PADDING, SLIDER_HANDLE_WIDTH, SLIDER_HORIZONTAL_INSET,
    SLIDER_HORIZONTAL_SPACING, SLIDER_MIN_TRACK_WIDTH, SLIDER_PRESSED_HANDLE_WIDTH,
    SLIDER_STOP_INDICATOR_SIZE, SLIDER_TRACK_HEIGHT, SLIDER_TRACK_INSIDE_CORNER_SIZE,
    SLIDER_VERTICAL_SPACING,
};
use crate::theme::colors::MaterialColorScheme;
use crate::{SliderMetrics, WidgetInteractionState};
use cherenkov::kurbo::{Circle, RoundedRect, RoundedRectRadii};
use cherenkov::{Draw as _, Recorder};

pub const fn metrics() -> SliderMetrics {
    SliderMetrics::new(
        SLIDER_HORIZONTAL_INSET,
        SLIDER_HORIZONTAL_SPACING,
        SLIDER_VERTICAL_SPACING,
        SLIDER_MIN_TRACK_WIDTH,
        SLIDER_TRACK_HEIGHT,
        SLIDER_HANDLE_WIDTH,
        SLIDER_HANDLE_HEIGHT,
    )
}

/// Draws the Expressive slider track: two bars separated by a gap around the
/// handle, with a stop indicator marking the far end.
///
/// Earlier Material ran a single hairline under a circular thumb. Expressive
/// splits the track at the handle — `ActiveHandlePadding` of clear space on each
/// side — so the handle reads as sitting *in* the track rather than on top of it.
pub fn draw_track(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    track_rect: cherenkov::kurbo::Rect,
    fill_rect: cherenkov::kurbo::Rect,
    state: WidgetInteractionState,
) {
    // MD3 disabled slider: the inactive track drops to on-surface at 12% and
    // the active track to on-surface at 38%.
    let (track_color, fill_color) = if state.disabled {
        (
            colors.on_surface.working_disabled_container(),
            colors.on_surface.working_disabled_content(),
        )
    } else {
        (
            colors.secondary_container.working(),
            colors.primary.working(),
        )
    };
    let outside = SLIDER_TRACK_HEIGHT / 2.0;
    let inside = SLIDER_TRACK_INSIDE_CORNER_SIZE;
    // The gap clears the handle's edge, not its centre, and it follows the
    // live handle width so pressing (a narrower handle) shrinks it.
    let handle_width = if state.pressed || state.focus_visible {
        SLIDER_PRESSED_HANDLE_WIDTH
    } else {
        SLIDER_HANDLE_WIDTH
    };
    let gap = handle_width / 2.0 + SLIDER_HANDLE_PADDING;

    // Inactive remainder, starting clear of the handle.
    let inactive_start = fill_rect.x1 + gap;
    if inactive_start < track_rect.x1 - outside {
        draw.fill(
            RoundedRect::from_rect(
                cherenkov::kurbo::Rect::new(
                    inactive_start,
                    track_rect.y0,
                    track_rect.x1,
                    track_rect.y1,
                ),
                cherenkov::kurbo::RoundedRectRadii::new(inside, outside, outside, inside),
            ),
            track_color,
        );
    }

    // Active portion, stopping clear of the handle.
    let active_end = fill_rect.x1 - gap;
    if active_end > track_rect.x0 + outside {
        draw.fill(
            RoundedRect::from_rect(
                cherenkov::kurbo::Rect::new(
                    track_rect.x0,
                    track_rect.y0,
                    active_end,
                    track_rect.y1,
                ),
                cherenkov::kurbo::RoundedRectRadii::new(outside, inside, inside, outside),
            ),
            fill_color,
        );
    }

    // Stop indicator: a dot at the track's far end, hidden once the handle
    // reaches it so the two never overlap.
    let indicator_center = cherenkov::kurbo::Point::new(
        track_rect.x1 - SLIDER_HANDLE_PADDING - SLIDER_STOP_INDICATOR_SIZE / 2.0,
        track_rect.y0 + track_rect.height() / 2.0,
    );
    if indicator_center.x > inactive_start {
        draw.fill(
            Circle::new(indicator_center, SLIDER_STOP_INDICATOR_SIZE / 2.0),
            if state.disabled {
                colors.on_surface.working_disabled_content()
            } else {
                colors.on_secondary_container.working()
            },
        );
    }
}

pub fn draw_thumb(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    center: cherenkov::kurbo::Point,
    _radius: f64,
    state: WidgetInteractionState,
) {
    let width = if state.pressed || state.focus_visible {
        SLIDER_PRESSED_HANDLE_WIDTH
    } else {
        SLIDER_HANDLE_WIDTH
    };
    let bounds = cherenkov::kurbo::Rect::from_center_size(center, (width, SLIDER_HANDLE_HEIGHT));
    // MD3 disabled slider handle: on-surface at 38% over an opaque surface
    // underlay, so content behind the semi-transparent handle cannot bleed
    // through (the reference implementation paints the handle over the background role).
    if state.disabled {
        draw.fill(
            RoundedRect::from_rect(bounds, RoundedRectRadii::from_single_radius(width / 2.0)),
            colors.background.working(),
        );
        draw.fill(
            RoundedRect::from_rect(bounds, RoundedRectRadii::from_single_radius(width / 2.0)),
            colors.on_surface.working_disabled_content(),
        );
        return;
    }
    draw.fill(
        RoundedRect::from_rect(bounds, RoundedRectRadii::from_single_radius(width / 2.0)),
        colors.primary.working(),
    );
}

/// The Expressive slider handle has no state layer.
///
/// `SliderTokens` carries no `StateLayerSize`: the feedback is the handle
/// itself narrowing from `HandleWidth` to `PressedHandleWidth` under the
/// finger. Drawing a ripple here would put a circle around a bar that Material
/// never shows.
pub const fn draw_thumb_state_layer(
    _colors: &MaterialColorScheme,
    _draw: &mut Recorder,
    _center: cherenkov::kurbo::Point,
    _radius: f64,
    _state: WidgetInteractionState,
) {
}

#[cfg(test)]
mod tests {
    use crate::test_support::Recorded;
    use cherenkov::kurbo::{Point, Rect};
    use cherenkov::{Paint, Recorder};

    use super::{MaterialColorScheme, WidgetInteractionState, draw_thumb, draw_track, metrics};
    use crate::dimensions::{
        SLIDER_HANDLE_HEIGHT, SLIDER_HANDLE_PADDING, SLIDER_HANDLE_WIDTH,
        SLIDER_PRESSED_HANDLE_WIDTH, SLIDER_STOP_INDICATOR_SIZE, SLIDER_TRACK_HEIGHT,
    };

    /// Values from `androidx.compose.material3.tokens.SliderTokens`.
    #[test]
    fn slider_metrics_match_compose_slider_tokens() {
        let metrics = metrics();

        assert_eq!(metrics.track_height, SLIDER_TRACK_HEIGHT);
        assert_eq!(metrics.handle_width, SLIDER_HANDLE_WIDTH);
        assert_eq!(metrics.handle_height, SLIDER_HANDLE_HEIGHT);
        // InactiveTrackHeight / ActiveTrackHeight
        assert_eq!(SLIDER_TRACK_HEIGHT, 16.0);
        // HandleWidth / HandleHeight
        assert_eq!(SLIDER_HANDLE_WIDTH, 4.0);
        assert_eq!(SLIDER_HANDLE_HEIGHT, 44.0);
        // PressedHandleWidth
        assert_eq!(SLIDER_PRESSED_HANDLE_WIDTH, 2.0);
        // ActiveHandlePadding
        assert_eq!(SLIDER_HANDLE_PADDING, 6.0);
        // StopIndicatorSize
        assert_eq!(SLIDER_STOP_INDICATOR_SIZE, 4.0);
    }

    /// The Expressive handle is a bar: 4dp wide and 44dp tall, not a circle.
    #[test]
    fn slider_thumb_draws_the_expressive_bar_handle() {
        let mut draw = Recorder::new();
        draw_thumb(
            &MaterialColorScheme::baseline_light(),
            &mut draw,
            Point::new(64.0, 48.0),
            SLIDER_HANDLE_HEIGHT / 2.0,
            WidgetInteractionState::NONE,
        );

        let draw = Recorded::from(draw);

        assert_eq!(draw.circle_fills.len(), 0);
        assert_eq!(draw.rounded_fills.len(), 1);
        assert_eq!(draw.rounded_fills[0].0.width(), SLIDER_HANDLE_WIDTH);
        assert_eq!(draw.rounded_fills[0].0.height(), SLIDER_HANDLE_HEIGHT);
    }

    /// Pressing narrows the handle rather than growing a state layer.
    #[test]
    fn slider_pressed_thumb_narrows() {
        let mut draw = Recorder::new();
        draw_thumb(
            &MaterialColorScheme::baseline_light(),
            &mut draw,
            Point::new(64.0, 48.0),
            SLIDER_HANDLE_HEIGHT / 2.0,
            WidgetInteractionState {
                pressed: true,
                ..WidgetInteractionState::NONE
            },
        );

        let draw = Recorded::from(draw);

        assert_eq!(draw.rounded_fills.len(), 1);
        assert_eq!(draw.rounded_fills[0].0.width(), SLIDER_PRESSED_HANDLE_WIDTH);
        assert_eq!(draw.rounded_fills[0].0.height(), SLIDER_HANDLE_HEIGHT);
    }

    #[test]
    fn disabled_slider_uses_disabled_palette() {
        // MD3 disabled slider: inactive track on-surface at 12%, active track
        // on-surface at 38%, handle on-surface at 38% over an opaque surface
        // underlay.
        let colors = MaterialColorScheme::baseline_light();
        let disabled = WidgetInteractionState {
            disabled: true,
            ..WidgetInteractionState::NONE
        };

        let mut track = Recorder::new();
        draw_track(
            &colors,
            &mut track,
            Rect::new(0.0, 0.0, 120.0, SLIDER_TRACK_HEIGHT),
            Rect::new(0.0, 0.0, 72.0, SLIDER_TRACK_HEIGHT),
            disabled,
        );
        let track = Recorded::from(track);
        assert!(matches!(
            &track.rounded_fills[0].2,
            Paint::Solid(color) if *color == colors.on_surface.working_disabled_container()
        ));
        assert!(matches!(
            &track.rounded_fills[1].2,
            Paint::Solid(color) if *color == colors.on_surface.working_disabled_content()
        ));

        let mut thumb = Recorder::new();
        draw_thumb(
            &colors,
            &mut thumb,
            Point::new(64.0, 48.0),
            SLIDER_HANDLE_HEIGHT / 2.0,
            disabled,
        );
        let thumb = Recorded::from(thumb);
        assert_eq!(
            thumb.rounded_fills.len(),
            2,
            "surface underlay then 38% handle"
        );
        assert!(matches!(
            &thumb.rounded_fills[0].2,
            Paint::Solid(color) if *color == colors.background.working()
        ));
        assert!(matches!(
            &thumb.rounded_fills[1].2,
            Paint::Solid(color) if *color == colors.on_surface.working_disabled_content()
        ));
    }

    #[test]
    fn slider_track_uses_material_role_colors() {
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = Recorder::new();
        draw_track(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 120.0, SLIDER_TRACK_HEIGHT),
            Rect::new(0.0, 0.0, 72.0, SLIDER_TRACK_HEIGHT),
            WidgetInteractionState::NONE,
        );

        let draw = Recorded::from(draw);

        assert_eq!(draw.rounded_fills.len(), 2);
        assert!(matches!(
            &draw.rounded_fills[0].2,
            Paint::Solid(color) if *color == colors.secondary_container.working()
        ));
        assert!(matches!(
            &draw.rounded_fills[1].2,
            Paint::Solid(color) if *color == colors.primary.working()
        ));
        // The track is split around the handle: each bar stops half a handle
        // width plus `ActiveHandlePadding` short of the value position.
        let gap = SLIDER_HANDLE_WIDTH / 2.0 + SLIDER_HANDLE_PADDING;
        assert_eq!(draw.rounded_fills[1].0.x1, 72.0 - gap);
        assert_eq!(draw.rounded_fills[0].0.x0, 72.0 + gap);
    }

    /// Corners facing the handle gap use the 2dp inside-corner size; only the
    /// outer ends are stadium.
    #[test]
    fn slider_track_gap_corners_use_the_inside_corner_size() {
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = Recorder::new();
        draw_track(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 120.0, SLIDER_TRACK_HEIGHT),
            Rect::new(0.0, 0.0, 72.0, SLIDER_TRACK_HEIGHT),
            WidgetInteractionState::NONE,
        );

        let outside = SLIDER_TRACK_HEIGHT / 2.0;
        let inside = 2.0;
        // rounded_fills[0] is the inactive remainder: inside corner on the
        // leading (gap) edge, stadium on the trailing edge.
        // rounded_fills[1] is the active bar: stadium leading, inside trailing.
        let draw = Recorded::from(draw);
        assert_eq!(draw.rounded_fills.len(), 2);
        let inactive_radii = draw.rounded_fills[0].1;
        assert_eq!(inactive_radii.top_left, inside);
        assert_eq!(inactive_radii.bottom_left, inside);
        assert_eq!(inactive_radii.top_right, outside);
        assert_eq!(inactive_radii.bottom_right, outside);
        let active_radii = draw.rounded_fills[1].1;
        assert_eq!(active_radii.top_left, outside);
        assert_eq!(active_radii.bottom_left, outside);
        assert_eq!(active_radii.top_right, inside);
        assert_eq!(active_radii.bottom_right, inside);
    }

    /// Pressing narrows the handle, so the track gap narrows with it.
    #[test]
    fn slider_track_gap_follows_the_pressed_handle_width() {
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = Recorder::new();
        draw_track(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 120.0, SLIDER_TRACK_HEIGHT),
            Rect::new(0.0, 0.0, 72.0, SLIDER_TRACK_HEIGHT),
            WidgetInteractionState {
                pressed: true,
                ..WidgetInteractionState::NONE
            },
        );

        let gap = SLIDER_PRESSED_HANDLE_WIDTH / 2.0 + SLIDER_HANDLE_PADDING;
        let draw = Recorded::from(draw);
        assert_eq!(draw.rounded_fills[1].0.x1, 72.0 - gap);
        assert_eq!(draw.rounded_fills[0].0.x0, 72.0 + gap);
    }
}
