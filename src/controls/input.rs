//! The Material 3 filled text field look from the `md.comp.filled-text-field.*`
//! tokens; the secure field wears the same look.
//!
//! The field is a `surface-container-highest` container rounded on top only,
//! with a hover tint as its only state layer and an active indicator along
//! its bottom edge that thickens and turns `primary` while the field is
//! focused. The label rests centred in the container and floats to the top,
//! scaled down to the populated label size, while the field is focused or
//! holds content. The caret and the selection are `primary`. Every part is a
//! function of the field's interaction state, so a hover, a focus change or
//! the label floating records nothing.
//!
//! The menu picker's field is the same container and active indicator
//! (`md.comp.filled-select.text-field.*` repeats the filled text field's
//! values), so [`picker`](super::picker) takes them from here.

use hydrolysis::draw::WorkingColor;
use hydrolysis::draw::kurbo::RoundedRectRadii;
use hydrolysis::theme::{
    Border, Caret, Chrome, CornerShape, Corners, Driven, FloatingLabel, FocusIndicator,
    InputFieldMetrics, Inputs, Label, PressFeedback, StateLayer, Surface, TextFieldLook,
    TextFieldSpec,
};
use waterui::interaction::InteractionState;
use waterui::reactive::Computed;
use waterui_core::Environment;
use waterui_core::interaction::StateValue;
use waterui_graphics::color::Color;

use crate::color::{OnSurface, OnSurfaceVariant, Primary, SurfaceContainerHighest};
use crate::controls::button::{disabled_paint, solid_paint};
use crate::controls::toggle::gather;
use crate::dimensions::{
    INPUT_FIELD_HORIZONTAL_INSET, INPUT_FIELD_MIN_HEIGHT, INPUT_FIELD_MIN_WIDTH,
    INPUT_FIELD_VERTICAL_INSET, INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT,
    INPUT_FILLED_CONTAINER_TOP_RADIUS, INPUT_FILLED_DISABLED_CONTAINER_OPACITY,
    INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT, INPUT_LABEL_FLOAT_SCALE, INPUT_LABEL_FLOAT_TOP,
    INPUT_LABEL_HEIGHT, INPUT_SELECTION_OPACITY,
};
use crate::theme::colors::DISABLED_CONTENT_OPACITY;
use crate::theme::state_layer::HOVER_STATE_LAYER_OPACITY;
use crate::theme::{motion, typography};

/// The look of a text field or a secure field in `spec`.
pub fn look(_spec: &TextFieldSpec, env: &Environment) -> TextFieldLook {
    TextFieldLook::new(
        container(env),
        metrics(),
        FloatingLabel::new(
            Label::new()
                .font(typography::body_large())
                .color(label_color()),
            motion::text_field_label(),
        )
        .scale(INPUT_LABEL_FLOAT_SCALE)
        .top(INPUT_LABEL_FLOAT_TOP),
        Caret::new(solid_paint(Primary, env), motion::text_caret()),
        selection(env),
    )
    .active_indicator(active_indicator(env))
    .placeholder(placeholder())
}

/// The measures of the field: at least 280 wide
/// (`TextFieldDefaults.MinWidth`) and 56 tall, inset 16 by 8, with the
/// floated label's 16 line (`body-small`) reserved above the text.
const fn metrics() -> InputFieldMetrics {
    InputFieldMetrics::new(
        INPUT_LABEL_HEIGHT,
        INPUT_FIELD_MIN_WIDTH,
        INPUT_FIELD_MIN_HEIGHT,
        INPUT_FIELD_HORIZONTAL_INSET,
        INPUT_FIELD_VERTICAL_INSET,
    )
}

/// The container: `corner-extra-small-top`, square below the active
/// indicator (`md.comp.filled-text-field.container.shape`).
pub const fn container_radii() -> RoundedRectRadii {
    RoundedRectRadii::new(
        INPUT_FILLED_CONTAINER_TOP_RADIUS,
        INPUT_FILLED_CONTAINER_TOP_RADIUS,
        0.0,
        0.0,
    )
}

/// The container's chrome: `surface-container-highest`, on-surface at 4%
/// while disabled (`md.comp.filled-text-field.{container.color,
/// disabled.container.*}`), under the on-surface hover tint
/// (`md.comp.filled-text-field.hover.state-layer.{color,opacity}`) clipped to
/// it. Focus shows through the active indicator, so the chrome has no focus
/// tint and no ring, and a press has no ripple.
pub fn container(env: &Environment) -> Chrome {
    let fill = disabled_paint(
        SurfaceContainerHighest,
        OnSurface.with_opacity(INPUT_FILLED_DISABLED_CONTAINER_OPACITY),
        env,
    );
    Chrome::new(
        Surface::new(CornerShape::new(Corners::from(container_radii())), fill),
        StateLayer::new(OnSurface).opacities(HOVER_STATE_LAYER_OPACITY, 0.0, 0.0, 0.0),
        PressFeedback::new(),
        FocusIndicator::Tint,
        motion::chrome(),
    )
    .hit(motion::hit_region())
}

/// The active indicator's height: 1 and 2 while focused
/// (`md.comp.filled-text-field.{active-indicator,focus.active-indicator}.height`),
/// 1 while disabled.
pub const fn indicator_height(state: InteractionState) -> f64 {
    if state.contains(InteractionState::FOCUSED) && !state.contains(InteractionState::DISABLED) {
        INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT
    } else {
        INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT
    }
}

/// The colours of the active indicator: at rest, hovered, focused and
/// disabled.
pub type IndicatorColors = [WorkingColor; 4];

/// The active indicator's colours resolved in `env`:
/// `on-surface-variant`, `on-surface` hovered, `primary` focused and
/// on-surface at 38% disabled
/// (`md.comp.filled-text-field.*active-indicator.color`).
pub fn indicator_colors(env: &Environment) -> Computed<IndicatorColors> {
    gather(
        [
            OnSurfaceVariant.into(),
            OnSurface.into(),
            Primary.into(),
            OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
        ],
        env,
    )
}

/// The active indicator's colour in `state`: disabled outranks focused,
/// which outranks hovered.
pub fn indicator_color(
    &[rest, hovered, focused, dim]: &IndicatorColors,
    state: InteractionState,
) -> WorkingColor {
    if state.contains(InteractionState::DISABLED) {
        dim
    } else if state.contains(InteractionState::FOCUSED) {
        focused
    } else if state.contains(InteractionState::HOVERED) {
        hovered
    } else {
        rest
    }
}

/// The active indicator as the field's border along its bottom edge.
fn active_indicator(env: &Environment) -> Driven<Border> {
    Driven::follow(
        indicator_colors(env),
        |colors: &IndicatorColors, inputs: &Inputs<'_, ()>| {
            Border::new(
                indicator_height(inputs.state),
                indicator_color(colors, inputs.state),
            )
        },
    )
}

/// The floating label's colour: `on-surface-variant`, `primary` while
/// focused and on-surface at 38% while disabled
/// (`md.comp.filled-text-field.{label-text,hover.label-text,focus.label-text,
/// disabled.label-text}.color`).
///
/// The label rests in `body-large` and floats to `body-small`
/// (`md.comp.filled-text-field.{label-text,label-text-populated}.*`), the
/// 12 over 16 of [`INPUT_LABEL_FLOAT_SCALE`].
fn label_color() -> StateValue<Color> {
    StateValue::new(Color::from(OnSurfaceVariant))
        .when(
            InteractionState::DISABLED,
            Color::from(OnSurface.with_opacity(DISABLED_CONTENT_OPACITY)),
        )
        .when(InteractionState::FOCUSED, Color::from(Primary))
}

/// The placeholder: `body-large` in `on-surface-variant`, on-surface at 38%
/// while disabled
/// (`md.comp.filled-text-field.{input-text.placeholder,disabled.input-text}.*`).
fn placeholder() -> Label {
    Label::new().font(typography::body_large()).color(
        StateValue::new(Color::from(OnSurfaceVariant)).when(
            InteractionState::DISABLED,
            Color::from(OnSurface.with_opacity(DISABLED_CONTENT_OPACITY)),
        ),
    )
}

/// The selection: `primary` at [`INPUT_SELECTION_OPACITY`], over the text.
fn selection(env: &Environment) -> Driven<hydrolysis::draw::Paint> {
    solid_paint(Primary.with_opacity(INPUT_SELECTION_OPACITY), env)
}

#[cfg(test)]
mod tests {
    use super::{
        container_radii, indicator_color, indicator_colors, indicator_height, look, metrics,
    };
    use crate::MaterialColorScheme;
    use crate::dimensions::{
        INPUT_FIELD_MIN_HEIGHT, INPUT_FIELD_MIN_WIDTH, INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT,
        INPUT_FILLED_CONTAINER_TOP_RADIUS, INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT,
        INPUT_LABEL_FLOAT_SCALE,
    };
    use hydrolysis::Style as _;
    use hydrolysis::draw::kurbo::{Rect, RoundedRectRadii};
    use hydrolysis::draw::{Paint, WorkingColor};
    use hydrolysis::theme::{Inputs, SpecTint, TextFieldLook, TextFieldSpec};
    use waterui::interaction::InteractionState;
    use waterui_core::layout::LayoutDirection;
    use waterui_core::{Environment, Signal as _};
    use waterui_graphics::color::Color;

    const FIELD: Rect = Rect::new(0.0, 0.0, 280.0, 56.0);

    fn env() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    fn field_look() -> TextFieldLook {
        look(
            &TextFieldSpec::new(LayoutDirection::LeftToRight, SpecTint::default()),
            &env(),
        )
    }

    fn inputs(state: InteractionState) -> Inputs<'static, ()> {
        Inputs::at_rest(FIELD, state, &())
    }

    fn resolved(color: &Color) -> WorkingColor {
        color.resolve(&env()).snapshot()
    }

    #[test]
    fn filled_text_field_metrics_match_compose_filled_text_field_tokens() {
        let metrics = metrics();

        assert_eq!(metrics.min_height, INPUT_FIELD_MIN_HEIGHT);
        assert_eq!(metrics.min_width, INPUT_FIELD_MIN_WIDTH);
        assert_eq!(INPUT_FIELD_MIN_HEIGHT, 56.0);
        assert_eq!(INPUT_FIELD_MIN_WIDTH, 280.0);
        assert_eq!(INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT, 1.0);
        assert_eq!(field_look().metrics().min_height, metrics.min_height);
    }

    /// `md.comp.filled-text-field.container.shape` is rounded on top only.
    #[test]
    fn filled_text_field_uses_top_only_container_shape() {
        assert_eq!(
            container_radii(),
            RoundedRectRadii::new(
                INPUT_FILLED_CONTAINER_TOP_RADIUS,
                INPUT_FILLED_CONTAINER_TOP_RADIUS,
                0.0,
                0.0
            )
        );
        assert_eq!(INPUT_FILLED_CONTAINER_TOP_RADIUS, 4.0);
    }

    #[test]
    fn filled_text_field_focus_indicator_matches_compose_filled_text_field_tokens() {
        let indicator = field_look()
            .get_active_indicator()
            .expect("the filled field has an active indicator")
            .clone();
        let width = |state| indicator.target(&inputs(state)).width;

        assert_eq!(width(InteractionState::empty()), 1.0);
        assert_eq!(width(InteractionState::HOVERED), 1.0);
        assert_eq!(
            width(InteractionState::FOCUSED),
            INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT
        );
        assert_eq!(INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT, 2.0);
        assert_eq!(
            indicator_height(InteractionState::DISABLED | InteractionState::FOCUSED),
            INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT
        );
    }

    #[test]
    fn the_active_indicator_is_coloured_by_state() {
        let colors = MaterialColorScheme::baseline_light();
        let gathered = indicator_colors(&env()).snapshot();
        let at = |state| Paint::from(indicator_color(&gathered, state));
        let solid = |color: WorkingColor| Paint::from(color);

        assert_eq!(
            at(InteractionState::empty()),
            solid(colors.on_surface_variant.working())
        );
        assert_eq!(
            at(InteractionState::HOVERED),
            solid(colors.on_surface.working())
        );
        assert_eq!(
            at(InteractionState::HOVERED | InteractionState::FOCUSED),
            solid(colors.primary.working())
        );
        let Paint::Solid(disabled) = at(InteractionState::DISABLED | InteractionState::FOCUSED)
        else {
            panic!("the active indicator paints solid colours");
        };
        assert_eq!(
            disabled.components[3],
            colors.on_surface.working().components[3] * 0.38
        );
        let border = field_look()
            .get_active_indicator()
            .expect("the filled field has an active indicator")
            .target(&inputs(InteractionState::FOCUSED));
        assert_eq!(border.paint, solid(colors.primary.working()));
    }

    #[test]
    fn filled_text_field_caret_and_selection_use_primary_role() {
        let colors = MaterialColorScheme::baseline_light();
        let look = field_look();
        let primary = colors.primary.working();

        let Paint::Solid(selection) = look.selection().target(&inputs(InteractionState::FOCUSED))
        else {
            panic!("Material text selection must be a solid primary color layer");
        };
        let Paint::Solid(caret) = look
            .caret()
            .paint()
            .target(&inputs(InteractionState::FOCUSED))
        else {
            panic!("Material text caret must be a solid primary color layer");
        };

        assert_eq!(selection.components[..3], primary.components[..3]);
        assert_eq!(selection.components[3], 0.28);
        assert_eq!(caret, primary);
    }

    /// The label is `on-surface-variant`, `primary` while focused and dim
    /// while disabled; the placeholder never turns `primary`.
    #[test]
    fn the_label_and_placeholder_are_coloured_by_state() {
        let colors = MaterialColorScheme::baseline_light();
        let look = field_look();
        let label = look
            .floating_label()
            .label()
            .get_color()
            .expect("the label has a colour");
        let placeholder = look
            .get_placeholder()
            .get_color()
            .expect("the placeholder has a colour");
        let label_at = |state| resolved(&label.target(&inputs(state)));
        let placeholder_at = |state| resolved(&placeholder.target(&inputs(state)));

        assert_eq!(
            label_at(InteractionState::empty()),
            colors.on_surface_variant.working()
        );
        assert_eq!(
            label_at(InteractionState::HOVERED),
            colors.on_surface_variant.working()
        );
        assert_eq!(
            label_at(InteractionState::FOCUSED),
            colors.primary.working()
        );
        assert_eq!(
            label_at(InteractionState::DISABLED),
            colors.on_surface.working().with_alpha(0.38)
        );
        assert_eq!(
            placeholder_at(InteractionState::FOCUSED),
            colors.on_surface_variant.working()
        );
        assert_eq!(
            placeholder_at(InteractionState::DISABLED),
            colors.on_surface.working().with_alpha(0.38)
        );
        assert!(look.get_placeholder().get_font().is_some());
        assert!(look.floating_label().label().get_font().is_some());
    }

    /// The populated label is `body-small` (12) where the resting label is
    /// `body-large` (16).
    #[test]
    fn the_floated_label_is_body_small_over_body_large() {
        assert_eq!(INPUT_LABEL_FLOAT_SCALE, 12.0 / 16.0);
        assert_eq!(
            field_look().floating_label().get_scale(),
            INPUT_LABEL_FLOAT_SCALE
        );
    }
}
