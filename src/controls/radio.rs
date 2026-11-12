//! The Material 3 radio group look from the `md.comp.radio-button.*` tokens.
//!
//! Every item draws one indicator, keyed by the item: a ring, a dot, and the
//! unbounded state layer and ripple the other toggles have. Each part is a
//! [`Custom`] part over whether the item is the selected one, so a selection
//! change retargets the operands of the two rows it touches and records
//! nothing: the dot grows or shrinks on the emphasized decelerate curve over
//! `medium2` while its opacity and the ring's colour snap over at `short1`.

use hydrolysis::draw::kurbo::{Circle, Size, Stroke};
use hydrolysis::draw::{Draw as _, Paint, WorkingColor};
use hydrolysis::theme::{
    Custom, Driven, Inputs, Label, PickerSpec, RadioGroupLook, RadioLayout, ToggleControl,
};
use waterui::interaction::InteractionState;
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{OnSurface, OnSurfaceVariant};
use crate::controls::toggle::{
    STATE_LAYER_RADIUS, Toggled, centered_ripple, centered_state_layer, disabled, faded, gather,
    toggle_motion,
};
use crate::dimensions::{
    PICKER_HORIZONTAL_INSET, PICKER_LABEL_SPACING, PICKER_MIN_HEIGHT, PICKER_RADIO_INDICATOR_SIZE,
    PICKER_RADIO_INNER_DOT_RADIUS, PICKER_RADIO_LABEL_SPACING, PICKER_RADIO_MIN_WIDTH,
    PICKER_RADIO_OUTER_RING_WIDTH, PICKER_RADIO_ROW_SPACING, PICKER_VERTICAL_INSET,
};
use crate::theme::colors::DISABLED_CONTENT_OPACITY;
use crate::theme::{motion, typography};

/// The look of a radio group in `spec`.
///
/// The selected indicator is drawn in the spec's tint, which is `primary`
/// unless the subtree tints it; the rows' labels read in `body-large` over
/// `on-surface`.
pub fn look(spec: &PickerSpec, env: &Environment) -> RadioGroupLook {
    let tint = spec.tint();
    let indicator = ToggleControl::new(
        Size::new(PICKER_RADIO_INDICATOR_SIZE, PICKER_RADIO_INDICATOR_SIZE),
        toggle_motion(),
    )
    .label_spacing(PICKER_RADIO_LABEL_SPACING)
    // The state layer's circle reaches past the indicator by the difference
    // of their radii.
    .reach(STATE_LAYER_RADIUS - PICKER_RADIO_INDICATOR_SIZE / 2.0)
    .hit(motion::hit_region())
    .part(ring(&tint, env))
    .part(dot(&tint, env))
    .part(centered_state_layer(&tint, env))
    .part(centered_ripple(&tint, env));
    RadioGroupLook::new(indicator, layout()).label(
        Label::new()
            .font(typography::body_large())
            .color(Color::from(OnSurface)),
    )
}

/// The measures of the group: at least 72 wide, as no spec floors a radio
/// column but a lone row must stay tappable, and 56 tall, inset 16 by 8,
/// with the rows 6 apart.
const fn layout() -> RadioLayout {
    RadioLayout {
        min_width: PICKER_RADIO_MIN_WIDTH,
        min_height: PICKER_MIN_HEIGHT,
        horizontal_inset: PICKER_HORIZONTAL_INSET,
        vertical_inset: PICKER_VERTICAL_INSET,
        label_spacing: PICKER_LABEL_SPACING,
        row_spacing: PICKER_RADIO_ROW_SPACING,
    }
}

/// The ring's paint: `on-surface-variant` while unselected, `on-surface`
/// while the unselected item is hovered, focused or pressed, the tint while
/// selected, on-surface at 38% while disabled
/// (`md.comp.radio-button.*.icon.color`).
fn ring_paint(tint: &Color, env: &Environment) -> Toggled<Paint> {
    Driven::follow(
        gather(
            [
                OnSurfaceVariant.into(),
                OnSurface.into(),
                tint.clone(),
                OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
            ],
            env,
        ),
        |&[rest, engaged, selected, dim]: &[WorkingColor; 4], inputs: &Inputs<'_, bool>| {
            Paint::from(if disabled(inputs) {
                dim
            } else if *inputs.value {
                selected
            } else if inputs.state.intersects(
                InteractionState::HOVERED | InteractionState::FOCUSED | InteractionState::PRESSED,
            ) {
                engaged
            } else {
                rest
            })
        },
    )
    .motion(motion::value(motion::radio_snap()))
}

/// The ring: `md.comp.radio-button.icon.size` across, stroked inside the
/// indicator's frame.
fn ring(tint: &Color, env: &Environment) -> Custom<bool, (Toggled<Paint>,)> {
    Custom::new((ring_paint(tint, env),), |recorder, frame, (paint,)| {
        recorder.stroke(
            Circle::new(
                frame.center(),
                frame.width() / 2.0 - PICKER_RADIO_OUTER_RING_WIDTH / 2.0,
            ),
            Stroke::new(PICKER_RADIO_OUTER_RING_WIDTH),
            paint,
        );
    })
}

/// The dot's circle: `RadioButtonDotSize` across once selected, nothing
/// before.
fn dot_circle() -> Toggled<Circle> {
    Driven::new(|inputs: &Inputs<'_, bool>| {
        Circle::new(
            inputs.frame.center(),
            if *inputs.value {
                PICKER_RADIO_INNER_DOT_RADIUS
            } else {
                0.0
            },
        )
    })
    .motion(motion::value(motion::radio_dot_grow()))
}

/// The dot's paint: the tint, on-surface at 38% while disabled, fading in
/// with the selection.
fn dot_paint(tint: &Color, env: &Environment) -> Toggled<Paint> {
    Driven::follow(
        gather(
            [
                tint.clone(),
                OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
            ],
            env,
        ),
        |&[selected, dim]: &[WorkingColor; 2], inputs: &Inputs<'_, bool>| {
            Paint::from(faded(
                if disabled(inputs) { dim } else { selected },
                if *inputs.value { 1.0 } else { 0.0 },
            ))
        },
    )
    .motion(motion::value(motion::radio_snap()))
}

fn dot(tint: &Color, env: &Environment) -> Custom<bool, (Toggled<Circle>, Toggled<Paint>)> {
    Custom::new(
        (dot_circle(), dot_paint(tint, env)),
        |recorder, _frame, (circle, paint)| recorder.fill(circle, paint),
    )
}

#[cfg(test)]
mod tests {
    use super::{dot_circle, dot_paint, layout, look, ring_paint};
    use crate::MaterialColorScheme;
    use crate::color::Primary;
    use crate::dimensions::{
        PICKER_LABEL_SPACING, PICKER_RADIO_INDICATOR_SIZE, PICKER_RADIO_INNER_DOT_RADIUS,
        PICKER_RADIO_LABEL_SPACING, PICKER_RADIO_OUTER_RING_WIDTH,
    };
    use hydrolysis::Style as _;
    use hydrolysis::draw::kurbo::{Rect, Size};
    use hydrolysis::draw::{Paint, WorkingColor};
    use hydrolysis::theme::{Inputs, PickerSpec, SpecTint};
    use waterui::interaction::InteractionState;
    use waterui_controls::ControlSize;
    use waterui_core::layout::LayoutDirection;
    use waterui_core::{Environment, Signal as _};
    use waterui_graphics::color::Color;

    const INDICATOR: Rect = Rect::new(0.0, 0.0, 20.0, 20.0);

    fn env() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    fn at(value: &bool, state: InteractionState) -> Inputs<'_, bool> {
        Inputs::at_rest(INDICATOR, state, value)
    }

    fn tint() -> Color {
        Color::from(Primary)
    }

    fn alpha(paint: Paint) -> f32 {
        match paint {
            Paint::Solid(color) => color.components[3],
            other => panic!("a Material radio paints solid colours, not {other:?}"),
        }
    }

    /// Values from `androidx.compose.material3.tokens.RadioButtonTokens` and
    /// the private dimensions in `RadioButton.kt`.
    #[test]
    fn the_indicator_and_layout_match_the_compose_radio_tokens() {
        let spec = PickerSpec::new(
            ControlSize::Small,
            LayoutDirection::LeftToRight,
            SpecTint::default(),
        );
        let look = look(&spec, &env());
        let indicator = look.indicator();

        assert_eq!(indicator.size(), Size::new(20.0, 20.0));
        assert_eq!(indicator.get_label_spacing(), PICKER_RADIO_LABEL_SPACING);
        assert_eq!(layout().label_spacing, PICKER_LABEL_SPACING);
        assert_eq!(PICKER_RADIO_INDICATOR_SIZE, 20.0);
        assert_eq!(PICKER_RADIO_OUTER_RING_WIDTH, 2.0);
        // RadioButtonDotSize is 12dp across, so half of it here.
        assert_eq!(PICKER_RADIO_INNER_DOT_RADIUS, 6.0);
        assert_eq!(look.layout().min_width, 72.0);
    }

    /// The donut icon: a ring either way, the dot only once selected.
    #[test]
    fn the_dot_exists_only_while_selected() {
        let env = env();
        let rest = InteractionState::empty();
        let circle = dot_circle();
        let paint = dot_paint(&tint(), &env);

        assert_eq!(circle.target(&at(&false, rest)).radius, 0.0);
        assert_eq!(
            circle.target(&at(&true, rest)).radius,
            PICKER_RADIO_INNER_DOT_RADIUS
        );
        assert_eq!(alpha(paint.target(&at(&false, rest))), 0.0);
        assert_eq!(alpha(paint.target(&at(&true, rest))), 1.0);
    }

    /// The ring is on-surface-variant unselected, on-surface once the
    /// unselected item is engaged, and the tint when selected.
    #[test]
    fn the_ring_is_coloured_by_selection_and_state() {
        let colors = MaterialColorScheme::baseline_light();
        let paint = ring_paint(&tint(), &env());
        let solid = |color: WorkingColor| Paint::from(color);
        let rest = InteractionState::empty();

        assert_eq!(
            paint.target(&at(&false, rest)),
            solid(colors.on_surface_variant.working())
        );
        for engaged in [
            InteractionState::HOVERED,
            InteractionState::FOCUSED,
            InteractionState::PRESSED,
        ] {
            assert_eq!(
                paint.target(&at(&false, engaged)),
                solid(colors.on_surface.working())
            );
        }
        assert_eq!(
            paint.target(&at(&true, InteractionState::HOVERED)),
            solid(colors.primary.working())
        );
    }

    /// Disabled, the ring and the dot drop to on-surface at 38%.
    #[test]
    fn a_disabled_indicator_is_on_surface_at_38_percent() {
        let env = env();
        let disabled = InteractionState::DISABLED;
        let dim = Paint::from(
            Color::from(
                crate::color::OnSurface
                    .with_opacity(crate::theme::colors::DISABLED_CONTENT_OPACITY),
            )
            .resolve(&env)
            .snapshot(),
        );

        assert_eq!(ring_paint(&tint(), &env).target(&at(&false, disabled)), dim);
        assert_eq!(ring_paint(&tint(), &env).target(&at(&true, disabled)), dim);
        assert_eq!(dot_paint(&tint(), &env).target(&at(&true, disabled)), dim);
    }
}
