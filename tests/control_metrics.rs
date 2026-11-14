//! Material 3 control metrics seen through rendered frames: the switch
//! track, the checkbox box, and the stepper's two buttons report exactly the
//! sizes the theme draws, and every point inside the reported bounds acts on
//! the control — no dead zone where a hidden label used to sit.
//!
//! Moved out of hydrolysis's own suite under water-rs/hydrolysis-m3#113: the
//! assertions name Material 3 dimensions (a 52×32 pt switch track, an 18 pt
//! checkbox, two 40 pt stepper buttons), so they live beside the theme that
//! produces them.

use waterui::Binding;
use waterui::Signal as _;
use waterui::View;
use waterui::ViewExt as _;
use waterui::component::{vstack, zstack};
use waterui::graphics::color::Srgb;
use waterui_controls::{Toggle, stepper};
use waterui_testing::{Role, Styled, UiBuilder};

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

// water-rs/waterui#2241: a hidden label contributes no frame — the toggle's
// reported bounds are the switch's own, and a tap anywhere inside them
// toggles the control (no dead zone beside the switch).
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 240))]
fn hidden_label_toggle_is_switch_sized_and_toggles_throughout(
    ui: UiBuilder<Styled<hydrolysis_m3::Material3>>,
) {
    let enabled = Binding::bool(false);
    let enabled_for_view = enabled.clone();

    let mut app = ui.mount_offscreen(move || {
        control_shell(
            Toggle::new("Dark mode", &enabled_for_view)
                .switch()
                .hide_label(),
        )
    });

    let element = app.query().role(Role::SWITCH).label("Dark mode").single();
    let bounds = element.bounds();
    // Material3 switch metrics: a 52x32 pt track; the hidden label adds
    // nothing to either axis.
    assert_close(
        f64::from(bounds.width()),
        52.0,
        0.5,
        "hidden-label toggle: width must be the switch's own",
    );
    assert_close(
        f64::from(bounds.height()),
        32.0,
        0.5,
        "hidden-label toggle: height must be the switch's own",
    );

    // Every point inside the reported bounds must toggle — sweep the row.
    let (cx, cy) = bounds.center();
    for x in [bounds.x() + 1.0, cx, bounds.x() + bounds.width() - 1.0] {
        app.tap_at(x, cy);
        assert!(
            enabled.snapshot(),
            "hidden-label toggle: tap at x={x} inside the reported bounds must toggle"
        );
        enabled.set(false);
    }

    // The accessibility action still toggles it too.
    app.query().role(Role::SWITCH).label("Dark mode").tap();
    assert!(
        enabled.snapshot(),
        "hidden-label toggle: accessibility action must toggle"
    );
}

// water-rs/waterui#2326 (layout spec §3, §6): under one offer, a switch with
// a visible label fills the width while a checkbox and any toggle with a
// hidden label answer the control's intrinsic size. The `ZStack` keeps a
// stretcher's finite answer instead of promoting it, so the switch inside
// it fills only because the leaf itself answers the proposed width.
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 320))]
fn toggle_frames_follow_style_and_label_visibility(
    ui: UiBuilder<Styled<hydrolysis_m3::Material3>>,
) {
    let views: [_; 5] = core::array::from_fn(|_| Binding::bool(false));

    let mut app = ui.mount_offscreen(move || {
        let [wifi, bluetooth, airdrop, hotspot, cellular] = views.clone();
        control_shell(vstack((
            Toggle::new("Wi-Fi", &wifi).switch(),
            Toggle::new("Bluetooth", &bluetooth).switch().hide_label(),
            Toggle::new("AirDrop", &airdrop).checkbox(),
            Toggle::new("Hotspot", &hotspot).checkbox().hide_label(),
            zstack((Toggle::new("Cellular", &cellular).switch(),)),
        )))
    });

    // The 320 pt viewport less `control_shell`'s 16 pt padding on each side.
    let offered = 288.0;
    let mut frame = |role: Role, label: &str| app.query().role(role).label(label).single().bounds();

    let wifi = frame(Role::SWITCH, "Wi-Fi");
    assert_close(
        f64::from(wifi.width()),
        offered,
        0.5,
        "visible-label switch: width must be the offered width",
    );
    assert_close(
        f64::from(wifi.height()),
        32.0,
        0.5,
        "visible-label switch: height must stay intrinsic",
    );

    let cellular = frame(Role::SWITCH, "Cellular");
    assert_close(
        f64::from(cellular.width()),
        offered,
        0.5,
        "visible-label switch: the leaf must answer the proposed width",
    );

    // Material3 metrics: a 52x32 pt switch track, an 18 pt checkbox, 8 pt
    // between the control and its label.
    let bluetooth = frame(Role::SWITCH, "Bluetooth");
    assert_close(
        f64::from(bluetooth.width()),
        52.0,
        0.5,
        "hidden-label switch: width must be the switch's own",
    );
    let hotspot = frame(Role::CHECKBOX, "Hotspot");
    assert_close(
        f64::from(hotspot.width()),
        18.0,
        0.5,
        "hidden-label checkbox: width must be the box's own",
    );
    assert_close(
        f64::from(hotspot.height()),
        18.0,
        0.5,
        "hidden-label checkbox: height must be the box's own",
    );

    let airdrop = frame(Role::CHECKBOX, "AirDrop");
    let airdrop_width = f64::from(airdrop.width());
    assert!(
        airdrop_width > 18.0 + 8.0 && airdrop_width < offered - 1.0,
        "visible-label checkbox: width {airdrop_width} must be box, gap and label — not the offer"
    );
}

// water-rs/waterui#2364 (layout spec §3, §6): a hidden label contributes no
// frame — the stepper's reported bounds are the buttons' own, and a tap
// anywhere inside them steps the value (no dead zone beside the buttons).
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (320, 240))]
fn hidden_label_stepper_is_button_sized_and_steps_throughout(
    ui: UiBuilder<Styled<hydrolysis_m3::Material3>>,
) {
    let quantity = Binding::i32(0);
    let quantity_for_view = quantity.clone();

    let mut app = ui.mount_offscreen(move || {
        control_shell(stepper("Quantity", &quantity_for_view).hide_label())
    });

    let element = app
        .query()
        .role(Role::SPIN_BUTTON)
        .label("Quantity")
        .single();
    let bounds = element.bounds();
    // Material3 stepper metrics: two 40 pt buttons with 2 pt between them;
    // the hidden label adds nothing to either axis.
    assert_close(
        f64::from(bounds.width()),
        82.0,
        0.5,
        "hidden-label stepper: width must be the buttons' own",
    );
    assert_close(
        f64::from(bounds.height()),
        40.0,
        0.5,
        "hidden-label stepper: height must be the buttons' own",
    );

    // Every point inside the reported bounds is a button: the leading half
    // decrements, the trailing half increments.
    let (_, cy) = bounds.center();
    app.tap_at(bounds.x() + 1.0, cy);
    assert_eq!(
        quantity.snapshot(),
        -1,
        "hidden-label stepper: tap inside the decrement button must step down"
    );
    app.tap_at(bounds.x() + bounds.width() - 1.0, cy);
    assert_eq!(
        quantity.snapshot(),
        0,
        "hidden-label stepper: tap inside the increment button must step up"
    );

    // The accessibility actions still step it too.
    app.query()
        .role(Role::SPIN_BUTTON)
        .label("Quantity")
        .increment();
    assert_eq!(
        quantity.snapshot(),
        1,
        "hidden-label stepper: accessibility increment must step up"
    );
}
