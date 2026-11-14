//! Visual acceptance for the MD3 switch: M3-faithful thumb geometry, track
//! outline shrink, 200ms standard-easing value motion, and the staged
//! checked-icon transition.
//!
//! The PNG-producing tests are ignored by default and reviewed by eye.

use core::time::Duration;
use waterui::ViewExt as _;
use waterui::reactive::binding;
use waterui_controls::toggle;
use waterui_core::View;
use waterui_testing::{OffscreenApp, Role};

fn save(app: &mut OffscreenApp, stage: &str) {
    let _ = app.capture_snapshot("material3-preview", "switch", stage);
}

fn wifi_toggle_view() -> impl View {
    let value = binding(false);
    // `toggle()` takes the platform default, a checkbox on macOS, so the
    // switch this capture names is spelled out.
    toggle("Wi-Fi", &value).switch().padding_with(16.0)
}

#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(wifi_toggle_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (200, 120), offscreen)]
fn switch_toggle_slides_and_reveals_checkmark(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::SWITCH)
            .label("Wi-Fi")
            .wait_for_existence(Duration::from_secs(3)),
        "toggle must mount"
    );
    save(app, "off");
    let bounds = app
        .query()
        .role(Role::SWITCH)
        .label("Wi-Fi")
        .single()
        .bounds();
    let (cx, cy) = (
        bounds.x() + bounds.width() / 2.0,
        bounds.y() + bounds.height() / 2.0,
    );
    app.queue_pointer_down(cx, cy);
    let _ = app.snapshot();
    app.queue_pointer_up(cx, cy);
    let _ = app.snapshot();
    // Mid-transition (200ms value motion): thumb sliding, icon not yet in.
    // The pointer still hovers the switch, so the thumb shows the
    // on-surface-variant hover color per the M3 reference.
    app.pump_for(Duration::from_millis(60));
    save(app, "toggling-60ms");
    // Park the pointer away from the switch: the settled state must be free
    // of hover chrome (no dark thumb, no hover halo).
    app.queue_pointer_move(5.0, 5.0);
    app.pump_for(Duration::from_millis(500));
    save(app, "on-idle");
    // Toggle back off: icon leaves within the first 50ms.
    app.queue_pointer_down(cx, cy);
    let _ = app.snapshot();
    app.queue_pointer_up(cx, cy);
    let _ = app.snapshot();
    app.pump_for(Duration::from_millis(60));
    save(app, "untoggling-60ms");
    app.queue_pointer_move(5.0, 5.0);
    app.pump_for(Duration::from_millis(500));
    save(app, "off-idle");
}

/// A drag moves the thumb with the finger and settles on the nearer end.
#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(wifi_toggle_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (200, 120), offscreen)]
fn switch_thumb_follows_a_drag_and_settles(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::SWITCH)
            .label("Wi-Fi")
            .wait_for_existence(Duration::from_secs(3)),
        "toggle must mount"
    );
    let bounds = app
        .query()
        .role(Role::SWITCH)
        .label("Wi-Fi")
        .single()
        .bounds();
    // The node fills its row: the switch sits at the trailing edge, and the
    // drag starts on its track.
    let (cx, cy) = (
        bounds.x() + bounds.width() - 26.0,
        bounds.y() + bounds.height() / 2.0,
    );
    app.queue_pointer_down(cx, cy);
    let _ = app.snapshot();
    app.queue_pointer_move(cx + 6.0, cy);
    app.queue_pointer_move(cx + 14.0, cy);
    app.pump_for(Duration::from_millis(60));
    save(app, "dragging");
    app.queue_pointer_up(cx + 14.0, cy);
    app.queue_pointer_move(5.0, 5.0);
    app.pump_for(Duration::from_millis(500));
    save(app, "dragged-on");
}

fn notify_checkbox_view() -> impl View {
    let value = binding(false);
    // The state layer is a 40dp circle around the 18dp box: padded so the
    // capture shows it whole.
    toggle("Notify me", &value).checkbox().padding_with(16.0)
}

#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(notify_checkbox_view, theme = hydrolysis_m3::Material3::defaults(), viewport = (200, 120), offscreen)]
fn checkbox_checks_and_unchecks(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::CHECKBOX)
            .label("Notify me")
            .wait_for_existence(Duration::from_secs(3)),
        "checkbox must mount"
    );
    let _ = app.capture_snapshot("material3-preview", "checkbox", "off");
    let bounds = app
        .query()
        .role(Role::CHECKBOX)
        .label("Notify me")
        .single()
        .bounds();
    // The node fills its row: the box sits at the leading edge.
    let (cx, cy) = (bounds.x() + 9.0, bounds.y() + bounds.height() / 2.0);
    app.queue_pointer_down(cx, cy);
    app.pump_for(Duration::from_millis(150));
    let _ = app.capture_snapshot("material3-preview", "checkbox", "pressed");
    app.queue_pointer_up(cx, cy);
    app.queue_pointer_move(5.0, 5.0);
    app.pump_for(Duration::from_millis(500));
    let _ = app.capture_snapshot("material3-preview", "checkbox", "on");
}
