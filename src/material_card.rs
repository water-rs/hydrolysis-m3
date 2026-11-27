//! Material Design 3 card components composed from `WaterUI` card primitives.

use core::fmt::{self, Debug};

use crate::view_ext::ViewExt as _;
use waterui_accessibility::AccessibilityRole;
use waterui_controls::card::{Card as WaterCard, CardStyle};
use waterui_core::{AnyView, Environment, Str, View};

/// A Material Design 3 card container.
#[derive(Clone)]
pub struct MaterialCard<Content> {
    content: Content,
    style: CardStyle,
    accessibility_label: Option<Str>,
}

impl<Content> Debug for MaterialCard<Content> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaterialCard")
            .field("style", &self.style)
            .field("accessibility_label", &self.accessibility_label)
            .finish_non_exhaustive()
    }
}

impl<Content> MaterialCard<Content> {
    /// Creates a filled Material card.
    #[must_use]
    pub const fn new(content: Content) -> Self {
        Self {
            content,
            style: CardStyle::Filled,
            accessibility_label: None,
        }
    }

    /// Uses the elevated Material card style.
    #[must_use]
    pub const fn elevated(mut self) -> Self {
        self.style = CardStyle::Elevated;
        self
    }

    /// Uses the filled Material card style.
    #[must_use]
    pub const fn filled(mut self) -> Self {
        self.style = CardStyle::Filled;
        self
    }

    /// Uses the outlined Material card style.
    #[must_use]
    pub const fn outlined(mut self) -> Self {
        self.style = CardStyle::Outlined;
        self
    }

    /// Sets the card accessibility label.
    #[must_use]
    pub fn label(mut self, label: impl Into<Str>) -> Self {
        self.accessibility_label = Some(label.into());
        self
    }
}

impl<Content> View for MaterialCard<Content>
where
    Content: View,
{
    fn body(self, _env: &Environment) -> impl View {
        let card = WaterCard::new(self.content)
            .style(self.style)
            .a11y_role(AccessibilityRole::Group);

        match self.accessibility_label {
            Some(label) => AnyView::new(card.a11y_label(label)),
            None => AnyView::new(card),
        }
    }
}

/// Creates a filled Material Design 3 card.
#[must_use]
pub const fn material_card<Content>(content: Content) -> MaterialCard<Content> {
    MaterialCard::new(content)
}

#[cfg(test)]
mod tests {
    use hydrolysis_style::Style as _;
    use hydrolysis_style::draw::kurbo::Rect;
    use hydrolysis_style::theme::{Inputs, WidgetTheme as _};
    use waterui_controls::card::CardStyle;
    use waterui_core::Environment;
    use waterui_interaction::interaction::InteractionState;

    use crate::Material3;

    #[test]
    fn material_card_look_matches_compose_card_tokens() {
        let style = Material3::defaults();
        let mut env = Environment::new();
        style.install_tokens(&mut env);
        let rest = Inputs::at_rest(
            Rect::new(0.0, 0.0, 200.0, 100.0),
            InteractionState::empty(),
            &(),
        );

        let elevated = style.card(CardStyle::Elevated, &env);
        let filled = style.card(CardStyle::Filled, &env);
        let outlined = style.card(CardStyle::Outlined, &env);

        assert!(!elevated.surface().get_elevation().target(&rest).is_none());
        assert!(filled.surface().get_elevation().target(&rest).is_none());
        assert!(outlined.surface().get_border().is_some());
        assert_eq!(outlined.layout().horizontal_padding, 16.0);
    }
}
