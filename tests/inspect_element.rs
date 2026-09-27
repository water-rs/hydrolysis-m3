//! Right-clicking a view's context menu offers to inspect it in a debug build.
//!
//! Reached the way a user reaches it: press the secondary button and read the
//! menu. The menu opens in its own window, so this also exercises the harness
//! seeing more than the main one.

use hydrolysis_m3::Material3;
use waterui::prelude::*;
use waterui_testing::ui as test_ui;

/// A plain view opens no menu at all: hydrolysis extends a menu a view already
/// declared — it never creates one on a bare secondary press.
#[core::prelude::v1::test]
fn right_clicking_a_plain_view_opens_no_menu() {
    let mut app = test_ui()
        .viewport(320, 200)
        .theme(Material3::defaults())
        .mount_offscreen(|| text("Hello").padding_with(EdgeInsets::all(20.0)));

    app.query().label("Inspect element").assert_not_exists();

    app.secondary_click_at(40.0, 30.0);

    app.query().label("Inspect element").assert_not_exists();
}

/// An application's own menu keeps its items: the inspector entry is appended
/// after what the application put there, not instead of it.
#[core::prelude::v1::test]
fn an_application_menu_keeps_its_own_items() {
    let mut app = test_ui()
        .viewport(320, 200)
        .theme(Material3::defaults())
        .mount_offscreen(|| {
            text("Hello")
                .padding_with(EdgeInsets::all(20.0))
                .context_menu(vec!["Rename".action(|| {})])
        });

    app.secondary_click_at(40.0, 30.0);

    app.query().label("Rename").assert_exists();
    app.query().label("Inspect element").assert_exists();
}
