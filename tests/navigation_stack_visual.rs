//! Offscreen visual stages for retained navigation motion and Material chrome.

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use hydrolysis_m3::Material3;
use waterui::layout::stack::vstack;
use waterui::text::Text;
use waterui_navigation::{NavigationLink, NavigationStack, NavigationView};
use waterui_testing::{Role, Styled, UiBuilder};

const VIEWPORT_WIDTH: u16 = 390;
const VIEWPORT_HEIGHT: u16 = 844;

fn visual_stack(destination_appeared: Rc<Cell<bool>>) -> impl waterui::View {
    NavigationStack::new(NavigationView::new(
        "Library",
        vstack((
            Text::new("Recently viewed"),
            NavigationLink::new("Open Atlas", move || {
                let destination_appeared = Rc::clone(&destination_appeared);
                NavigationView::new("Atlas", Text::new("Atlas destination"))
                    .on_navigation_appear(move || destination_appeared.set(true))
            }),
        )),
    ))
}

// Moved from waterui's components/foundation/navigation/tests/e2e_visual.rs.
#[waterui::test(
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (390, 844)
)]
fn navigation_stack_exports_root_destination_and_interactive_pop_stages(
    ui: UiBuilder<Styled<Material3>>,
) {
    let destination_appeared = Rc::new(Cell::new(false));
    let destination_appeared_for_view = Rc::clone(&destination_appeared);
    let mut app =
        ui.mount_offscreen(move || visual_stack(Rc::clone(&destination_appeared_for_view)));

    app.query().label("Library").assert_exists();
    let _root = app.capture_snapshot("navigation", "stack", "root");

    app.query().role(Role::BUTTON).label("Open Atlas").tap();
    let deadline = Instant::now() + Duration::from_secs(2);
    while !destination_appeared.get() {
        assert!(
            Instant::now() < deadline,
            "navigation destination did not finish appearing"
        );
        let _ = app.snapshot();
    }
    let _destination = app.capture_snapshot("navigation", "stack", "destination");

    app.queue_pointer_down(2.0, f32::from(VIEWPORT_HEIGHT) * 0.5);
    let _ = app.snapshot();
    app.queue_pointer_move(
        f32::from(VIEWPORT_WIDTH) * 0.45,
        f32::from(VIEWPORT_HEIGHT) * 0.5,
    );
    let _interactive = app.capture_snapshot("navigation", "stack", "interactive-pop-progress");
    app.queue_pointer_up(
        f32::from(VIEWPORT_WIDTH) * 0.45,
        f32::from(VIEWPORT_HEIGHT) * 0.5,
    );
    let _ = app.snapshot();
}
