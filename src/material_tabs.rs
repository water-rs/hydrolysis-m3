//! Material Design 3 tabs composed from `WaterUI` navigation tabs.

use core::fmt::{self, Debug};

use waterui::navigation::tab::{TabsLayout, tab_style};
use waterui::reactive::Signal as _;
use waterui::{Binding, Environment, View};
use waterui_controls::label::IntoLabel;
use waterui_core::handler::ViewBuilder;
use waterui_core::id::Mapping;
use waterui_core::views::{AnyViews, SharedAnyViews, Views};

/// A Material Design 3 tab item — the shared [`Tab`] shape under the
/// Material name: a tab is label + content on every platform.
pub use waterui::navigation::tab::Tab as MaterialTab;

/// A Material Design 3 primary tabs container.
///
/// `tabs` is any [`Views`] collection whose view is [`MaterialTab`] and
/// whose id is the selection's type — a fixed `Vec<MaterialTab>` selects by
/// `usize` index, a `ForEach` over a `List` of `SelfId`s keeps the
/// application's own tab type. Membership changes keep a surviving tab's
/// retained state — navigation stack, focus, in-flight gestures.
pub struct MaterialTabs<T: 'static> {
    selection: Binding<T>,
    items: SharedAnyViews<MaterialTab>,
    ids: Mapping<T>,
}

impl<T: Debug + 'static> Debug for MaterialTabs<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MaterialTabs")
            .field("tabs", &self.items.len().snapshot())
            .finish_non_exhaustive()
    }
}

impl<T: Ord + Clone + 'static> MaterialTabs<T> {
    /// Creates an adaptive Material tabs container.
    #[must_use]
    pub fn new(
        selection: &Binding<T>,
        tabs: impl Views<View = MaterialTab, Id = T> + 'static,
    ) -> Self {
        let (items, ids) = AnyViews::new_with_ids(tabs);
        let items = SharedAnyViews::from(items);
        Self {
            selection: selection.clone(),
            items,
            ids,
        }
    }
}

impl<T: Ord + Clone + 'static> View for MaterialTabs<T> {
    fn body(self, _env: &Environment) -> impl View {
        TabsLayout::new(self.ids.binding(&self.selection), self.items).style(tab_style::tab_bar())
    }
}

/// Creates a Material Design 3 primary tabs container.
#[must_use]
pub fn material_tabs<T: Ord + Clone + 'static>(
    selection: &Binding<T>,
    tabs: impl Views<View = MaterialTab, Id = T> + 'static,
) -> MaterialTabs<T> {
    MaterialTabs::new(selection, tabs)
}

/// Creates a Material Design 3 tab.
#[must_use]
pub fn material_tab(
    label: impl IntoLabel,
    content: impl ViewBuilder<Output = waterui::navigation::NavigationView>,
) -> MaterialTab {
    MaterialTab::new(label, content)
}

#[cfg(test)]
mod tests {
    use crate::dimensions::{TABS_BAR_HEIGHT, TABS_BUTTON_HORIZONTAL_INSET, TABS_BUTTON_MIN_WIDTH};

    #[test]
    fn material_tabs_tokens_match_compose_primary_navigation_tab_tokens() {
        assert_eq!(TABS_BAR_HEIGHT, 48.0);
        assert_eq!(TABS_BUTTON_MIN_WIDTH, 48.0);
        assert_eq!(TABS_BUTTON_HORIZONTAL_INSET, 16.0);
    }
}
