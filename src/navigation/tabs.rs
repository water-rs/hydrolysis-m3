use crate::dimensions::{
    MEDIUM_WINDOW_WIDTH, NAV_BAR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
    NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_INDICATOR_RADIUS,
    NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE, TABS_ACTIVE_INDICATOR_HEIGHT,
    TABS_ACTIVE_INDICATOR_RADIUS, TABS_BAR_HEIGHT, TABS_BUTTON_HORIZONTAL_INSET,
    TABS_BUTTON_MIN_WIDTH,
};
use crate::theme::colors::MaterialColorScheme;
use crate::theme::state_layer;
use crate::{TabItemLayout, TabsMetrics, WidgetInteractionState};
use cherenkov::kurbo::RoundedRect;
use cherenkov::kurbo::{Rect, RoundedRectRadii};
use cherenkov::{Draw as _, Recorder};

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
        // md.comp.nav-bar.item.vertical: 3-high label-width strip.
        TabItemLayout::Vertical => TabsMetrics::new(
            TABS_BAR_HEIGHT,
            TABS_BUTTON_MIN_WIDTH,
            TABS_BUTTON_HORIZONTAL_INSET,
            TABS_ACTIVE_INDICATOR_HEIGHT,
            TABS_ACTIVE_INDICATOR_RADIUS,
            NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE,
        ),
        // md.comp.nav-bar.item.horizontal: 40-high full-width pill inside the
        // 64-high nav-bar container.
        TabItemLayout::Horizontal => TabsMetrics::new(
            NAV_BAR_HEIGHT,
            TABS_BUTTON_MIN_WIDTH,
            TABS_BUTTON_HORIZONTAL_INSET,
            NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT,
            NAV_BAR_ITEM_HORIZONTAL_INDICATOR_RADIUS,
            NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
        ),
    }
}

pub fn draw_bar(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect, top_edge: bool) {
    draw.fill(bounds, colors.surface.working());
    let separator = if top_edge {
        Rect::new(bounds.x0, bounds.y1 - 1.0, bounds.x1, bounds.y1)
    } else {
        Rect::new(bounds.x0, bounds.y0, bounds.x1, bounds.y0 + 1.0)
    };
    // md.comp.primary-navigation-tab.divider.color = surface-variant.
    draw.fill(separator, colors.surface_variant.working());
}

pub fn draw_highlight(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: Rect,
    layout: TabItemLayout,
) {
    match layout {
        TabItemLayout::Vertical => draw.fill(
            RoundedRect::from_rect(
                bounds,
                RoundedRectRadii::new(
                    TABS_ACTIVE_INDICATOR_RADIUS,
                    TABS_ACTIVE_INDICATOR_RADIUS,
                    0.0,
                    0.0,
                ),
            ),
            colors.primary.working(),
        ),
        // md.comp.nav-bar.item.active.indicator.color = secondary-container,
        // shape corner-full: a capsule spanning the item.
        TabItemLayout::Horizontal => draw.fill(
            RoundedRect::from_rect(
                bounds,
                RoundedRectRadii::from_single_radius(NAV_BAR_ITEM_HORIZONTAL_INDICATOR_RADIUS),
            ),
            colors.secondary_container.working(),
        ),
    }
}

pub fn draw_button_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: Rect,
    selected: bool,
    state: WidgetInteractionState,
    layout: TabItemLayout,
) {
    // A horizontal item's state layer sits on the item's capsule indicator —
    // md.comp.nav-bar.item.horizontal state layer follows the 40-high
    // corner-full indicator shape — not the whole item bounds.
    let (bounds, radii) = match layout {
        TabItemLayout::Horizontal => {
            let height = NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT.min(bounds.height());
            let y0 = (bounds.height() - height).mul_add(0.5, bounds.y0);
            (
                Rect::new(bounds.x0, y0, bounds.x1, y0 + height),
                RoundedRectRadii::from_single_radius(NAV_BAR_ITEM_HORIZONTAL_INDICATOR_RADIUS),
            )
        }
        TabItemLayout::Vertical => (bounds, 0.0.into()),
    };
    state_layer::draw_bounded(
        draw,
        bounds,
        radii,
        if selected || state.pressed {
            colors.primary.working()
        } else {
            colors.on_surface.working()
        },
        state,
    );
}

#[cfg(test)]
mod tests {
    use super::{item_layout, metrics};
    use crate::dimensions::{
        MEDIUM_WINDOW_WIDTH, NAV_BAR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
        NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_INDICATOR_RADIUS,
        NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE, TABS_ACTIVE_INDICATOR_HEIGHT,
        TABS_ACTIVE_INDICATOR_RADIUS, TABS_BAR_HEIGHT, TABS_BUTTON_HORIZONTAL_INSET,
        TABS_BUTTON_MIN_WIDTH,
    };
    use waterui_backend_core::widget::TabItemLayout;

    #[test]
    fn primary_tab_metrics_match_compose_primary_navigation_tab_tokens() {
        let metrics = metrics(TabItemLayout::Vertical);

        assert_eq!(metrics.bar_height, TABS_BAR_HEIGHT);
        assert_eq!(metrics.button_min_width, TABS_BUTTON_MIN_WIDTH);
        assert_eq!(
            metrics.button_horizontal_inset,
            TABS_BUTTON_HORIZONTAL_INSET
        );
        assert_eq!(
            metrics.active_indicator_height,
            TABS_ACTIVE_INDICATOR_HEIGHT
        );
        assert_eq!(
            metrics.active_indicator_radius,
            TABS_ACTIVE_INDICATOR_RADIUS
        );
        assert_eq!(TABS_BAR_HEIGHT, 48.0);
        assert_eq!(TABS_BUTTON_MIN_WIDTH, 48.0);
        assert_eq!(TABS_BUTTON_HORIZONTAL_INSET, 16.0);
        assert_eq!(TABS_ACTIVE_INDICATOR_HEIGHT, 3.0);
        assert_eq!(TABS_ACTIVE_INDICATOR_RADIUS, 3.0);
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
        assert_eq!(NAV_BAR_ITEM_HORIZONTAL_INDICATOR_RADIUS, 20.0);
        // md.comp.nav-bar.item.horizontal.icon-label-space = 4
        assert_eq!(NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE, 4.0);
        assert_eq!(metrics.icon_label_spacing, 4.0);
        assert_eq!(metrics.bar_height, NAV_BAR_HEIGHT);
        assert_eq!(
            metrics.active_indicator_height,
            NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT
        );
        assert_eq!(
            metrics.active_indicator_radius,
            NAV_BAR_ITEM_HORIZONTAL_INDICATOR_RADIUS
        );
    }
}
