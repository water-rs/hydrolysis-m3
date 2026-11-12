//! The Material 3 segmented button look from the `md.comp.segmented-button.*`
//! tokens.
//!
//! The container outline and its separators are one control part over the
//! row. Each segment records three parts of its own, in its own shape: the
//! selected container, the hover and focus state layer, and the press
//! ripple. Material segmented buttons have no drag, so the look takes taps
//! only, and every part is a function of the segment's interaction state, so
//! a hover, a press or a selection change records nothing.

use hydrolysis::draw::kurbo::{
    BezPath, Circle, Point, RoundedRect, RoundedRectRadii, Stroke, Vec2,
};
use hydrolysis::draw::{Draw as _, Paint, WorkingColor};
use hydrolysis::theme::{
    Custom, Driven, Inputs, Motion, Segment, SegmentLabel, SegmentedLayout, SegmentedLook,
    SegmentedSpec, SegmentedValue, Transition,
};
use num_traits::ToPrimitive;
use waterui::interaction::InteractionState;
use waterui::reactive::zip::zip;
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{OnSecondaryContainer, OnSurface, Outline, SecondaryContainer};
use crate::controls::button::disabled_paint;
use crate::dimensions::{
    PICKER_LABEL_SPACING, PICKER_SEGMENTED_CONTAINER_RADIUS, PICKER_SEGMENTED_HORIZONTAL_INSET,
    PICKER_SEGMENTED_MIN_HEIGHT, PICKER_SEGMENTED_MIN_WIDTH, PICKER_SEGMENTED_OUTLINE_WIDTH,
};
use crate::theme::colors::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY};
use crate::theme::state_layer::{
    FOCUS_STATE_LAYER_OPACITY, HOVER_STATE_LAYER_OPACITY, PRESSED_STATE_LAYER_OPACITY,
    RIPPLE_INITIAL_SCALE, ripple_diameter,
};
use crate::theme::{motion, typography};

/// A segment's shape over the segment, and its paint.
type ShapePart = Custom<Segment, (Driven<RoundedRect, Segment>, Driven<Paint, Segment>)>;
/// A segment's press ripple: the shape it is clipped to, the wave and its
/// paint.
type RipplePart = Custom<
    Segment,
    (
        Driven<RoundedRect, Segment>,
        Driven<Circle, Segment>,
        Driven<Paint, Segment>,
    ),
>;

/// The look of a segmented picker in `spec`.
///
/// The selected segment is filled `secondary-container` and labelled
/// `on-secondary-container`; the others are labelled `on-surface`. The
/// outline is `outline`, and a disabled picker draws its outline at
/// on-surface 12%, its selected container likewise and its labels at 38%.
pub fn look(spec: &SegmentedSpec, env: &Environment) -> SegmentedLook {
    let layout = SegmentedLayout {
        row_height: PICKER_SEGMENTED_MIN_HEIGHT,
        label_padding: Vec2::new(PICKER_SEGMENTED_HORIZONTAL_INSET, 0.0),
        minimum_segment_width: PICKER_SEGMENTED_MIN_WIDTH,
        minimum_width: PICKER_SEGMENTED_MIN_WIDTH,
        label_spacing: PICKER_LABEL_SPACING,
        // The outline is stroked centred on the row's edge.
        reach: PICKER_SEGMENTED_OUTLINE_WIDTH / 2.0,
    };
    // md.comp.segmented-button.label-text = label-large.
    let label = SegmentLabel::new(typography::label_large(), label_color());
    SegmentedLook::new(layout, label, motion::chrome())
        .hit(motion::hit_region())
        .part(container(env))
        .part(separators(spec.segment_count(), env))
        .segment_part(selected_container(env))
        .segment_part(state_layer(env))
        .segment_part(ripple(env))
}

/// A segment's label colour: `on-secondary-container` while selected,
/// `on-surface` otherwise, on-surface at 38% while disabled.
fn label_color() -> Driven<Color, Segment> {
    waterui_core::interaction::StateValue::new(Color::from(OnSurface))
        .when(
            InteractionState::DISABLED,
            Color::from(OnSurface.with_opacity(DISABLED_CONTENT_OPACITY)),
        )
        .when(
            InteractionState::SELECTED,
            Color::from(OnSecondaryContainer),
        )
        .into()
}

/// One segment's shape, per Compose `itemShape(index, count)`: only the
/// outside edge of each end segment takes the group's full rounding — the
/// first item rounds its leading corners, the last its trailing corners, and
/// middle items stay square against the separator strokes.
const fn segment_radii(segment: Segment) -> RoundedRectRadii {
    let radius = PICKER_SEGMENTED_CONTAINER_RADIUS;
    match (segment.is_first(), segment.is_last()) {
        (true, true) => RoundedRectRadii::new(radius, radius, radius, radius),
        (true, false) => RoundedRectRadii::new(radius, 0.0, 0.0, radius),
        (false, true) => RoundedRectRadii::new(0.0, radius, radius, 0.0),
        (false, false) => RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0),
    }
}

/// The segment's frame in its own shape.
fn segment_shape() -> Driven<RoundedRect, Segment> {
    Driven::new(|inputs: &Inputs<'_, Segment>| {
        RoundedRect::from_rect(inputs.frame, segment_radii(*inputs.value))
    })
}

const fn disabled<V>(inputs: &Inputs<'_, V>) -> bool {
    inputs.state.contains(InteractionState::DISABLED)
}

/// `color` at `opacity`.
fn faded(color: WorkingColor, opacity: f32) -> WorkingColor {
    color.with_alpha(color.components[3] * opacity)
}

/// The outline paint: `outline`, on-surface at 12% while disabled.
fn outline_paint(env: &Environment) -> Driven<Paint, SegmentedValue> {
    disabled_paint(
        Outline,
        OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY),
        env,
    )
}

/// The outline around the row.
fn container(env: &Environment) -> Custom<SegmentedValue, (Driven<Paint, SegmentedValue>,)> {
    Custom::new((outline_paint(env),), |recorder, frame, (paint,)| {
        recorder.stroke(
            RoundedRect::from_rect(
                frame,
                RoundedRectRadii::from_single_radius(PICKER_SEGMENTED_CONTAINER_RADIUS),
            ),
            Stroke::new(PICKER_SEGMENTED_OUTLINE_WIDTH),
            paint,
        );
    })
}

/// The separators between the row's equal segments, one vertical line at
/// each boundary.
fn separators(
    segment_count: usize,
    env: &Environment,
) -> Custom<SegmentedValue, (Driven<Paint, SegmentedValue>,)> {
    Custom::new((outline_paint(env),), move |recorder, frame, (paint,)| {
        let segment_width = frame.width()
            / segment_count
                .to_f64()
                .expect("picker segment count must be representable as f64");
        let mut path = BezPath::new();
        for index in 1..segment_count {
            let x = segment_width.mul_add(
                index
                    .to_f64()
                    .expect("picker segment index must be representable as f64"),
                frame.x0,
            );
            path.move_to(Point::new(x, frame.y0));
            path.line_to(Point::new(x, frame.y1));
        }
        recorder.stroke(path, Stroke::new(PICKER_SEGMENTED_OUTLINE_WIDTH), paint);
    })
}

/// The selected segment's container: `secondary-container`, on-surface at
/// 12% while disabled, nothing while unselected.
fn selected_container(env: &Environment) -> ShapePart {
    let paint = Driven::follow(
        zip(
            Color::from(SecondaryContainer).resolve(env),
            Color::from(OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY)).resolve(env),
        ),
        |&(rest, dim): &(WorkingColor, WorkingColor), inputs: &Inputs<'_, Segment>| {
            Paint::from(if !inputs.state.contains(InteractionState::SELECTED) {
                WorkingColor::TRANSPARENT
            } else if disabled(inputs) {
                dim
            } else {
                rest
            })
        },
    );
    Custom::new(
        (segment_shape(), paint),
        |recorder, _frame, (shape, paint)| recorder.fill(shape, paint),
    )
}

/// The state-layer colour: `on-secondary-container` while selected,
/// `on-surface` otherwise, at `opacity`.
fn layer_paint(
    env: &Environment,
    opacity: impl Fn(&Inputs<'_, Segment>) -> f32 + 'static,
) -> Driven<Paint, Segment> {
    Driven::follow(
        zip(
            Color::from(OnSecondaryContainer).resolve(env),
            Color::from(OnSurface).resolve(env),
        ),
        move |&(selected, unselected): &(WorkingColor, WorkingColor),
              inputs: &Inputs<'_, Segment>| {
            let color = if inputs.state.contains(InteractionState::SELECTED) {
                selected
            } else {
                unselected
            };
            Paint::from(faded(color, opacity(inputs)))
        },
    )
}

/// The state layer's opacity while hovered or focused; the press is the
/// ripple's.
const fn state_layer_opacity(inputs: &Inputs<'_, Segment>) -> f32 {
    if disabled(inputs) {
        0.0
    } else if inputs.state.contains(InteractionState::FOCUSED) {
        FOCUS_STATE_LAYER_OPACITY
    } else if inputs.state.contains(InteractionState::HOVERED) {
        HOVER_STATE_LAYER_OPACITY
    } else {
        0.0
    }
}

/// The hover and focus layer over the segment's own shape.
fn state_layer(env: &Environment) -> ShapePart {
    Custom::new(
        (segment_shape(), layer_paint(env, state_layer_opacity)),
        |recorder, _frame, (shape, paint)| recorder.fill(shape, paint),
    )
}

/// The press ripple's opacity.
const fn ripple_opacity(inputs: &Inputs<'_, Segment>) -> f32 {
    if inputs.state.contains(InteractionState::PRESSED) && !disabled(inputs) {
        PRESSED_STATE_LAYER_OPACITY
    } else {
        0.0
    }
}

/// The ripple's wave, centred on the segment: it grows from the initial
/// scale to the full ripple diameter while pressed.
fn ripple_wave(inputs: &Inputs<'_, Segment>) -> Circle {
    let full = ripple_diameter(inputs.frame) / 2.0;
    let radius = if inputs.state.contains(InteractionState::PRESSED) {
        full
    } else {
        full * RIPPLE_INITIAL_SCALE
    };
    Circle::new(inputs.frame.center(), radius)
}

/// The press ripple's motion: the wave grows with the ripple's growth and
/// fades in, and a release fades it out.
fn ripple_motion() -> Motion {
    let ripple = motion::ripple();
    let mut motion = motion::chrome();
    motion.press = Transition::new(ripple.grow, ripple.fade_out);
    motion
}

/// The press ripple, clipped to the segment's shape.
fn ripple(env: &Environment) -> RipplePart {
    let wave = Driven::new(ripple_wave).motion(ripple_motion());
    let paint = layer_paint(env, ripple_opacity).motion(ripple_motion());
    Custom::new(
        (segment_shape(), wave, paint),
        |recorder, _frame, (clip, wave, paint)| {
            recorder.clip(clip, |recorder| recorder.fill(wave, paint));
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{label_color, look, segment_radii};
    use crate::MaterialColorScheme;
    use crate::dimensions::{
        PICKER_SEGMENTED_CONTAINER_RADIUS, PICKER_SEGMENTED_HORIZONTAL_INSET,
        PICKER_SEGMENTED_MIN_HEIGHT, PICKER_SEGMENTED_MIN_WIDTH,
    };
    use hydrolysis::Style as _;
    use hydrolysis::draw::kurbo::{Rect, RoundedRectRadii};
    use hydrolysis::theme::{Inputs, Segment, SegmentedSpec, SpecTint};
    use waterui::interaction::InteractionState;
    use waterui_controls::ControlSize;
    use waterui_core::layout::LayoutDirection;
    use waterui_core::{Environment, Signal as _};

    fn segment(index: usize, count: usize) -> Segment {
        Segment { index, count }
    }

    /// Compose `itemShape(index, count)`: only the outside edge of each end
    /// segment rounds; middle segments are square.
    #[test]
    fn segmented_item_shape_rounds_only_the_outside_edge() {
        let radius = PICKER_SEGMENTED_CONTAINER_RADIUS;
        let zero = RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0);

        assert_eq!(
            segment_radii(segment(0, 3)),
            RoundedRectRadii::new(radius, 0.0, 0.0, radius)
        );
        assert_eq!(
            segment_radii(segment(2, 3)),
            RoundedRectRadii::new(0.0, radius, radius, 0.0)
        );
        assert_eq!(segment_radii(segment(1, 3)), zero);
        assert_eq!(
            segment_radii(segment(0, 1)),
            RoundedRectRadii::new(radius, radius, radius, radius)
        );
    }

    #[test]
    fn the_layout_follows_the_compose_segmented_button_tokens() {
        let spec = SegmentedSpec::new(
            ControlSize::Small,
            3,
            LayoutDirection::LeftToRight,
            SpecTint::default(),
        );
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        let look = look(&spec, &env);
        let layout = look.layout();

        assert_eq!(layout.row_height, PICKER_SEGMENTED_MIN_HEIGHT);
        assert_eq!(layout.label_padding.x, PICKER_SEGMENTED_HORIZONTAL_INSET);
        assert_eq!(layout.minimum_segment_width, PICKER_SEGMENTED_MIN_WIDTH);
        assert_eq!(PICKER_SEGMENTED_MIN_HEIGHT, 40.0);
        assert_eq!(PICKER_SEGMENTED_MIN_WIDTH, 58.0);
        assert!(
            look.get_drag().is_none(),
            "a Material segmented button has no drag"
        );
    }

    #[test]
    fn labels_follow_the_selected_and_disabled_state() {
        let colors = MaterialColorScheme::baseline_light();
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        let label = label_color();
        let frame = Rect::new(0.0, 0.0, 80.0, 40.0);
        let value = segment(0, 3);
        let at = |state| {
            label
                .target(&Inputs::at_rest(frame, state, &value))
                .resolve(&env)
                .snapshot()
        };

        assert_eq!(
            at(InteractionState::SELECTED),
            colors.on_secondary_container.working()
        );
        assert_eq!(at(InteractionState::empty()), colors.on_surface.working());
        assert_eq!(
            at(InteractionState::SELECTED | InteractionState::DISABLED).components[3],
            colors.on_surface.working().components[3] * 0.38
        );
    }
}
