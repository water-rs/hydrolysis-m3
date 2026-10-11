//! A picker announces the label it was constructed with.
//!
//! `PickerConfig::label` is mandatory precisely so assistive technology has a
//! name to announce, but the self-drawn renderer never read it: the menu style
//! announced the *selected option* as the control's name — the same string it
//! already reports as the value — and the radio and segmented styles announced
//! nothing at all. A screen reader could tell you "Medium" without ever saying
//! what "Medium" was choosing.

#![expect(
    clippy::needless_pass_by_value,
    reason = "the binding is moved into the mounted body, which must own it"
)]

use hydrolysis_m3::{Material3, MaterialColorScheme};
use waterui::component::vstack;
use waterui::form::{Picker, PickerItem, PickerStyle};
use waterui::prelude::*;
use waterui::reactive::binding;
use waterui_core::AnyView;
use waterui_form::picker::picker;
use waterui_testing::{UiBuilder, ui};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    const ALL: [Self; 3] = [Self::Small, Self::Medium, Self::Large];

    const fn name(self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Medium => "Medium",
            Self::Large => "Large",
        }
    }
}

/// Mounts a picker whose label names the control rather than the choice.
///
/// The binding is owned here because the mounted body must outlive the call,
/// and `Picker::new` only borrows it to build the config.
fn sized_picker(selection: Binding<Size>, style: PickerStyle) -> impl View {
    let items: Vec<PickerItem<Size>> = Size::ALL
        .into_iter()
        .map(|size| PickerItem::new(size, text::text(size.name())))
        .collect();
    Picker::new("Shirt size", items, &selection).style(style)
}

/// The menu style names the control and reports the selection as its value.
/// Those are two different strings and it used to give the selection for both.
#[waterui::test(viewport = (400, 300))]
fn a_menu_picker_announces_its_own_label(ui: UiBuilder) {
    let selection = binding(Size::Medium);
    let mut app = ui.mount(move || sized_picker(selection.clone(), PickerStyle::Menu));
    app.settle();

    let _ = app.query().label("Shirt size").single();
}

/// The radio style groups its options, and the group is what carries the name.
#[waterui::test(viewport = (400, 300))]
fn a_radio_picker_announces_its_own_label(ui: UiBuilder) {
    let selection = binding(Size::Small);
    let mut app = ui.mount(move || sized_picker(selection.clone(), PickerStyle::Radio));
    app.settle();

    let _ = app.query().label("Shirt size").single();
}

// ---------------------------------------------------------------------------
// Rendered evidence: the label's placement and styling under Material 3.
// Moved out of hydrolysis's own suite (water-rs/hydrolysis-m3#113): the
// assertions mount through the styled builder, so they live beside the theme
// that draws them.
// ---------------------------------------------------------------------------

fn labelled_menu_picker() -> impl Fn() -> AnyView {
    let selection = binding(0i32);
    move || {
        AnyView::new(
            vstack((picker(
                "Size",
                vec![text::text("Small").tag(0i32), text::text("Large").tag(1i32)],
                &selection,
            )
            .style(PickerStyle::Menu),))
            .padding(),
        )
    }
}

#[test]
fn menu_picker_label_material3_snapshots() {
    let mut light = ui()
        .viewport(360, 140)
        .theme(Material3::with_colors(MaterialColorScheme::baseline_light()))
        .mount_offscreen(labelled_menu_picker());
    light.capture_snapshot("hydrolysis", "menu-picker-label", "light");

    let mut dark = ui()
        .viewport(360, 140)
        .theme(Material3::dark())
        .mount_offscreen(labelled_menu_picker());
    dark.capture_snapshot("hydrolysis", "menu-picker-label", "dark");
}

fn labelled_group_picker(style: PickerStyle, hide_label: bool) -> impl Fn() -> AnyView {
    let selection = binding(0i32);
    move || {
        let group = picker(
            "Size",
            vec![text::text("Small").tag(0i32), text::text("Large").tag(1i32)],
            &selection,
        )
        .style(style);
        let group = if hide_label {
            group.hide_label()
        } else {
            group
        };
        AnyView::new(vstack((group,)).padding())
    }
}

/// Visual evidence for water-rs/hydrolysis#101: the radio group's heading
/// sits above the option rows, styled like the menu field's label.
#[test]
fn radio_picker_label_material3_snapshots() {
    let mut light = ui()
        .viewport(360, 200)
        .theme(Material3::with_colors(MaterialColorScheme::baseline_light()))
        .mount_offscreen(labelled_group_picker(PickerStyle::Radio, false));
    light.capture_snapshot("hydrolysis", "radio-picker-label", "light");

    let mut dark = ui()
        .viewport(360, 200)
        .theme(Material3::dark())
        .mount_offscreen(labelled_group_picker(PickerStyle::Radio, false));
    dark.capture_snapshot("hydrolysis", "radio-picker-label", "dark");
}

/// Visual evidence for water-rs/hydrolysis#101: the segmented group's heading
/// sits above the segment row, styled like the menu field's label.
#[test]
fn segmented_picker_label_material3_snapshots() {
    let mut light = ui()
        .viewport(360, 120)
        .theme(Material3::with_colors(MaterialColorScheme::baseline_light()))
        .mount_offscreen(labelled_group_picker(PickerStyle::Segmented, false));
    light.capture_snapshot("hydrolysis", "segmented-picker-label", "light");

    let mut dark = ui()
        .viewport(360, 120)
        .theme(Material3::dark())
        .mount_offscreen(labelled_group_picker(PickerStyle::Segmented, false));
    dark.capture_snapshot("hydrolysis", "segmented-picker-label", "dark");

    // The same picker with a hidden label: the segment row keeps the exact
    // height it had before the label work, so the pair can be judged by eye.
    let mut nolabel = ui()
        .viewport(360, 120)
        .theme(Material3::with_colors(MaterialColorScheme::baseline_light()))
        .mount_offscreen(labelled_group_picker(PickerStyle::Segmented, true));
    nolabel.capture_snapshot("hydrolysis", "segmented-picker-nolabel", "light");
}
