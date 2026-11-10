//! Visual acceptance for the Material 3 Expressive progress indicators.
//!
//! Expressive rounds both ends of the linear bar, separates the active
//! indicator from the track by `TrackActiveSpace`, and marks the track's end
//! with a stop indicator. The PNG is reviewed by eye.

use core::time::Duration;
use waterui::component::progress::{Progress, progress};
use waterui::prelude::*;
use waterui_testing::{OffscreenApp, Styled, UiBuilder};

fn indicators() -> impl View {
    vstack((
        progress(Binding::f64(0.25)).label("Quarter"),
        progress(Binding::f64(0.5)).label("Half"),
        progress(Binding::f64(0.9)).label("Nearly done"),
        progress(Binding::f64(1.0)).label("Complete"),
    ))
    .spacing(20.0)
    .padding()
}

#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (420, 300))]
fn linear_progress_renders_the_expressive_track_gap(
    ui: UiBuilder<Styled<hydrolysis_m3::Material3>>,
) {
    let mut app: OffscreenApp = ui.mount_offscreen(indicators);
    let _ = app.capture_snapshot("material3-preview", "progress", "linear-expressive");
}

fn indeterminate_indicators() -> impl View {
    vstack((
        Progress::infinity().label("Syncing"),
        hstack((
            Progress::infinity().circular(),
            Progress::infinity().circular().four_color(),
            Progress::infinity().four_color(),
        )),
    ))
    .spacing(20.0)
    .padding()
}

/// The indeterminate motion is the engine's keyframes, so a capture is a
/// sample of it: pumping advances the animation clock, wall-clock sleeping
/// would show the same frame every time.
#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (420, 220))]
fn indeterminate_progress_moves_with_pumped_time(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let mut app: OffscreenApp = ui.mount_offscreen(indeterminate_indicators);
    for step in 0..6 {
        app.pump_for(Duration::from_millis(300));
        let _ = app.capture_snapshot(
            "material3-preview",
            "progress",
            format!("indeterminate-{step}"),
        );
    }
}

/// A value change animates the determinate parts in place.
#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (300, 180))]
fn determinate_progress_animates_to_its_new_value(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let value = Binding::f64(0.1);
    let mut app: OffscreenApp = ui.mount_offscreen({
        let value = value.clone();
        move || {
            vstack((
                progress(value.clone()).label("Uploading"),
                progress(value.clone()).circular(),
            ))
            .spacing(20.0)
            .padding()
        }
    });
    let _ = app.capture_snapshot("material3-preview", "progress", "value-0.1");
    value.set(0.75);
    app.pump_for(Duration::from_millis(120));
    let _ = app.capture_snapshot("material3-preview", "progress", "value-0.75-mid");
    app.pump_for(Duration::from_millis(600));
    let _ = app.capture_snapshot("material3-preview", "progress", "value-0.75");
}

fn circular_indicators() -> impl View {
    hstack((
        progress(Binding::f64(0.25)).circular(),
        progress(Binding::f64(0.6)).circular(),
        progress(Binding::f64(0.95)).circular(),
    ))
    .spacing(24.0)
    .padding()
}

#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (300, 120))]
fn circular_progress_draws_its_track(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let mut app: OffscreenApp = ui.mount_offscreen(circular_indicators);
    let _ = app.capture_snapshot("material3-preview", "progress", "circular-expressive");
}
