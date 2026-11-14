//! `.dialog(...)` under the rendered runtime: the presented card's place in
//! the offscreen accessibility tree, the modal layer's scrim and its
//! rendered capture, and keyboard focus while a dialog is up
//! (water-rs/waterui#1210).
//!
//! Moved out of hydrolysis's own suite under water-rs/hydrolysis-m3#113:
//! every case mounts offscreen under Material 3 — the dialog's scrim, card
//! and buttons are this theme's chrome. The semantic-mount siblings stay in
//! waterui as unstyled tests.

use std::cell::Cell;
use std::rc::Rc;

use hydrolysis_m3::Material3;
use waterui::Binding;
use waterui::Signal as _;
use waterui::ViewExt as _;
use waterui::component::{text, vstack};
use waterui::dialog::{Dialog, DialogAction};
use waterui_controls::button;
use waterui_core::Environment;
use waterui_testing::{AccessKitRole, Role, RuntimeDriver, SemanticApp, ui};

/// A presented dialog publishes its card as the window's modal alert.
const ALERT: Role = Role::new(AccessKitRole::AlertDialog);

/// The standard fixture: content that opens a two-action dialog on
/// `presented`; `flag` records the destructive action's handler.
fn fixture(presented: &Binding<bool>, flag: Rc<Cell<bool>>) -> impl waterui::View + use<> {
    let open = presented.clone();
    vstack((
        text("Content"),
        button("Open").action(move |_: Environment| open.set(true)),
    ))
    .dialog(
        Dialog::new(presented, "Delete photo?")
            .message("This cannot be undone.")
            .action(DialogAction::cancel("Cancel", || {}))
            .action(DialogAction::destructive(
                "Delete",
                move |_: Environment| {
                    flag.set(true);
                },
            )),
    )
}

#[test]
fn presented_dialog_is_in_the_offscreen_a11y_tree() {
    let presented = Binding::bool(false);
    let flag = Rc::new(Cell::new(false));
    let shown = presented.clone();
    let mut app = ui()
        .viewport(400, 300)
        .theme(Material3::defaults())
        .mount_offscreen(move || fixture(&presented, flag.clone()));
    app.settle();
    app.query().role(ALERT).assert_not_exists();

    shown.set(true);
    app.settle();

    app.query().role(ALERT).assert_exists();
    app.query().label("Delete photo?").assert_exists();
    app.query()
        .role(Role::BUTTON)
        .label("Delete")
        .assert_exists();
}

/// A window with real content beneath the modal layer, so the capture shows
/// whether the scrim dims it or paints over it.
fn content_fixture(presented: &Binding<bool>) -> impl waterui::View + use<> {
    let open = presented.clone();
    vstack((
        text("Inbox").title(),
        text("Quarterly report draft — shared with the finance team"),
        text("Design review notes from Thursday"),
        text("Travel itinerary for the October offsite"),
        button("Open").action(move |_: Environment| open.set(true)),
    ))
    .dialog(
        Dialog::new(presented, "Delete 3 messages?")
            .message("Deleted messages move to the trash and are removed after 30 days.")
            .action(DialogAction::cancel("Cancel", || {}))
            .action(DialogAction::destructive("Delete", || {})),
    )
}

fn capture_presented(style: Material3, case: &str) {
    let presented = Binding::bool(false);
    let shown = presented.clone();
    let mut app = ui()
        .viewport(800, 600)
        .theme(style)
        .mount_offscreen(move || content_fixture(&presented));
    app.settle();
    // The window before the dialog: what the scrim must leave visible.
    app.capture_snapshot("dialog", case, "closed");
    shown.set(true);
    app.settle();

    let captured = app.capture_snapshot("dialog", case, "presented");
    assert!(
        captured.path().exists(),
        "the offscreen render wrote no PNG at {}",
        captured.path().display()
    );
}

#[test]
fn presented_dialog_renders_over_content_light() {
    capture_presented(Material3::defaults(), "light");
}

#[test]
fn presented_dialog_renders_over_content_dark() {
    capture_presented(Material3::dark(), "dark");
}

/// A dialog attached to one view of a larger window: the sibling content
/// lies outside the `.dialog` subtree but beneath the modal layer.
fn sibling_fixture(presented: &Binding<bool>) -> impl waterui::View + use<> {
    vstack((
        text("Sidebar"),
        button("Sibling action").action(|| {}),
        text("Editor").dialog(
            Dialog::new(presented, "Close editor?")
                .action(DialogAction::cancel("Stay", || {}))
                .action(DialogAction::new("Close", || {})),
        ),
    ))
}

#[test]
fn presented_dialog_makes_the_whole_window_inert_on_offscreen_mount() {
    let presented = Binding::bool(false);
    let shown = presented.clone();
    let mut app = ui()
        .viewport(400, 300)
        .theme(Material3::defaults())
        .mount_offscreen(move || sibling_fixture(&presented));
    app.settle();
    app.query().label("Sidebar").assert_exists();

    shown.set(true);
    app.settle();

    app.query().role(ALERT).assert_exists();
    app.query().label("Sidebar").assert_not_exists();
    app.query().label("Editor").assert_not_exists();
    app.query()
        .role(Role::BUTTON)
        .label("Sibling action")
        .assert_not_exists();
}

#[test]
fn scrim_tap_runs_the_cancel_path() {
    let presented = Binding::bool(false);
    let shown = presented.clone();
    let cancelled = Rc::new(Cell::new(false));
    let hit = cancelled.clone();
    let mut app = ui()
        .viewport(400, 300)
        .theme(Material3::defaults())
        .mount_offscreen(move || {
            let cancelled = cancelled.clone();
            text("Content").dialog(
                Dialog::new(&presented, "Leave?")
                    .action(DialogAction::cancel("Stay", move |_: Environment| {
                        cancelled.set(true);
                    }))
                    .action(DialogAction::new("Leave", || {})),
            )
        });
    app.settle();
    shown.set(true);
    app.settle();

    app.tap_at(8.0, 8.0);

    assert!(hit.get(), "the scrim tap did not reach the cancel handler");
    assert!(!shown.snapshot());
}

/// Two dialogs on one window, the later-declared one presented first.
fn two_dialogs(first: &Binding<bool>, second: &Binding<bool>) -> impl waterui::View + use<> {
    vstack((
        text("Top").dialog(
            Dialog::new(second, "Second dialog")
                .action(DialogAction::cancel("Dismiss second", || {})),
        ),
        text("Bottom").dialog(
            Dialog::new(first, "First dialog").action(DialogAction::cancel("Dismiss first", || {})),
        ),
    ))
}

fn assert_second_dialog_waits<R: RuntimeDriver>(
    app: &mut SemanticApp<R>,
    first: &Binding<bool>,
    second: &Binding<bool>,
) {
    first.set(true);
    app.settle();
    app.query().label("First dialog").assert_exists();

    second.set(true);
    app.settle();
    app.query().label("First dialog").assert_exists();
    app.query().label("Second dialog").assert_not_exists();

    app.query()
        .role(Role::BUTTON)
        .label("Dismiss first")
        .single()
        .tap(app);
    app.settle();
    assert!(!first.snapshot());
    assert!(
        second.snapshot(),
        "the waiting dialog lost its presentation"
    );
    app.query().label("Second dialog").assert_exists();
}

#[test]
fn a_second_dialog_waits_in_presentation_order_on_offscreen_mount() {
    let first = Binding::bool(false);
    let second = Binding::bool(false);
    let (f, s) = (first.clone(), second.clone());
    let mut app = ui()
        .viewport(400, 300)
        .theme(Material3::defaults())
        .mount_offscreen(move || two_dialogs(&f, &s));
    app.settle();
    assert_second_dialog_waits(&mut app, &first, &second);
}

/// The label of the node holding keyboard focus — the focus the window
/// publishes with its accessibility tree.
fn focused_label<R>(app: &SemanticApp<R>) -> Option<String> {
    let tree = app.tree();
    tree.node(tree.focus())
        .and_then(|node| node.label().map(str::to_owned))
}

#[test]
fn focus_moves_into_the_dialog_and_returns_on_dismissal() {
    let presented = Binding::bool(false);
    let flag = Rc::new(Cell::new(false));
    let shown = presented.clone();
    let mut app = ui()
        .viewport(400, 300)
        .theme(Material3::defaults())
        .mount_offscreen(move || fixture(&presented, flag.clone()));
    app.settle();
    app.press_named_key("Tab");
    app.settle();
    assert_eq!(focused_label(&app).as_deref(), Some("Open"));

    shown.set(true);
    app.settle();
    assert_eq!(focused_label(&app).as_deref(), Some("Cancel"));

    app.press_named_key("Escape");
    app.settle();
    assert!(!shown.snapshot());
    assert_eq!(focused_label(&app).as_deref(), Some("Open"));
}
