//! Material Design 3 list components composed from `WaterUI` primitives.

use core::fmt::{self, Debug};

use waterui::color::Color;
use waterui::component::list::{
    List as WaterList, ListContent, ListItem as WaterListItem, ListItemSink,
};
use waterui::component::{hstack, spacer, vstack};
use waterui::interaction::{InteractionState, StateValue};
use waterui::layout::{HorizontalAlignment, padding::EdgeInsets};
use waterui::prelude::dynamic::watch;
use waterui::reactive::{Computed, SignalExt as _, binding, signal::IntoComputed};
use waterui::shape::{FixedRoundedRectangle, ShapeExt as _};
use waterui::{AnyView, Environment, Str, View, ViewExt as _};
use waterui_controls::label::{IntoLabel, Label};
use waterui_core::handler::{AnyViewBuilder, Handler, SharedAction, boxed_action};

use crate::color::{OnSecondaryContainer, OnSurface, OnSurfaceVariant, SecondaryContainer};
use crate::dimensions::{LIST_ONE_LINE_ROW_HEIGHT, LIST_VERTICAL_INSET};
use crate::semantics::{interaction_style, label_plain_text};
use crate::theme::typography;
use num_traits::ToPrimitive;

const LIST_CONTAINER_TOP_SPACE: f32 = 8.0;
const LIST_CONTAINER_BOTTOM_SPACE: f32 = 8.0;
const LIST_ITEM_TWO_LINE_CONTAINER_HEIGHT: f32 = 72.0;
const LIST_ITEM_SLOT_GAP: f32 = 16.0;
/// `md.comp.list.list-item.leading-icon.expressive.size`.
const LIST_ITEM_LEADING_ICON_SIZE: f32 = 20.0;
/// `md.comp.list.list-item.trailing-icon.expressive.size`.
const LIST_ITEM_TRAILING_ICON_SIZE: f32 = 20.0;

/// A Material Design 3 list.
pub struct MaterialList<Content> {
    content: Content,
}

impl<Content> Debug for MaterialList<Content> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaterialList").finish_non_exhaustive()
    }
}

impl<Content> MaterialList<Content> {
    /// Creates a Material list with the provided list item children.
    #[must_use]
    pub const fn new(content: Content) -> Self {
        Self { content }
    }
}

impl<Content> View for MaterialList<Content>
where
    Content: ListContent + 'static,
{
    fn body(self, _env: &Environment) -> impl View {
        WaterList::content(self.content).padding_with(EdgeInsets::new(
            LIST_CONTAINER_TOP_SPACE,
            LIST_CONTAINER_BOTTOM_SPACE,
            0.0,
            0.0,
        ))
    }
}

/// A Material Design 3 list item.
#[derive(Clone)]
pub struct MaterialListItem {
    headline: Label,
    accessibility_label: Str,
    supporting_text: Option<Label>,
    trailing_supporting_text: Option<Label>,
    leading: Option<AnyViewBuilder>,
    trailing: Option<AnyViewBuilder>,
    action: Option<SharedAction>,
    selected: Option<Computed<bool>>,
}

impl Debug for MaterialListItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaterialListItem")
            .field("headline", &self.headline)
            .field("supporting_text", &self.supporting_text)
            .field("trailing_supporting_text", &self.trailing_supporting_text)
            .finish_non_exhaustive()
    }
}

impl MaterialListItem {
    /// Creates a one-line Material list item.
    #[must_use]
    pub fn new(headline: impl IntoLabel) -> Self {
        let headline = headline.into_label();
        let accessibility_label = label_plain_text(&headline);
        Self {
            headline,
            accessibility_label,
            supporting_text: None,
            trailing_supporting_text: None,
            leading: None,
            trailing: None,
            action: None,
            selected: None,
        }
    }

    /// Adds supporting text and uses the two-line list item height.
    #[must_use]
    pub fn supporting_text(mut self, text: impl IntoLabel) -> Self {
        self.supporting_text = Some(text.into_label());
        self
    }

    /// Adds trailing supporting text.
    #[must_use]
    pub fn trailing_supporting_text(mut self, text: impl IntoLabel) -> Self {
        self.trailing_supporting_text = Some(text.into_label());
        self
    }

    /// Adds a leading icon or visual slot.
    #[must_use]
    pub fn leading<V>(mut self, leading: V) -> Self
    where
        V: Clone + View + 'static,
    {
        self.leading = Some(AnyViewBuilder::new(move || {
            AnyView::new(
                leading
                    .clone()
                    .foreground(OnSurfaceVariant)
                    .size(LIST_ITEM_LEADING_ICON_SIZE, LIST_ITEM_LEADING_ICON_SIZE),
            )
        }));
        self
    }

    /// Adds a trailing icon or visual slot.
    #[must_use]
    pub fn trailing<V>(mut self, trailing: V) -> Self
    where
        V: Clone + View + 'static,
    {
        self.trailing = Some(AnyViewBuilder::new(move || {
            AnyView::new(
                trailing
                    .clone()
                    .foreground(OnSurfaceVariant)
                    .size(LIST_ITEM_TRAILING_ICON_SIZE, LIST_ITEM_TRAILING_ICON_SIZE),
            )
        }));
        self
    }

    /// Sets the action performed when the list item is tapped.
    #[must_use]
    pub fn action<F, Args>(mut self, action: F) -> Self
    where
        F: Handler<Args, ()> + 'static,
    {
        let mut action = boxed_action(action);
        self.action = Some(SharedAction::new(move |env: Environment| action(&env)));
        self
    }

    /// Marks the list item selected: the container fills
    /// `md.comp.list.list-item.selected.container.color`
    /// (secondary-container) and assistive technology announces it.
    #[must_use]
    pub fn selected(mut self, is_selected: impl IntoComputed<bool>) -> Self {
        self.selected = Some(is_selected.into_computed());
        self
    }

    fn row_view(self) -> AnyView {
        let height = if self.supporting_text.is_some() {
            LIST_ITEM_TWO_LINE_CONTAINER_HEIGHT
        } else {
            LIST_ONE_LINE_ROW_HEIGHT
                .to_f32()
                .expect("list row height must be representable as f32")
        };
        let content_height = LIST_VERTICAL_INSET
            .to_f32()
            .expect("list vertical inset must be representable as f32")
            .mul_add(-2.0, height);
        let headline = self
            .headline
            .font(typography::body_large())
            .foreground(OnSurface);
        let text_content = match self.supporting_text {
            Some(supporting_text) => AnyView::new(
                vstack((
                    headline,
                    supporting_text
                        .font(typography::body_medium())
                        .foreground(OnSurfaceVariant),
                ))
                .alignment(HorizontalAlignment::Leading)
                .spacing(0.0),
            ),
            None => AnyView::new(headline),
        };
        let trailing_content = match (self.trailing_supporting_text, self.trailing) {
            (Some(text), Some(trailing)) => AnyView::new(
                hstack((
                    text.font(typography::label_small())
                        .foreground(OnSurfaceVariant),
                    trailing.build(),
                ))
                .spacing(LIST_ITEM_SLOT_GAP),
            ),
            (Some(text), None) => AnyView::new(
                text.font(typography::label_small())
                    .foreground(OnSurfaceVariant),
            ),
            (None, Some(trailing)) => trailing.build(),
            (None, None) => AnyView::new(()),
        };
        let content = match self.leading {
            Some(leading) => AnyView::new(
                hstack((leading.build(), text_content, spacer(), trailing_content))
                    .spacing(LIST_ITEM_SLOT_GAP),
            ),
            None => AnyView::new(hstack((text_content, spacer(), trailing_content)).spacing(0.0)),
        };
        let item = content
            .height(content_height)
            .a11y_label(self.accessibility_label);

        // md.comp.list.list-item.container.*-state.expressive.shape: the
        // container and its state layer rest at corner-extra-small (8), morph
        // to corner-medium (12) hovered and corner-large (16)
        // pressed/focused/dragged.
        let radii = |r: f64| vello::kurbo::RoundedRectRadii::from(r);
        let state_layer_radii = StateValue::new(radii(8.0))
            .when(InteractionState::DRAGGED, radii(16.0))
            .when(InteractionState::PRESSED, radii(16.0))
            .when(InteractionState::FOCUSED, radii(16.0))
            .when(InteractionState::HOVERED, radii(12.0));

        let state = binding(InteractionState::empty());
        // md.comp.list.list-item.selected.container.color = secondary-container
        // — the report's SELECTED bit drives both the fill and its morphing
        // corners; unselected items stay transparent.
        let container = watch(state.clone(), |state: InteractionState| {
            let fill = if state.contains(InteractionState::SELECTED) {
                Color::from(SecondaryContainer)
            } else {
                Color::transparent()
            };
            FixedRoundedRectangle::new(list_item_corner_radius(state)).fill(fill)
        });
        // md.comp.list.list-item.selected.*.state-layer.color =
        // on-secondary-container while selected, on-surface otherwise.
        let layer_color = crate::semantics::conditional_color(
            state.map(|state| state.contains(InteractionState::SELECTED)),
            OnSecondaryContainer,
            OnSurface,
        );
        let style = interaction_style(layer_color, 8.0).state_layer_radii(state_layer_radii);

        let selected = self.selected.unwrap_or_else(|| Computed::constant(false));
        match self.action {
            Some(action) => AnyView::new(
                item.on_tap(move |env: Environment| action.call(&env))
                    .selected(selected)
                    .interaction_state(&state)
                    .background(container)
                    .install(style),
            ),
            None => AnyView::new(
                item.selected(selected)
                    .interaction_state(&state)
                    .background(container)
                    .install(style),
            ),
        }
    }
}

impl ListContent for MaterialListItem {
    fn collect_items(self, sink: &mut ListItemSink) {
        sink.push(AnyViewBuilder::new(move || {
            WaterListItem::new(self.clone().row_view())
        }));
    }
}

impl View for MaterialListItem {
    fn body(self, _env: &Environment) -> impl View {
        self.row_view()
    }
}

/// `md.comp.list.list-item.container.*-state.expressive.shape` for `state`.
fn list_item_corner_radius(state: InteractionState) -> f32 {
    if state.intersects(
        InteractionState::PRESSED | InteractionState::FOCUSED | InteractionState::DRAGGED,
    ) {
        16.0
    } else if state.contains(InteractionState::HOVERED) {
        12.0
    } else {
        8.0
    }
}

/// Creates a Material Design 3 list.
#[must_use]
pub const fn material_list<Content>(content: Content) -> MaterialList<Content> {
    MaterialList::new(content)
}

/// Creates a Material Design 3 list item.
#[must_use]
pub fn material_list_item(headline: impl IntoLabel) -> MaterialListItem {
    MaterialListItem::new(headline)
}

#[cfg(test)]
mod tests {
    use super::{
        LIST_CONTAINER_BOTTOM_SPACE, LIST_CONTAINER_TOP_SPACE, LIST_ITEM_LEADING_ICON_SIZE,
        LIST_ITEM_TRAILING_ICON_SIZE, LIST_ITEM_TWO_LINE_CONTAINER_HEIGHT,
    };
    use crate::dimensions::{LIST_ONE_LINE_ROW_HEIGHT, LIST_VERTICAL_INSET};

    #[test]
    fn material_list_tokens_match_compose_list_tokens() {
        assert_eq!(LIST_CONTAINER_TOP_SPACE, 8.0);
        assert_eq!(LIST_CONTAINER_BOTTOM_SPACE, 8.0);
        assert_eq!(LIST_ONE_LINE_ROW_HEIGHT, 56.0);
        assert_eq!(LIST_ITEM_TWO_LINE_CONTAINER_HEIGHT, 72.0);
        assert_eq!(LIST_VERTICAL_INSET, 10.0);
        assert_eq!(LIST_ITEM_LEADING_ICON_SIZE, 20.0);
        assert_eq!(LIST_ITEM_TRAILING_ICON_SIZE, 20.0);
    }
}
