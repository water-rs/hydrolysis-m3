//! The Material 3 tabs look: the primary navigation tab bar from the
//! `md.comp.primary-navigation-tab.*` tokens, and the `md.comp.nav-bar.*`
//! navigation bar its items take their active indicator from.
//!
//! A bar is its container and its divider; each item is a chrome of its own
//! in the shape of its active-indicator pill — the state layer, one press
//! ripple wave per press and the focus indicator — and a highlight that is
//! the pill itself. The highlight is a part driven by the item's selection:
//! a selection change grows the pill in from the item's centre while its
//! colour fades in, and shrinks and fades the old one out, under the
//! look's motion, so a selection change, a hover or a press records
//! nothing. Items stand vertical in a compact bar and horizontal from the
//! 600dp medium window width.

use crate::color::{
    OnSurface, Primary, SecondaryContainer, Surface as SurfaceColor, SurfaceVariant,
};
use crate::controls::button::{interactive, solid_paint};
use crate::dimensions::{
    MEDIUM_WINDOW_WIDTH, NAV_BAR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
    NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE,
    TABS_BAR_HEIGHT, TABS_BUTTON_HORIZONTAL_INSET, TABS_BUTTON_MIN_WIDTH,
};
use crate::navigation_bar::{
    NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT, NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH,
};
use crate::navigation_rail::{COLLAPSED_ACTIVE_INDICATOR_HEIGHT, COLLAPSED_ACTIVE_INDICATOR_WIDTH};
use crate::theme::motion;
use hydrolysis::draw::kurbo::{Rect, RoundedRect, RoundedRectRadii, Vec2};
use hydrolysis::draw::{Draw as _, Paint, WorkingColor};
use hydrolysis::theme::{
    Chrome, CornerShape, Custom, Driven, Inputs, Surface, TabItem, TabItemGeometry, TabItemLayout,
    TabsLook, TabsMetrics,
};
use waterui::navigation::tab::NativeTabStyle;
use waterui_core::{Computed, Environment, SignalExt as _};
use waterui_graphics::color::{Color, signal_color};

/// The measures of the compact bar's vertical items
/// (`md.comp.primary-navigation-tab.container.height` and
/// `md.comp.nav-bar.item.vertical.icon-label-space`).
const VERTICAL: TabsMetrics = TabsMetrics::new(
    TABS_BAR_HEIGHT,
    TABS_BUTTON_MIN_WIDTH,
    TABS_BUTTON_HORIZONTAL_INSET,
    NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE,
);

/// The measures of the medium-width bar's horizontal items: the
/// `md.comp.nav-bar` container and
/// `md.comp.nav-bar.item.horizontal.icon-label-space`.
const HORIZONTAL: TabsMetrics = TabsMetrics::new(
    NAV_BAR_HEIGHT,
    TABS_BUTTON_MIN_WIDTH,
    TABS_BUTTON_HORIZONTAL_INSET,
    NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
);

/// What a tab item's highlight draws: the pill as a shape, and its paint.
type Highlight = Custom<TabItem, (Driven<RoundedRect, TabItem>, Driven<Paint, TabItem>)>;

/// The look of a tab container.
///
/// The bar is `surface` over a `surface-variant` divider; the bar switches
/// to horizontal items at the medium window width class, whose boundary is
/// the bar's own width, which tracks the window width for a bottom bar.
pub fn look(env: &Environment) -> TabsLook {
    let env = env.clone();
    TabsLook::new(
        VERTICAL,
        fill(SurfaceColor, &env),
        // md.comp.primary-navigation-tab.divider.color = surface-variant.
        fill(SurfaceVariant, &env),
        motion::chrome(),
        item_chrome,
    )
    .horizontal(MEDIUM_WINDOW_WIDTH, HORIZONTAL)
    .highlight(move |item| highlight(item, &env))
}

/// A part filling its frame with `color`.
fn fill(color: impl Into<Color>, env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new((solid_paint(color, env),), |recorder, frame, (paint,)| {
        recorder.fill(frame, paint);
    })
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
fn indicator_rect(item: &TabItemGeometry) -> Rect {
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

/// Where a chrome of `pill`'s size sits in `bounds`, as the fraction of the
/// room left on each axis that lies before it.
fn alignment(pill: Rect, bounds: Rect) -> Vec2 {
    let fraction = |room: f64, before: f64| if room > 0.0 { before / room } else { 0.5 };
    Vec2::new(
        fraction(bounds.width() - pill.width(), pill.x0 - bounds.x0),
        fraction(bounds.height() - pill.height(), pill.y0 - bounds.y0),
    )
}

/// An item's chrome: no container of its own — the highlight is the
/// container — and the state layer in `primary` while the item is selected
/// and `on-surface` otherwise, following the item's selection in place, the
/// press ripple growing from the press point and the focus indicator, all in
/// the active-indicator pill.
///
/// The state layer colour is a token of the primary navigation tab: active
/// hover, focus and pressed are `primary`, inactive hover and focus are
/// `on-surface`. An inactive item's pressed layer is `primary` in the
/// token set, which the layer's single colour cannot follow.
#[expect(
    clippy::needless_pass_by_value,
    reason = "`TabsLook::new` fixes the callback signature, which hands the flag over by value"
)]
fn item_chrome(item: &TabItem, selected: Computed<bool>) -> Chrome {
    let pill = indicator_rect(&item.geometry);
    let tint = signal_color(selected.map(|selected| {
        if selected {
            Color::from(Primary)
        } else {
            Color::from(OnSurface)
        }
    }));
    interactive(Surface::clear(CornerShape::full()), tint)
        .sized(pill.size(), alignment(pill, item.geometry.bounds))
}

/// The pill of `item` as it is drawn: the whole pill while the item is
/// selected, collapsed to a line at its centre otherwise, so a selection
/// grows it out of the centre as Compose's `NavigationBarItem` indicator
/// does.
fn pill_shape(item: TabItemGeometry) -> Driven<RoundedRect, TabItem> {
    Driven::new(move |inputs: &Inputs<'_, TabItem>| {
        let pill = indicator_rect(&item);
        let shown = if inputs.value.selected {
            pill
        } else {
            Rect::from_center_size(pill.center(), (0.0, pill.height()))
        };
        RoundedRect::from_rect(shown, indicator_radii(pill))
    })
}

/// The pill's paint: `secondary-container` while selected, the same colour
/// faded out of sight otherwise, so the fade runs on the alpha alone and
/// never passes through a darker blend with transparent black.
fn pill_paint(env: &Environment) -> Driven<Paint, TabItem> {
    Driven::follow(
        Color::from(SecondaryContainer).resolve(env),
        |color: &WorkingColor, inputs: &Inputs<'_, TabItem>| {
            let [red, green, blue, alpha] = color.components;
            Paint::from(WorkingColor::new([
                red,
                green,
                blue,
                if inputs.value.selected { alpha } else { 0.0 },
            ]))
        },
    )
}

/// The highlight of `item`: the active-indicator pill,
/// `md.comp.nav-bar.item.active-indicator.color` = secondary-container.
fn highlight(item: &TabItem, env: &Environment) -> Highlight {
    Custom::new(
        (pill_shape(item.geometry), pill_paint(env)),
        |recorder, _frame, (shape, paint)| recorder.fill(shape, paint),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        HORIZONTAL, VERTICAL, alignment, indicator_rect, item_chrome, look, pill_paint, pill_shape,
    };
    use crate::MaterialColorScheme;
    use crate::dimensions::{
        MEDIUM_WINDOW_WIDTH, NAV_BAR_HEIGHT, NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE,
        NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, NAV_BAR_ITEM_VERTICAL_ICON_LABEL_SPACE,
        TABS_BAR_HEIGHT, TABS_BUTTON_HORIZONTAL_INSET, TABS_BUTTON_MIN_WIDTH,
    };
    use hydrolysis::Style as _;
    use hydrolysis::draw::kurbo::Rect;
    use hydrolysis::draw::{Paint, WorkingColor};
    use hydrolysis::theme::{Inputs, TabItem, TabItemGeometry, TabItemLayout};
    use waterui::interaction::InteractionState;
    use waterui::navigation::tab::NativeTabStyle;
    use waterui_core::{Binding, Environment, Signal as _, SignalExt as _};

    fn environment() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    #[test]
    fn primary_tab_metrics_match_compose_primary_navigation_tab_tokens() {
        let look = look(&environment());
        let metrics = look.metrics(TabItemLayout::Vertical);

        assert_eq!(metrics, VERTICAL);
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
        let look = look(&environment());

        assert_eq!(
            look.item_layout(MEDIUM_WINDOW_WIDTH - 1.0),
            TabItemLayout::Vertical
        );
        assert_eq!(
            look.item_layout(MEDIUM_WINDOW_WIDTH),
            TabItemLayout::Horizontal
        );
    }

    #[test]
    fn horizontal_item_metrics_match_compose_nav_bar_tokens() {
        let look = look(&environment());
        let metrics = look.metrics(TabItemLayout::Horizontal);

        // md.comp.nav-bar.container.height = 64;
        // md.comp.nav-bar.item.horizontal.active-indicator.height = 40;
        // md.comp.nav-bar.item.active-indicator.shape = corner-full.
        assert_eq!(NAV_BAR_HEIGHT, 64.0);
        assert_eq!(NAV_BAR_ITEM_HORIZONTAL_INDICATOR_HEIGHT, 40.0);
        // md.comp.nav-bar.item.horizontal.icon-label-space = 4
        assert_eq!(NAV_BAR_ITEM_HORIZONTAL_ICON_LABEL_SPACE, 4.0);
        assert_eq!(metrics, HORIZONTAL);
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

    fn horizontal_item() -> TabItemGeometry {
        TabItemGeometry {
            bounds: Rect::new(10.0, 0.0, 210.0, 64.0),
            icon: Some(Rect::new(40.0, 20.0, 64.0, 44.0)),
            title: Rect::new(68.0, 24.0, 180.0, 40.0),
            layout: TabItemLayout::Horizontal,
            placement: NativeTabStyle::TabBar,
        }
    }

    #[test]
    fn a_horizontal_item_indicator_spans_the_item_at_the_horizontal_height() {
        let pill = indicator_rect(&horizontal_item());

        assert_eq!(pill, Rect::new(10.0, 12.0, 210.0, 52.0));
    }

    fn item(selected: bool, geometry: TabItemGeometry) -> TabItem {
        TabItem {
            index: 0,
            count: 3,
            selected,
            geometry,
        }
    }

    /// The chrome's frame in an item's bounds is the pill, wherever the
    /// pill lies in them.
    #[test]
    fn the_chrome_alignment_places_the_chrome_on_the_pill() {
        let geometry = vertical_item(
            NativeTabStyle::TabBar,
            Some(Rect::new(138.0, 16.0, 162.0, 40.0)),
        );
        let pill = indicator_rect(&geometry);
        let align = alignment(pill, geometry.bounds);
        let x0 = (geometry.bounds.width() - pill.width()).mul_add(align.x, geometry.bounds.x0);
        let y0 = (geometry.bounds.height() - pill.height()).mul_add(align.y, geometry.bounds.y0);

        assert_eq!((x0, y0), (pill.x0, pill.y0));
        // A pill as wide as the item has no room to place: it centres.
        assert_eq!(
            alignment(horizontal_item().bounds, horizontal_item().bounds).x,
            0.5
        );
    }

    /// The item's layer follows the item's selection in place: `primary`
    /// while selected, `on-surface` otherwise.
    #[test]
    fn an_items_state_layer_follows_its_selection() {
        let colors = MaterialColorScheme::baseline_light();
        let env = environment();
        let selected = Binding::container(false);
        let chrome = item_chrome(&item(false, horizontal_item()), selected.clone().computed());
        let color = || chrome.state_layer().color.resolve(&env).snapshot();

        assert_eq!(color(), colors.on_surface.working());
        selected.set(true);
        assert_eq!(color(), colors.primary.working());
        assert_eq!(
            chrome.state_layer().pressed,
            crate::theme::state_layer::PRESSED_STATE_LAYER_OPACITY
        );
        assert!(chrome.press().get_ripple().is_some());
    }

    /// The pill's colour is `secondary-container` while selected and the
    /// same colour with no alpha otherwise.
    #[test]
    fn the_pill_fades_in_secondary_container() {
        let colors = MaterialColorScheme::baseline_light();
        let env = environment();
        let geometry = horizontal_item();
        let paint = pill_paint(&env);
        let at = |selected| {
            let value = item(selected, geometry);
            paint.target(&Inputs::at_rest(
                geometry.bounds,
                InteractionState::empty(),
                &value,
            ))
        };
        let [red, green, blue, alpha] = colors.secondary_container.working().components;

        assert_eq!(
            at(true),
            Paint::from(WorkingColor::new([red, green, blue, alpha]))
        );
        assert_eq!(
            at(false),
            Paint::from(WorkingColor::new([red, green, blue, 0.0]))
        );
    }

    /// The highlight is the pill while selected and a line at the pill's
    /// centre otherwise, the retarget the engine animates.
    #[test]
    fn the_highlight_grows_the_pill_out_of_its_centre() {
        let geometry = horizontal_item();
        let pill = indicator_rect(&geometry);
        let shape = pill_shape(geometry);
        let at = |selected| {
            let value = item(selected, geometry);
            shape
                .target(&Inputs::at_rest(
                    geometry.bounds,
                    InteractionState::empty(),
                    &value,
                ))
                .rect()
        };

        assert_eq!(at(true), pill);
        assert_eq!(at(false).width(), 0.0);
        assert_eq!(at(false).center(), pill.center());
        assert_eq!(at(false).height(), pill.height());
    }
}
