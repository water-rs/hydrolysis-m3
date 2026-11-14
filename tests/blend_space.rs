//! Verifies that `Material3` configures sRGB-encoded blending on the surface
//! so translucent M3 layers (such as the 12% `on-surface` disabled container
//! fill over the baseline light surface) match M3's sRGB compositing model.

use core::time::Duration;

use hydrolysis::draw::BlendSpace;
use hydrolysis_m3::{Material3, WidgetTheme as _};
use waterui::ViewExt as _;
use waterui::component::vstack;
use waterui_controls::{button, label};
use waterui_core::View;
use waterui_testing::{OffscreenApp, Role};

fn disabled_filled_icon_button() -> impl View {
    vstack((button(label("Disabled").icon(()).icon_only())
        .bordered_prominent()
        .disabled(true),))
    .padding_with(16.0)
}

#[test]
fn material3_uses_srgb_encoded_blend_space() {
    assert_eq!(Material3::defaults().blend_space(), BlendSpace::SrgbEncoded);
}

#[waterui::test(
    disabled_filled_icon_button,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (160, 120),
    offscreen
)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "button bounds are non-negative logical coordinates at 1x scale"
)]
fn disabled_container_blends_in_srgb_encoded_space(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::BUTTON)
            .label("Disabled")
            .wait_for_existence(Duration::from_secs(3)),
        "button must mount"
    );
    let bounds = app
        .query()
        .role(Role::BUTTON)
        .label("Disabled")
        .single()
        .bounds();
    let cx = (bounds.x() + bounds.width() / 2.0) as usize;
    let cy = (bounds.y() + bounds.height() / 2.0) as usize;
    let snap = app.snapshot();
    let offset = (cy * snap.width() as usize + cx) * 4;
    let pixel = &snap.rgba8()[offset..offset + 4];
    // Baseline light surface (#FEF7FF: 254, 247, 255) beneath 12% on-surface
    // (#1D1B20: 29, 27, 32) yields round(0.12 * fg + 0.88 * bg) = [227, 221, 228]
    // in sRGB-encoded blending (linear blending would yield ~[238, 232, 239]).
    assert!(
        pixel[0].abs_diff(227) <= 2 && pixel[1].abs_diff(221) <= 2 && pixel[2].abs_diff(228) <= 2,
        "expected sRGB-encoded disabled container near [227, 221, 228], got {pixel:?}"
    );
}
