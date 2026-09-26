use crate::dimensions::{
    TABS_ACTIVE_INDICATOR_HEIGHT, TABS_ACTIVE_INDICATOR_RADIUS, TABS_BAR_HEIGHT,
    TABS_BUTTON_HORIZONTAL_INSET, TABS_BUTTON_MIN_WIDTH,
};
use crate::theme::colors::MaterialColorScheme;
use crate::theme::state_layer;
use crate::{TabsMetrics, WidgetInteractionState};
use cherenkov::kurbo::RoundedRect;
use cherenkov::kurbo::{Rect, RoundedRectRadii};
use cherenkov::{Draw as _, Recorder};

pub const fn metrics() -> TabsMetrics {
    TabsMetrics::new(
        TABS_BAR_HEIGHT,
        TABS_BUTTON_MIN_WIDTH,
        TABS_BUTTON_HORIZONTAL_INSET,
        TABS_ACTIVE_INDICATOR_HEIGHT,
        TABS_ACTIVE_INDICATOR_RADIUS,
    )
}

pub fn draw_bar(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect, top_edge: bool) {
    draw.fill(bounds, colors.surface.working());
    let separator = if top_edge {
        Rect::new(bounds.x0, bounds.y1 - 1.0, bounds.x1, bounds.y1)
    } else {
        Rect::new(bounds.x0, bounds.y0, bounds.x1, bounds.y0 + 1.0)
    };
    draw.fill(separator, colors.outline_variant.working());
}

pub fn draw_highlight(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    draw.fill(
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
    );
}

pub fn draw_button_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: Rect,
    selected: bool,
    state: WidgetInteractionState,
) {
    state_layer::draw_bounded(
        draw,
        bounds,
        0.0.into(),
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
    use super::metrics;
    use crate::dimensions::{
        TABS_ACTIVE_INDICATOR_HEIGHT, TABS_ACTIVE_INDICATOR_RADIUS, TABS_BAR_HEIGHT,
        TABS_BUTTON_HORIZONTAL_INSET, TABS_BUTTON_MIN_WIDTH,
    };

    #[test]
    fn primary_tab_metrics_match_compose_primary_navigation_tab_tokens() {
        let metrics = metrics();

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
    }
}
