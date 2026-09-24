//! Chrome acceptance for the icon-button variant contract: an icon-only
//! `Automatic` button or menu trigger draws the standard variant — no
//! container fill at rest, only the icon — with the state layer appearing on
//! hover and press, while `BorderedProminent` keeps its filled container and
//! `Bordered` its outline.

use core::time::Duration;

use hydrolysis_m3::{Argb, MaterialColorScheme, material_menu, material_menu_icon_label};
use waterui::ViewExt as _;
use waterui::component::{hstack, text, vstack};
use waterui::graphics::color::Srgb;
use waterui_controls::{button, label};
use waterui_core::View;
use waterui_testing::{NodeBounds, OffscreenApp, Role, Snapshot};

/// White host: every painted pixel the test counts inside the bounds is one
/// the chrome drew. The labels carry an empty icon so the label resolves to
/// `IconOnly` and nothing but the chrome paints inside the touch target.
fn material_shell<V: View>(content: V) -> impl View {
    vstack((content,))
        .spacing(12.0)
        .padding_with(16.0)
        .background(Srgb::WHITE)
}

fn icon_button_chrome_view() -> impl View {
    material_shell(vstack((
        hstack((
            button(label("Auto").icon(()).icon_only()),
            material_menu(
                material_menu_icon_label(label("More").icon(())),
                (button("Refresh").action(|| {}),),
            ),
        ))
        .spacing(12.0),
        hstack((
            button(label("Prominent").icon(()).icon_only()).bordered_prominent(),
            button(label("Outlined").icon(()).icon_only()).bordered(),
        ))
        .spacing(12.0),
    )))
}

/// Text-glyph icons resolve the environment `Foreground`, so they paint the
/// variant's content color: `Automatic` resolves to the standard icon
/// button's on-surface-variant, `BorderedProminent` keeps on-primary.
fn icon_button_content_color_view() -> impl View {
    material_shell(
        hstack((
            button(label("Auto").icon(text("*")).icon_only()),
            material_menu(
                material_menu_icon_label(label("More").icon(text("+"))),
                (button("Refresh").action(|| {}),),
            ),
            button(label("Prominent").icon(text("#")).icon_only()).bordered_prominent(),
        ))
        .spacing(12.0),
    )
}

fn button_bounds(app: &mut OffscreenApp, label: &str) -> NodeBounds {
    app.query()
        .role(Role::BUTTON)
        .label(label)
        .single()
        .bounds()
}

fn pixel_is(pixel: &[u8], argb: Argb) -> bool {
    pixel[0].abs_diff(argb.red()) <= 2
        && pixel[1].abs_diff(argb.green()) <= 2
        && pixel[2].abs_diff(argb.blue()) <= 2
}

/// Counts the pixels inside `bounds` that satisfy `predicate`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "accessibility bounds are logical pixels and the capture runs at \
              scale factor 1.0, so truncating them to snapshot indices is the \
              intended conversion"
)]
fn count_pixels(
    snapshot: &Snapshot,
    bounds: NodeBounds,
    predicate: impl Fn(&[u8]) -> bool,
) -> usize {
    let x0 = bounds.x().max(0.0) as usize;
    let y0 = bounds.y().max(0.0) as usize;
    let x1 = ((bounds.x() + bounds.width()) as usize).min(snapshot.width as usize);
    let y1 = ((bounds.y() + bounds.height()) as usize).min(snapshot.height as usize);
    let width = snapshot.width as usize;
    (y0..y1)
        .flat_map(|y| (x0..x1).map(move |x| y * width + x))
        .filter(|&index| predicate(&snapshot.rgba8[index * 4..index * 4 + 4]))
        .count()
}

/// Pixels matching the scheme's `primary` — the filled container.
fn primary_pixels(snapshot: &Snapshot, bounds: NodeBounds) -> usize {
    let primary = MaterialColorScheme::baseline_light().primary.argb();
    count_pixels(snapshot, bounds, |pixel| pixel_is(pixel, primary))
}

/// The square `side`×`side` centred inside `bounds` — the part of the touch
/// target a ring stroke leaves unpainted.
fn centered_box(bounds: NodeBounds, side: f32) -> NodeBounds {
    NodeBounds::new(
        bounds.x() + (bounds.width() - side) / 2.0,
        bounds.y() + (bounds.height() - side) / 2.0,
        side,
        side,
    )
}

/// Pixels inside `bounds` that are not the white host — anything the chrome
/// painted, fill, stroke, or state layer.
fn painted_pixels(snapshot: &Snapshot, bounds: NodeBounds) -> usize {
    count_pixels(snapshot, bounds, |pixel| {
        !(pixel[0] >= 253 && pixel[1] >= 253 && pixel[2] >= 253)
    })
}

#[waterui::test(
    icon_button_chrome_view,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (360, 160),
    offscreen
)]
fn automatic_icon_button_draws_no_container_at_rest(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::BUTTON)
            .label("Auto")
            .wait_for_existence(Duration::from_secs(3)),
        "icon-only automatic button must mount"
    );
    let bounds = button_bounds(app, "Auto");
    let snapshot = app.snapshot();
    assert_eq!(
        primary_pixels(&snapshot, bounds),
        0,
        "an automatic icon button must not paint the filled container in {bounds:?}"
    );
    assert_eq!(
        painted_pixels(&snapshot, bounds),
        0,
        "the standard icon button has no container: {bounds:?} should hold only the icon"
    );
}

#[waterui::test(
    icon_button_chrome_view,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (360, 160),
    offscreen
)]
fn icon_only_menu_trigger_draws_no_container_at_rest(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::BUTTON)
            .label("More")
            .wait_for_existence(Duration::from_secs(3)),
        "icon-only menu trigger must mount"
    );
    let bounds = button_bounds(app, "More");
    let snapshot = app.snapshot();
    assert_eq!(
        primary_pixels(&snapshot, bounds),
        0,
        "an automatic menu trigger must not paint the filled container in {bounds:?}"
    );
    assert_eq!(
        painted_pixels(&snapshot, bounds),
        0,
        "the standard icon button has no container: {bounds:?} should hold only the icon"
    );
}

#[waterui::test(
    icon_button_chrome_view,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (360, 160),
    offscreen
)]
fn automatic_icon_button_lights_its_state_layer_on_hover_and_press(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::BUTTON)
            .label("Auto")
            .wait_for_existence(Duration::from_secs(3)),
        "icon-only automatic button must mount"
    );
    let bounds = button_bounds(app, "Auto");
    let (cx, cy) = (
        bounds.x() + bounds.width() / 2.0,
        bounds.y() + bounds.height() / 2.0,
    );

    app.query().role(Role::BUTTON).label("Auto").hover();
    app.pump_for(Duration::from_millis(120));
    let hovered = app.snapshot();
    assert!(
        painted_pixels(&hovered, bounds) > 400,
        "hover must light the icon-button state layer in {bounds:?}"
    );

    app.queue_pointer_down(cx, cy);
    let _ = app.snapshot();
    app.pump_for(Duration::from_millis(250));
    let pressed = app.snapshot();
    assert!(
        painted_pixels(&pressed, bounds) > 400,
        "press must light the icon-button state layer in {bounds:?}"
    );
    app.queue_pointer_up(cx, cy);
}

#[waterui::test(
    icon_button_chrome_view,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (360, 160),
    offscreen
)]
fn icon_only_menu_trigger_lights_its_state_layer_on_hover(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::BUTTON)
            .label("More")
            .wait_for_existence(Duration::from_secs(3)),
        "icon-only menu trigger must mount"
    );
    let bounds = button_bounds(app, "More");

    app.query().role(Role::BUTTON).label("More").hover();
    app.pump_for(Duration::from_millis(120));
    let hovered = app.snapshot();
    assert!(
        painted_pixels(&hovered, bounds) > 400,
        "hover must light the menu trigger state layer in {bounds:?}"
    );
}

#[waterui::test(
    icon_button_chrome_view,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (360, 160),
    offscreen
)]
fn prominent_and_outlined_icon_buttons_keep_their_containers(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::BUTTON)
            .label("Prominent")
            .wait_for_existence(Duration::from_secs(3)),
        "bordered-prominent icon button must mount"
    );
    let prominent = button_bounds(app, "Prominent");
    let outlined = button_bounds(app, "Outlined");
    let snapshot = app.snapshot();
    assert!(
        primary_pixels(&snapshot, prominent) > 400,
        "the filled icon button keeps its primary container in {prominent:?}"
    );
    let stroke_pixels = painted_pixels(&snapshot, outlined);
    assert!(
        stroke_pixels > 20,
        "the outlined icon button keeps its outline stroke in {outlined:?}: \
         found {stroke_pixels} painted pixels"
    );
    assert_eq!(
        primary_pixels(&snapshot, outlined),
        0,
        "the outlined icon button has no fill in {outlined:?}"
    );
    assert_eq!(
        painted_pixels(&snapshot, centered_box(outlined, 24.0)),
        0,
        "the outlined icon button's interior stays unfilled in {outlined:?}"
    );
}

#[waterui::test(
    icon_button_content_color_view,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (360, 160),
    offscreen
)]
fn icon_only_labels_paint_the_standard_content_color(app: &mut OffscreenApp) {
    assert!(
        app.query()
            .role(Role::BUTTON)
            .label("Auto")
            .wait_for_existence(Duration::from_secs(3)),
        "icon-only automatic button must mount"
    );
    // The standard variant paints its icon in on-surface-variant — visible
    // on the white host. `on_primary` would draw nothing.
    for label in ["Auto", "More"] {
        let icon = centered_box(button_bounds(app, label), 24.0);
        let snapshot = app.snapshot();
        assert!(
            painted_pixels(&snapshot, icon) > 4,
            "the {label:?} icon must be visible at rest in {icon:?}"
        );
    }

    // The filled icon button keeps on-primary content: the white glyph is
    // only visible against its primary container, whose channels never
    // reach 200.
    let prominent = centered_box(button_bounds(app, "Prominent"), 24.0);
    let light = count_pixels(&app.snapshot(), prominent, |pixel| {
        pixel[0] > 200 && pixel[1] > 200 && pixel[2] > 200
    });
    assert!(
        light > 4,
        "the filled icon button keeps on-primary content in {prominent:?}: \
         found {light} light pixels"
    );
}
