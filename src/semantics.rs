use hydrolysis::draw::WorkingColor;
use hydrolysis::theme::{
    Chrome, CornerShape, Corners, Driven, FocusIndicator, PressFeedback, StateLayer, Surface,
};
use waterui::color::Color;
use waterui::contract_violation;
use waterui::gesture::{DragEvent, GesturePhase};
use waterui::reactive::{Computed, SignalExt as _, signal::IntoComputed, zip};
use waterui::text::font::{Font, ResolvedFont};
use waterui::{Environment, Signal as _, Str};
use waterui_controls::label::Label;
use waterui_core::resolve::Resolvable;

use crate::color::Secondary;
use crate::theme::{motion, state_layer};

pub fn label_plain_text(label: &Label) -> Str {
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
    StateLayer::new(color).opacities(
        state_layer::HOVER_STATE_LAYER_OPACITY,
        state_layer::FOCUS_STATE_LAYER_OPACITY,
        state_layer::PRESSED_STATE_LAYER_OPACITY,
        state_layer::DRAGGED_STATE_LAYER_OPACITY,
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
    interaction_chrome(state_layer_color, Corners::from(radius))
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

/// Whether the drag gesture `env` delivers holds its press (started or
/// updated), for `owner`'s press tracking.
///
/// A backend places a `DragEvent` in the environment before it runs a drag
/// handler; a handler that finds none is a backend contract violation. A
/// release build reports it and returns `None`, and the caller leaves its
/// press state as it was.
pub fn drag_held(env: &Environment, owner: &str) -> Option<bool> {
    let Some(event) = env.get::<DragEvent>() else {
        contract_violation!(
            "{owner} drag gesture ran without a DragEvent in its environment; its press state is left unchanged"
        );
        return None;
    };
    Some(matches!(
        event.phase(),
        GesturePhase::Started | GesturePhase::Updated
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(
        debug_assertions,
        should_panic(expected = "test owner drag gesture ran without a DragEvent")
    )]
    fn a_drag_handler_without_an_event_leaves_the_press_unchanged() {
        assert_eq!(drag_held(&Environment::new(), "test owner"), None);
    }
}
