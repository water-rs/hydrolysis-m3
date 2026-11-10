//! The Material 3 stepper look.
//!
//! Material has no stepper component, but it does have a rule for a pair of
//! buttons that belong together: a connected button group. The two far ends are
//! fully round, the corners either side of the seam are tucked in, and holding
//! one tightens its corners further. That is what a stepper is, so it is drawn
//! with `ConnectedButtonGroupSmallTokens` rather than as two loose circles.
//!
//! Each end is a chrome of its own: its silhouette morphs under a press, its
//! icon is a part of it, and an end at the limit of the range is disabled, so
//! it reads in the disabled colours.

use hydrolysis::theme::{
    Chrome, CornerShape, Corners, Custom, Driven, Inputs, StepperLook, StepperSpec, Surface,
};
use waterui::interaction::InteractionState;
use waterui_core::Environment;
use waterui_core::layout::LayoutDirection;
use waterui_graphics::color::Color;
use waterui_graphics::draw::kurbo::{BezPath, Rect, RoundedRectRadii, Size};
use waterui_graphics::draw::{Draw as _, Paint};

use crate::color::{OnSurface, OnSurfaceVariant, SurfaceContainer};
use crate::controls::button::{disabled_paint, interactive};
use crate::dimensions::{
    STEPPER_BUTTON_SIZE, STEPPER_BUTTON_SPACING, STEPPER_ICON_SIZE, STEPPER_INNER_CORNER_RADIUS,
    STEPPER_LABEL_SPACING, STEPPER_PRESSED_INNER_CORNER_RADIUS,
};
use crate::icon_paths::{IconGrid, add, remove};
use crate::theme::colors::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY};
use crate::theme::motion;

/// Which of the pair an end is.
#[derive(Clone, Copy, PartialEq, Eq)]
enum End {
    Decrement,
    Increment,
}

/// The look of a stepper in `spec`.
///
/// The decrement end leads the pair, so a right-to-left layout puts it at
/// the right, with its fully round corners on the right too.
pub fn look(spec: &StepperSpec, env: &Environment) -> StepperLook {
    let direction = spec.direction();
    StepperLook::new(
        end(End::Decrement, direction, env),
        end(End::Increment, direction, env),
        Size::new(STEPPER_BUTTON_SIZE, STEPPER_BUTTON_SIZE),
    )
    .spacing(STEPPER_BUTTON_SPACING)
    .label_spacing(STEPPER_LABEL_SPACING)
}

/// One end's chrome: the `surface-container` container on the end's
/// silhouette, its icon, and the `on-surface-variant` state layer, ripple
/// and focus indicator.
fn end(end: End, direction: LayoutDirection, env: &Environment) -> Chrome {
    let surface = Surface::new(
        CornerShape::new(corners(outer_on_left(end, direction))),
        disabled_paint(
            SurfaceContainer,
            OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY),
            env,
        ),
    );
    interactive(surface, Color::from(OnSurfaceVariant))
        // The two ends touch within two dp of each other, so a minimum touch
        // target around each would overlap its neighbour's.
        .hit(motion::hit_region().minimum(Size::ZERO))
        .custom(icon(end, env))
}

/// Whether the end's fully round corners are its left ones: the leading end
/// of the pair lies at the left in a left-to-right layout.
fn outer_on_left(end: End, direction: LayoutDirection) -> bool {
    (end == End::Decrement) == (direction == LayoutDirection::LeftToRight)
}

/// The four corner radii for one end of the pair.
///
/// The outer corners are `CornerFull`, which on a container of this height is
/// half of it. The inner pair is `InnerCornerCornerSize`, tightening to
/// `PressedInnerCornerCornerSize` while the end is held.
fn corners(outer_on_left: bool) -> Driven<Corners> {
    Driven::new(move |inputs: &Inputs<'_, ()>| {
        let outer = inputs.frame.height() / 2.0;
        let inner = if inputs.state.contains(InteractionState::PRESSED) {
            STEPPER_PRESSED_INNER_CORNER_RADIUS
        } else {
            STEPPER_INNER_CORNER_RADIUS
        }
        .min(outer);
        Corners::Radii(if outer_on_left {
            RoundedRectRadii::new(outer, inner, inner, outer)
        } else {
            RoundedRectRadii::new(inner, outer, outer, inner)
        })
    })
}

/// The end's icon: `on-surface-variant`, on-surface at 38% while disabled.
fn icon(end: End, env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (disabled_paint(
            OnSurfaceVariant,
            OnSurface.with_opacity(DISABLED_CONTENT_OPACITY),
            env,
        ),),
        move |recorder, frame, (paint,)| {
            recorder.fill(glyph(end, frame), paint);
        },
    )
}

/// The Material `remove` or `add` outline centred in `frame`.
fn glyph(end: End, frame: Rect) -> BezPath {
    let grid = IconGrid::centered(frame.center(), STEPPER_ICON_SIZE);
    match end {
        End::Decrement => remove(grid),
        End::Increment => add(grid),
    }
}

#[cfg(test)]
mod tests {
    use super::{End, corners, glyph, look, outer_on_left};
    use crate::dimensions::{
        STEPPER_BUTTON_SIZE, STEPPER_BUTTON_SPACING, STEPPER_ICON_SIZE,
        STEPPER_INNER_CORNER_RADIUS, STEPPER_LABEL_SPACING, STEPPER_PRESSED_INNER_CORNER_RADIUS,
    };
    use hydrolysis::Style as _;
    use hydrolysis::theme::{Corners, Inputs, SpecTint, StepperSpec};
    use waterui::interaction::InteractionState;
    use waterui_core::Environment;
    use waterui_core::layout::LayoutDirection;
    use waterui_graphics::draw::kurbo::{Rect, RoundedRectRadii, Shape as _};

    const BOUNDS: Rect = Rect::new(0.0, 0.0, 40.0, 40.0);

    fn radii(outer_on_left: bool, state: InteractionState) -> RoundedRectRadii {
        match corners(outer_on_left).target(&Inputs::at_rest(BOUNDS, state, &())) {
            Corners::Radii(radii) => radii,
            other => panic!("the end's corners are per-corner radii, got {other:?}"),
        }
    }

    #[test]
    fn stepper_uses_the_connected_button_group_tokens() {
        let spec = StepperSpec::new(LayoutDirection::LeftToRight, SpecTint::default());
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        let look = look(&spec, &env);
        assert_eq!(look.button().width, STEPPER_BUTTON_SIZE);
        assert_eq!(look.button().height, STEPPER_BUTTON_SIZE);
        assert_eq!(look.get_spacing(), STEPPER_BUTTON_SPACING);
        assert_eq!(look.get_label_spacing(), STEPPER_LABEL_SPACING);
        assert_eq!(STEPPER_BUTTON_SIZE, 40.0);
        assert_eq!(STEPPER_ICON_SIZE, 24.0);
        // ConnectedButtonGroupSmallTokens: BetweenSpace, InnerCornerCornerSize
        // (CornerValueSmall) and PressedInnerCornerCornerSize
        // (CornerValueExtraSmall).
        assert_eq!(STEPPER_BUTTON_SPACING, 2.0);
        assert_eq!(STEPPER_INNER_CORNER_RADIUS, 8.0);
        assert_eq!(STEPPER_PRESSED_INNER_CORNER_RADIUS, 4.0);
    }

    /// The pair reads as one silhouette: round on the outside, tucked at the
    /// seam, and each end tucks on the side that faces the other.
    #[test]
    fn only_the_pairs_outer_corners_are_fully_round() {
        let outer = BOUNDS.height() / 2.0;

        let minus = radii(true, InteractionState::empty());
        assert_eq!(minus.top_left, outer);
        assert_eq!(minus.bottom_left, outer);
        assert_eq!(minus.top_right, STEPPER_INNER_CORNER_RADIUS);
        assert_eq!(minus.bottom_right, STEPPER_INNER_CORNER_RADIUS);

        let plus = radii(false, InteractionState::empty());
        assert_eq!(plus.top_right, outer);
        assert_eq!(plus.bottom_right, outer);
        assert_eq!(plus.top_left, STEPPER_INNER_CORNER_RADIUS);
        assert_eq!(plus.bottom_left, STEPPER_INNER_CORNER_RADIUS);
    }

    /// A press tightens the seam corners and leaves the outer ones alone.
    #[test]
    fn a_press_tightens_only_the_seam_corners() {
        let pressed = radii(true, InteractionState::PRESSED);

        assert_eq!(pressed.top_right, STEPPER_PRESSED_INNER_CORNER_RADIUS);
        assert_eq!(pressed.bottom_right, STEPPER_PRESSED_INNER_CORNER_RADIUS);
        assert_eq!(pressed.top_left, BOUNDS.height() / 2.0);
    }

    /// The decrement end leads the pair: its round corners are on the left
    /// in a left-to-right layout and on the right in a right-to-left one.
    #[test]
    fn the_decrement_end_leads_the_pair_in_either_direction() {
        for (end, direction, left) in [
            (End::Decrement, LayoutDirection::LeftToRight, true),
            (End::Increment, LayoutDirection::LeftToRight, false),
            (End::Decrement, LayoutDirection::RightToLeft, false),
            (End::Increment, LayoutDirection::RightToLeft, true),
        ] {
            assert_eq!(outer_on_left(end, direction), left);
        }
    }

    /// The icons are centred in their end.
    #[test]
    fn the_glyphs_are_centred_in_the_end() {
        for end in [End::Decrement, End::Increment] {
            let bounds = glyph(end, BOUNDS).bounding_box();
            assert!((bounds.center().x - BOUNDS.center().x).abs() < 1e-6);
            assert!((bounds.center().y - BOUNDS.center().y).abs() < 1e-6);
        }
    }
}
