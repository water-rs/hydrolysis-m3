//! Snackbar layout under Material 3: the bar hugs its message (pinned at the
//! `SnackbarTheme` `min_width` floor when a close control rides along) rather
//! than stretching to the `max_width` cap.
//!
//! Moved out of hydrolysis's own suite under water-rs/hydrolysis-m3#113 —
//! origin: waterui `tests/snackbar_layout.rs`. The assertions mount
//! offscreen through the styled builder, so they live beside the theme that
//! lays the bar out.

use std::time::Duration;

use waterui::prelude::*;
use waterui::snackbar::{Snackbar, SnackbarManager, SnackbarTheme};
use waterui_testing::{OffscreenApp, Styled, UiBuilder};

const MIN_WIDTH: f32 = 300.0;
const MAX_WIDTH: f32 = 600.0;

/// Mounts an app whose window shows `snackbar`, and returns it settled.
fn show(
    ui: UiBuilder<Styled<hydrolysis_m3::Material3>>,
    snackbar: Snackbar,
) -> (OffscreenApp, SnackbarManager) {
    let (manager, overlay) = SnackbarManager::new();
    let theme = SnackbarTheme {
        min_width: MIN_WIDTH,
        max_width: MAX_WIDTH,
        ..SnackbarTheme::default()
    };
    let mut env = Environment::new();
    env.insert(manager.clone());
    env.insert(theme);
    let mut app = ui
        .environment(env)
        .mount_offscreen(move || zstack((text("app content"), overlay.clone())));
    manager.show(snackbar);
    app.settle();
    (app, manager)
}

/// A closeable bar with a short message sits at the `min_width` floor with its
/// close control pinned to the trailing edge — nowhere near the `max_width`
/// cap it used to stretch to.
///
/// Origin: waterui `tests/snackbar_layout.rs`.
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (900, 600))]
fn a_short_closeable_snackbar_hugs_the_min_width_floor(
    ui: UiBuilder<Styled<hydrolysis_m3::Material3>>,
) {
    let (mut app, _manager) = show(
        ui,
        Snackbar::new("Saved").duration(Duration::ZERO).closeable(),
    );

    let message = app.query().label("Saved").single().bounds();
    let close = app.query().label("Close").single().bounds();

    let span = close.x() + close.width() - message.x();
    assert!(
        span <= MIN_WIDTH,
        "the bar's content spans {span} logical pixels, wider than the \
         {MIN_WIDTH} floor — it is still stretching toward the {MAX_WIDTH} cap"
    );
    assert!(
        span > MIN_WIDTH * 0.7,
        "the close control sits {span} from the message's leading edge; the \
         spacer should pin it near the {MIN_WIDTH} floor's trailing edge"
    );
}

/// A plain bar hugs its message rather than stretching to the cap.
///
/// Origin: waterui `tests/snackbar_layout.rs`.
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (900, 600))]
fn a_plain_snackbar_hugs_its_message(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let (mut app, _manager) = show(
        ui,
        Snackbar::new("Copied to clipboard").duration(Duration::ZERO),
    );

    let message = app.query().label("Copied to clipboard").single().bounds();
    assert!(
        message.width() < MIN_WIDTH,
        "a short message stays narrower than the bar's own floor"
    );
}
