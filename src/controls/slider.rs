//! The Material 3 Expressive slider look from the `md.comp.slider.*` tokens.
//!
//! Expressive replaced the circular thumb on a hairline track with a narrow
//! vertical bar on a thick track, split by a gap around the handle and marked
//! with a stop indicator at the far end. Every part is a function of the
//! normalized value and the interaction state, so a drag moves operands and
//! records nothing.

use hydrolysis::draw::kurbo::{Circle, Point, Rect, RoundedRect, RoundedRectRadii, Size};
use hydrolysis::draw::{Draw as _, Paint, Shadow, WorkingColor};
use hydrolysis::theme::{
    Custom, DragMapping, Driven, Inputs, Knob, SliderLayout, SliderLook, SliderSpec, ValueIndicator,
};
use waterui::interaction::InteractionState;
use waterui::reactive::zip::zip;
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{
    Background, InverseOnSurface, InverseSurface, OnSecondaryContainer, OnSurface,
    SecondaryContainer, Shadow as ShadowRole,
};
use crate::controls::button::disabled_paint;
use crate::dimensions::{
    SLIDER_HANDLE_PADDING, SLIDER_HANDLE_WIDTH, SLIDER_HORIZONTAL_INSET, SLIDER_HORIZONTAL_SPACING,
    SLIDER_MIN_TRACK_WIDTH, SLIDER_PRESSED_HANDLE_WIDTH, SLIDER_STOP_INDICATOR_END_SPACE,
    SLIDER_STOP_INDICATOR_SIZE, SLIDER_TRACK_INSIDE_CORNER_SIZE,
    SLIDER_VALUE_INDICATOR_BOTTOM_SPACE, SLIDER_VALUE_INDICATOR_HORIZONTAL_PADDING,
    SLIDER_VALUE_INDICATOR_MIN_HEIGHT, SLIDER_VALUE_INDICATOR_MIN_WIDTH,
    SLIDER_VALUE_INDICATOR_VERTICAL_PADDING, SLIDER_VERTICAL_SPACING, SliderSizeTokens,
    slider_size_tokens,
};
use crate::elevation::{MaterialElevationLevel, chrome_elevation};
use crate::theme::colors::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY};
use crate::theme::state_layer::{
    FOCUS_STATE_LAYER_OPACITY, HOVER_STATE_LAYER_OPACITY, PRESSED_STATE_LAYER_OPACITY,
};
use crate::theme::{motion, typography};

/// A bar of the track: its shape over the value, and its paint.
type BarPart = Custom<f64, (Driven<RoundedRect, f64>, Driven<Paint, f64>)>;
/// The state layer: its halo over the value, and its paint.
type HaloPart = Custom<f64, (Driven<Circle, f64>, Driven<Paint, f64>)>;
/// The handle: its shape over the value, its key and ambient shadows, the
/// underlay of a disabled handle and its paint.
type HandlePart = Custom<
    f64,
    (
        Driven<RoundedRect, f64>,
        Driven<Shadow, f64>,
        Driven<Shadow, f64>,
        Driven<Paint, f64>,
        Driven<Paint, f64>,
    ),
>;

/// `md.comp.slider.state-layer.size` = 40: the halo around the handle.
const STATE_LAYER_RADIUS: f64 = 20.0;

/// The look of a slider in `spec`.
///
/// The active track, the handle and the state layer are drawn in the spec's
/// tint, which is `Primary` unless a subtree tints the slider.
pub fn look(spec: &SliderSpec, env: &Environment) -> SliderLook {
    let tokens = slider_size_tokens(spec.size());
    let tint = spec.tint();
    let layout = SliderLayout {
        row_height: tokens.handle_height.max(tokens.track_height),
        track_inset: SLIDER_HORIZONTAL_INSET,
        label_inset: SLIDER_HORIZONTAL_INSET,
        label_gap: SLIDER_HORIZONTAL_SPACING,
        minimum_track: SLIDER_MIN_TRACK_WIDTH,
        label_spacing: Some(SLIDER_VERTICAL_SPACING),
    };
    let knob = Knob {
        size: Size::new(SLIDER_HANDLE_WIDTH, tokens.handle_height),
        hit: motion::hit_region(),
    };
    let mut look = SliderLook::new(layout, knob, DragMapping::Absolute, motion::chrome())
        .part(inactive_track(&tokens, env))
        .part(active_track(&tokens, tint.clone(), env))
        .part(stop_indicator(env))
        .part(state_layer(&tint, env))
        .part(handle(&tokens, tint, env));
    if spec.has_value_indicator() {
        look = look.value_indicator(value_indicator(env));
    }
    look
}

/// The handle's width: narrower while it is held or focused.
fn handle_width(state: InteractionState) -> f64 {
    if state.intersects(InteractionState::PRESSED | InteractionState::FOCUSED) {
        SLIDER_PRESSED_HANDLE_WIDTH
    } else {
        SLIDER_HANDLE_WIDTH
    }
}

/// The clear space either side of the handle's centre: the gap clears the
/// handle's edge, not its centre, and follows the live width so pressing
/// shrinks it.
fn handle_gap(state: InteractionState) -> f64 {
    handle_width(state) / 2.0 + SLIDER_HANDLE_PADDING
}

/// The handle's centre for the normalized `value` on `frame`.
const fn handle_center(frame: Rect, value: f64) -> Point {
    Point::new(value.mul_add(frame.width(), frame.x0), frame.center().y)
}

/// The track's extent: `frame` is the row the handle travels in, the track
/// is centred in it.
fn track_rect(frame: Rect, tokens: &SliderSizeTokens) -> Rect {
    Rect::from_center_size(frame.center(), (frame.width(), tokens.track_height))
}

/// The inactive remainder, starting clear of the handle: the inside corner
/// on the leading edge facing the gap, the size's shape token on the trailing
/// end. Empty where the handle's gap reaches the end.
fn inactive_rect(
    frame: Rect,
    value: f64,
    state: InteractionState,
    tokens: &SliderSizeTokens,
) -> RoundedRect {
    let track = track_rect(frame, tokens);
    let outside = tokens.track_corner;
    let start = handle_center(frame, value).x + handle_gap(state);
    let rect = if start < track.x1 - outside {
        Rect::new(start, track.y0, track.x1, track.y1)
    } else {
        Rect::new(track.x1, track.y0, track.x1, track.y1)
    };
    RoundedRect::from_rect(
        rect,
        RoundedRectRadii::new(
            SLIDER_TRACK_INSIDE_CORNER_SIZE,
            outside,
            outside,
            SLIDER_TRACK_INSIDE_CORNER_SIZE,
        ),
    )
}

/// The active bar, stopping clear of the handle. Empty where the handle's
/// gap reaches the start.
fn active_rect(
    frame: Rect,
    value: f64,
    state: InteractionState,
    tokens: &SliderSizeTokens,
) -> RoundedRect {
    let track = track_rect(frame, tokens);
    let outside = tokens.track_corner;
    let end = handle_center(frame, value).x - handle_gap(state);
    let rect = if end > track.x0 + outside {
        Rect::new(track.x0, track.y0, end, track.y1)
    } else {
        Rect::new(track.x0, track.y0, track.x0, track.y1)
    };
    RoundedRect::from_rect(
        rect,
        RoundedRectRadii::new(
            outside,
            SLIDER_TRACK_INSIDE_CORNER_SIZE,
            SLIDER_TRACK_INSIDE_CORNER_SIZE,
            outside,
        ),
    )
}

/// The handle bar at the normalized `value`.
fn handle_rect(
    frame: Rect,
    value: f64,
    state: InteractionState,
    tokens: &SliderSizeTokens,
) -> RoundedRect {
    let width = handle_width(state);
    RoundedRect::from_rect(
        Rect::from_center_size(handle_center(frame, value), (width, tokens.handle_height)),
        RoundedRectRadii::from_single_radius(width / 2.0),
    )
}

/// The inactive remainder of the track: secondary-container, on-surface at
/// 12% while disabled.
fn inactive_track(tokens: &SliderSizeTokens, env: &Environment) -> BarPart {
    let tokens = *tokens;
    Custom::new(
        (
            Driven::new(move |inputs: &Inputs<'_, f64>| {
                inactive_rect(inputs.frame, *inputs.value, inputs.state, &tokens)
            }),
            disabled_paint(
                SecondaryContainer,
                OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY),
                env,
            ),
        ),
        |recorder, _frame, (shape, paint)| recorder.fill(shape, paint),
    )
}

/// The active bar: the tint, on-surface at 38% while disabled.
fn active_track(tokens: &SliderSizeTokens, tint: Color, env: &Environment) -> BarPart {
    let tokens = *tokens;
    Custom::new(
        (
            Driven::new(move |inputs: &Inputs<'_, f64>| {
                active_rect(inputs.frame, *inputs.value, inputs.state, &tokens)
            }),
            disabled_paint(tint, OnSurface.with_opacity(DISABLED_CONTENT_OPACITY), env),
        ),
        |recorder, _frame, (shape, paint)| recorder.fill(shape, paint),
    )
}

/// The stop indicator: a single dot at the track's trailing end,
/// `stop-indicator.trailing-space` in from the end edge, hidden where the
/// inactive remainder no longer reaches it. A single-value slider has no
/// leading dot.
fn stop_indicator(env: &Environment) -> Custom<f64, (Driven<Paint, f64>,)> {
    let colors = zip(
        Color::from(OnSecondaryContainer).resolve(env),
        Color::from(OnSurface.with_opacity(DISABLED_CONTENT_OPACITY)).resolve(env),
    );
    Custom::new(
        (Driven::follow(
            colors,
            |&(rest, disabled): &(WorkingColor, WorkingColor), inputs: &Inputs<'_, f64>| {
                let center_x = stop_indicator_center(inputs.frame).x;
                let start = handle_center(inputs.frame, *inputs.value).x + handle_gap(inputs.state);
                if center_x <= start {
                    Paint::from(WorkingColor::TRANSPARENT)
                } else if inputs.state.contains(InteractionState::DISABLED) {
                    Paint::from(disabled)
                } else {
                    Paint::from(rest)
                }
            },
        ),),
        |recorder, frame, (paint,)| {
            recorder.fill(
                Circle::new(
                    stop_indicator_center(frame),
                    SLIDER_STOP_INDICATOR_SIZE / 2.0,
                ),
                paint,
            );
        },
    )
}

/// The stop indicator's centre on `frame`.
fn stop_indicator_center(frame: Rect) -> Point {
    Point::new(
        frame.x1 - SLIDER_STOP_INDICATOR_END_SPACE - SLIDER_STOP_INDICATOR_SIZE / 2.0,
        frame.center().y,
    )
}

/// The state layer: `md.comp.slider.state-layer.size` = 40, a 20dp-radius
/// halo around the handle in the tint at the standard hover, focus and
/// pressed opacities. The disabled handle takes none.
fn state_layer(tint: &Color, env: &Environment) -> HaloPart {
    Custom::new(
        (
            Driven::new(|inputs: &Inputs<'_, f64>| {
                Circle::new(
                    handle_center(inputs.frame, *inputs.value),
                    STATE_LAYER_RADIUS,
                )
            }),
            Driven::follow(
                tint.resolve(env),
                |tint: &WorkingColor, inputs: &Inputs<'_, f64>| {
                    Paint::from(
                        tint.with_alpha(tint.components[3] * state_layer_opacity(inputs.state)),
                    )
                },
            ),
        ),
        |recorder, _frame, (shape, paint)| recorder.fill(shape, paint),
    )
}

/// The state layer's opacity: focus outranks press outranks hover.
const fn state_layer_opacity(state: InteractionState) -> f32 {
    if state.contains(InteractionState::DISABLED) {
        0.0
    } else if state.contains(InteractionState::FOCUSED) {
        FOCUS_STATE_LAYER_OPACITY
    } else if state.contains(InteractionState::PRESSED) {
        PRESSED_STATE_LAYER_OPACITY
    } else if state.contains(InteractionState::HOVERED) {
        HOVER_STATE_LAYER_OPACITY
    } else {
        0.0
    }
}

/// The handle: `md.comp.slider.handle.elevation` = level1 and the tint; a
/// disabled handle is level0, on-surface at 38% over an opaque surface
/// underlay so content behind it cannot bleed through.
fn handle(tokens: &SliderSizeTokens, tint: Color, env: &Environment) -> HandlePart {
    let tokens = *tokens;
    let elevation = |pick: fn(&hydrolysis::theme::Elevation) -> Shadow| {
        Driven::follow(
            Color::from(ShadowRole).resolve(env),
            move |shadow: &WorkingColor, inputs: &Inputs<'_, f64>| {
                if inputs.state.contains(InteractionState::DISABLED) {
                    pick(&hydrolysis::theme::Elevation::NONE)
                } else {
                    pick(&chrome_elevation(MaterialElevationLevel::LEVEL1, *shadow))
                }
            },
        )
    };
    let underlay = Driven::follow(
        Color::from(Background).resolve(env),
        |background: &WorkingColor, inputs: &Inputs<'_, f64>| {
            Paint::from(if inputs.state.contains(InteractionState::DISABLED) {
                *background
            } else {
                WorkingColor::TRANSPARENT
            })
        },
    );
    Custom::new(
        (
            Driven::new(move |inputs: &Inputs<'_, f64>| {
                handle_rect(inputs.frame, *inputs.value, inputs.state, &tokens)
            }),
            elevation(|elevation| elevation.key),
            elevation(|elevation| elevation.ambient),
            underlay,
            disabled_paint(tint, OnSurface.with_opacity(DISABLED_CONTENT_OPACITY), env),
        ),
        |recorder, _frame, (shape, key, ambient, underlay, paint)| {
            let shape = shape.into_shared();
            recorder.shadow(shape.clone(), key);
            recorder.shadow(shape.clone(), ambient);
            recorder.fill(shape.clone(), underlay);
            recorder.fill(shape, paint);
        },
    )
}

/// `md.comp.slider.value-indicator.*`: a stadium bubble at inverse-surface
/// 44 high and at least 48 wide, `active.bottom-space` above the handle's
/// top edge, its label in `label-large` at inverse-on-surface.
fn value_indicator(env: &Environment) -> ValueIndicator {
    ValueIndicator::new(
        typography::slider_value_indicator_label(),
        Color::from(InverseOnSurface),
        Custom::<f64, _>::new(
            (Driven::follow(
                Color::from(InverseSurface).resolve(env),
                |color: &WorkingColor, _: &Inputs<'_, f64>| Paint::from(*color),
            ),),
            |recorder, frame, (paint,)| {
                recorder.fill(
                    RoundedRect::from_rect(
                        frame,
                        RoundedRectRadii::from_single_radius(frame.height() / 2.0),
                    ),
                    paint,
                );
            },
        ),
    )
    .padding(
        SLIDER_VALUE_INDICATOR_HORIZONTAL_PADDING,
        SLIDER_VALUE_INDICATOR_VERTICAL_PADDING,
    )
    .minimum_size(Size::new(
        SLIDER_VALUE_INDICATOR_MIN_WIDTH,
        SLIDER_VALUE_INDICATOR_MIN_HEIGHT,
    ))
    .gap(SLIDER_VALUE_INDICATOR_BOTTOM_SPACE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydrolysis::Style as _;
    use hydrolysis::theme::SpecTint;
    use waterui_controls::ControlSize;
    use waterui_core::layout::LayoutDirection;

    fn spec(size: ControlSize, value_indicator: bool) -> SliderSpec {
        SliderSpec::new(
            size,
            0.0..=1.0,
            None,
            LayoutDirection::LeftToRight,
            SpecTint::default(),
            true,
            false,
            value_indicator,
        )
    }

    fn env() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    /// `md.comp.slider.<size>.*`: the row, the knob and the track corners
    /// follow the control size; the width constants are size-invariant.
    #[test]
    fn the_look_follows_the_size_token_tables() {
        for (size, track_height, handle_height, corner) in [
            (ControlSize::ExtraSmall, 16.0, 44.0, 8.0),
            (ControlSize::Small, 24.0, 44.0, 8.0),
            (ControlSize::Medium, 40.0, 52.0, 12.0),
            (ControlSize::Large, 56.0, 68.0, 16.0),
            (ControlSize::ExtraLarge, 96.0, 108.0, 28.0),
        ] {
            let tokens = slider_size_tokens(size);
            assert_eq!(tokens.track_height, track_height);
            assert_eq!(tokens.handle_height, handle_height);
            assert_eq!(tokens.track_corner, corner);

            let look = look(&spec(size, false), &env());
            assert_eq!(look.layout().row_height, handle_height.max(track_height));
            assert_eq!(look.knob().size, Size::new(4.0, handle_height));
            assert_eq!(look.get_drag(), DragMapping::Absolute);
            assert_eq!(look.part_count(), 5);
        }
    }

    /// The Expressive handle is touchable in the 48dp minimum interactive
    /// size, taking a press at once.
    #[test]
    fn the_knob_takes_the_minimum_touch_target() {
        let look = look(&spec(ControlSize::ExtraSmall, false), &env());
        let target = look.knob().target(Point::new(100.0, 50.0));
        assert_eq!(target.width(), 48.0);
        assert_eq!(target.height(), 48.0);
    }

    /// The value indicator exists only for a slider that has one.
    #[test]
    fn the_value_indicator_follows_the_spec() {
        assert!(
            look(&spec(ControlSize::Small, false), &env())
                .get_value_indicator()
                .is_none()
        );
        let look = look(&spec(ControlSize::Small, true), &env());
        let indicator = look.get_value_indicator().expect("the spec asks for one");
        assert_eq!(indicator.get_gap(), SLIDER_VALUE_INDICATOR_BOTTOM_SPACE);
        assert_eq!(indicator.get_minimum_size(), Size::new(48.0, 44.0));
        assert_eq!(
            indicator.get_padding().x,
            SLIDER_VALUE_INDICATOR_HORIZONTAL_PADDING
        );
        assert_eq!(
            indicator.get_padding().y,
            SLIDER_VALUE_INDICATOR_VERTICAL_PADDING
        );
    }

    /// Pressing narrows the handle, and the track gap narrows with it.
    #[test]
    fn the_track_is_split_around_the_handle() {
        let tokens = slider_size_tokens(ControlSize::ExtraSmall);
        let frame = Rect::new(0.0, 0.0, 120.0, 44.0);
        let rest = InteractionState::empty();
        let gap = SLIDER_HANDLE_WIDTH / 2.0 + SLIDER_HANDLE_PADDING;
        assert_eq!(active_rect(frame, 0.6, rest, &tokens).rect().x1, 72.0 - gap);
        assert_eq!(
            inactive_rect(frame, 0.6, rest, &tokens).rect().x0,
            72.0 + gap
        );

        let pressed = InteractionState::PRESSED;
        let gap = SLIDER_PRESSED_HANDLE_WIDTH / 2.0 + SLIDER_HANDLE_PADDING;
        assert_eq!(
            active_rect(frame, 0.6, pressed, &tokens).rect().x1,
            72.0 - gap
        );
        assert_eq!(
            inactive_rect(frame, 0.6, pressed, &tokens).rect().x0,
            72.0 + gap
        );
        assert_eq!(
            handle_rect(frame, 0.6, pressed, &tokens).rect().width(),
            SLIDER_PRESSED_HANDLE_WIDTH
        );
        assert_eq!(handle_rect(frame, 0.6, rest, &tokens).rect().height(), 44.0);
    }

    /// Corners facing the handle gap use the 2dp inside-corner size; only the
    /// outer ends take the size's shape token.
    #[test]
    fn the_gap_corners_are_the_inside_corner_size() {
        let tokens = slider_size_tokens(ControlSize::ExtraSmall);
        let frame = Rect::new(0.0, 0.0, 120.0, 44.0);
        let rest = InteractionState::empty();
        let inactive = inactive_rect(frame, 0.5, rest, &tokens).radii();
        assert_eq!(inactive.top_left, SLIDER_TRACK_INSIDE_CORNER_SIZE);
        assert_eq!(inactive.bottom_left, SLIDER_TRACK_INSIDE_CORNER_SIZE);
        assert_eq!(inactive.top_right, tokens.track_corner);
        assert_eq!(inactive.bottom_right, tokens.track_corner);
        let active = active_rect(frame, 0.5, rest, &tokens).radii();
        assert_eq!(active.top_left, tokens.track_corner);
        assert_eq!(active.bottom_left, tokens.track_corner);
        assert_eq!(active.top_right, SLIDER_TRACK_INSIDE_CORNER_SIZE);
        assert_eq!(active.bottom_right, SLIDER_TRACK_INSIDE_CORNER_SIZE);
    }

    /// At the ends of the range one half of the track has nothing left to
    /// show.
    #[test]
    fn a_half_with_no_room_is_empty() {
        let tokens = slider_size_tokens(ControlSize::ExtraSmall);
        let frame = Rect::new(0.0, 0.0, 120.0, 44.0);
        let rest = InteractionState::empty();
        assert_eq!(active_rect(frame, 0.0, rest, &tokens).rect().width(), 0.0);
        assert_eq!(inactive_rect(frame, 1.0, rest, &tokens).rect().width(), 0.0);
    }

    /// The halo is the state layer at the standard opacities: focus outranks
    /// press outranks hover, and a disabled handle has none.
    #[test]
    fn the_state_layer_takes_the_standard_opacities() {
        assert_eq!(state_layer_opacity(InteractionState::empty()), 0.0);
        assert_eq!(
            state_layer_opacity(InteractionState::HOVERED),
            HOVER_STATE_LAYER_OPACITY
        );
        assert_eq!(
            state_layer_opacity(InteractionState::PRESSED),
            PRESSED_STATE_LAYER_OPACITY
        );
        assert_eq!(
            state_layer_opacity(InteractionState::PRESSED | InteractionState::FOCUSED),
            FOCUS_STATE_LAYER_OPACITY
        );
        assert_eq!(
            state_layer_opacity(InteractionState::DISABLED | InteractionState::HOVERED),
            0.0
        );
    }
}
