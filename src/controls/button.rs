use crate::dimensions::{
    BUTTON_LINK_HORIZONTAL_PADDING, BUTTON_LINK_UNDERLINE_BOTTOM_INSET,
    BUTTON_LINK_UNDERLINE_THICKNESS, BUTTON_LINK_VERTICAL_PADDING, BUTTON_MIN_WIDTH,
    BUTTON_TEXT_VERTICAL_PADDING, SHAPE_CORNER_LARGE, SHAPE_CORNER_MEDIUM, SHAPE_CORNER_SMALL,
    button_size_tokens,
};
use crate::theme::colors::MaterialColorScheme;
use crate::theme::state_layer;
use crate::{Brush, ButtonMetrics, DrawContext, WidgetInteractionState};
use waterui_controls::button::{ButtonSize, ButtonStyle};
use waterui_graphics::color::Color;

pub fn metrics(style: ButtonStyle, size: ButtonSize) -> ButtonMetrics {
    let tokens = button_size_tokens(size);
    match style {
        ButtonStyle::Automatic
        | ButtonStyle::Bordered
        | ButtonStyle::BorderedProminent
        | ButtonStyle::Plain
        | ButtonStyle::Borderless => ButtonMetrics::new(
            tokens.horizontal_space,
            BUTTON_TEXT_VERTICAL_PADDING,
            BUTTON_MIN_WIDTH,
            tokens.container_height,
        ),
        // A link has no container, so it takes neither the size scale's
        // padding nor its minimum box.
        ButtonStyle::Link => ButtonMetrics::new(
            BUTTON_LINK_HORIZONTAL_PADDING,
            BUTTON_LINK_VERTICAL_PADDING,
            0.0,
            0.0,
        ),
        _ => panic!("hydrolysis ButtonStyle variant is not implemented"),
    }
}

pub fn label_color(colors: &MaterialColorScheme, style: ButtonStyle, disabled: bool) -> Color {
    // MD3 disabled button label: on-surface at the 38% disabled-content
    // opacity, regardless of variant.
    if disabled {
        return colors
            .on_surface
            .view_color()
            .with_opacity(crate::theme::colors::DISABLED_CONTENT_OPACITY);
    }
    match style {
        // `Automatic` is the theme's default button, which in M3 is the
        // filled button for a text label — on-primary content — and the
        // standard button for an icon-only label — on-surface-variant
        // content over no container. The backend marks an icon-only label's
        // subtree so the two resolve differently.
        ButtonStyle::Automatic => Color::new(AutomaticLabelColor {
            filled: colors.on_primary.view_color(),
            standard: colors.on_surface_variant.view_color(),
        }),
        ButtonStyle::BorderedProminent => colors.on_primary.view_color(),
        // md.comp.button.outlined.label-text.color
        ButtonStyle::Bordered => colors.on_surface_variant.view_color(),
        // md.comp.button.text.label-text.color
        ButtonStyle::Plain | ButtonStyle::Link | ButtonStyle::Borderless => {
            colors.primary.view_color()
        }
        _ => panic!("hydrolysis ButtonStyle variant is not implemented"),
    }
}

/// Resolves to `filled` for an ordinary button label and to `standard` in an
/// icon-only label's subtree — `hydrolysis::IconOnlyButtonLabel` is installed
/// there — because `Automatic` maps to a different variant for the two.
#[derive(Debug, Clone)]
struct AutomaticLabelColor {
    filled: Color,
    standard: Color,
}

impl waterui_core::resolve::Resolvable for AutomaticLabelColor {
    type Resolved = waterui_graphics::color::ResolvedColor;

    fn resolve(
        &self,
        env: &waterui_core::Environment,
    ) -> impl waterui_core::Signal<Output = Self::Resolved> {
        if env.get::<hydrolysis::IconOnlyButtonLabel>().is_some() {
            self.standard.resolve(env)
        } else {
            self.filled.resolve(env)
        }
    }
}

/// A button container is `CornerFull` at every size in the tokens, so its
/// corner radius is half its height rather than a fixed number of dp. Reading
/// it off the bounds keeps a large or extra-large button a capsule, and keeps a
/// button stretched to fill a taller row — a navigation drawer line, say —
/// rounded to match whatever it is sitting on.
fn container_radius(bounds: vello::kurbo::Rect) -> f64 {
    bounds.height() / 2.0
}

/// `md.comp.button.<size>.pressed.container.shape`, reached through the
/// container's resting height because the draw callback does not carry a
/// `ButtonSize`: corner-small up to 40dp, corner-medium to 56, corner-large
/// beyond (a stretched button behaves as large).
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

/// How far the most recent press has grown, which the corner morph follows —
/// the same driving signal the stepper's seam corners use.
fn press_progress(state: WidgetInteractionState) -> f64 {
    f64::from(
        state
            .press_waves
            .latest()
            .map_or(0.0, |wave| wave.progress)
            .clamp(0.0, 1.0),
    )
}

/// The container's radius in `state`: resting `CornerFull`, morphing to
/// `PressedContainerShape` as the press grows.
fn container_radii(
    bounds: vello::kurbo::Rect,
    state: WidgetInteractionState,
) -> vello::kurbo::RoundedRectRadii {
    let resting = container_radius(bounds);
    let pressed = pressed_corner_radius(bounds.height());
    (pressed - resting)
        .mul_add(press_progress(state), resting)
        .into()
}

pub fn draw_chrome(
    colors: &MaterialColorScheme,
    draw: &mut dyn DrawContext,
    bounds: vello::kurbo::Rect,
    style: ButtonStyle,
    state: WidgetInteractionState,
) {
    // MD3 disabled button: the filled container drops to on-surface at the
    // token's 10% (`md.comp.button.filled.disabled.container.opacity`), and
    // the link underline follows the disabled label (on-surface at 38%). The
    // outlined border is outline-variant in every state, disabled included
    // (`md.comp.button.outlined.disabled.outline.color`). Text buttons have
    // no container to dim.
    match style {
        ButtonStyle::Automatic | ButtonStyle::BorderedProminent => {
            let fill = if state.disabled {
                colors.on_surface.peniko().multiply_alpha(0.1)
            } else {
                colors.primary.peniko()
            };
            draw.fill_rounded_rect(bounds, container_radii(bounds, state), &Brush::from(fill));
        }
        ButtonStyle::Bordered => {
            draw.stroke_rounded_rect(
                bounds,
                container_radii(bounds, state),
                &Brush::from(colors.outline_variant.peniko()),
                outline_width(bounds.height()),
            );
        }
        ButtonStyle::Link => {
            let underline = if state.disabled {
                colors.on_surface.peniko_disabled_content()
            } else {
                colors.primary.peniko()
            };
            let underline_y = (bounds.y1 - BUTTON_LINK_UNDERLINE_BOTTOM_INSET).max(bounds.y0);
            draw.stroke_line(
                vello::kurbo::Point::new(bounds.x0 + BUTTON_LINK_HORIZONTAL_PADDING, underline_y),
                vello::kurbo::Point::new(bounds.x1 - BUTTON_LINK_HORIZONTAL_PADDING, underline_y),
                &Brush::from(underline),
                BUTTON_LINK_UNDERLINE_THICKNESS,
            );
        }
        ButtonStyle::Plain | ButtonStyle::Borderless => {}
        _ => panic!("hydrolysis ButtonStyle variant is not implemented"),
    }
}

pub fn draw_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut dyn DrawContext,
    bounds: vello::kurbo::Rect,
    style: ButtonStyle,
    state: WidgetInteractionState,
) {
    // md.comp.button.filled.*.state-layer.color = on-primary;
    // md.comp.button.outlined.*.state-layer.color = on-surface-variant;
    // md.comp.button.text.*.state-layer.color = primary.
    let color = match style {
        ButtonStyle::Automatic | ButtonStyle::BorderedProminent => colors.on_primary.peniko(),
        ButtonStyle::Bordered => colors.on_surface_variant.peniko(),
        ButtonStyle::Link | ButtonStyle::Plain | ButtonStyle::Borderless => colors.primary.peniko(),
        _ => panic!("hydrolysis ButtonStyle variant is not implemented"),
    };
    state_layer::draw_bounded(draw, bounds, container_radii(bounds, state), color, state);
}

#[cfg(test)]
mod tests {
    use super::{draw_chrome, metrics};
    use crate::dimensions::{
        BUTTON_EXTRA_LARGE, BUTTON_EXTRA_SMALL, BUTTON_LARGE, BUTTON_MEDIUM, BUTTON_MIN_WIDTH,
        BUTTON_SMALL, BUTTON_TEXT_VERTICAL_PADDING,
    };
    use crate::{Brush, DrawContext, MaterialColorScheme};
    use vello::kurbo::{Affine, BezPath, Point, Rect, RoundedRectRadii};
    use waterui_controls::button::{ButtonSize, ButtonStyle};

    fn assert_button_metrics(style: ButtonStyle, expected_padding_x: f64) {
        let metrics = metrics(style, ButtonSize::Small);

        assert_eq!(metrics.padding_x, expected_padding_x);
        assert_eq!(metrics.padding_y, BUTTON_TEXT_VERTICAL_PADDING);
        assert_eq!(metrics.min_width, BUTTON_MIN_WIDTH);
        assert_eq!(metrics.min_height, BUTTON_SMALL.container_height);
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

    /// A button's measured height follows the size it was given.
    #[test]
    fn button_metrics_follow_the_requested_size() {
        for (size, tokens) in [
            (ButtonSize::ExtraSmall, BUTTON_EXTRA_SMALL),
            (ButtonSize::Small, BUTTON_SMALL),
            (ButtonSize::Medium, BUTTON_MEDIUM),
            (ButtonSize::Large, BUTTON_LARGE),
            (ButtonSize::ExtraLarge, BUTTON_EXTRA_LARGE),
        ] {
            let metrics = metrics(ButtonStyle::BorderedProminent, size);
            assert_eq!(metrics.min_height, tokens.container_height, "{size:?}");
            assert_eq!(metrics.padding_x, tokens.horizontal_space, "{size:?}");
        }
    }

    /// Values from `androidx.compose.material3`: `BaselineButtonTokens` for the
    /// container, `ButtonDefaults` for the minimum size and content padding.
    #[test]
    fn button_metrics_match_compose_button_tokens() {
        for style in [
            ButtonStyle::Plain,
            ButtonStyle::Borderless,
            ButtonStyle::Automatic,
            ButtonStyle::Bordered,
            ButtonStyle::BorderedProminent,
        ] {
            assert_button_metrics(style, BUTTON_SMALL.horizontal_space);
        }
        // ButtonSmallTokens.ContainerHeight
        assert_eq!(BUTTON_SMALL.container_height, 40.0);
        // BaselineButtonTokens.ContainerShapeRound is CornerFull, so every size
        // is a capsule and the radius follows the container's own height.
        for size in [
            BUTTON_EXTRA_SMALL,
            BUTTON_SMALL,
            BUTTON_MEDIUM,
            BUTTON_LARGE,
            BUTTON_EXTRA_LARGE,
        ] {
            let bounds = Rect::new(0.0, 0.0, 200.0, size.container_height);
            assert_eq!(super::container_radius(bounds), size.container_height / 2.0);
        }
        // ButtonDefaults.MinWidth
        assert_eq!(BUTTON_MIN_WIDTH, 58.0);
        // md.comp.button.small.leading-space / trailing-space.
        assert_eq!(BUTTON_SMALL.horizontal_space, 16.0);
        // ButtonDefaults.ContentPadding vertical
        assert_eq!(BUTTON_TEXT_VERTICAL_PADDING, 8.0);
    }

    #[derive(Default)]
    struct RecordingDrawContext {
        rounded_fills: Vec<(Rect, RoundedRectRadii, Brush)>,
        rounded_strokes: Vec<(Rect, RoundedRectRadii, Brush, f64)>,
    }

    impl DrawContext for RecordingDrawContext {
        fn fill_rect(&mut self, _rect: Rect, _brush: &Brush) {}

        fn fill_rounded_rect(&mut self, rect: Rect, radii: RoundedRectRadii, brush: &Brush) {
            self.rounded_fills.push((rect, radii, brush.clone()));
        }

        fn stroke_rect(&mut self, _rect: Rect, _brush: &Brush, _width: f64) {}

        fn stroke_rounded_rect(
            &mut self,
            rect: Rect,
            radii: RoundedRectRadii,
            brush: &Brush,
            width: f64,
        ) {
            self.rounded_strokes
                .push((rect, radii, brush.clone(), width));
        }

        fn stroke_line(&mut self, _from: Point, _to: Point, _brush: &Brush, _width: f64) {}

        fn stroke_circle(&mut self, _center: Point, _radius: f64, _brush: &Brush, _width: f64) {}

        fn fill_circle(&mut self, _center: Point, _radius: f64, _brush: &Brush) {}

        fn fill_path(&mut self, _path: &BezPath, _brush: &Brush) {}

        fn stroke_path(&mut self, _path: &BezPath, _brush: &Brush, _width: f64) {}
        fn draw_shadow(
            &mut self,
            _rect: Rect,
            _radii: RoundedRectRadii,
            _offset: vello::kurbo::Vec2,
            _blur: f64,
            _color: vello::peniko::Color,
        ) {
        }

        fn push_layer(&mut self, _alpha: f32, _clip: Option<&Rect>) {}

        fn pop_layer(&mut self) {}

        fn push_transform(&mut self, _affine: Affine) {}

        fn pop_transform(&mut self) {}
    }

    #[test]
    fn disabled_outlined_button_keeps_outline_variant_border() {
        // md.comp.button.outlined.disabled.outline.color: the border stays
        // outline-variant rather than dimming to on-surface.
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = RecordingDrawContext::default();

        draw_chrome(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 120.0, BUTTON_SMALL.container_height),
            ButtonStyle::Bordered,
            crate::WidgetInteractionState {
                disabled: true,
                ..crate::WidgetInteractionState::NONE
            },
        );

        assert_eq!(draw.rounded_strokes.len(), 1);
        assert!(matches!(
            &draw.rounded_strokes[0].2,
            Brush::Solid(color) if *color == colors.outline_variant.peniko()
        ));
    }

    #[test]
    fn disabled_button_label_color_is_on_surface_38() {
        // MD3 disabled button label: on-surface at 38% for every variant.
        use waterui_core::Signal as _;
        let colors = MaterialColorScheme::baseline_light();
        for style in [
            ButtonStyle::Automatic,
            ButtonStyle::Bordered,
            ButtonStyle::BorderedProminent,
            ButtonStyle::Plain,
        ] {
            let color = super::label_color(&colors, style, true);
            let resolved = color.resolve(&waterui_core::Environment::new()).snapshot();
            let expected = colors
                .on_surface
                .view_color()
                .with_opacity(crate::theme::colors::DISABLED_CONTENT_OPACITY)
                .resolve(&waterui_core::Environment::new())
                .snapshot();
            assert_eq!(
                (
                    resolved.red,
                    resolved.green,
                    resolved.blue,
                    resolved.opacity
                ),
                (
                    expected.red,
                    expected.green,
                    expected.blue,
                    expected.opacity
                ),
                "style {style:?}"
            );
        }
    }

    /// `Automatic` resolves differently for the two button kinds: the text
    /// button keeps on-primary content, while an icon-only label — marked by
    /// `hydrolysis::IconOnlyButtonLabel` — paints the standard icon button's
    /// on-surface-variant.
    #[test]
    fn automatic_label_color_resolves_standard_under_icon_only_marker() {
        use waterui_core::Signal as _;
        let colors = MaterialColorScheme::baseline_light();
        let color = super::label_color(&colors, ButtonStyle::Automatic, false);

        let mut env = waterui_core::Environment::new();
        let filled = color.resolve(&env).snapshot();
        let expected_filled = colors.on_primary.view_color().resolve(&env).snapshot();
        assert_eq!(filled.red, expected_filled.red);
        assert_eq!(filled.green, expected_filled.green);
        assert_eq!(filled.blue, expected_filled.blue);

        env.insert(hydrolysis::IconOnlyButtonLabel);
        let standard = color.resolve(&env).snapshot();
        let expected_standard = colors
            .on_surface_variant
            .view_color()
            .resolve(&env)
            .snapshot();
        assert_eq!(standard.red, expected_standard.red);
        assert_eq!(standard.green, expected_standard.green);
        assert_eq!(standard.blue, expected_standard.blue);
    }

    /// `Automatic` is the theme's default button; in M3 that is the filled
    /// button — a primary container with on-primary content, identical to the
    /// prominent style.
    #[test]
    fn automatic_button_renders_filled() {
        let colors = MaterialColorScheme::baseline_light();
        for style in [ButtonStyle::Automatic, ButtonStyle::BorderedProminent] {
            let mut draw = RecordingDrawContext::default();

            draw_chrome(
                &colors,
                &mut draw,
                Rect::new(0.0, 0.0, 120.0, BUTTON_SMALL.container_height),
                style,
                crate::WidgetInteractionState::NONE,
            );

            assert_eq!(draw.rounded_fills.len(), 1, "style {style:?}");
            assert!(
                matches!(
                    &draw.rounded_fills[0].2,
                    Brush::Solid(color) if *color == colors.primary.peniko()
                ),
                "style {style:?}"
            );
        }
    }

    #[test]
    fn outlined_button_uses_outline_variant_at_its_size_width() {
        // md.comp.button.outlined.outline.color = outline-variant;
        // md.comp.button.small.outlined.outline-width = 1.
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = RecordingDrawContext::default();

        draw_chrome(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 120.0, BUTTON_SMALL.container_height),
            ButtonStyle::Bordered,
            crate::WidgetInteractionState::NONE,
        );

        assert_eq!(draw.rounded_strokes.len(), 1);
        assert!(matches!(
            &draw.rounded_strokes[0].2,
            Brush::Solid(color) if *color == colors.outline_variant.peniko()
        ));
        assert_eq!(draw.rounded_strokes[0].3, 1.0);

        // md.comp.button.xlarge.outlined.outline-width = 3.
        let mut draw = RecordingDrawContext::default();
        draw_chrome(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 240.0, BUTTON_EXTRA_LARGE.container_height),
            ButtonStyle::Bordered,
            crate::WidgetInteractionState::NONE,
        );
        assert_eq!(draw.rounded_strokes[0].3, 3.0);
    }

    /// A press morphs the capsule toward the size band's
    /// `PressedContainerShape` — corner-small for small, corner-large for
    /// extra-large.
    #[test]
    fn pressed_button_morphs_its_corners() {
        use waterui_backend_core::widget::{PressWave, PressWaves};
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = RecordingDrawContext::default();

        let mut waves = PressWaves::EMPTY;
        waves.push(PressWave {
            origin: None,
            progress: 1.0,
            opacity: 0.1,
        });
        draw_chrome(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 120.0, BUTTON_SMALL.container_height),
            ButtonStyle::BorderedProminent,
            crate::WidgetInteractionState {
                pressed: true,
                press_waves: waves,
                ..crate::WidgetInteractionState::NONE
            },
        );

        let radii = draw.rounded_fills[0].1;
        // md.comp.button.small.pressed.container.shape = corner-small (8).
        assert!((radii.top_left - 8.0).abs() < 0.01);
    }
}
