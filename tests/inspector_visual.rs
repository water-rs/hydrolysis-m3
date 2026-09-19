//! Visual acceptance for the Inspector's panes under Material 3.
//!
//! Each pane is mounted against the same populated [`Model`] the previews use,
//! so what is reviewed is the real view code driven by real protocol types
//! rather than a mock. The PNG-producing tests are ignored by default and
//! reviewed by eye.

use hydrolysis_m3::Material3;
use waterui_inspector_app::model::Section;
use waterui_inspector_app::{connection, preview_model, ui};
use waterui_testing::{OffscreenApp, ui as test_ui};

fn save(app: &mut OffscreenApp, case: &str) {
    let _ = app.capture_snapshot("material3-preview", case, "default");
}

/// Mounts one pane at a size that shows its whole layout.
fn pane(case: &str, width: u32, height: u32, section: Section) {
    let mut app = test_ui()
        .viewport(width, height)
        .theme(Material3::defaults())
        .mount_offscreen(move || {
            let model = preview_model();
            let (sender, _receiver) = connection::subscription_channel();
            ui::pane(section, model, sender)
        });
    save(&mut app, case);
}

// Moved from waterui's components/devtools/inspector/app/tests/visual.rs.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn overview_pane() {
    pane("inspector-overview", 900, 700, Section::Overview);
}

// Moved from waterui's components/devtools/inspector/app/tests/visual.rs.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn frames_pane() {
    pane("inspector-frames", 900, 700, Section::Frames);
}

// Moved from waterui's components/devtools/inspector/app/tests/visual.rs.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn tree_pane() {
    pane("inspector-tree", 900, 700, Section::Tree);
}

// Moved from waterui's components/devtools/inspector/app/tests/visual.rs.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn tasks_pane() {
    pane("inspector-tasks", 900, 700, Section::Tasks);
}

// Moved from waterui's components/devtools/inspector/app/tests/visual.rs.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn logs_pane() {
    pane("inspector-logs", 900, 700, Section::Logs);
}

// Moved from waterui's components/devtools/inspector/app/tests/visual.rs.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn signals_pane() {
    pane("inspector-signals", 900, 700, Section::Signals);
}

/// The whole window, sidebar included.
// Moved from waterui's components/devtools/inspector/app/tests/visual.rs.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn full_window() {
    let mut app = test_ui()
        .viewport(1200, 800)
        .theme(Material3::defaults())
        .mount_offscreen(move || {
            let model = preview_model();
            let (sender, _receiver) = connection::subscription_channel();
            ui::inspector(model, sender)
        });
    save(&mut app, "inspector-window");
}
