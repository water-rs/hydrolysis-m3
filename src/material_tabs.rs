//! Material Design 3 tabs composed from `WaterUI` navigation tabs.

use core::fmt::{self, Debug};

use waterui::navigation::tab::{TabSet, TabStyle, Tabs};
use waterui::navigation::{NavigationToolbarItems, NavigationView};
use waterui::{Binding, Environment, View};
use waterui_controls::label::IntoLabel;
use waterui_core::handler::ViewBuilder;

/// A Material Design 3 tab item — the shared [`Tab`] shape under the
/// Material name: a tab is label + content on every platform.
///
/// [`Tab`]: waterui::navigation::tab::Tab
pub use waterui::navigation::tab::Tab as MaterialTab;

/// A Material Design 3 primary tabs container.
///
/// `tabs` is a [`TabSet`] whose id is the selection's type — a tuple or a
/// `Vec` of [`MaterialTab`]s selects by `usize` index, a `ForEach` over a
/// `List` of `SelfId`s keeps the application's own tab type. Membership
/// changes keep a surviving tab's retained state — navigation stack, focus,
/// in-flight gestures.
pub struct MaterialTabs<T: TabSet>(Tabs<T, ()>);

impl<T: TabSet> Debug for MaterialTabs<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MaterialTabs").field(&self.0).finish()
    }
}

impl<T: TabSet> MaterialTabs<T> {
    /// Creates an adaptive Material tabs container.
    #[must_use]
    pub fn new(selection: &Binding<T::Id>, tabs: T) -> Self {
        Self(Tabs::new(selection, tabs).style(TabStyle::TabBar))
    }
}

impl<T: TabSet> View for MaterialTabs<T> {
    fn body(self, _env: &Environment) -> impl View {
        self.0
    }
}

/// Creates a Material Design 3 primary tabs container.
#[must_use]
pub fn material_tabs<T: TabSet>(selection: &Binding<T::Id>, tabs: T) -> MaterialTabs<T> {
    MaterialTabs::new(selection, tabs)
}

/// Creates a Material Design 3 tab.
#[must_use]
pub fn material_tab<L, C, V, Tb>(label: L, content: C) -> MaterialTab<C, L::Kind>
where
    L: IntoLabel,
    C: ViewBuilder<Output = NavigationView<V, Tb>>,
    V: View,
    Tb: NavigationToolbarItems,
{
    MaterialTab::new(label, content)
}

#[cfg(test)]
mod tests {
    use crate::dimensions::TABS_BUTTON_HORIZONTAL_INSET;

    #[test]
    fn material_tabs_tokens_match_compose_primary_navigation_tab_tokens() {
        assert_eq!(TABS_BUTTON_HORIZONTAL_INSET, 16.0);
    }
}
