//! Visual acceptance for the Material 3 stepper.
//!
//! Material has no stepper, but a connected button group: the far corners
//! fully round, the corners either side of the seam tucked in, and a held end
//! tightening its seam corner further. An end at the limit of the range reads
//! as a disabled button. The PNG is reviewed by eye.

use core::time::Duration;
use waterui::prelude::*;
use waterui::reactive::binding;
use waterui_controls::stepper::stepper;
use waterui_testing::{OffscreenApp, Role, Styled, UiBuilder};

fn steppers() -> impl View {
    let mid = binding(3_i32);
    let low = binding(0_i32);
    let high = binding(5_i32);
    vstack((
        stepper("Quantity", &mid).range(0..=5),
        stepper("At the minimum", &low).range(0..=5),
        stepper("At the maximum", &high).range(0..=5),
        stepper("Disabled", &mid).range(0..=5).disabled(true),
        stepper("Hidden label", &mid).range(0..=5).hide_label(),
    ))
    .spacing(16.0)
    .padding()
}

#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (420, 320))]
fn stepper_renders_the_connected_button_group(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let mut app: OffscreenApp = ui.mount_offscreen(steppers);
    let _ = app.capture_snapshot("material3-preview", "stepper", "states");
}

fn quantity() -> impl View {
    let value = binding(3_i32);
    vstack((stepper("Quantity", &value).range(0..=5),)).padding()
}

/// A held end: its seam corner tightens while the far corner stays round, and
/// the state layer and ripple follow the silhouette.
#[ignore = "writes visual acceptance PNG files for direct image review"]
#[waterui::test(quantity, theme = hydrolysis_m3::Material3::defaults(), viewport = (420, 120), offscreen)]
fn stepper_pressed_end_tightens_its_seam_corner(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::SPIN_BUTTON)
            .label("Quantity")
            .wait_for_existence(Duration::from_secs(3)),
        "the stepper must mount"
    );
    let bounds = app
        .query()
        .role(Role::SPIN_BUTTON)
        .label("Quantity")
        .single()
        .bounds();
    let (_, y) = bounds.center();
    let x = bounds.x() + bounds.width() - 20.0;
    let _ = app.capture_snapshot("material3-preview", "stepper", "rest");
    app.queue_pointer_down(x, y);
    app.pump_for(Duration::from_millis(400));
    let _ = app.capture_snapshot("material3-preview", "stepper", "pressed");
}
