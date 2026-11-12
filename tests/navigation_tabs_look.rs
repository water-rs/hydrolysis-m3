//! Visual evidence for the navigation bar and tabs looks: the pushed page's
//! back button at rest, hovered and pressed, and the tab bar's highlight at
//! rest, part way through a selection change and settled, each in the light
//! and the dark scheme.

use core::time::Duration;

use hydrolysis_m3::{Material3, MaterialColorScheme, material_tab, material_tabs};
use waterui::component::vstack;
use waterui::prelude::*;
use waterui::reactive::binding;
use waterui_navigation::{NavigationLink, NavigationStack, NavigationView};
use waterui_testing::{OffscreenApp, Role, ui};

/// The pushed page's back button centre in the 360 by 200 viewport: the
/// 40 point button sits 4 points in and 12 points down.
const BACK: (f32, f32) = (24.0, 32.0);

fn schemes() -> [(&'static str, MaterialColorScheme); 2] {
    [
        ("light", MaterialColorScheme::baseline_light()),
        ("dark", MaterialColorScheme::baseline_dark()),
    ]
}

/// A navigation stack pushed onto its detail page, so the bar shows a back
/// button.
fn pushed(scheme: MaterialColorScheme) -> OffscreenApp {
    let mut app = ui()
        .viewport(360, 200)
        .theme(Material3::with_colors(scheme))
        .mount_offscreen(|| {
            NavigationStack::new(NavigationView::new(
                "Library",
                vstack((NavigationLink::new("Open Atlas", || {
                    NavigationView::new("Atlas", text("Atlas destination"))
                }),)),
            ))
        });
    app.query().role(Role::BUTTON).label("Open Atlas").tap();
    app.pump_for(Duration::from_millis(800));
    app
}

#[test]
fn navigation_back_button_material3_snapshots() {
    for (scheme_name, scheme) in schemes() {
        let stage = |state: &str| format!("{scheme_name}-{state}");
        let mut app = pushed(scheme);
        let _ = app.capture_snapshot("hydrolysis", "navigation-back-button", &stage("rest"));

        app.queue_pointer_move(BACK.0, BACK.1);
        app.pump_for(Duration::from_millis(600));
        let _ = app.capture_snapshot("hydrolysis", "navigation-back-button", &stage("hovered"));

        app.queue_pointer_down(BACK.0, BACK.1);
        app.pump_for(Duration::from_millis(600));
        let _ = app.capture_snapshot("hydrolysis", "navigation-back-button", &stage("pressed"));
    }
}

#[test]
fn tabs_selection_material3_snapshots() {
    for (scheme_name, scheme) in schemes() {
        let stage = |state: &str| format!("{scheme_name}-{state}");
        let selection = binding(0usize);
        let for_view = selection.clone();
        let mut app = ui()
            .viewport(360, 200)
            .theme(Material3::with_colors(scheme))
            .mount_offscreen(move || {
                material_tabs(
                    &for_view,
                    vec![
                        material_tab(label("Photos").icon(text("*")), || {
                            NavigationView::new("Photos", text("photos content"))
                        }),
                        material_tab(label("Albums").icon(text("*")), || {
                            NavigationView::new("Albums", text("albums content"))
                        }),
                        material_tab(label("Places").icon(text("*")), || {
                            NavigationView::new("Places", text("places content"))
                        }),
                    ],
                )
            });
        app.pump_for(Duration::from_millis(800));
        let _ = app.capture_snapshot("hydrolysis", "tabs-selection", &stage("rest"));

        // The highlight grows into the new item while it fades out of the
        // old one.
        selection.set(1);
        app.pump_for(Duration::from_millis(70));
        let _ = app.capture_snapshot("hydrolysis", "tabs-selection", &stage("mid-transition"));

        app.pump_for(Duration::from_millis(800));
        let _ = app.capture_snapshot("hydrolysis", "tabs-selection", &stage("settled"));
    }
}
