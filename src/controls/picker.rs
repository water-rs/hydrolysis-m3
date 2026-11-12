//! The Material 3 menu picker look: the exposed dropdown menu's field from
//! the `md.comp.filled-select.*` tokens, which are the filled text field's.
//!
//! The field is a `surface-container-highest` container rounded on top only,
//! with its active indicator along the bottom edge and the dropdown chevron
//! at the trailing end. Its hover tint is the only state layer a filled
//! field has: focus shows through the active indicator, and a press has no
//! ripple. Every part is a function of the field's interaction state, so a
//! hover or a focus records nothing. The popup the field opens is the
//! [menu look](crate::layout::menu).

use hydrolysis::draw::kurbo::{BezPath, Point, Rect, RoundedRectRadii, Stroke};
use hydrolysis::draw::{Draw as _, Paint, WorkingColor};
use hydrolysis::theme::{
    Chrome, CornerShape, Corners, Custom, Driven, FocusIndicator, Inputs, Label, MenuPickerLayout,
    MenuPickerLook, PickerSpec, PressFeedback, StateLayer, Surface,
};
use waterui::interaction::InteractionState;
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{OnSurface, OnSurfaceVariant, Primary, SurfaceContainerHighest};
use crate::controls::button::disabled_paint;
use crate::controls::toggle::gather;
use crate::dimensions::{
    INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT, INPUT_FILLED_CONTAINER_TOP_RADIUS,
    INPUT_FILLED_DISABLED_CONTAINER_OPACITY, INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT,
    PICKER_CHEVRON_HALF_HEIGHT, PICKER_CHEVRON_HALF_WIDTH, PICKER_CHEVRON_STROKE_WIDTH,
    PICKER_HORIZONTAL_INSET, PICKER_INDICATOR_SPACE, PICKER_LABEL_SPACING, PICKER_MENU_MIN_WIDTH,
    PICKER_MENU_POPUP_TOP_SPACING, PICKER_MIN_HEIGHT, PICKER_VERTICAL_INSET,
};
use crate::theme::colors::DISABLED_CONTENT_OPACITY;
use crate::theme::state_layer::HOVER_STATE_LAYER_OPACITY;
use crate::theme::{motion, typography};

/// The look of a menu picker's field in `spec`.
///
/// The selected value reads in `body-large` over `on-surface`
/// (`md.comp.filled-select.text-field.input-text.*`).
pub fn look(_spec: &PickerSpec, env: &Environment) -> MenuPickerLook {
    MenuPickerLook::new(chrome(env), layout()).value(
        Label::new()
            .font(typography::body_large())
            .color(Color::from(OnSurface)),
    )
}

/// The measures of the field: at least 280 wide
/// (`TextFieldDefaults.MinWidth`, which `ExposedDropdownMenuBox` inherits)
/// and 56 tall, inset 16 by 8, with 18 reserved for the chevron.
const fn layout() -> MenuPickerLayout {
    MenuPickerLayout {
        min_width: PICKER_MENU_MIN_WIDTH,
        min_height: PICKER_MIN_HEIGHT,
        horizontal_inset: PICKER_HORIZONTAL_INSET,
        vertical_inset: PICKER_VERTICAL_INSET,
        label_spacing: PICKER_LABEL_SPACING,
        indicator_space: PICKER_INDICATOR_SPACE,
        popup_gap: PICKER_MENU_POPUP_TOP_SPACING,
    }
}

/// The field's container: `corner-extra-small-top`, square below the active
/// indicator (`md.comp.filled-select.text-field.container.shape`).
const fn container_radii() -> RoundedRectRadii {
    RoundedRectRadii::new(
        INPUT_FILLED_CONTAINER_TOP_RADIUS,
        INPUT_FILLED_CONTAINER_TOP_RADIUS,
        0.0,
        0.0,
    )
}

/// The field's chrome: the container, the on-surface hover tint clipped to
/// it, no ripple and no ring, with the active indicator and the chevron.
fn chrome(env: &Environment) -> Chrome {
    let container = disabled_paint(
        SurfaceContainerHighest,
        OnSurface.with_opacity(INPUT_FILLED_DISABLED_CONTAINER_OPACITY),
        env,
    );
    Chrome::new(
        Surface::new(
            CornerShape::new(Corners::from(container_radii())),
            container,
        ),
        StateLayer::new(OnSurface).opacities(HOVER_STATE_LAYER_OPACITY, 0.0, 0.0, 0.0),
        PressFeedback::new(),
        FocusIndicator::Tint,
        motion::chrome(),
    )
    .hit(motion::hit_region())
    .custom(active_indicator(env))
    .custom(chevron(env))
}

/// The active indicator's rect: a line along the field's bottom edge, 1 tall
/// and 2 while focused.
fn indicator_rect(inputs: &Inputs<'_, ()>) -> Rect {
    let height = if inputs.state.contains(InteractionState::FOCUSED) {
        INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT
    } else {
        INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT
    };
    Rect::new(
        inputs.frame.x0,
        inputs.frame.y1 - height,
        inputs.frame.x1,
        inputs.frame.y1,
    )
}

/// The active indicator's paint: `on-surface-variant`, `on-surface` while
/// hovered and `primary` while focused
/// (`md.comp.filled-select.text-field.*active-indicator.color`), on-surface
/// at 38% while disabled.
fn indicator_paint(env: &Environment) -> Driven<Paint> {
    Driven::follow(
        gather(
            [
                OnSurfaceVariant.into(),
                OnSurface.into(),
                Primary.into(),
                OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
            ],
            env,
        ),
        |&[rest, hovered, focused, dim]: &[WorkingColor; 4], inputs: &Inputs<'_, ()>| {
            Paint::from(if inputs.state.contains(InteractionState::DISABLED) {
                dim
            } else if inputs.state.contains(InteractionState::FOCUSED) {
                focused
            } else if inputs.state.contains(InteractionState::HOVERED) {
                hovered
            } else {
                rest
            })
        },
    )
}

fn active_indicator(env: &Environment) -> Custom<(), (Driven<Rect>, Driven<Paint>)> {
    Custom::new(
        (Driven::new(indicator_rect), indicator_paint(env)),
        |recorder, _frame, (rect, paint)| recorder.fill(rect, paint),
    )
}

/// The dropdown chevron, centred in the indicator space at the trailing end.
fn chevron_path(frame: Rect) -> BezPath {
    let x = PICKER_INDICATOR_SPACE.mul_add(-0.5, frame.x1 - PICKER_HORIZONTAL_INSET);
    let y = frame.center().y;
    let mut path = BezPath::new();
    path.move_to(Point::new(
        x - PICKER_CHEVRON_HALF_WIDTH,
        y - PICKER_CHEVRON_HALF_HEIGHT,
    ));
    path.line_to(Point::new(x, y + PICKER_CHEVRON_HALF_HEIGHT));
    path.line_to(Point::new(
        x + PICKER_CHEVRON_HALF_WIDTH,
        y - PICKER_CHEVRON_HALF_HEIGHT,
    ));
    path
}

/// The chevron: `on-surface-variant`, on-surface at 38% while disabled
/// (`md.comp.filled-select.text-field.trailing-icon.color`).
fn chevron(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (disabled_paint(
            OnSurfaceVariant,
            OnSurface.with_opacity(DISABLED_CONTENT_OPACITY),
            env,
        ),),
        |recorder, frame, (paint,)| {
            recorder.stroke(
                chevron_path(frame),
                Stroke::new(PICKER_CHEVRON_STROKE_WIDTH),
                paint,
            );
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{chevron_path, container_radii, indicator_paint, indicator_rect, layout, look};
    use crate::MaterialColorScheme;
    use crate::dimensions::{
        INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT, INPUT_FILLED_CONTAINER_TOP_RADIUS,
        INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT, PICKER_INDICATOR_SPACE,
    };
    use hydrolysis::Style as _;
    use hydrolysis::draw::Paint;
    use hydrolysis::draw::kurbo::{Rect, RoundedRectRadii, Shape as _};
    use hydrolysis::theme::{Inputs, PickerSpec, SpecTint};
    use waterui::interaction::InteractionState;
    use waterui_controls::ControlSize;
    use waterui_core::layout::LayoutDirection;
    use waterui_core::{Environment, Signal as _};

    const FIELD: Rect = Rect::new(0.0, 0.0, 280.0, 56.0);

    fn env() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    fn at(state: InteractionState) -> Inputs<'static, ()> {
        Inputs::at_rest(FIELD, state, &())
    }

    /// The menu-style field is filled-TextField chrome:
    /// `ExposedDropdownMenuBox` inherits `TextFieldDefaults.MinWidth`.
    #[test]
    fn the_field_matches_the_filled_text_field_tokens() {
        let layout = layout();

        assert_eq!(layout.min_width, 280.0);
        assert_eq!(layout.min_height, 56.0);
        assert_eq!(layout.horizontal_inset, 16.0);
        assert_eq!(layout.vertical_inset, 8.0);
        assert_eq!(layout.indicator_space, 18.0);
        assert_eq!(layout.popup_gap, 4.0);
    }

    /// `md.comp.filled-select.text-field.container.shape` is rounded on top
    /// only.
    #[test]
    fn the_container_rounds_only_its_top() {
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
    fn the_active_indicator_thickens_while_focused() {
        let rest = indicator_rect(&at(InteractionState::empty()));
        let focused = indicator_rect(&at(InteractionState::FOCUSED));

        assert_eq!(rest.height(), INPUT_FILLED_ACTIVE_INDICATOR_HEIGHT);
        assert_eq!(focused.height(), INPUT_FILLED_FOCUS_ACTIVE_INDICATOR_HEIGHT);
        assert_eq!(rest.y1, FIELD.y1);
        assert_eq!(focused.y1, FIELD.y1);
        assert_eq!(rest.width(), FIELD.width());
    }

    #[test]
    fn the_active_indicator_is_coloured_by_state() {
        let colors = MaterialColorScheme::baseline_light();
        let paint = indicator_paint(&env());
        let at = |state| paint.target(&at(state));
        let solid = |color: hydrolysis::draw::WorkingColor| Paint::from(color);

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
        let Paint::Solid(disabled) = at(InteractionState::DISABLED) else {
            panic!("the active indicator paints solid colours");
        };
        assert_eq!(
            disabled.components[3],
            colors.on_surface.working().components[3] * 0.38
        );
    }

    /// The chevron sits in the space reserved at the trailing end, centred
    /// on the field's height.
    #[test]
    fn the_chevron_is_centred_in_the_indicator_space() {
        let bounds = chevron_path(FIELD).bounding_box();

        assert!((bounds.center().y - FIELD.center().y).abs() < 1e-6);
        assert!(
            (bounds.center().x - (FIELD.x1 - 16.0 - PICKER_INDICATOR_SPACE / 2.0)).abs() < 1e-6
        );
    }

    #[test]
    fn the_look_reports_its_layout() {
        let spec = PickerSpec::new(
            ControlSize::Small,
            LayoutDirection::LeftToRight,
            SpecTint::default(),
        );
        let look = look(&spec, &env());

        assert_eq!(look.layout(), layout());
        assert!(look.get_value().get_font().is_some());
        let value_color = look
            .get_value()
            .get_color()
            .expect("the value has a colour")
            .target(&Inputs::at_rest(Rect::ZERO, InteractionState::empty(), &()))
            .resolve(&env())
            .snapshot();
        assert_eq!(
            value_color,
            MaterialColorScheme::baseline_light().on_surface.working()
        );
    }
}
