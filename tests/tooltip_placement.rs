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
//! Placement is delegated to `AnchoredOverlay`: the backend resolves flip and
//! clamp against the real window bounds, and `placed_edge` reports the edge the
//! popup landed on so the scale transition grows out of the side touching the
//! anchor. Dismissal plays the scale-out before the overlay leaves the window.

use core::time::Duration;

use hydrolysis_m3::{Material3, plain_tooltip, rich_tooltip};
use waterui::prelude::{PositionExt as _, UnitPoint, absolute};
use waterui_controls::button;
use waterui_core::View;
use waterui_testing::{NodeBounds, OffscreenApp, Role, Snapshot, Styled, UiBuilder};

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

/// Whether a pixel is dark enough to belong to the `InverseSurface` tooltip
/// container — the darkest fill in the scene.
fn is_dark(pixel: &[u8]) -> bool {
    u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2]) < 380
}

/// Dark pixels inside the rect — the drawn portion of the popup.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "accessibility bounds are logical pixels and the capture runs at \
              scale factor 1.0, so truncating them to snapshot indices is the \
              intended conversion"
)]
fn dark_pixels(snapshot: &Snapshot, x0: f32, y0: f32, x1: f32, y1: f32) -> usize {
    let x0 = x0.max(0.0) as usize;
    let y0 = y0.max(0.0) as usize;
    let x1 = (x1 as usize).min(snapshot.width as usize);
    let y1 = (y1 as usize).min(snapshot.height as usize);
    let width = snapshot.width as usize;
    (y0..y1)
        .flat_map(|y| (x0..x1).map(move |x| y * width + x))
        .filter(|&index| is_dark(&snapshot.rgba8[index * 4..index * 4 + 4]))
        .count()
}

/// The mean row of the dark pixels inside a rect — where the drawn popup's
/// mass sits vertically.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    reason = "accessibility bounds are logical pixels and the capture runs at \
              scale factor 1.0, so truncating them to snapshot indices is the \
              intended conversion"
)]
fn dark_row_centroid(snapshot: &Snapshot, x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    let x0 = x0.max(0.0) as usize;
    let y0 = y0.max(0.0) as usize;
    let x1 = (x1 as usize).min(snapshot.width as usize);
    let y1 = (y1 as usize).min(snapshot.height as usize);
    let width = snapshot.width as usize;
    let (sum, count) = (y0..y1)
        .flat_map(|y| (x0..x1).map(move |x| (y, x)))
        .filter(|&(y, x)| is_dark(&snapshot.rgba8[(y * width + x) * 4..(y * width + x) * 4 + 4]))
        .fold((0usize, 0usize), |(sum, count), (y, _)| {
            (sum + y, count + 1)
        });
    assert!(
        count > 0,
        "expected the tooltip to be drawing in {x0}..{x1} x {y0}..{y1}"
    );
    sum as f32 / count as f32
}

/// The plain tooltip container rect around its label's semantic bounds
/// (4 pt vertical, 8 pt horizontal padding).
fn container_bounds(label: NodeBounds) -> (f32, f32, f32, f32) {
    (
        label.x() - PLAIN_TOOLTIP_LEADING_PAD,
        label.y() - PLAIN_TOOLTIP_TOP_PAD,
        label.x() + label.width() + PLAIN_TOOLTIP_LEADING_PAD,
        label.y() + label.height() + PLAIN_TOOLTIP_TOP_PAD,
    )
}

const PLAIN_TOOLTIP_TOP_PAD: f32 = 4.0;
const PLAIN_TOOLTIP_LEADING_PAD: f32 = 8.0;

fn hover_over(app: &mut OffscreenApp, target: NodeBounds) {
    let (x, y) = target.center();
    app.queue_hover_at(x, y);
}

fn hover_off(app: &mut OffscreenApp) {
    app.queue_hover_at(4.0, SCENE_H - 4.0);
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

/// `Top` was requested but the top edge leaves no room, so `Bottom` was
/// placed: mid-scale-in the drawn popup grows out of its top edge — the side
/// touching the anchor — not the bottom edge the requested edge would imply.
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn flipped_plain_tooltip_scales_from_the_placed_edge(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(plain_target(), UnitPoint::TOP));
    let target = app.query().role(Role::BUTTON).label("+").single().bounds();
    hover_over(&mut app, target);
    app.pump_for(Duration::from_millis(400));
    let popup = app
        .query()
        .role(Role::LABEL)
        .label(PLAIN_LABEL)
        .single()
        .bounds();

    // Close fully, then reopen and freeze the transition mid-flight.
    hover_off(&mut app);
    app.pump_for(Duration::from_millis(400));
    app.query()
        .role(Role::LABEL)
        .label(PLAIN_LABEL)
        .assert_not_exists();

    hover_over(&mut app, target);
    app.pump_for(Duration::from_millis(48));
    let mid = app.snapshot();

    let (x0, y0, x1, y1) = container_bounds(popup);
    // The scale origin is the top edge (the side touching the anchor once
    // `Bottom` is placed), so the drawn region's vertical centre of mass sits
    // above the container's midline; a bottom-edge origin would draw the
    // lower half first.
    let centroid = dark_row_centroid(&mid, x0, y0, x1, y1);
    assert!(
        centroid < (y1 - y0).mul_add(0.5, y0),
        "flipped tooltip should grow from its top edge mid-scale-in \
         (centroid row {centroid:.1} of {y0:.1}..{y1:.1})"
    );
}

/// A dismissed tooltip keeps drawing while its scale-out plays — the overlay
/// leaves only once the transition inside it has finished.
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 180))]
fn dismissed_tooltip_plays_its_scale_out(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(move || scene(plain_target(), UnitPoint::CENTER));
    let target = app.query().role(Role::BUTTON).label("+").single().bounds();
    hover_over(&mut app, target);
    app.pump_for(Duration::from_millis(400));
    let popup = app
        .query()
        .role(Role::LABEL)
        .label(PLAIN_LABEL)
        .single()
        .bounds();
    let (x0, y0, x1, y1) = container_bounds(popup);

    // Moving the pointer off the target dismisses the tooltip; the scale-out
    // has 200ms (SHORT_4) to play, so mid-transition the popup still draws.
    hover_off(&mut app);
    app.pump_for(Duration::from_millis(80));
    let mid = app.snapshot();
    assert!(
        dark_pixels(&mid, x0, y0, x1, y1) > 0,
        "mid-exit the tooltip still draws — the overlay waits on its animation"
    );

    // Once the transition finishes the overlay leaves entirely.
    app.pump_for(Duration::from_millis(400));
    app.query()
        .role(Role::LABEL)
        .label(PLAIN_LABEL)
        .assert_not_exists();
    let gone = app.snapshot();
    assert_eq!(
        dark_pixels(&gone, x0, y0, x1, y1),
        0,
        "after the exit animation nothing of the popup remains"
    );
}
