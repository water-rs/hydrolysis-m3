use vello::kurbo::RoundedRectRadii;
use waterui::color::Color;
use waterui::reactive::{Computed, SignalExt as _, signal::IntoComputed, zip};
use waterui::text::font::{Font, ResolvedFont};
use waterui::{Environment, Signal as _, Str};
use waterui_backend_core::widget::{ButtonMetrics, InteractionStyle};
use waterui_controls::label::Label;
use waterui_core::resolve::Resolvable;
use waterui_graphics::color::ResolvedColor;

pub fn label_plain_text(label: &Label) -> Str {
    label
        .semantic_text()
        .clone()
        .resolve(&Environment::new())
        .content
        .snapshot()
        .to_plain()
}

pub fn interaction_style(
    state_layer_color: impl Into<Color>,
    corner_radius: f64,
) -> InteractionStyle {
    InteractionStyle::new(
        ButtonMetrics::new(0.0, 0.0, 0.0, 0.0),
        state_layer_color,
        RoundedRectRadii::from(corner_radius),
    )
}

pub fn interaction_style_with_radii(
    state_layer_color: impl Into<Color>,
    radii: RoundedRectRadii,
) -> InteractionStyle {
    InteractionStyle::new(
        ButtonMetrics::new(0.0, 0.0, 0.0, 0.0),
        state_layer_color,
        radii,
    )
}

pub fn conditional_color(
    condition: impl IntoComputed<bool>,
    when_true: impl Into<Color>,
    when_false: impl Into<Color>,
) -> ConditionalColor {
    ConditionalColor {
        condition: condition.into_computed(),
        when_true: when_true.into(),
        when_false: when_false.into(),
    }
}

#[derive(Debug, Clone)]
pub struct ConditionalColor {
    condition: Computed<bool>,
    when_true: Color,
    when_false: Color,
}

impl Resolvable for ConditionalColor {
    type Resolved = ResolvedColor;

    fn resolve(&self, env: &Environment) -> impl waterui::Signal<Output = Self::Resolved> {
        zip::zip(
            zip::zip(self.condition.clone(), self.when_true.resolve(env)),
            self.when_false.resolve(env),
        )
        .map(
            |((condition, when_true), when_false)| {
                if condition { when_true } else { when_false }
            },
        )
    }
}

/// A font that resolves to `when_true` or `when_false` with `condition` — the
/// typeface equivalent of [`conditional_color`], which the spec uses for the
/// prominent (bold) label of a selected navigation item.
pub fn conditional_font(
    condition: impl IntoComputed<bool>,
    when_true: Font,
    when_false: Font,
) -> Font {
    Font::new(ConditionalFont {
        condition: condition.into_computed(),
        when_true,
        when_false,
    })
}

#[derive(Debug, Clone)]
struct ConditionalFont {
    condition: Computed<bool>,
    when_true: Font,
    when_false: Font,
}

impl Resolvable for ConditionalFont {
    type Resolved = ResolvedFont;

    fn resolve(&self, env: &Environment) -> impl waterui::Signal<Output = Self::Resolved> {
        zip::zip(
            zip::zip(self.condition.clone(), self.when_true.resolve(env)),
            self.when_false.resolve(env),
        )
        .map(
            |((condition, when_true), when_false)| {
                if condition { when_true } else { when_false }
            },
        )
    }
}
