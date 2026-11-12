//! Visual evidence for the Material 3 filled text field look
//! (`md.comp.filled-text-field.*`): the container and its top-rounded shape,
//! the hover tint, the active indicator thickening and turning `primary` with
//! focus, the label resting and floated, the placeholder, the caret and the
//! disabled field, in the light and the dark scheme.
//!
//! The captures are reviewed by eye; the assertions only pin that the field
//! mounted where the pointer is aimed.

use core::time::Duration;

use hydrolysis_m3::Material3;
use waterui::ViewExt as _;
use waterui::component::vstack;
use waterui::reactive::binding;
use waterui_controls::field;
use waterui_core::AnyView;
use waterui_form::secure::{Secure, secure};
use waterui_testing::{OffscreenApp, Role, ui};

/// The viewport the fields are captured in.
const VIEWPORT: (u32, u32) = (320, 120);

/// How long a captured transition is given to finish.
const SETTLE: Duration = Duration::from_millis(600);

/// A labelled field holding `text`, disabled or not.
fn labelled_field(text: &'static str, disabled: bool) -> impl Fn() -> AnyView {
    let value = binding(waterui::Str::from(text));
    move || AnyView::new(vstack((field("Name", &value).disabled(disabled),)).padding())
}

/// A labelled field with no label view, standing its prompt in as the label.
fn prompted_field() -> impl Fn() -> AnyView {
    let value = binding(waterui::Str::from(""));
    move || {
        AnyView::new(vstack((field("Name", &value).hide_label().prompt("Search chats"),)).padding())
    }
}

/// A labelled secure field holding `text`.
fn secure_field(text: &'static str) -> impl Fn() -> AnyView {
    let value = binding(Secure::new(text.into()));
    move || AnyView::new(vstack((secure("Password", &value),)).padding())
}

fn mount(theme: Material3, view: impl Fn() -> AnyView + 'static) -> OffscreenApp {
    let mut app = ui()
        .viewport(VIEWPORT.0, VIEWPORT.1)
        .theme(theme)
        .mount_offscreen(view);
    app.pump_for(SETTLE);
    app
}

const fn schemes() -> [(&'static str, Material3); 2] {
    [
        ("light", Material3::defaults()),
        ("dark", Material3::dark()),
    ]
}

fn centre(app: &mut OffscreenApp) -> (f32, f32) {
    app.query()
        .role(Role::TEXT_INPUT)
        .single()
        .bounds()
        .center()
}

/// The field at rest, empty: the label rests centred in the container, the
/// active indicator is 1 tall in `on-surface-variant`.
#[test]
fn text_field_rest_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, labelled_field("", false));
        app.capture_snapshot("material3-preview", "text-field", format!("rest-{scheme}"));
    }
}

/// The field hovered: the on-surface hover tint over the container and the
/// active indicator in `on-surface`.
#[test]
fn text_field_hovered_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, labelled_field("", false));
        let (x, y) = centre(&mut app);
        app.queue_pointer_move(x, y);
        app.pump_for(SETTLE);
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("hovered-{scheme}"),
        );
    }
}

/// The field focused and holding text: the label floated to the top in
/// `primary`, the caret in `primary` and the active indicator 2 tall in
/// `primary`.
#[test]
fn text_field_focused_with_text_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, labelled_field("Hydrolysis", false));
        app.query().role(Role::TEXT_INPUT).label("Name").focus();
        app.pump_for(SETTLE);
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("focused-text-{scheme}"),
        );
    }
}

/// The field holding text but not focused: the label stays floated, in
/// `on-surface-variant`, with the active indicator back to 1 tall.
#[test]
fn text_field_floated_label_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, labelled_field("Hydrolysis", false));
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("floated-label-{scheme}"),
        );
    }
}

/// The field focused and empty: the label has floated to the top over the
/// empty text area.
#[test]
fn text_field_focused_empty_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, labelled_field("", false));
        app.query().role(Role::TEXT_INPUT).label("Name").focus();
        app.pump_for(SETTLE);
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("focused-empty-{scheme}"),
        );
    }
}

/// The disabled field: the container at on-surface 4%, the label, the text
/// and the active indicator at on-surface 38%, no hover tint under the
/// pointer.
#[test]
fn text_field_disabled_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, labelled_field("Hydrolysis", true));
        let (x, y) = centre(&mut app);
        app.queue_pointer_move(x, y);
        app.pump_for(SETTLE);
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("disabled-{scheme}"),
        );
    }
}

/// A field with no label view: the prompt stands in as the label and
/// floats on focus.
#[test]
fn text_field_prompt_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, prompted_field());
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("prompt-rest-{scheme}"),
        );
        app.query().role(Role::TEXT_INPUT).focus();
        app.pump_for(SETTLE);
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("prompt-focused-{scheme}"),
        );
    }
}

/// The secure field wears the same look.
#[test]
fn secure_field_material3_snapshots() {
    for (scheme, theme) in schemes() {
        let mut app = mount(theme, secure_field("hunter2"));
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("secure-rest-{scheme}"),
        );
        app.query().role(Role::PASSWORD_INPUT).focus();
        app.pump_for(SETTLE);
        app.capture_snapshot(
            "material3-preview",
            "text-field",
            format!("secure-focused-{scheme}"),
        );
    }
}
