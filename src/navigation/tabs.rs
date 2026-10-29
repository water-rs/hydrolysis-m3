use crate::dimensions::{
    MEDIUM_WINDOW_WIDTH, NAV_BAR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
    NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE,
    TABS_BAR_HEIGHT, TABS_BUTTON_HORIZONTAL_INSET, TABS_BUTTON_MIN_WIDTH,
};
use crate::navigation_bar::{
    NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT, NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH,
};
use crate::navigation_rail::{COLLAPSED_ACTIVE_INDICATOR_HEIGHT, COLLAPSED_ACTIVE_INDICATOR_WIDTH};
use crate::theme::colors::MaterialColorScheme;
use crate::theme::state_layer;
use crate::{TabItemGeometry, TabItemLayout, TabsMetrics, WidgetInteractionState};
use waterui::interaction::InteractionState;
use waterui::navigation::tab::NativeTabStyle;
use waterui_graphics::draw::kurbo::RoundedRect;
use waterui_graphics::draw::kurbo::{Rect, RoundedRectRadii};
use waterui_graphics::draw::{Draw as _, Recorder};

/// Whether the bar shows vertical or horizontal items. The M3 navigation bar
/// switches to horizontal items at the medium window width class boundary —
/// `bar_width` is the bar's own width, which tracks the window width for a
/// bottom bar.
pub const fn item_layout(bar_width: f64) -> TabItemLayout {
    if bar_width >= MEDIUM_WINDOW_WIDTH {
        TabItemLayout::Horizontal
    } else {
        TabItemLayout::Vertical
    }
}

pub const fn metrics(layout: TabItemLayout) -> TabsMetrics {
    match layout {
        TabItemLayout::Vertical => TabsMetrics::new(
            TABS_BAR_HEIGHT,
            TABS_BUTTON_MIN_WIDTH,
            TABS_BUTTON_HORIZONTAL_INSET,
            NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE,
        ),
        TabItemLayout::Horizontal => TabsMetrics::new(
            NAV_BAR_HEIGHT,
            TABS_BUTTON_MIN_WIDTH,
            TABS_BUTTON_HORIZONTAL_INSET,
            NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
        ),
    }
}

pub fn draw_bar(colors: &MaterialColorScheme, draw: &mut Recorder, surface: Rect) {
    draw.fill(surface, colors.surface.working());
}

pub fn draw_bar_divider(colors: &MaterialColorScheme, draw: &mut Recorder, divider: Rect) {
    // md.comp.primary-navigation-tab.divider.color = surface-variant.
    draw.fill(divider, colors.surface_variant.working());
}

/// The active-indicator pill of `item`: the shape the selection highlight and
/// the state layer both follow.
///
/// A vertical item's pill is centred on its icon, 56x32dp
/// (`md.comp.nav-bar.item.vertical.active-indicator` in a bar,
/// `NavigationRailVerticalItemTokens.ActiveIndicator` in a sidebar strip).
/// A horizontal item's pill spans the item at the 40dp horizontal indicator
/// height. A vertical item without an icon has nothing to centre on, so its
/// pill wraps the title at the horizontal indicator height instead.
pub fn indicator_rect(item: &TabItemGeometry) -> Rect {
    match (item.layout, item.icon) {
        (TabItemLayout::Horizontal, _) => {
            let height = NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT.min(item.bounds.height());
            let y0 = (item.bounds.height() - height).mul_add(0.5, item.bounds.y0);
            Rect::new(item.bounds.x0, y0, item.bounds.x1, y0 + height)
        }
        (TabItemLayout::Vertical, Some(icon)) => {
            let (width, height) = match item.placement {
                NativeTabStyle::Automatic | NativeTabStyle::TabBar => (
                    f64::from(NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH),
                    f64::from(NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT),
                ),
                NativeTabStyle::Sidebar => (
                    f64::from(COLLAPSED_ACTIVE_INDICATOR_WIDTH),
                    f64::from(COLLAPSED_ACTIVE_INDICATOR_HEIGHT),
                ),
            };
            Rect::from_center_size(icon.center(), (width, height))
        }
        (TabItemLayout::Vertical, None) => Rect::from_center_size(
            item.title.center(),
            (
                2.0f64.mul_add(TABS_BUTTON_HORIZONTAL_INSET, item.title.width()),
                NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT,
            ),
        ),
    }
}

/// `md.comp.nav-bar.item.active-indicator.shape` = corner-full.
fn indicator_radii(indicator: Rect) -> RoundedRectRadii {
    RoundedRectRadii::from_single_radius(indicator.height() * 0.5)
}

pub fn draw_highlight(colors: &MaterialColorScheme, draw: &mut Recorder, item: TabItemGeometry) {
    let indicator = indicator_rect(&item);
    // md.comp.nav-bar.item.active.indicator.color = secondary-container.
    draw.fill(
        RoundedRect::from_rect(indicator, indicator_radii(indicator)),
        colors.secondary_container.working(),
    );
}

pub fn draw_button_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    item: TabItemGeometry,
    selected: bool,
    state: WidgetInteractionState,
) {
    // The state layer follows the active-indicator pill, not the whole item.
    let indicator = indicator_rect(&item);
    state_layer::draw_bounded(
        draw,
        indicator,
        indicator_radii(indicator),
        if selected || state.state.contains(InteractionState::PRESSED) {
            colors.primary.working()
        } else {
            colors.on_surface.working()
        },
        state,
    );
}

#[cfg(test)]
mod tests {
    use super::{indicator_rect, item_layout, metrics};
    use crate::dimensions::{
        MEDIUM_WINDOW_WIDTH, NAV_BAR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
        NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE,
        TABS_BAR_HEIGHT, TABS_BUTTON_HORIZONTAL_INSET, TABS_BUTTON_MIN_WIDTH,
    };
    use hydrolysis::theme::{TabItemGeometry, TabItemLayout};
    use waterui::navigation::tab::NativeTabStyle;
    use waterui_graphics::draw::kurbo::Rect;

    #[test]
    fn primary_tab_metrics_match_compose_primary_navigation_tab_tokens() {
        let metrics = metrics(TabItemLayout::Vertical);

        assert_eq!(metrics.bar_height, TABS_BAR_HEIGHT);
        assert_eq!(metrics.button_min_width, TABS_BUTTON_MIN_WIDTH);
        assert_eq!(
            metrics.button_horizontal_inset,
            TABS_BUTTON_HORIZONTAL_INSET
        );
        assert_eq!(TABS_BAR_HEIGHT, 48.0);
        assert_eq!(TABS_BUTTON_MIN_WIDTH, 48.0);
        assert_eq!(TABS_BUTTON_HORIZONTAL_INSET, 16.0);
        // md.comp.nav-bar.item.vertical.icon-label-space = 4
        assert_eq!(NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE, 4.0);
        assert_eq!(metrics.icon_label_spacing, 4.0);
    }

    #[test]
    fn the_bar_uses_horizontal_items_at_the_medium_width_class() {
        assert_eq!(
            item_layout(MEDIUM_WINDOW_WIDTH - 1.0),
            TabItemLayout::Vertical
        );
        assert_eq!(item_layout(MEDIUM_WINDOW_WIDTH), TabItemLayout::Horizontal);
    }

    #[test]
    fn horizontal_item_metrics_match_compose_nav_bar_tokens() {
        let metrics = metrics(TabItemLayout::Horizontal);

        // md.comp.nav-bar.container.height = 64;
        // md.comp.nav-bar.item.horizontal.active-indicator.height = 40;
        // md.comp.nav-bar.item.active-indicator.shape = corner-full.
        assert_eq!(NAV_BAR_HEIGHT, 64.0);
        assert_eq!(NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, 40.0);
        // md.comp.nav-bar.item.horizontal.icon-label-space = 4
        assert_eq!(NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE, 4.0);
        assert_eq!(metrics.icon_label_spacing, 4.0);
        assert_eq!(metrics.bar_height, NAV_BAR_HEIGHT);
    }

    fn vertical_item(placement: NativeTabStyle, icon: Option<Rect>) -> TabItemGeometry {
        TabItemGeometry {
            bounds: Rect::new(100.0, 0.0, 200.0, 80.0),
            icon,
            title: Rect::new(130.0, 52.0, 170.0, 68.0),
            layout: TabItemLayout::Vertical,
            placement,
        }
    }

    #[test]
    fn a_bar_item_indicator_is_a_pill_centred_on_the_icon() {
        let icon = Rect::new(138.0, 16.0, 162.0, 40.0);
        let pill = indicator_rect(&vertical_item(NativeTabStyle::TabBar, Some(icon)));

        // The nav-bar active indicator is 56x32 dp, centred on the icon.
        assert_eq!(pill.size().width, 56.0);
        assert_eq!(pill.size().height, 32.0);
        assert_eq!(pill.center(), icon.center());
    }

    #[test]
    fn a_sidebar_item_indicator_is_the_rail_pill_centred_on_the_icon() {
        let icon = Rect::new(138.0, 16.0, 162.0, 40.0);
        let pill = indicator_rect(&vertical_item(NativeTabStyle::Sidebar, Some(icon)));

        assert_eq!(pill.size().width, 56.0);
        assert_eq!(pill.size().height, 32.0);
        assert_eq!(pill.center(), icon.center());
    }

    #[test]
    fn an_iconless_vertical_item_wraps_its_title() {
        let item = vertical_item(NativeTabStyle::TabBar, None);
        let pill = indicator_rect(&item);

        assert_eq!(pill.center(), item.title.center());
        assert_eq!(
            pill.size().width,
            2.0f64.mul_add(TABS_BUTTON_HORIZONTAL_INSET, item.title.width())
        );
        assert_eq!(pill.size().height, NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT);
    }

    #[test]
    fn a_horizontal_item_indicator_spans_the_item_at_the_horizontal_height() {
        let item = TabItemGeometry {
            bounds: Rect::new(10.0, 0.0, 210.0, 64.0),
            icon: Some(Rect::new(40.0, 20.0, 64.0, 44.0)),
            title: Rect::new(68.0, 24.0, 180.0, 40.0),
            layout: TabItemLayout::Horizontal,
            placement: NativeTabStyle::TabBar,
        };
        let pill = indicator_rect(&item);

        assert_eq!(pill, Rect::new(10.0, 12.0, 210.0, 52.0));
    }
}
