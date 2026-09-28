//! Acceptance tests for issue #93: tooltip placement must follow the M3
//! popup rules — plain tooltips open above the target and flip below at the
//! top edge, rich tooltips open below and flip above at the bottom edge, and
//! both clamp horizontally inside the scene.
//!
//! The flip and clamp rules follow Compose Material3's
//! `TooltipPositionProviderImpl` (`abovePositioning`/`belowPositioning`): the
//! popup is centred on the anchor horizontally, offset by the
//! `SpacingBetweenTooltipAndAnchor` token (4 dp), flipped when the preferred
//! side leaves the window, then clamped to the window bounds. The horizontal
//! viewport margin comes from `Widget.Material3.Tooltip`'s
//! `android:layout_margin` in material-components-android.
//!
//! These tests currently FAIL on `dev`: `TooltipLayout::place` places the
//! popup unconditionally (plain above, rich below) and never consults the
//! scene bounds, so a target at the top edge renders its popup off-window.

use hydrolysis_m3::{Material3, plain_tooltip, rich_tooltip};
use waterui::prelude::{PositionExt as _, UnitPoint, absolute};
use waterui_controls::button;
use waterui_core::View;
use waterui_testing::{NodeBounds, OffscreenApp, Role, Styled, UiBuilder};

/// A small scene; the popup for a wide label overflows it unless the layout
/// clamps to the scene bounds.
const SCENE_W: f32 = 320.0;
const SCENE_H: f32 = 180.0;
const EPS: f32 = 0.75;

const PLAIN_LABEL: &str = "Adds this item to your favorites";
const RICH_SUBHEAD: &str = "Grant permission";
const RICH_TEXT: &str = "Allow the app to add this item to your favorites";

/// Mounts `content` at `position` inside the scene — the smallest wrapper
/// that pins the target to an edge or the centre of the window.
fn scene(content: impl View, position: UnitPoint) -> impl View {
    absolute((content.position_in(position),))
}

fn open_tooltip(app: &mut OffscreenApp, target: &str, popup: &str) -> (NodeBounds, NodeBounds) {
    app.press_named_key("Tab");
    let target_bounds = app
        .query()
        .role(Role::BUTTON)
        .label(target)
        .single()
        .bounds();
    let popup_bounds = app.query().role(Role::LABEL).label(popup).single().bounds();
    (target_bounds, popup_bounds)
}

fn assert_above(popup: NodeBounds, target: NodeBounds, context: &str) {
    assert!(
        popup.y() + popup.height() <= target.y() + EPS,
        "{context}: popup {popup:?} should end above target {target:?}"
    );
}

fn assert_below(popup: NodeBounds, target: NodeBounds, context: &str) {
    assert!(
        popup.y() >= target.y() + target.height() - EPS,
        "{context}: popup {popup:?} should start below target {target:?}"
    );
}

fn assert_inside_scene(popup: NodeBounds, context: &str) {
    assert!(
        popup.x() >= -EPS,
        "{context}: popup {popup:?} escapes the leading scene edge"
    );
    assert!(
        popup.y() >= -EPS,
        "{context}: popup {popup:?} escapes the top scene edge"
    );
    assert!(
        popup.x() + popup.width() <= SCENE_W + EPS,
        "{context}: popup {popup:?} escapes the trailing scene edge ({SCENE_W})"
    );
    assert!(
        popup.y() + popup.height() <= SCENE_H + EPS,
        "{context}: popup {popup:?} escapes the bottom scene edge ({SCENE_H})"
    );
}

fn plain_target() -> impl View {
    plain_tooltip(PLAIN_LABEL).for_target(button("+").action(|| {}))
}

fn rich_target() -> impl View {
    rich_tooltip(RICH_SUBHEAD, RICH_TEXT).for_target(button("+").action(|| {}))
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn plain_tooltip_opens_above_target_by_default(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(plain_target(), UnitPoint::CENTER));
    let (target, popup) = open_tooltip(&mut app, "+", PLAIN_LABEL);
    assert_above(popup, target, "plain tooltip default");
    assert_inside_scene(popup, "plain tooltip default");
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn plain_tooltip_flips_below_target_at_top_edge(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(plain_target(), UnitPoint::TOP));
    let (target, popup) = open_tooltip(&mut app, "+", PLAIN_LABEL);
    assert_below(popup, target, "plain tooltip at top edge");
    assert_inside_scene(popup, "plain tooltip at top edge");
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn plain_tooltip_clamps_inside_scene_at_leading_edge(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(plain_target(), UnitPoint::LEADING));
    let (_target, popup) = open_tooltip(&mut app, "+", PLAIN_LABEL);
    assert_inside_scene(popup, "plain tooltip at leading edge");
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn plain_tooltip_clamps_inside_scene_at_trailing_edge(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(plain_target(), UnitPoint::TRAILING));
    let (_target, popup) = open_tooltip(&mut app, "+", PLAIN_LABEL);
    assert_inside_scene(popup, "plain tooltip at trailing edge");
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn rich_tooltip_opens_below_target_by_default(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(rich_target(), UnitPoint::TOP));
    let (target, popup) = open_tooltip(&mut app, "+", RICH_TEXT);
    assert_below(popup, target, "rich tooltip default");
    assert_inside_scene(popup, "rich tooltip default");
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn rich_tooltip_flips_above_target_at_bottom_edge(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(rich_target(), UnitPoint::BOTTOM));
    let (target, popup) = open_tooltip(&mut app, "+", RICH_TEXT);
    assert_above(popup, target, "rich tooltip at bottom edge");
    assert_inside_scene(popup, "rich tooltip at bottom edge");
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn rich_tooltip_clamps_inside_scene_at_leading_edge(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(rich_target(), UnitPoint::LEADING));
    let (_target, popup) = open_tooltip(&mut app, "+", RICH_TEXT);
    assert_inside_scene(popup, "rich tooltip at leading edge");
}

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn rich_tooltip_clamps_inside_scene_at_trailing_edge(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(rich_target(), UnitPoint::TRAILING));
    let (_target, popup) = open_tooltip(&mut app, "+", RICH_TEXT);
    assert_inside_scene(popup, "rich tooltip at trailing edge");
}
