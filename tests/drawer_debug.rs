//! Debug dump for the modal navigation drawer semantics failure.

use hydrolysis_m3::{navigation_drawer, navigation_drawer_item};
use waterui::component::text;
use waterui::{Binding, Str};
use waterui_core::View;
use waterui_testing::UiBuilder;
use hydrolysis_m3::Material3;
use waterui_testing::Styled;

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (360, 320))]
fn dump_modal_drawer_nodes(ui: UiBuilder<Styled<Material3>>) {
    let opened = Binding::bool(true);
    let opened_for_view = opened.clone();
    let mut app = ui.mount_offscreen(move || {
        navigation_drawer(
            &opened_for_view,
            navigation_drawer_item("Inbox", text("I"), &Binding::bool(true)),
        )
        .modal()
        .close_on_escape()
        .close_on_overlay_click()
        .overlay_action(move || {})
    });

    for (id, node) in app.tree().nodes() {
        eprintln!(
            "node {:?}: role={:?} label={:?} expanded={:?} hidden={} actions={:?}",
            id,
            node.role(),
            node.label(),
            node.expanded(),
            node.hidden(),
            node.actions()
        );
    }
    panic!("dump complete");
}
