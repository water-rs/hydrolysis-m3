//! Material Design 3 top app bar navigation API.

use crate::color::{Surface, SurfaceContainer};
use crate::semantics::label_plain_text;
use waterui::View;
use waterui::color::Color;
use waterui::component::IntoLabel;
use waterui::layout::Point;
use waterui::navigation::{NavigationTitleDisplayMode, NavigationView};
use waterui::reactive::{Binding, SignalExt as _};
use waterui::{Environment, Str, ViewExt as _};

/// A Material Design 3 navigation view with top app bar chrome.
#[derive(Debug)]
pub struct MaterialNavigationView {
    view: NavigationView,
    accessibility_label: Str,
    scroll_offset: Option<Binding<Point>>,
}

/// Creates a Material Design 3 navigation view.
pub fn material_navigation_view(
    title: impl IntoLabel,
    content: impl View,
) -> MaterialNavigationView {
    let title = title.into_label();
    let accessibility_label = label_plain_text(&title);
    MaterialNavigationView {
        view: NavigationView::new(title.semantic_text().clone(), content),
        accessibility_label,
        scroll_offset: None,
    }
}

/// Binds the scroll-position signal that drives `md.comp.app-bar.scrolled.*`.
///
/// Pass the same [`Binding`] given to a `ScrollView::report_offset` inside
/// the view's content. While the reported `y` offset is greater than zero,
/// the bar renders in the scrolled container color
/// (`md.comp.app-bar.scrolled.container.color`, `surface-container` — the
/// tonal expression of the level-2 scrolled container elevation) and returns
/// to `surface` at rest.
#[must_use]
pub fn material_scrolled_app_bar(
    mut view: MaterialNavigationView,
    offset: &Binding<Point>,
) -> MaterialNavigationView {
    view.scroll_offset = Some(offset.clone());
    view
}

/// Uses the small top app bar presentation.
#[must_use]
pub fn material_small_top_app_bar(mut view: MaterialNavigationView) -> MaterialNavigationView {
    view.view = view
        .view
        .navigation_title_display_mode(NavigationTitleDisplayMode::Inline);
    view
}

/// Uses the large top app bar presentation.
#[must_use]
pub fn material_large_top_app_bar(mut view: MaterialNavigationView) -> MaterialNavigationView {
    view.view = view
        .view
        .navigation_title_display_mode(NavigationTitleDisplayMode::Large);
    view
}

impl View for MaterialNavigationView {
    fn body(self, _env: &Environment) -> impl View {
        let view = match &self.scroll_offset {
            Some(offset) => self.view.navigation_bar_color(
                offset
                    .map(|offset: Point| offset.y > 0.0)
                    .computed()
                    .select(Color::new(SurfaceContainer), Color::new(Surface))
                    .computed(),
            ),
            None => self.view,
        };
        view.a11y_label(self.accessibility_label)
    }
}

#[cfg(test)]
mod tests {
    use crate::navigation_chrome::metrics;

    #[test]
    fn material_top_app_bar_tokens_match_compose_app_bar_tokens() {
        let metrics = metrics();

        assert_eq!(metrics.inline_bar_height, 64.0);
        assert_eq!(metrics.automatic_bar_height, 64.0);
        assert_eq!(metrics.large_bar_height, 152.0);
        assert_eq!(metrics.title_leading_inset, 16.0);
        assert_eq!(metrics.title_trailing_inset, 16.0);
        assert_eq!(metrics.horizontal_inset, 4.0);
    }
}
