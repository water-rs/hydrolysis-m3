//! Material Design 3 navigation bar composed from `WaterUI` primitives.

use core::fmt::{self, Debug};

use crate::view_ext::ViewExt as _;
use nami::Binding;
use nami::SignalExt as _;
use waterui_accessibility::{AccessibilityChildren, AccessibilityRole, AccessibilityState};
use waterui_controls::label::{IntoLabel, Label, LabelKind};
use waterui_core::handler::{ExtractionError, Handler, ReportExtraction as _, boxed_action};
use waterui_core::{Environment, Str, View};
use waterui_graphics::color::Color;
use waterui_layout::padding::EdgeInsets;
use waterui_shape::{Capsule, ShapeExt as _};

use crate::action::{NoAction, noop};
use waterui_core::view::ViewSequence;

use crate::color::{
    OnSecondaryContainer, OnSurfaceVariant, Secondary, SecondaryContainer, SurfaceContainer,
};
use crate::elevation::{MaterialElevationLevel, material_elevation};
use crate::semantics::{
    conditional_color, conditional_font, interaction_chrome_radius, label_plain_text,
};
use crate::theme::typography;

/// `NavigationBarTokens.ContainerHeight`. Compose's 80dp value is
/// `TallContainerHeight`, the opt-in variant, not the default bar.
const NAVIGATION_BAR_CONTAINER_HEIGHT: f32 = 64.0;
const NAVIGATION_BAR_ITEM_MIN_WIDTH: f32 = 48.0;
const NAVIGATION_BAR_ITEM_TOP_PADDING: f32 = 4.0;
const NAVIGATION_BAR_ITEM_BOTTOM_PADDING: f32 = 8.0;
const NAVIGATION_BAR_ICON_SIZE: f32 = 24.0;
const NAVIGATION_BAR_ICON_SLOT_HEIGHT: f32 = 32.0;
/// `m3_comp_nav_bar_item_vertical_active_indicator_width` — the Expressive
/// indicator is 56dp wide, not the 64dp of the classic spec.
pub(crate) const NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH: f32 = 56.0;
/// `m3_comp_nav_bar_item_vertical_active_indicator_height`.
pub(crate) const NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT: f32 = 32.0;
const NAVIGATION_BAR_LABEL_TOP_SPACE: f32 = 4.0;

/// A Material Design 3 navigation bar.
pub struct NavigationBar<Tabs> {
    tabs: Tabs,
}

impl<Tabs> Debug for NavigationBar<Tabs> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NavigationBar").finish_non_exhaustive()
    }
}

impl<Tabs> NavigationBar<Tabs> {
    /// Creates a navigation bar containing Material navigation tabs.
    #[must_use]
    pub const fn new(tabs: Tabs) -> Self {
        Self { tabs }
    }
}

impl<Tabs> View for NavigationBar<Tabs>
where
    Tabs: ViewSequence,
{
    fn body(self, _env: &Environment) -> impl View {
        material_elevation(
            MaterialElevationLevel::LEVEL2,
            // The bar is a full-width rectangle: no corner rounding.
            0.0,
            waterui_layout::stack::hstack(self.tabs)
                .height(NAVIGATION_BAR_CONTAINER_HEIGHT)
                .max_width(f32::INFINITY)
                .background(SurfaceContainer),
        )
        .a11y_label("Navigation")
        .a11y_role(AccessibilityRole::TabList)
    }
}

/// A Material Design 3 navigation tab.
pub struct NavigationTab<L, Icon, Action = NoAction> {
    label: Label<L>,
    accessibility_label: Str,
    icon: Icon,
    selected: Binding<bool>,
    action: Action,
}

impl<L: Debug, Icon, Action> Debug for NavigationTab<L, Icon, Action> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NavigationTab")
            .field("label", &self.label)
            .finish_non_exhaustive()
    }
}

impl<L: LabelKind, Icon> NavigationTab<L, Icon, NoAction> {
    /// Creates a navigation tab with a semantic label, icon, and selected state.
    #[must_use]
    pub fn new(label: impl IntoLabel<Kind = L>, icon: Icon, selected: &Binding<bool>) -> Self {
        let label = label.into_label();
        let accessibility_label = label_plain_text(&label);
        Self {
            label,
            accessibility_label,
            icon,
            selected: selected.clone(),
            action: noop,
        }
    }
}

impl<L, Icon, Action> NavigationTab<L, Icon, Action> {
    /// Sets the action performed when the navigation tab is tapped.
    #[must_use]
    pub fn action<F, Args>(
        self,
        action: F,
    ) -> NavigationTab<L, Icon, impl FnMut(&Environment) -> Result<(), ExtractionError>>
    where
        F: Handler<Args, ()> + 'static,
    {
        NavigationTab {
            label: self.label,
            accessibility_label: self.accessibility_label,
            icon: self.icon,
            selected: self.selected,
            action: boxed_action(action),
        }
    }
}

impl<L, Icon, Action> View for NavigationTab<L, Icon, Action>
where
    L: LabelKind,
    Icon: Clone + View + 'static,
    Action: FnMut(&Environment) -> Result<(), ExtractionError> + 'static,
{
    fn body(self, _env: &Environment) -> impl View {
        let mut action = self.action;
        let accessibility_label = self.accessibility_label.clone();
        let accessibility_state = self
            .selected
            .map(|selected| AccessibilityState::new().selected(selected));
        let indicator_color = conditional_color(
            self.selected.clone(),
            SecondaryContainer,
            Color::transparent(),
        );
        let icon_color = conditional_color(
            self.selected.clone(),
            OnSecondaryContainer,
            OnSurfaceVariant,
        );
        let label_color = conditional_color(self.selected.clone(), Secondary, OnSurfaceVariant);
        // md.comp.nav-bar.item.*.{hovered,focused,pressed}.state-layer.color:
        // on-secondary-container for the active and inactive items alike.
        let state_layer_color = OnSecondaryContainer;
        // The active label is label-medium at the prominent (bold) weight.
        let label_font = conditional_font(
            self.selected,
            typography::label_medium_prominent(),
            typography::label_medium(),
        );

        navigation_tab_content(
            self.label,
            self.icon,
            indicator_color.into(),
            icon_color.into(),
            label_color.into(),
            label_font,
        )
        // Padding applies inside the fixed item height: the 32dp indicator
        // slot starts `TOP_PADDING` below the item top and the label sits
        // `BOTTOM_PADDING` above the bottom — matching Compose, where the
        // 48dp content block centers in the item and the indicator pill
        // extends `IndicatorVerticalPadding` above the icon.
        .padding_with(EdgeInsets::new(
            NAVIGATION_BAR_ITEM_TOP_PADDING,
            0.0,
            NAVIGATION_BAR_ITEM_BOTTOM_PADDING,
            0.0,
        ))
        .min_width(NAVIGATION_BAR_ITEM_MIN_WIDTH)
        .max_width(f32::INFINITY)
        .height(NAVIGATION_BAR_CONTAINER_HEIGHT)
        .on_tap(move |env: Environment| {
            action(&env).report_extraction();
        })
        .a11y_label(accessibility_label)
        .a11y_role(AccessibilityRole::Tab)
        .a11y_state(accessibility_state)
        .a11y_children(AccessibilityChildren::ExcludeDescendants)
        .install(
            interaction_chrome_radius(
                state_layer_color,
                NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT * 0.5,
            )
            // The state layer is the active indicator's pill, centred
            // horizontally at the item's top padding.
            .sized(
                hydrolysis_style::draw::kurbo::Size::new(
                    f64::from(NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH),
                    f64::from(NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT),
                ),
                hydrolysis_style::draw::kurbo::Vec2::new(
                    0.5,
                    f64::from(NAVIGATION_BAR_ITEM_TOP_PADDING)
                        / f64::from(
                            NAVIGATION_BAR_CONTAINER_HEIGHT
                                - NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT,
                        ),
                ),
            ),
        )
    }
}

fn navigation_tab_content<L: LabelKind>(
    label: Label<L>,
    icon: impl View,
    indicator_color: Color,
    icon_color: Color,
    label_color: Color,
    label_font: waterui_text::font::Font,
) -> impl View {
    let icon_slot = icon
        .foreground(icon_color)
        .width(NAVIGATION_BAR_ICON_SIZE)
        .height(NAVIGATION_BAR_ICON_SIZE);
    let icon_container = waterui_layout::stack::zstack((
        Capsule
            .fill(indicator_color)
            .width(NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH)
            .height(NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT),
        icon_slot,
    ));

    waterui_layout::stack::vstack((
        icon_container
            .width(NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH)
            .height(NAVIGATION_BAR_ICON_SLOT_HEIGHT),
        label.font(label_font).foreground(label_color).height(16.0),
    ))
    .spacing(NAVIGATION_BAR_LABEL_TOP_SPACE)
}

/// Creates a Material Design 3 navigation bar.
#[must_use]
pub const fn navigation_bar<Tabs>(tabs: Tabs) -> NavigationBar<Tabs> {
    NavigationBar::new(tabs)
}

/// Creates a Material Design 3 navigation tab.
#[must_use]
pub fn navigation_tab<L: IntoLabel, Icon>(
    label: L,
    icon: Icon,
    selected: &Binding<bool>,
) -> NavigationTab<L::Kind, Icon> {
    NavigationTab::new(label, icon, selected)
}

#[cfg(test)]
mod tests {
    use super::{
        NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT, NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH,
        NAVIGATION_BAR_CONTAINER_HEIGHT, NAVIGATION_BAR_ICON_SIZE,
        NAVIGATION_BAR_ITEM_BOTTOM_PADDING, NAVIGATION_BAR_ITEM_MIN_WIDTH,
        NAVIGATION_BAR_ITEM_TOP_PADDING, NAVIGATION_BAR_LABEL_TOP_SPACE,
    };
    use crate::elevation::MaterialElevationLevel;

    #[test]
    /// Values from `androidx.compose.material3.tokens.NavigationBarTokens`.
    fn navigation_bar_tokens_match_compose_navigation_bar_tokens() {
        assert_eq!(NAVIGATION_BAR_CONTAINER_HEIGHT, 64.0);
        assert_eq!(NAVIGATION_BAR_ITEM_MIN_WIDTH, 48.0);
        // md.comp.nav-bar.item.vertical.active-indicator.width = 56.
        assert_eq!(NAVIGATION_BAR_ACTIVE_INDICATOR_WIDTH, 56.0);
        assert_eq!(NAVIGATION_BAR_ACTIVE_INDICATOR_HEIGHT, 32.0);
        assert_eq!(NAVIGATION_BAR_ICON_SIZE, 24.0);
        assert_eq!(NAVIGATION_BAR_LABEL_TOP_SPACE, 4.0);
        // The 48dp content block (24 icon + 8 + 16 label) centers in the 64dp
        // item, and the 32dp indicator pill starts 4dp above the icon.
        assert_eq!(NAVIGATION_BAR_ITEM_TOP_PADDING, 4.0);
        assert_eq!(NAVIGATION_BAR_ITEM_BOTTOM_PADDING, 8.0);
        assert_eq!(
            MaterialElevationLevel::new(2),
            MaterialElevationLevel::LEVEL2
        );
    }
}
