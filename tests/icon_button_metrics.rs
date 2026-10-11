//! Material 3 icon-button metrics: an icon-only `button` or `Menu` trigger
//! resolves through the theme's icon-button contract — the 48×48 touch
//! target with the 40dp state layer drawn centred inside — not the
//! text-button chrome.
//!
//! Moved out of hydrolysis's own suite under water-rs/hydrolysis-m3#113:
//! every case mounts offscreen under `Material3::defaults()` and asserts
//! geometry or pixels only this theme provides. The semantic halves of the
//! origin tests — accessible names, disabled input rejection — stay in
//! waterui's controls crate as unstyled tests.

mod support {
    /// Converts a `f32` coordinate to `usize` with `as` saturating truncation.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "test coordinates are non-negative and within usize range"
    )]
    pub const fn f32_as_usize(v: f32) -> usize {
        v as usize
    }

    /// Widens a `usize` count to `f64`, rounding to nearest.
    #[expect(clippy::cast_precision_loss, reason = "test counts are far below 2^53")]
    pub const fn usize_as_f64(v: usize) -> f64 {
        v as f64
    }
}

use waterui::View;
use waterui::ViewExt as _;
use waterui::component::{hstack, vstack};
use waterui::graphics::color::Srgb;
use waterui_controls::{Menu, button, label};
use waterui_testing::{OffscreenApp, Role};

fn control_shell<V: View>(content: V) -> impl View {
    vstack((content,))
        .spacing(12.0)
        .padding_with(16.0)
        .background(Srgb::BLACK)
}

fn assert_close(actual: f64, expected: f64, epsilon: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta <= epsilon,
        "{context}: expected {expected:.4}, got {actual:.4}, delta={delta:.4}, epsilon={epsilon:.4}"
    );
}

// water-rs/hydrolysis#115: an icon-only label resolves the button through
// the theme's icon-button metrics — the M3 icon-button touch target (48×48),
// with the 40dp state layer drawn centred inside. The layout bounds are the
// hit area.
fn icon_only_button_view() -> impl waterui::View {
    control_shell(
        vstack((
            button(label("Search").icon(()).icon_only()),
            button("Cancel"),
        ))
        .spacing(12.0),
    )
}

#[waterui::test(icon_only_button_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 240), offscreen)]
fn icon_only_button_measures_the_icon_button_touch_target(app: &mut OffscreenApp) {
    let icon_button = app.query().role(Role::BUTTON).label("Search").single();
    let bounds = icon_button.bounds();
    assert_close(
        f64::from(bounds.width()),
        48.0,
        0.5,
        "icon-only button layout width",
    );
    assert_close(
        f64::from(bounds.height()),
        48.0,
        0.5,
        "icon-only button layout height",
    );

    // A text button beside it keeps the text-button metrics.
    let text_button = app.query().role(Role::BUTTON).label("Cancel").single();
    let bounds = text_button.bounds();
    assert!(
        f64::from(bounds.width()) >= 58.0,
        "text button width must keep the text-button minimum: {bounds:?}"
    );
    assert_close(f64::from(bounds.height()), 40.0, 0.5, "text button height");
}

#[waterui::test(icon_only_button_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 240), offscreen)]
fn icon_only_button_draws_the_state_layer_centred_in_its_bounds(app: &mut OffscreenApp) {
    let bounds = app
        .query()
        .role(Role::BUTTON)
        .label("Search")
        .single()
        .bounds();

    // The standard icon button carries no container: at rest nothing is
    // drawn inside the black-background touch target.
    let rest = app.snapshot();
    let rx0 = support::f32_as_usize(bounds.x().max(0.0));
    let ry0 = support::f32_as_usize(bounds.y().max(0.0));
    let rx1 = (support::f32_as_usize(bounds.x() + bounds.width())).min(rest.width as usize);
    let ry1 = (support::f32_as_usize(bounds.y() + bounds.height())).min(rest.height as usize);
    let mut painted_at_rest = 0usize;
    for y in ry0..ry1 {
        for x in rx0..rx1 {
            let px = &rest.rgba8[(y * rest.width as usize + x) * 4..][..4];
            if px[0].max(px[1]).max(px[2]) > 4 {
                painted_at_rest += 1;
            }
        }
    }
    assert_eq!(
        painted_at_rest, 0,
        "a standard icon button draws nothing at rest in {bounds:?}"
    );

    // On hover the only drawn pixels inside the bounds are the icon-button
    // state layer — a 40dp circle centred in the 48dp touch target. Measure
    // its rasterised bounding box.
    app.query().role(Role::BUTTON).label("Search").hover();
    app.pump_for(std::time::Duration::from_millis(120));
    let snapshot = app.snapshot();
    let x0 = support::f32_as_usize(bounds.x().max(0.0));
    let y0 = support::f32_as_usize(bounds.y().max(0.0));
    let x1 = (support::f32_as_usize(bounds.x() + bounds.width())).min(snapshot.width as usize);
    let y1 = (support::f32_as_usize(bounds.y() + bounds.height())).min(snapshot.height as usize);
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0usize;
    let mut max_y = 0usize;
    for y in y0..y1 {
        for x in x0..x1 {
            let px = &snapshot.rgba8[(y * snapshot.width as usize + x) * 4..][..4];
            if px[0].max(px[1]).max(px[2]) > 4 {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }
    assert!(
        min_x <= max_x,
        "the state layer drew no pixels in {bounds:?}"
    );
    let drawn_w = support::usize_as_f64(max_x - min_x + 1);
    let drawn_h = support::usize_as_f64(max_y - min_y + 1);
    assert_close(drawn_w, 40.0, 1.5, "drawn state-layer width");
    assert_close(drawn_h, 40.0, 1.5, "drawn state-layer height");
    assert_close(
        support::usize_as_f64(min_x + max_x + 1) / 2.0,
        f64::from(bounds.x() + bounds.width() / 2.0),
        1.0,
        "state-layer horizontal centre",
    );
    assert_close(
        support::usize_as_f64(min_y + max_y + 1) / 2.0,
        f64::from(bounds.y() + bounds.height() / 2.0),
        1.0,
        "state-layer vertical centre",
    );
}

// A row of icon-only buttons lays out at the touch-target width plus
// spacing — no text-button minimum width is applied to any of them.
fn icon_button_row_view() -> impl waterui::View {
    control_shell(
        hstack((
            button(label("First").icon(()).icon_only()),
            button(label("Second").icon(()).icon_only()),
            button(label("Third").icon(()).icon_only()),
            button(label("Fourth").icon(()).icon_only()),
            button(label("Fifth").icon(()).icon_only()),
        ))
        .spacing(12.0),
    )
}

#[waterui::test(icon_button_row_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 240), offscreen)]
fn a_row_of_icon_only_buttons_lays_out_at_touch_target_width(app: &mut OffscreenApp) {
    let buttons = app.query().role(Role::BUTTON).all();
    assert_eq!(buttons.len(), 5, "the row must expose five buttons");

    let mut xs: Vec<(f32, f32)> = buttons
        .iter()
        .map(|button| {
            let bounds = button.bounds();
            assert_close(
                f64::from(bounds.width()),
                48.0,
                0.5,
                "each icon button's layout width",
            );
            (bounds.x(), bounds.x() + bounds.width())
        })
        .collect();
    xs.sort_by(|a, b| a.0.total_cmp(&b.0));

    let span = f64::from(xs[4].1 - xs[0].0);
    // 5 × 48dp touch target + 4 × 12dp spacing.
    assert_close(span, 4.0f64.mul_add(12.0, 5.0 * 48.0), 0.5, "row span");
}

fn actions_menu_view() -> impl waterui::View {
    control_shell(Menu::new(
        label("Actions").icon(()),
        (
            button("Refresh").action(|| {}),
            Menu::new("Advanced", (button("Archive").action(|| {}),)),
        ),
    ))
}

// Origin: waterui `components/foundation/controls/tests/e2e_semantics.rs` —
// the geometric half of `menu_button_exposes_accessible_name`; the accessible
// name half stays in waterui as a semantic test.
#[waterui::test(actions_menu_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 240), offscreen)]
fn menu_button_exposes_accessible_name(app: &mut OffscreenApp) {
    let menu = app.query().role(Role::BUTTON).label("Actions").single();
    let bounds = menu.bounds();
    assert!(
        bounds.width() > 0.0 && bounds.height() > 0.0,
        "menu-button-exposes-accessible-name: menu trigger bounds must be non-zero"
    );
}

// Origin: water-rs/hydrolysis#149 — an icon-only menu trigger sizes under the
// icon-button contract, not the text-button chrome.
fn icon_only_menu_view() -> impl waterui::View {
    control_shell(
        vstack((
            Menu::new(
                label("More").icon(()).icon_only(),
                (button("Refresh").action(|| {}),),
            ),
            button(label("Search").icon(()).icon_only()),
        ))
        .spacing(12.0),
    )
}

#[waterui::test(icon_only_menu_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 240), offscreen)]
fn icon_only_menu_trigger_measures_the_icon_button_touch_target(app: &mut OffscreenApp) {
    let menu = app.query().role(Role::BUTTON).label("More").single();
    let icon_button = app.query().role(Role::BUTTON).label("Search").single();

    let menu_bounds = menu.bounds();
    let button_bounds = icon_button.bounds();
    assert_close(
        f64::from(menu_bounds.width()),
        f64::from(button_bounds.width()),
        0.5,
        "icon-only menu trigger width must match the icon-only button",
    );
    assert_close(
        f64::from(menu_bounds.height()),
        f64::from(button_bounds.height()),
        0.5,
        "icon-only menu trigger height must match the icon-only button",
    );
    assert_close(
        f64::from(menu_bounds.width()),
        48.0,
        0.5,
        "icon-only menu trigger width",
    );
    assert_close(
        f64::from(menu_bounds.height()),
        48.0,
        0.5,
        "icon-only menu trigger height",
    );
}
