//! The Material 3 segmented button look from the `md.comp.segmented-button.*`
//! tokens.
//!
//! The container outline and its separators are one control part over the
//! row. Each segment has a chrome of its own, in its own shape, that
//! Hydrolysis realizes the way a button's is — the hover and focus state
//! layer, one press ripple wave per press from the press point, and the
//! focus indicator — and a selected container part. Material segmented
//! buttons have no drag, so the look takes taps only, and every part is a
//! function of the segment's interaction state, so a hover, a press or a
//! selection change records nothing.

use hydrolysis::draw::kurbo::{BezPath, Point, RoundedRect, RoundedRectRadii, Stroke, Vec2};
use hydrolysis::draw::{Draw as _, Paint, WorkingColor};
use hydrolysis::theme::{
    Chrome, CornerShape, Corners, Custom, Driven, Inputs, PressFeedback, Segment, SegmentLabel,
    SegmentedLayout, SegmentedLook, SegmentedSpec, SegmentedValue, Surface,
};
use num_traits::ToPrimitive;
use waterui::interaction::InteractionState;
use waterui::reactive::zip::zip;
use waterui_core::{Computed, Environment, SignalExt};
use waterui_graphics::color::{Color, signal_color};

use crate::color::{OnSecondaryContainer, OnSurface, Outline, SecondaryContainer};
use crate::controls::button::disabled_paint;
use crate::dimensions::{
    PICKER_LABEL_SPACING, PICKER_SEGMENTED_CONTAINER_RADIUS, PICKER_SEGMENTED_HORIZONTAL_INSET,
    PICKER_SEGMENTED_MIN_HEIGHT, PICKER_SEGMENTED_MIN_WIDTH, PICKER_SEGMENTED_OUTLINE_WIDTH,
};
use crate::semantics::{focus_indicator, state_layer};
use crate::theme::colors::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY};
use crate::theme::{motion, typography};

/// A segment's shape over the segment, and its paint.
type ShapePart = Custom<Segment, (Driven<RoundedRect, Segment>, Driven<Paint, Segment>)>;

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
        .segment_chrome(segment_chrome)
        .segment_part(selected_container(env))
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

/// A segment's chrome: no container of its own — the selected container is
/// a segment part — and the state layer at the Material opacities, in
/// `on-secondary-container` while the segment is selected and `on-surface`
/// otherwise and following the segment's selection in place, the press
/// ripple growing from the press point, and the focus indicator, all in the
/// segment's own shape.
#[expect(
    clippy::needless_pass_by_value,
    reason = "`SegmentedLook::segment_chrome` fixes the callback signature, which hands the flag over by value"
)]
fn segment_chrome(segment: Segment, selected: Computed<bool>) -> Chrome {
    let tint = signal_color(selected.map(|selected| {
        if selected {
            Color::from(OnSecondaryContainer)
        } else {
            Color::from(OnSurface)
        }
    }));
    Chrome::new(
        Surface::clear(CornerShape::new(Corners::from(segment_radii(segment)))),
        state_layer(tint),
        PressFeedback::new().ripple(motion::ripple()),
        focus_indicator(),
        motion::chrome(),
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
