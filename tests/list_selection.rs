//! Selection chrome on `List` rows.
//!
//! The self-drawn renderer ignored row selection outright. The row's
//! own content already flipped to `SelectionForeground` — that part is the
//! list component's job and always worked — but nothing painted the container
//! behind it and nothing told the accessibility tree, so a sidebar had no way
//! to show which row was current.

use hydrolysis_m3::Material3;
use waterui::component::list::{List, ListItem};
use waterui::id::SelfId;
use waterui::prelude::*;
use waterui::reactive::binding;
use waterui_testing::{Role, Styled, UiBuilder};

const LABELS: [&str; 3] = ["First", "Second", "Third"];

#[expect(
    clippy::needless_pass_by_value,
    reason = "the binding is cloned into the mounted body, which must own it"
)]
fn selectable_list(current: Binding<Option<SelfId<usize>>>) -> impl View {
    let row = |index: usize| move || ListItem::new(text(LABELS[index]));
    List::content((row(0), row(1), row(2))).selection(&current)
}

/// Exactly the selected row reports itself selected, and the flag follows the
/// signal without the list being rebuilt.
#[waterui::test(viewport = (400, 300))]
fn only_the_selected_row_is_marked_selected(ui: UiBuilder) {
    let current = binding(Some(SelfId::new(1usize)));
    let mut app = ui.mount({
        let current = current.clone();
        move || selectable_list(current.clone())
    });
    app.settle();

    let _ = app
        .query()
        .role(Role::LIST_ITEM)
        .label("First")
        .selected(false)
        .single();
    let _ = app
        .query()
        .role(Role::LIST_ITEM)
        .label("Second")
        .selected(true)
        .single();
    let _ = app
        .query()
        .role(Role::LIST_ITEM)
        .label("Third")
        .selected(false)
        .single();

    current.set(Some(SelfId::new(2usize)));
    app.settle();

    let _ = app
        .query()
        .role(Role::LIST_ITEM)
        .label("Second")
        .selected(false)
        .single();
    let _ = app
        .query()
        .role(Role::LIST_ITEM)
        .label("Third")
        .selected(true)
        .single();
}

/// Visual acceptance: the selected row carries the theme's selection fill and
/// its content reads against that fill, while its neighbours are untouched.
#[ignore = "writes a visual acceptance PNG for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (400, 300))]
fn the_selected_row_shows_its_selection_fill(ui: UiBuilder<Styled<Material3>>) {
    let current = binding(Some(SelfId::new(1usize)));
    let mut app = ui.mount_offscreen(move || selectable_list(current.clone()));
    let _ = app.capture_snapshot("material3-preview", "list-selection", "second-selected");
}
