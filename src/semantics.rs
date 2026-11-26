use hydrolysis_style::draw::WorkingColor;
use hydrolysis_style::draw::kurbo::RoundedRectRadii;
use hydrolysis_style::theme::{
    Chrome, CornerShape, Corners, Driven, FocusIndicator, PressFeedback, StateLayer, Surface,
};
use waterui::color::Color;
use waterui::gesture::{DragEvent, GesturePhase};
use waterui::reactive::{Computed, SignalExt as _, impl_constant, signal::IntoComputed, zip};
use waterui::text::font::{Font, ResolvedFont};
use waterui::{Environment, Signal as _, Str};
use waterui_controls::label::{Label, LabelKind};
use waterui_core::resolve::Resolvable;

use crate::color::Secondary;
use crate::theme::motion;

pub fn label_plain_text<K: LabelKind>(label: &Label<K>) -> Str {
    label
        .semantic_text()
        .clone()
        .resolve(&Environment::new())
        .content()
        .snapshot()
        .to_plain()
}

/// `md.comp.focus-indicator.*`: the 3dp secondary ring that rides 2dp
/// outside a keyboard-focused control's state layer.
pub fn focus_indicator() -> FocusIndicator {
    FocusIndicator::Ring {
        width: 3.0,
        offset: 2.0,
        color: Secondary.into(),
    }
}

/// The MD3 state layer in `color`, at the `StateTokens` opacities.
pub fn state_layer(color: impl Into<Color>) -> StateLayer {
    StateLayer::new(color.into()).opacities(
        motion::HOVER_STATE_LAYER_OPACITY,
        motion::FOCUS_STATE_LAYER_OPACITY,
        motion::PRESSED_STATE_LAYER_OPACITY,
        motion::DRAGGED_STATE_LAYER_OPACITY,
    )
}

/// The interaction chrome of a composed Material control: no container of
/// its own — the control draws it — and a state layer in
/// `state_layer_color` on `corners`, with the Material ripple, the focus
/// indicator and the minimum touch target.
pub fn interaction_chrome(
    state_layer_color: impl Into<Color>,
    corners: impl Into<Driven<Corners>>,
) -> Chrome {
    Chrome::new(
        Surface::clear(CornerShape::new(corners)),
        state_layer(state_layer_color),
        PressFeedback::new().ripple(motion::ripple()),
        focus_indicator(),
        motion::chrome(),
    )
    .hit(motion::hit_region())
}

/// [`interaction_chrome`] on corners of `radius`.
pub fn interaction_chrome_radius(state_layer_color: impl Into<Color>, radius: f64) -> Chrome {
    interaction_chrome(
        state_layer_color,
        Corners::Radii(RoundedRectRadii::from_single_radius(radius)),
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
    type Resolved = WorkingColor;

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

impl_constant!(ConditionalColor);

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

/// Whether `drag` holds its press (started or updated).
pub const fn is_drag_held(drag: &DragEvent) -> bool {
    matches!(drag.phase(), GesturePhase::Started | GesturePhase::Updated)
}

#[cfg(any(
    feature = "toggle",
    feature = "picker",
    feature = "text-field",
    feature = "secure-field",
    feature = "date-picker",
    feature = "color-picker"
))]
/// `colors` resolved in `env` together, so one [`Driven`] reads them all.
pub fn gather<const N: usize>(
    colors: [Color; N],
    env: &Environment,
) -> Computed<[WorkingColor; N]> {
    let mut gathered: Computed<Vec<WorkingColor>> = Computed::constant(Vec::with_capacity(N));
    for color in colors {
        gathered = zip::zip(gathered, color.resolve(env))
            .map(|(mut all, color): (Vec<WorkingColor>, WorkingColor)| {
                all.push(color);
                all
            })
            .computed();
    }
    gathered
        .map(|all| <[WorkingColor; N]>::try_from(all).expect("every colour was gathered"))
        .computed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use waterui::gesture::PointerButton;
    use waterui::layout::Point;

    #[test]
    fn drag_is_held_only_while_started_or_updated() {
        for (phase, expected) in [
            (GesturePhase::Started, true),
            (GesturePhase::Updated, true),
            (GesturePhase::Ended, false),
            (GesturePhase::Cancelled, false),
        ] {
            let event = DragEvent::new(
                phase,
                Point::new(0.0, 0.0),
                Point::new(0.0, 0.0),
                Point::new(0.0, 0.0),
                PointerButton::Primary,
            );
            assert_eq!(is_drag_held(&event), expected);
        }
    }
}
