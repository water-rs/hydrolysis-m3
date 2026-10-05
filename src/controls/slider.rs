use crate::dimensions::{
    SLIDER_HANDLE_PADDING, SLIDER_HANDLE_WIDTH, SLIDER_HORIZONTAL_INSET, SLIDER_HORIZONTAL_SPACING,
    SLIDER_MIN_TRACK_WIDTH, SLIDER_PRESSED_HANDLE_WIDTH, SLIDER_STOP_INDICATOR_END_SPACE,
    SLIDER_STOP_INDICATOR_SIZE, SLIDER_TRACK_INSIDE_CORNER_SIZE,
    SLIDER_VALUE_INDICATOR_BOTTOM_SPACE, SLIDER_VALUE_INDICATOR_HORIZONTAL_PADDING,
    SLIDER_VALUE_INDICATOR_MIN_HEIGHT, SLIDER_VALUE_INDICATOR_MIN_WIDTH,
    SLIDER_VALUE_INDICATOR_VERTICAL_PADDING, SLIDER_VERTICAL_SPACING, slider_size_tokens,
};
use crate::theme::colors::MaterialColorScheme;
use crate::{SliderMetrics, SliderValueIndicatorMetrics, WidgetInteractionState};
use waterui::interaction::InteractionState;
use waterui::text::font::Font;
use waterui_controls::ControlSize;
use waterui_graphics::color::Color;
use waterui_graphics::draw::kurbo::{Circle, RoundedRect, RoundedRectRadii};
use waterui_graphics::draw::{Draw as _, Recorder};

/// `md.comp.slider.<size>.*`: the Expressive slider's track and handle grow
/// with the control size.
pub const fn metrics(size: ControlSize) -> SliderMetrics {
    let tokens = slider_size_tokens(size);
    SliderMetrics::new(
        SLIDER_HORIZONTAL_INSET,
        SLIDER_HORIZONTAL_SPACING,
        SLIDER_VERTICAL_SPACING,
        SLIDER_MIN_TRACK_WIDTH,
        tokens.track_height,
        SLIDER_HANDLE_WIDTH,
        tokens.handle_height,
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
    track_rect: waterui_graphics::draw::kurbo::Rect,
    fill_rect: waterui_graphics::draw::kurbo::Rect,
    size: ControlSize,
    state: WidgetInteractionState,
) {
    // MD3 disabled slider: the inactive track drops to on-surface at 12% and
    // the active track to on-surface at 38%.
    let (track_color, fill_color) = if state.state.contains(InteractionState::DISABLED) {
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
    // The outer corners take the size's shape token rather than the stadium
    // cap: `*-track-shape-{leading,trailing}` is not track_height / 2 above
    // x-small.
    let outside = slider_size_tokens(size).track_corner;
    let inside = SLIDER_TRACK_INSIDE_CORNER_SIZE;
    // The gap clears the handle's edge, not its centre, and it follows the
    // live handle width so pressing (a narrower handle) shrinks it.
    let handle_width = if state
        .state
        .intersects(InteractionState::PRESSED | InteractionState::FOCUSED)
    {
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
                waterui_graphics::draw::kurbo::Rect::new(
                    inactive_start,
                    track_rect.y0,
                    track_rect.x1,
                    track_rect.y1,
                ),
                waterui_graphics::draw::kurbo::RoundedRectRadii::new(
                    inside, outside, outside, inside,
                ),
            ),
            track_color,
        );
    }

    // Active portion, stopping clear of the handle.
    let active_end = fill_rect.x1 - gap;
    if active_end > track_rect.x0 + outside {
        draw.fill(
            RoundedRect::from_rect(
                waterui_graphics::draw::kurbo::Rect::new(
                    track_rect.x0,
                    track_rect.y0,
                    active_end,
                    track_rect.y1,
                ),
                waterui_graphics::draw::kurbo::RoundedRectRadii::new(
                    outside, inside, inside, outside,
                ),
            ),
            fill_color,
        );
    }

    // Stop indicator: a single dot at the track's trailing end,
    // `stop-indicator.trailing-space = 4` from the end edge, hidden where the
    // handle's track segment reaches it. A single-value slider has no leading
    // dot: `stop-indicator.trailing-space` is the only edge token and Compose
    // `SliderDefaults.TrackStopIndicatorSize` documents the dot "at the end of
    // the track" (MDC-Android exposes the size through the shared slider attr
    // `trackStopIndicatorSize`).
    let track_mid_y = track_rect.y0 + track_rect.height() / 2.0;
    let dot_offset = SLIDER_STOP_INDICATOR_END_SPACE + SLIDER_STOP_INDICATOR_SIZE / 2.0;
    let indicator_center =
        waterui_graphics::draw::kurbo::Point::new(track_rect.x1 - dot_offset, track_mid_y);
    if indicator_center.x > inactive_start {
        draw.fill(
            Circle::new(indicator_center, SLIDER_STOP_INDICATOR_SIZE / 2.0),
            if state.state.contains(InteractionState::DISABLED) {
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
    center: waterui_graphics::draw::kurbo::Point,
    _radius: f64,
    size: ControlSize,
    state: WidgetInteractionState,
) {
    let width = if state
        .state
        .intersects(InteractionState::PRESSED | InteractionState::FOCUSED)
    {
        SLIDER_PRESSED_HANDLE_WIDTH
    } else {
        SLIDER_HANDLE_WIDTH
    };
    let handle_height = slider_size_tokens(size).handle_height;
    let bounds =
        waterui_graphics::draw::kurbo::Rect::from_center_size(center, (width, handle_height));
    // MD3 disabled slider handle: on-surface at 38% over an opaque surface
    // underlay, so content behind the semi-transparent handle cannot bleed
    // through (the reference implementation paints the handle over the background role).
    if state.state.contains(InteractionState::DISABLED) {
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
    // md.comp.slider.handle.elevation = level1 (the disabled handle is level0,
    // handled above).
    crate::elevation::draw_shadows(
        draw,
        bounds,
        (width / 2.0).into(),
        crate::elevation::MaterialElevationLevel::LEVEL1,
        colors,
    );
    draw.fill(
        RoundedRect::from_rect(bounds, RoundedRectRadii::from_single_radius(width / 2.0)),
        colors.primary.working(),
    );
}

/// md.comp.slider.state-layer.size = 40: a 20dp-radius halo around the handle,
/// primary, gated on the standard hover/focus/pressed opacities. The disabled
/// handle takes no layer.
pub fn draw_thumb_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    center: waterui_graphics::draw::kurbo::Point,
    _radius: f64,
    _size: ControlSize,
    state: WidgetInteractionState,
) {
    if state.state.contains(InteractionState::DISABLED) {
        return;
    }
    crate::theme::state_layer::draw_unbounded_circle(
        draw,
        center,
        20.0,
        colors.primary.working(),
        state,
    );
}

/// `md.comp.slider.value-indicator.*` per m3.material.io/components/sliders/specs:
/// `active.bottom-space` is the gap to the handle's top edge, and the
/// `label.container` tokens fix the bubble at 44 high and at least 48 wide —
/// the renderer grows the width from the label's measure past 48.
pub const fn value_indicator_metrics() -> SliderValueIndicatorMetrics {
    SliderValueIndicatorMetrics::new(
        SLIDER_VALUE_INDICATOR_HORIZONTAL_PADDING,
        SLIDER_VALUE_INDICATOR_VERTICAL_PADDING,
        SLIDER_VALUE_INDICATOR_BOTTOM_SPACE,
        SLIDER_VALUE_INDICATOR_MIN_WIDTH,
        SLIDER_VALUE_INDICATOR_MIN_HEIGHT,
    )
}

/// `md.comp.slider.value-indicator.label.label-text.color`: inverse-on-surface.
pub fn value_indicator_color(colors: &MaterialColorScheme) -> Color {
    colors.inverse_on_surface.view_color()
}

/// `md.comp.slider.value-indicator.label.label-text.*`.
pub fn value_indicator_font() -> Font {
    crate::theme::typography::slider_value_indicator_label()
}

/// `md.comp.slider.value-indicator.container.color` (inverse-surface) on a
/// stadium bubble.
pub fn draw_value_indicator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: waterui_graphics::draw::kurbo::Rect,
) {
    draw.fill(
        RoundedRect::from_rect(
            bounds,
            RoundedRectRadii::from_single_radius(bounds.height() / 2.0),
        ),
        colors.inverse_surface.working(),
    );
}

#[cfg(test)]
mod tests {
    use crate::test_support::recorded;
    use waterui::interaction::InteractionState;
    use waterui_controls::ControlSize;
    use waterui_graphics::draw::Paint;
    use waterui_graphics::draw::kurbo::{Point, Rect, RoundedRectRadii};

    use super::{
        MaterialColorScheme, WidgetInteractionState, draw_thumb, draw_track, draw_value_indicator,
        metrics, value_indicator_metrics,
    };
    use crate::dimensions::{
        SLIDER_HANDLE_PADDING, SLIDER_HANDLE_WIDTH, SLIDER_PRESSED_HANDLE_WIDTH,
        SLIDER_STOP_INDICATOR_SIZE, SLIDER_VALUE_INDICATOR_BOTTOM_SPACE,
        SLIDER_VALUE_INDICATOR_HORIZONTAL_PADDING, SLIDER_VALUE_INDICATOR_MIN_HEIGHT,
        SLIDER_VALUE_INDICATOR_MIN_WIDTH, SLIDER_VALUE_INDICATOR_VERTICAL_PADDING,
        slider_size_tokens,
    };

    /// `md.comp.slider.<size>.*`: the track and handle height follow the
    /// control size; the width constants are size-invariant.
    #[test]
    fn slider_metrics_match_the_size_token_tables() {
        for (size, track_height, handle_height, corner) in [
            (ControlSize::ExtraSmall, 16.0, 44.0, 8.0),
            (ControlSize::Small, 24.0, 44.0, 8.0),
            (ControlSize::Medium, 40.0, 52.0, 12.0),
            (ControlSize::Large, 56.0, 68.0, 16.0),
            (ControlSize::ExtraLarge, 96.0, 108.0, 28.0),
        ] {
            let m = metrics(size);
            let tokens = slider_size_tokens(size);
            assert_eq!(m.track_height, tokens.track_height);
            assert_eq!(m.track_height, track_height);
            assert_eq!(m.handle_width, SLIDER_HANDLE_WIDTH);
            assert_eq!(m.handle_height, handle_height);
            assert_eq!(tokens.handle_height, handle_height);
            assert_eq!(tokens.track_corner, corner);
        }
        // HandleWidth / PressedHandleWidth / ActiveHandlePadding
        assert_eq!(SLIDER_HANDLE_WIDTH, 4.0);
        assert_eq!(SLIDER_PRESSED_HANDLE_WIDTH, 2.0);
        assert_eq!(SLIDER_HANDLE_PADDING, 6.0);
        // StopIndicatorSize
        assert_eq!(SLIDER_STOP_INDICATOR_SIZE, 4.0);
    }

    /// The Expressive handle is a bar: 4dp wide and 44dp tall, not a circle.
    #[test]
    fn slider_thumb_draws_the_expressive_bar_handle() {
        let draw = recorded(|draw| {
            draw_thumb(
                &MaterialColorScheme::baseline_light(),
                draw,
                Point::new(64.0, 48.0),
                22.0,
                ControlSize::ExtraSmall,
                WidgetInteractionState::NONE,
            );
        });

        assert_eq!(draw.circle_fills.len(), 0);
        assert_eq!(draw.rounded_fills.len(), 1);
        assert_eq!(draw.rounded_fills[0].0.width(), SLIDER_HANDLE_WIDTH);
        assert_eq!(draw.rounded_fills[0].0.height(), 44.0);
    }

    /// Pressing narrows the handle rather than growing a state layer.
    #[test]
    fn slider_pressed_thumb_narrows() {
        let draw = recorded(|draw| {
            draw_thumb(
                &MaterialColorScheme::baseline_light(),
                draw,
                Point::new(64.0, 48.0),
                22.0,
                ControlSize::ExtraSmall,
                WidgetInteractionState {
                    state: InteractionState::PRESSED,
                    ..WidgetInteractionState::NONE
                },
            );
        });

        assert_eq!(draw.rounded_fills.len(), 1);
        assert_eq!(draw.rounded_fills[0].0.width(), SLIDER_PRESSED_HANDLE_WIDTH);
        assert_eq!(draw.rounded_fills[0].0.height(), 44.0);
    }

    #[test]
    fn disabled_slider_uses_disabled_palette() {
        // MD3 disabled slider: inactive track on-surface at 12%, active track
        // on-surface at 38%, handle on-surface at 38% over an opaque surface
        // underlay.
        let colors = MaterialColorScheme::baseline_light();
        let disabled = WidgetInteractionState {
            state: InteractionState::DISABLED,
            ..WidgetInteractionState::NONE
        };

        let track = recorded(|draw| {
            draw_track(
                &colors,
                draw,
                Rect::new(0.0, 0.0, 120.0, 16.0),
                Rect::new(0.0, 0.0, 72.0, 16.0),
                ControlSize::ExtraSmall,
                disabled,
            );
        });
        assert!(matches!(
            &track.rounded_fills[0].2,
            Paint::Solid(color) if *color == colors.on_surface.working_disabled_container()
        ));
        assert!(matches!(
            &track.rounded_fills[1].2,
            Paint::Solid(color) if *color == colors.on_surface.working_disabled_content()
        ));

        let thumb = recorded(|draw| {
            draw_thumb(
                &colors,
                draw,
                Point::new(64.0, 48.0),
                22.0,
                ControlSize::ExtraSmall,
                disabled,
            );
        });
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
        let draw = recorded(|draw| {
            draw_track(
                &colors,
                draw,
                Rect::new(0.0, 0.0, 120.0, 16.0),
                Rect::new(0.0, 0.0, 72.0, 16.0),
                ControlSize::ExtraSmall,
                WidgetInteractionState::NONE,
            );
        });

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
        let draw = recorded(|draw| {
            draw_track(
                &colors,
                draw,
                Rect::new(0.0, 0.0, 120.0, 16.0),
                Rect::new(0.0, 0.0, 72.0, 16.0),
                ControlSize::ExtraSmall,
                WidgetInteractionState::NONE,
            );
        });

        let outside = slider_size_tokens(ControlSize::ExtraSmall).track_corner;
        let inside = 2.0;
        // rounded_fills[0] is the inactive remainder: inside corner on the
        // leading (gap) edge, stadium on the trailing edge.
        // rounded_fills[1] is the active bar: stadium leading, inside trailing.
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
        let draw = recorded(|draw| {
            draw_track(
                &colors,
                draw,
                Rect::new(0.0, 0.0, 120.0, 16.0),
                Rect::new(0.0, 0.0, 72.0, 16.0),
                ControlSize::ExtraSmall,
                WidgetInteractionState {
                    state: InteractionState::PRESSED,
                    ..WidgetInteractionState::NONE
                },
            );
        });

        let gap = SLIDER_PRESSED_HANDLE_WIDTH / 2.0 + SLIDER_HANDLE_PADDING;
        assert_eq!(draw.rounded_fills[1].0.x1, 72.0 - gap);
        assert_eq!(draw.rounded_fills[0].0.x0, 72.0 + gap);
    }
    /// `md.comp.slider.value-indicator.*`: stadium bubble at inverse-surface,
    /// bottom-space 12 from the handle top.
    #[test]
    fn value_indicator_uses_the_inverse_surface_tokens() {
        let colors = MaterialColorScheme::baseline_light();
        let m = value_indicator_metrics();
        assert_eq!(m.thumb_gap, SLIDER_VALUE_INDICATOR_BOTTOM_SPACE);
        assert_eq!(m.padding_x, SLIDER_VALUE_INDICATOR_HORIZONTAL_PADDING);
        assert_eq!(m.padding_y, SLIDER_VALUE_INDICATOR_VERTICAL_PADDING);
        assert_eq!(SLIDER_VALUE_INDICATOR_BOTTOM_SPACE, 12.0);
        // `md.comp.slider.value-indicator.label.container.{height,min-width}`:
        // 44 high, never narrower than 48.
        assert_eq!(m.min_height, SLIDER_VALUE_INDICATOR_MIN_HEIGHT);
        assert_eq!(m.min_width, SLIDER_VALUE_INDICATOR_MIN_WIDTH);
        assert_eq!(m.min_height, 44.0);
        assert_eq!(m.min_width, 48.0);

        let bounds = Rect::new(10.0, 20.0, 50.0, 48.0);
        let draw = recorded(|draw| {
            draw_value_indicator(&colors, draw, bounds);
        });
        assert_eq!(draw.rounded_fills.len(), 1);
        assert_eq!(draw.rounded_fills[0].0, bounds);
        assert_eq!(
            draw.rounded_fills[0].1,
            RoundedRectRadii::from_single_radius(14.0)
        );
        assert!(matches!(
            &draw.rounded_fills[0].2,
            Paint::Solid(color) if *color == colors.inverse_surface.working()
        ));
    }
}
