//! The Material 3 button look: filled, outlined, text and link buttons
//! from the `md.comp.button.*` tokens.

use hydrolysis::draw::kurbo::{Line, Point, Stroke};
use hydrolysis::draw::{Draw as _, Paint, WorkingColor};
use hydrolysis::theme::{
    Border, ButtonLook, ButtonSpec, Chrome, CornerShape, Corners, Custom, Driven, Elevation,
    Inputs, Label, PressFeedback, Surface,
};
use waterui::contract_violation;
use waterui::interaction::InteractionState;
use waterui::reactive::zip::zip;
use waterui_controls::button::ButtonStyle;
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{OnPrimary, OnSurface, OnSurfaceVariant, OutlineVariant, Shadow};
use crate::dimensions::{
    BUTTON_LINK_HORIZONTAL_PADDING, BUTTON_LINK_UNDERLINE_BOTTOM_INSET,
    BUTTON_LINK_UNDERLINE_THICKNESS, BUTTON_LINK_VERTICAL_PADDING, BUTTON_MIN_WIDTH,
    BUTTON_TEXT_VERTICAL_PADDING, SHAPE_CORNER_LARGE, SHAPE_CORNER_MEDIUM, SHAPE_CORNER_SMALL,
    button_size_tokens,
};
use crate::elevation::{MaterialElevationLevel, chrome_elevation};
use crate::semantics::{focus_indicator, state_layer};
use crate::theme::colors::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY};
use crate::theme::{motion, typography};

/// `style` when this theme implements it. `ButtonStyle` is `#[non_exhaustive]`,
/// so a variant added to the framework after this theme is a contract
/// violation; it draws as [`ButtonStyle::Automatic`], the theme's default.
pub fn supported_style(style: ButtonStyle) -> ButtonStyle {
    match style {
        ButtonStyle::Automatic
        | ButtonStyle::BorderedProminent
        | ButtonStyle::Bordered
        | ButtonStyle::Plain
        | ButtonStyle::Borderless
        | ButtonStyle::Link => style,
        _ => {
            contract_violation!(
                "hydrolysis-m3 does not implement ButtonStyle {style:?}; drawing ButtonStyle::Automatic"
            );
            ButtonStyle::Automatic
        }
    }
}

/// The look of a text button in `spec`.
///
/// A `ButtonStyle` or `ControlSize` variant this theme does not implement is
/// a contract violation; a release build draws the automatic style and the
/// medium size.
pub fn look(spec: &ButtonSpec, env: &Environment) -> ButtonLook {
    let style = supported_style(spec.style());
    let tint = spec.tint();
    let shape = CornerShape::new(container_corners(pressed_corner_radius));
    let surface = match style {
        // `Automatic` is the theme's default button, which in M3 is the
        // filled button for a text label.
        ButtonStyle::Automatic | ButtonStyle::BorderedProminent => Surface::new(
            shape,
            disabled_paint(
                tint.clone(),
                OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY),
                env,
            ),
        )
        .elevation(filled_elevation(env)),
        // md.comp.button.outlined.outline.color = outline-variant in every
        // state, disabled included.
        ButtonStyle::Bordered => {
            Surface::clear(shape).border(outline(OutlineVariant, outline_width, env))
        }
        ButtonStyle::Plain | ButtonStyle::Borderless | ButtonStyle::Link => Surface::clear(shape),
        _ => unreachable!("`supported_style` maps every unimplemented variant to a known one"),
    };
    // md.comp.button.filled.*.state-layer.color = on-primary;
    // md.comp.button.outlined.*.state-layer.color = on-surface-variant;
    // md.comp.button.text.*.state-layer.color = primary.
    let layer = match style {
        ButtonStyle::Automatic | ButtonStyle::BorderedProminent => Color::from(OnPrimary),
        ButtonStyle::Bordered => Color::from(OnSurfaceVariant),
        ButtonStyle::Plain | ButtonStyle::Borderless | ButtonStyle::Link => tint.clone(),
        _ => unreachable!("`supported_style` maps every unimplemented variant to a known one"),
    };
    let mut chrome = interactive(surface, layer);
    if style == ButtonStyle::Link {
        chrome = chrome.custom(link_underline(tint, env));
    }
    let look = ButtonLook::new(chrome, label(spec));
    if style == ButtonStyle::Link {
        // A link has no container, so it takes neither the size scale's
        // padding nor its minimum box.
        look.padding(BUTTON_LINK_HORIZONTAL_PADDING, BUTTON_LINK_VERTICAL_PADDING)
    } else {
        let tokens = button_size_tokens(spec.size());
        look.padding(tokens.horizontal_space, BUTTON_TEXT_VERTICAL_PADDING)
            .minimum_size(hydrolysis::draw::kurbo::Size::new(
                BUTTON_MIN_WIDTH,
                tokens.container_height,
            ))
    }
}

/// A Material control's chrome on `surface`: the state layer in `layer`,
/// the ripple, the focus indicator, the control motion and the minimum
/// touch target.
pub fn interactive(surface: Surface, layer: Color) -> Chrome {
    Chrome::new(
        surface,
        state_layer(layer),
        PressFeedback::new().ripple(motion::ripple()),
        focus_indicator(),
        motion::chrome(),
    )
    .hit(motion::hit_region())
}

/// The label styling of a button in `spec`: `label-large`, in the
/// variant's content colour, on-surface at 38% while disabled.
pub fn label(spec: &ButtonSpec) -> Label {
    Label::new()
        .font(typography::label_large())
        .color(label_color(spec.style(), spec.icon_only(), spec.tint()))
}

/// The label colour of `style`, `icon_only` or not, in every state; the
/// text-like variants draw in `tint`.
///
/// A `ButtonStyle` variant this theme does not implement is a contract
/// violation; a release build draws the automatic style.
pub fn label_color(style: ButtonStyle, icon_only: bool, tint: Color) -> Driven<Color> {
    let rest = match supported_style(style) {
        // The filled button's on-primary content for a text label; the
        // standard icon button's on-surface-variant over no container for
        // an icon-only one.
        ButtonStyle::Automatic if icon_only => Color::from(OnSurfaceVariant),
        ButtonStyle::Automatic | ButtonStyle::BorderedProminent => Color::from(OnPrimary),
        // md.comp.button.outlined.label-text.color
        ButtonStyle::Bordered => Color::from(OnSurfaceVariant),
        // md.comp.button.text.label-text.color
        ButtonStyle::Plain | ButtonStyle::Link | ButtonStyle::Borderless => tint,
        _ => unreachable!("`supported_style` maps every unimplemented variant to a known one"),
    };
    // MD3 disabled button label: on-surface at the 38% disabled-content
    // opacity, regardless of variant.
    waterui_core::interaction::StateValue::new(rest)
        .when(
            InteractionState::DISABLED,
            Color::from(OnSurface.with_opacity(DISABLED_CONTENT_OPACITY)),
        )
        .into()
}

/// `color` as a paint that follows it.
pub fn solid_paint<V: 'static>(color: impl Into<Color>, env: &Environment) -> Driven<Paint, V> {
    Driven::follow(
        color.into().resolve(env),
        |color: &WorkingColor, _inputs: &Inputs<'_, V>| Paint::from(*color),
    )
}

/// `rest` resolved in `env` as a paint, `disabled` while the widget is
/// disabled.
pub fn disabled_paint<V: 'static>(
    rest: impl Into<Color>,
    disabled: impl Into<Color>,
    env: &Environment,
) -> Driven<Paint, V> {
    Driven::follow(
        zip(rest.into().resolve(env), disabled.into().resolve(env)),
        |&(rest, disabled): &(WorkingColor, WorkingColor), inputs: &Inputs<'_, V>| {
            Paint::from(if inputs.state.contains(InteractionState::DISABLED) {
                disabled
            } else {
                rest
            })
        },
    )
}

/// A border in `color` whose width follows the frame's height through
/// `width`.
pub fn outline(
    color: impl Into<Color>,
    width: fn(f64) -> f64,
    env: &Environment,
) -> Driven<Border> {
    Driven::follow(
        color.into().resolve(env),
        move |color: &WorkingColor, inputs: &Inputs<'_, ()>| {
            Border::new(width(inputs.frame.height()), *color)
        },
    )
}

/// Resting `CornerFull`, morphing while pressed to the radius `pressed`
/// gives the frame's height — the size band's `PressedContainerShape`. The
/// height carries the band because a button stretched to fill a taller row
/// behaves as the larger size.
pub fn container_corners(pressed: fn(f64) -> f64) -> Driven<Corners> {
    Driven::new(move |inputs: &Inputs<'_, ()>| {
        if inputs.state.contains(InteractionState::PRESSED) {
            Corners::from(pressed(inputs.frame.height()))
        } else {
            Corners::Full
        }
    })
}

/// `md.comp.button.filled.container.elevation`: level0 at rest, level1
/// while hovered, level0 again while pressed, focused or disabled.
fn filled_elevation(env: &Environment) -> Driven<Elevation> {
    Driven::follow(
        Color::from(Shadow).resolve(env),
        |shadow: &WorkingColor, inputs: &Inputs<'_, ()>| {
            let state = inputs.state;
            if state.contains(InteractionState::HOVERED)
                && !state.intersects(
                    InteractionState::PRESSED
                        | InteractionState::FOCUSED
                        | InteractionState::DISABLED,
                )
            {
                chrome_elevation(MaterialElevationLevel::LEVEL1, *shadow)
            } else {
                Elevation::NONE
            }
        },
    )
}

/// A link's underline: the tint, following the label to on-surface at 38%
/// while disabled.
fn link_underline(tint: Color, env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (disabled_paint(
            tint,
            OnSurface.with_opacity(DISABLED_CONTENT_OPACITY),
            env,
        ),),
        |recorder, frame, (paint,)| {
            let y = (frame.y1 - BUTTON_LINK_UNDERLINE_BOTTOM_INSET).max(frame.y0);
            recorder.stroke(
                Line::new(
                    Point::new(frame.x0 + BUTTON_LINK_HORIZONTAL_PADDING, y),
                    Point::new(frame.x1 - BUTTON_LINK_HORIZONTAL_PADDING, y),
                ),
                Stroke::new(BUTTON_LINK_UNDERLINE_THICKNESS),
                paint,
            );
        },
    )
}

/// `md.comp.button.<size>.pressed.container.shape` by the container's
/// height: corner-small up to 40dp, corner-medium to 56, corner-large
/// beyond.
fn pressed_corner_radius(height: f64) -> f64 {
    if height <= BUTTON_EXTRA_SMALL_HEIGHT_BAND {
        SHAPE_CORNER_SMALL
    } else if height <= BUTTON_MEDIUM_HEIGHT_BAND {
        SHAPE_CORNER_MEDIUM
    } else {
        SHAPE_CORNER_LARGE
    }
}

/// `md.comp.button.<size>.outlined.outline-width`: 1dp through medium, 2dp at
/// large, 3dp at extra-large.
fn outline_width(height: f64) -> f64 {
    if height <= BUTTON_MEDIUM_HEIGHT_BAND {
        1.0
    } else if height <= BUTTON_LARGE_HEIGHT_BAND {
        2.0
    } else {
        3.0
    }
}

const BUTTON_EXTRA_SMALL_HEIGHT_BAND: f64 = 40.0;
const BUTTON_MEDIUM_HEIGHT_BAND: f64 = 56.0;
const BUTTON_LARGE_HEIGHT_BAND: f64 = 96.0;

#[cfg(test)]
mod tests {
    use super::look;
    use crate::MaterialColorScheme;
    use crate::dimensions::{
        BUTTON_EXTRA_LARGE, BUTTON_EXTRA_SMALL, BUTTON_LARGE, BUTTON_MEDIUM, BUTTON_MIN_WIDTH,
        BUTTON_SMALL, BUTTON_TEXT_VERTICAL_PADDING,
    };
    use hydrolysis::Style as _;
    use hydrolysis::draw::kurbo::{Rect, Vec2};
    use hydrolysis::draw::{Paint, WorkingColor};
    use hydrolysis::theme::{ButtonLook, ButtonSpec, Corners, Fill, Inputs, SpecTint};
    use waterui::interaction::InteractionState;
    use waterui::theme::Tint;
    use waterui_controls::ControlSize;
    use waterui_controls::button::ButtonStyle;
    use waterui_core::Environment;
    use waterui_core::Signal as _;
    use waterui_core::layout::LayoutDirection;
    use waterui_graphics::color::{Color, Working};

    fn env() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    fn spec(style: ButtonStyle, size: ControlSize) -> ButtonSpec {
        ButtonSpec::new(
            style,
            size,
            false,
            LayoutDirection::LeftToRight,
            SpecTint::default(),
        )
    }

    fn small(style: ButtonStyle) -> ButtonLook {
        look(&spec(style, ControlSize::Small), &env())
    }

    /// The small `style` button of a subtree tinted `tint`, with the
    /// environment it draws in.
    fn tinted(style: ButtonStyle, tint: WorkingColor) -> (ButtonLook, Environment) {
        let env = env().extending(Tint::new(Color::new(Working(tint))));
        let spec = ButtonSpec::new(
            style,
            ControlSize::Small,
            false,
            LayoutDirection::LeftToRight,
            SpecTint::resolve(&env),
        );
        (look(&spec, &env), env)
    }

    fn frame(height: f64) -> Rect {
        Rect::new(0.0, 0.0, 120.0, height)
    }

    fn fill(look: &ButtonLook, state: InteractionState) -> WorkingColor {
        let Fill::Paint(paint) = look.chrome().surface().fill() else {
            panic!("a Material button fills with a paint")
        };
        match paint.target(&Inputs::at_rest(
            frame(BUTTON_SMALL.container_height),
            state,
            &(),
        )) {
            Paint::Solid(color) => color,
            other => panic!("a Material button fills with a solid colour, not {other:?}"),
        }
    }

    fn label(look: &ButtonLook, state: InteractionState) -> WorkingColor {
        look.label()
            .get_color()
            .expect("a Material button colours its label")
            .target(&Inputs::at_rest(Rect::ZERO, state, &()))
            .resolve(&env())
            .snapshot()
    }

    /// The Expressive size scale, from Compose's `ButtonXSmallTokens` through
    /// `ButtonXLargeTokens`. Height, padding, icon size and corner shape all
    /// move together, so a size is checked as a whole token set.
    #[test]
    fn button_size_scale_matches_compose_button_size_tokens() {
        // ButtonXSmallTokens
        assert_eq!(BUTTON_EXTRA_SMALL.container_height, 32.0);
        assert_eq!(BUTTON_EXTRA_SMALL.horizontal_space, 12.0);
        assert_eq!(BUTTON_EXTRA_SMALL.icon_size, 20.0);
        assert_eq!(BUTTON_EXTRA_SMALL.icon_label_space, 8.0);
        // ButtonSmallTokens
        assert_eq!(BUTTON_SMALL.container_height, 40.0);
        assert_eq!(BUTTON_SMALL.horizontal_space, 16.0);
        assert_eq!(BUTTON_SMALL.icon_size, 20.0);
        // ButtonMediumTokens
        assert_eq!(BUTTON_MEDIUM.container_height, 56.0);
        assert_eq!(BUTTON_MEDIUM.horizontal_space, 24.0);
        assert_eq!(BUTTON_MEDIUM.icon_size, 24.0);
        // ButtonLargeTokens
        assert_eq!(BUTTON_LARGE.container_height, 96.0);
        assert_eq!(BUTTON_LARGE.horizontal_space, 48.0);
        assert_eq!(BUTTON_LARGE.icon_size, 32.0);
        assert_eq!(BUTTON_LARGE.icon_label_space, 12.0);
        assert_eq!(BUTTON_LARGE.outline_width, 2.0);
        // ButtonXLargeTokens
        assert_eq!(BUTTON_EXTRA_LARGE.container_height, 136.0);
        assert_eq!(BUTTON_EXTRA_LARGE.horizontal_space, 64.0);
        assert_eq!(BUTTON_EXTRA_LARGE.icon_size, 40.0);
        assert_eq!(BUTTON_EXTRA_LARGE.icon_label_space, 16.0);
        assert_eq!(BUTTON_EXTRA_LARGE.outline_width, 3.0);

        // The scale is monotonic in every dimension it changes.
        let scale = [
            BUTTON_EXTRA_SMALL,
            BUTTON_SMALL,
            BUTTON_MEDIUM,
            BUTTON_LARGE,
            BUTTON_EXTRA_LARGE,
        ];
        for pair in scale.windows(2) {
            let (smaller, larger) = (pair[0], pair[1]);
            assert!(larger.container_height > smaller.container_height);
            assert!(larger.horizontal_space >= smaller.horizontal_space);
            assert!(larger.icon_size >= smaller.icon_size);
        }
    }

    /// A button measures with the size it was given: the size scale's
    /// height and padding, `ButtonDefaults.MinWidth` and the content
    /// padding's vertical 8dp, for every container style.
    #[test]
    fn button_look_measures_with_the_requested_size() {
        let env = env();
        for (size, tokens) in [
            (ControlSize::ExtraSmall, BUTTON_EXTRA_SMALL),
            (ControlSize::Small, BUTTON_SMALL),
            (ControlSize::Medium, BUTTON_MEDIUM),
            (ControlSize::Large, BUTTON_LARGE),
            (ControlSize::ExtraLarge, BUTTON_EXTRA_LARGE),
        ] {
            for style in [
                ButtonStyle::Plain,
                ButtonStyle::Borderless,
                ButtonStyle::Automatic,
                ButtonStyle::Bordered,
                ButtonStyle::BorderedProminent,
            ] {
                let look = look(&spec(style, size), &env);
                assert_eq!(
                    look.get_padding(),
                    Vec2::new(tokens.horizontal_space, BUTTON_TEXT_VERTICAL_PADDING),
                    "{style:?} {size:?}"
                );
                assert_eq!(look.get_minimum_size().width, BUTTON_MIN_WIDTH);
                assert_eq!(
                    look.get_minimum_size().height,
                    tokens.container_height,
                    "{style:?} {size:?}"
                );
            }
        }
        // ButtonDefaults.MinWidth and ContentPadding vertical.
        assert_eq!(BUTTON_MIN_WIDTH, 58.0);
        assert_eq!(BUTTON_TEXT_VERTICAL_PADDING, 8.0);
    }

    /// `Automatic` is the theme's default button; in M3 that is the filled
    /// button — a primary container with on-primary content, identical to
    /// the prominent style. Disabled, the container drops to on-surface at
    /// 12%.
    #[test]
    fn automatic_button_is_filled() {
        let colors = MaterialColorScheme::baseline_light();
        for style in [ButtonStyle::Automatic, ButtonStyle::BorderedProminent] {
            let look = small(style);
            assert_eq!(
                fill(&look, InteractionState::empty()),
                colors.primary.working(),
                "{style:?}"
            );
            assert_eq!(
                fill(&look, InteractionState::DISABLED),
                colors.on_surface.working_disabled_container(),
                "{style:?}"
            );
            assert_eq!(
                label(&look, InteractionState::empty()).components,
                colors.on_primary.working().components,
                "{style:?}"
            );
        }
    }

    /// A tinted subtree's filled container and text-button label draw in
    /// the tint; an untinted one's draw in `Primary`, the `Accent` token's
    /// value.
    #[test]
    fn buttons_draw_with_the_subtrees_tint() {
        let colors = MaterialColorScheme::baseline_light();
        let tint = WorkingColor::new([0.9, 0.1, 0.2, 1.0]);
        assert_ne!(tint, colors.primary.working());

        let (filled, _) = tinted(ButtonStyle::BorderedProminent, tint);
        let Fill::Paint(paint) = filled.chrome().surface().fill() else {
            panic!("a Material button fills with a paint")
        };
        let Paint::Solid(container) = paint.target(&Inputs::at_rest(
            frame(BUTTON_SMALL.container_height),
            InteractionState::empty(),
            &(),
        )) else {
            panic!("a Material button fills with a solid colour")
        };
        assert_eq!(container, tint);

        let (text, env) = tinted(ButtonStyle::Plain, tint);
        let color = text
            .label()
            .get_color()
            .expect("a Material button colours its label")
            .target(&Inputs::at_rest(Rect::ZERO, InteractionState::empty(), &()))
            .resolve(&env)
            .snapshot();
        assert_eq!(color, tint);

        assert_eq!(
            fill(
                &small(ButtonStyle::BorderedProminent),
                InteractionState::empty()
            ),
            colors.primary.working()
        );
        assert_eq!(
            label(&small(ButtonStyle::Plain), InteractionState::empty()),
            colors.primary.working()
        );
    }

    /// The filled button rises to level1 only while hovered.
    #[test]
    fn filled_button_elevates_while_hovered() {
        let look = small(ButtonStyle::BorderedProminent);
        let elevation = |state| {
            look.chrome()
                .surface()
                .get_elevation()
                .target(&Inputs::at_rest(frame(40.0), state, &()))
        };
        assert!(elevation(InteractionState::empty()).is_none());
        assert!(!elevation(InteractionState::HOVERED).is_none());
        assert!(elevation(InteractionState::HOVERED | InteractionState::PRESSED).is_none());
    }

    /// md.comp.button.outlined: an outline-variant border in every state,
    /// disabled included, 1dp through medium and 3dp at extra-large.
    #[test]
    fn outlined_button_uses_outline_variant_at_its_size_width() {
        let colors = MaterialColorScheme::baseline_light();
        let look = small(ButtonStyle::Bordered);
        let border = |height, state| {
            look.chrome()
                .surface()
                .get_border()
                .expect("the outlined button has a border")
                .target(&Inputs::at_rest(frame(height), state, &()))
        };
        for state in [InteractionState::empty(), InteractionState::DISABLED] {
            let border = border(BUTTON_SMALL.container_height, state);
            assert_eq!(border.width, 1.0);
            assert!(matches!(
                border.paint,
                Paint::Solid(color) if color == colors.outline_variant.working()
            ));
        }
        assert_eq!(
            border(
                BUTTON_EXTRA_LARGE.container_height,
                InteractionState::empty()
            )
            .width,
            3.0
        );
    }

    /// MD3 disabled button label: on-surface at 38% for every variant.
    #[test]
    fn disabled_button_label_is_on_surface_38() {
        let colors = MaterialColorScheme::baseline_light();
        for style in [
            ButtonStyle::Automatic,
            ButtonStyle::Bordered,
            ButtonStyle::BorderedProminent,
            ButtonStyle::Plain,
        ] {
            assert_eq!(
                label(&small(style), InteractionState::DISABLED).components,
                colors.on_surface.working_disabled_content().components,
                "{style:?}"
            );
        }
    }

    /// The capsule morphs to the size band's `PressedContainerShape` while
    /// pressed — corner-small (8dp) on a small button — and back at rest.
    #[test]
    fn pressed_button_morphs_its_corners() {
        let look = small(ButtonStyle::BorderedProminent);
        let corners = |state| {
            look.chrome()
                .surface()
                .shape()
                .corners()
                .target(&Inputs::at_rest(
                    frame(BUTTON_SMALL.container_height),
                    state,
                    &(),
                ))
        };
        assert_eq!(corners(InteractionState::empty()), Corners::Full);
        // md.comp.button.small.pressed.container.shape = corner-small (8).
        assert_eq!(corners(InteractionState::PRESSED), Corners::from(8.0));
    }
}
