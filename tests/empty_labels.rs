//! Empty `Label` nodes stay out of the accessibility tree: a navigation page
//! that binds no subtitle emits no subtitle node, and an empty text leaf
//! emits nothing until its content becomes non-empty — the same contract
//! decorative graphics leaves already follow (water-rs/hydrolysis#176; #167
//! introduced the subtitle node and started realizing one on every page).
//!
//! Moved out of hydrolysis's own suite under water-rs/hydrolysis-m3#113:
//! these are the `mount_offscreen` halves, which observe the rendered
//! runtime's accessibility tree under Material 3. The semantic-mount
//! siblings stay in waterui as unstyled tests.

use hydrolysis_m3::Material3;
use waterui::Str;
use waterui::component::text;
use waterui::navigation::NavigationView;
use waterui_testing::{Role, ui};

/// The issue's first shape: a titled navigation page that binds no subtitle.
fn inbox_view() -> NavigationView {
    NavigationView::new("Inbox", text("mail body"))
}

#[test]
fn navigation_without_subtitle_emits_no_subtitle_node_on_offscreen_mount() {
    let mut app = ui()
        .viewport(390, 844)
        .theme(Material3::defaults())
        .mount_offscreen(inbox_view);
    app.settle();
    let bar = app.query().role(Role::NAVIGATION).single();
    app.query()
        .role(Role::HEADER)
        .label("Inbox")
        .children_of(&bar)
        .assert_exists();
    assert!(
        !app.query().role(Role::LABEL).children_of(&bar).exists(),
        "an unbound subtitle must emit no Label child under the bar node"
    );
}

/// An empty `Str` leaf names nothing: like a decorative graphics leaf it
/// emits no node until there is a label to speak.
#[test]
fn empty_text_leaf_emits_no_node_on_offscreen_mount() {
    let mut app = ui()
        .viewport(390, 844)
        .theme(Material3::defaults())
        .mount_offscreen(|| Str::from(""));
    app.settle();
    assert!(
        !app.query().role(Role::LABEL).exists(),
        "an empty text leaf must emit no Label node"
    );
}
