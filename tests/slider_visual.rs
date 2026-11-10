//! Visual acceptance for the Material 3 Expressive slider.
//!
//! Expressive replaced the circular thumb on a hairline track with a narrow
//! vertical bar on a thick track, split by a gap around the handle and marked
//! with a stop indicator at the far end. The PNG is reviewed by eye.

use waterui::prelude::*;
use waterui::reactive::binding;
use waterui_controls::slider::slider;
use waterui_testing::{OffscreenApp, Styled, UiBuilder};

fn sliders() -> impl View {
    let quiet = binding(0.0_f64);
    let low = binding(0.25_f64);
    let mid = binding(0.5_f64);
    let full = binding(1.0_f64);
    vstack((
        slider("Zero", &quiet),
        slider("Low", &low),
        slider("Mid", &mid),
        slider("Full", &full),
        slider("Tinted", &mid).tint(Color::srgb(0, 121, 107)),
        slider("Disabled", &low).disabled(true),
        slider("Labelled", &mid)
            .min_value_label("Min")
            .max_value_label("Max"),
    ))
    .spacing(16.0)
    .padding()
}

#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (420, 640))]
fn slider_renders_the_expressive_bar_handle(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let mut app: OffscreenApp = ui.mount_offscreen(sliders);
    let _ = app.capture_snapshot("material3-preview", "slider", "expressive");
}

fn indicated() -> impl View {
    let value = binding(30.0_f64);
    vstack((slider("Volume", &value)
        .range(0.0..=100.0)
        .value_indicator(|value| format!("{value:.0}").into()),))
    .padding()
}

/// A held slider: the handle narrows, the track gap with it, the halo shows
/// and the value bubble rides above the handle.
#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(indicated, theme = hydrolysis_m3::Material3::defaults(), viewport = (420, 200), offscreen)]
fn slider_dragging_shows_the_value_indicator(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .label("Volume")
            .wait_for_existence(std::time::Duration::from_secs(3)),
        "the slider must mount"
    );
    let bounds = app.query().label("Volume").single().bounds();
    let y = bounds.y() + bounds.height() - 22.0;
    let x = bounds.width().mul_add(0.3, bounds.x());
    let _ = app.capture_snapshot("material3-preview", "slider", "rest");
    app.queue_pointer_down(x, y);
    app.queue_pointer_move(x + 60.0, y);
    app.pump_for(std::time::Duration::from_millis(400));
    let _ = app.capture_snapshot("material3-preview", "slider", "dragging");
}
