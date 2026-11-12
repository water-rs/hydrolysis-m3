//! The Material 3 top app bar look from the `md.comp.top-app-bar.small.*`
//! tokens: the bar's container, the leading navigation icon button, the
//! bar's measures and the page transition's motion.
//!
//! The container is one control part over the bar's frame and its colour,
//! recorded once while both hold. The back button is the standard icon
//! button's chrome — the state layer, one press ripple wave per press, the
//! focus indicator and the pressed corner morph — with the `arrow_back`
//! glyph as a part of it, so a hover, a press or a focus change moves live
//! operands and records nothing.

use crate::NavigationMetrics;
use crate::color::OnSurface;
use crate::controls::button::{container_corners, interactive, solid_paint};
use crate::dimensions::{
    NAVIGATION_BACK_BUTTON_LEADING_INSET, NAVIGATION_BACK_BUTTON_SIZE,
    NAVIGATION_BACK_BUTTON_TOP_INSET, NAVIGATION_BAR_AUTOMATIC_HEIGHT,
    NAVIGATION_BAR_HORIZONTAL_INSET, NAVIGATION_BAR_INLINE_HEIGHT, NAVIGATION_BAR_ITEM_SPACING,
    NAVIGATION_BAR_LARGE_HEIGHT, NAVIGATION_BAR_MEDIUM_HEIGHT, NAVIGATION_LARGE_TITLE_BOTTOM_INSET,
    NAVIGATION_SEARCH_HEIGHT, NAVIGATION_SEARCH_VERTICAL_INSET, NAVIGATION_TITLE_INLINE_HEIGHT,
    NAVIGATION_TITLE_LARGE_HEIGHT, NAVIGATION_TITLE_LEADING_INSET, NAVIGATION_TITLE_MEDIUM_HEIGHT,
    NAVIGATION_TITLE_TRAILING_INSET,
};
use crate::icon_button::pressed_corner_radius;
use crate::icon_paths::{self, IconGrid};
use crate::theme::motion;
use hydrolysis::draw::{Draw as _, Paint};
use hydrolysis::theme::{
    BarSurface, Chrome, CornerShape, Custom, Driven, Inputs, NavigationBarLook, Surface,
};
use waterui_core::Environment;
use waterui_graphics::color::Color;

pub const fn metrics() -> NavigationMetrics {
    NavigationMetrics {
        automatic_bar_height: NAVIGATION_BAR_AUTOMATIC_HEIGHT,
        inline_bar_height: NAVIGATION_BAR_INLINE_HEIGHT,
        medium_bar_height: NAVIGATION_BAR_MEDIUM_HEIGHT,
        large_bar_height: NAVIGATION_BAR_LARGE_HEIGHT,
        inline_title_height: NAVIGATION_TITLE_INLINE_HEIGHT,
        medium_title_height: NAVIGATION_TITLE_MEDIUM_HEIGHT,
        large_title_height: NAVIGATION_TITLE_LARGE_HEIGHT,
        title_leading_inset: NAVIGATION_TITLE_LEADING_INSET,
        title_trailing_inset: NAVIGATION_TITLE_TRAILING_INSET,
        large_title_bottom_inset: NAVIGATION_LARGE_TITLE_BOTTOM_INSET,
        horizontal_inset: NAVIGATION_BAR_HORIZONTAL_INSET,
        item_spacing: NAVIGATION_BAR_ITEM_SPACING,
        search_height: NAVIGATION_SEARCH_HEIGHT,
        search_vertical_inset: NAVIGATION_SEARCH_VERTICAL_INSET,
        back_button_size: NAVIGATION_BACK_BUTTON_SIZE,
        back_button_leading_inset: NAVIGATION_BACK_BUTTON_LEADING_INSET,
        back_button_top_inset: NAVIGATION_BACK_BUTTON_TOP_INSET,
    }
}

/// The look of the navigation chrome.
///
/// The container is the bar's colour — the application's, or `surface` —
/// and the bar draws no separator: `md.comp.top-app-bar` has no divider
/// token.
pub fn look(env: &Environment) -> NavigationBarLook {
    NavigationBarLook::new(metrics(), motion::navigation(), back_button(env), bar())
}

/// The bar's container: the colour the bar draws in, over the bar's frame.
fn bar() -> Custom<BarSurface, (Driven<Paint, BarSurface>,)> {
    Custom::new((container_paint(),), |recorder, frame, (paint,)| {
        recorder.fill(frame, paint);
    })
}

/// The container's paint: the colour the bar draws in.
fn container_paint() -> Driven<Paint, BarSurface> {
    Driven::new(|inputs: &Inputs<'_, BarSurface>| Paint::from(inputs.value.color))
}

/// The leading navigation icon button: the standard icon button's chrome —
/// no container, corner-full at rest and morphing to the small corner while
/// pressed — with its state layer in `on-surface`, the colour of the icon
/// (`md.comp.top-app-bar.small.leading-icon.color`, which the Compose
/// ripple takes from the content colour), and the 24dp `arrow_back` glyph
/// centred in it.
fn back_button(env: &Environment) -> Chrome {
    interactive(
        Surface::clear(CornerShape::new(container_corners(pressed_corner_radius))),
        Color::from(OnSurface),
    )
    .custom(Custom::new(
        (solid_paint(OnSurface, env),),
        |recorder, frame, (paint,)| {
            let grid = IconGrid::centered(frame.center(), icon_paths::icon_grid_size());
            recorder.fill(icon_paths::arrow_back(grid), paint);
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::{back_button, container_paint, look};
    use crate::MaterialColorScheme;
    use crate::dimensions::{
        NAVIGATION_BACK_BUTTON_LEADING_INSET, NAVIGATION_BACK_BUTTON_SIZE,
        NAVIGATION_BACK_BUTTON_TOP_INSET, NAVIGATION_BAR_AUTOMATIC_HEIGHT,
        NAVIGATION_BAR_HORIZONTAL_INSET, NAVIGATION_BAR_INLINE_HEIGHT, NAVIGATION_BAR_ITEM_SPACING,
        NAVIGATION_BAR_LARGE_HEIGHT, NAVIGATION_BAR_MEDIUM_HEIGHT,
        NAVIGATION_LARGE_TITLE_BOTTOM_INSET, NAVIGATION_SEARCH_HEIGHT,
        NAVIGATION_SEARCH_VERTICAL_INSET, NAVIGATION_TITLE_LEADING_INSET,
        NAVIGATION_TITLE_TRAILING_INSET,
    };
    use crate::theme::dimensions::NAVIGATION_SHARED_AXIS_SLIDE_DISTANCE;
    use crate::theme::motion::tokens::duration;
    use hydrolysis::Style as _;
    use hydrolysis::draw::Paint;
    use hydrolysis::draw::kurbo::Rect;
    use hydrolysis::theme::{BarSurface, Corners, Inputs};
    use waterui::interaction::InteractionState;
    use waterui_core::{Environment, Signal as _};

    fn environment() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    #[test]
    fn navigation_metrics_match_compose_app_bar_tokens() {
        let look = look(&environment());
        let metrics = look.metrics();

        assert_eq!(metrics, super::metrics());
        assert_eq!(
            metrics.automatic_bar_height,
            NAVIGATION_BAR_AUTOMATIC_HEIGHT
        );
        assert_eq!(metrics.inline_bar_height, NAVIGATION_BAR_INLINE_HEIGHT);
        assert_eq!(metrics.medium_bar_height, NAVIGATION_BAR_MEDIUM_HEIGHT);
        assert_eq!(metrics.large_bar_height, NAVIGATION_BAR_LARGE_HEIGHT);
        assert_eq!(metrics.title_leading_inset, NAVIGATION_TITLE_LEADING_INSET);
        assert_eq!(
            metrics.title_trailing_inset,
            NAVIGATION_TITLE_TRAILING_INSET
        );
        assert_eq!(
            metrics.large_title_bottom_inset,
            NAVIGATION_LARGE_TITLE_BOTTOM_INSET
        );
        assert_eq!(metrics.horizontal_inset, NAVIGATION_BAR_HORIZONTAL_INSET);
        assert_eq!(metrics.item_spacing, NAVIGATION_BAR_ITEM_SPACING);
        assert_eq!(metrics.search_height, NAVIGATION_SEARCH_HEIGHT);
        assert_eq!(
            metrics.search_vertical_inset,
            NAVIGATION_SEARCH_VERTICAL_INSET
        );
        assert_eq!(metrics.back_button_size, NAVIGATION_BACK_BUTTON_SIZE);
        assert_eq!(
            metrics.back_button_leading_inset,
            NAVIGATION_BACK_BUTTON_LEADING_INSET
        );
        assert_eq!(
            metrics.back_button_top_inset,
            NAVIGATION_BACK_BUTTON_TOP_INSET
        );
        assert_eq!(NAVIGATION_BAR_AUTOMATIC_HEIGHT, 64.0);
        assert_eq!(NAVIGATION_BAR_INLINE_HEIGHT, 64.0);
        // AppBarMediumFlexibleTokens.ContainerHeight, and
        // AppBarLargeFlexibleTokens.LargeContainerHeight.
        assert_eq!(NAVIGATION_BAR_MEDIUM_HEIGHT, 112.0);
        assert_eq!(NAVIGATION_BAR_LARGE_HEIGHT, 152.0);
        assert_eq!(NAVIGATION_TITLE_LEADING_INSET, 16.0);
        assert_eq!(NAVIGATION_LARGE_TITLE_BOTTOM_INSET, 28.0);
        assert_eq!(NAVIGATION_SEARCH_HEIGHT, 56.0);
        assert_eq!(NAVIGATION_BAR_HORIZONTAL_INSET, 4.0);
        assert_eq!(NAVIGATION_BACK_BUTTON_SIZE, 40.0);
    }

    #[test]
    fn pages_move_on_the_emphasized_shared_axis() {
        let motion = look(&environment()).motion();

        assert_eq!(motion.transition_duration, duration::LONG_1);
        assert_eq!(
            motion.shared_axis_slide_distance,
            NAVIGATION_SHARED_AXIS_SLIDE_DISTANCE
        );
    }

    #[test]
    fn the_bar_draws_the_colour_it_is_given() {
        let colors = MaterialColorScheme::baseline_light();
        let color = colors.surface_container.working();
        let value = BarSurface { color };
        let frame = Rect::new(0.0, 0.0, 360.0, 64.0);

        assert_eq!(
            container_paint().target(&Inputs::at_rest(frame, InteractionState::empty(), &value)),
            Paint::from(color)
        );
    }

    /// The navigation slot is a standard icon button: no container, the
    /// state layer in the icon's `on-surface`, and the pressed corner
    /// morph of the 40dp state layer.
    #[test]
    fn the_back_button_is_a_standard_icon_button_in_on_surface() {
        let colors = MaterialColorScheme::baseline_light();
        let env = environment();
        let chrome = back_button(&env);
        let frame = Rect::new(4.0, 12.0, 44.0, 52.0);
        let corners = |state| {
            chrome
                .surface()
                .shape()
                .corners()
                .target(&Inputs::at_rest(frame, state, &()))
        };

        assert_eq!(
            chrome.state_layer().color.resolve(&env).snapshot(),
            colors.on_surface.working()
        );
        assert_eq!(corners(InteractionState::empty()), Corners::Full);
        // md.sys.shape.corner-small for the 40dp state layer.
        assert_eq!(corners(InteractionState::PRESSED), Corners::from(8.0));
        assert_eq!(
            chrome.state_layer().hover,
            crate::theme::state_layer::HOVER_STATE_LAYER_OPACITY
        );
        assert!(chrome.press().get_ripple().is_some());
    }
}
