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
use crate::icon_button::pressed_corner_radius;
use crate::icon_paths::{self, IconGrid};
use crate::theme::motion;
use hydrolysis_style::draw::{Draw as _, Paint};
use hydrolysis_style::theme::{
    BarSurface, Chrome, CornerShape, Custom, Driven, Inputs, NavigationBarLook, Surface,
};
use waterui_core::Environment;
use waterui_graphics::color::Color;

pub const fn metrics() -> NavigationMetrics {
    NavigationMetrics {
        automatic_bar_height: 64.0,
        inline_bar_height: 64.0,
        medium_bar_height: 112.0,
        large_bar_height: 152.0,
        inline_title_height: 28.0,
        medium_title_height: 36.0,
        large_title_height: 36.0,
        title_leading_inset: 16.0,
        title_trailing_inset: 16.0,
        large_title_bottom_inset: 28.0,
        horizontal_inset: 4.0,
        item_spacing: 0.0,
        search_height: 56.0,
        search_vertical_inset: 4.0,
        back_button_size: 40.0,
        back_button_leading_inset: 4.0,
        back_button_top_inset: 12.0,
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
    use crate::theme::motion::tokens::duration;
    use hydrolysis_style::Style as _;
    use hydrolysis_style::draw::Paint;
    use hydrolysis_style::draw::kurbo::Rect;
    use hydrolysis_style::theme::{BarSurface, Corners, Inputs};
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
        assert_eq!(metrics.automatic_bar_height, 64.0);
        assert_eq!(metrics.inline_bar_height, 64.0);
        // AppBarMediumFlexibleTokens.ContainerHeight, and
        // AppBarLargeFlexibleTokens.LargeContainerHeight.
        assert_eq!(metrics.medium_bar_height, 112.0);
        assert_eq!(metrics.large_bar_height, 152.0);
        assert_eq!(metrics.title_leading_inset, 16.0);
        assert_eq!(metrics.title_trailing_inset, 16.0);
        assert_eq!(metrics.large_title_bottom_inset, 28.0);
        assert_eq!(metrics.horizontal_inset, 4.0);
        assert_eq!(metrics.item_spacing, 0.0);
        assert_eq!(metrics.search_height, 56.0);
        assert_eq!(metrics.search_vertical_inset, 4.0);
        assert_eq!(metrics.back_button_size, 40.0);
        assert_eq!(metrics.back_button_leading_inset, 4.0);
        assert_eq!(metrics.back_button_top_inset, 12.0);
    }

    #[test]
    fn pages_move_on_the_emphasized_shared_axis() {
        let motion = look(&environment()).motion();

        assert_eq!(motion.transition_duration, duration::LONG_1);
        assert_eq!(motion.shared_axis_slide_distance, 30.0);
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
            crate::theme::motion::HOVER_STATE_LAYER_OPACITY
        );
        assert!(chrome.press().get_ripple().is_some());
    }
}
