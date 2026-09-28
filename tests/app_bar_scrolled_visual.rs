//! `md.comp.app-bar.scrolled.*` — the top app bar's scrolled state.
//!
//! `material_scrolled_app_bar` drives the bar's container color from a
//! `ScrollView::report_offset` binding: `surface` at rest, `surface-container`
//! once content scrolls under the bar. These are visual acceptance captures —
//! run with `--ignored` to write the PNGs and review the images directly.

use std::time::Duration;

use hydrolysis_m3::{Material3, material_navigation_view, material_scrolled_app_bar};
use waterui::layout::Point;
use waterui::prelude::*;
use waterui::reactive::binding;
use waterui_testing::{Styled, UiBuilder};

fn mail() -> impl View {
    let offset = binding(Point::zero());
    let rows = (1..=40)
        .map(|index| text(format!("Message {index}")))
        .collect::<Vec<_>>();
    let content = scroll(vstack(rows).leading().spacing(4.0).padding_with(12.0))
        .report_offset(&offset)
        .a11y_label("message list");
    material_scrolled_app_bar(material_navigation_view("Inbox", content), &offset)
}

#[ignore = "writes a visual acceptance PNG for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (420, 520))]
fn app_bar_lifts_once_content_scrolls_under_it(ui: UiBuilder<Styled<Material3>>) {
    let mut app = ui.mount_offscreen(mail);
    app.settle();
    let _ = app.capture_snapshot("app-bar-scrolled", "gallery", "rest");

    app.query().label("message list").scroll_down();
    app.pump_for(Duration::from_millis(400));
    let _ = app.capture_snapshot("app-bar-scrolled", "gallery", "scrolled");
}
