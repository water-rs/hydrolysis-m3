//! Visual acceptance for the `Code` widget under Material 3 light and dark.
//!
//! A code block draws its chrome — the header, the Copy action, the syntax
//! palette — entirely from environment tokens, so the same view is captured
//! against the baseline light scheme and the baseline dark scheme.

use waterui::View;
use waterui::ViewExt as _;
use waterui::text::code;
use waterui_testing::{OffscreenApp, Role};

fn code_block() -> impl View {
    code("rust", include_str!("fixtures/code_sample.rs")).padding_with(16.0)
}

fn assert_code_block_semantics(app: &mut OffscreenApp) {
    app.query().role(Role::LABEL).label("Rust").assert_exists();
    app.query().role(Role::LABEL).label("Copy").assert_exists();
}

// Moved from waterui's components/foundation/text/tests/e2e_visual.rs.
#[waterui::test(
    code_block,
    theme = hydrolysis_m3::Material3::defaults(),
    viewport = (480, 300),
    offscreen
)]
fn a_code_block_draws_from_the_light_theme(app: &mut OffscreenApp) {
    assert_code_block_semantics(app);
    let _ = app.capture_snapshot("material3-preview", "code-block", "light");
}

// Moved from waterui's components/foundation/text/tests/e2e_visual.rs.
#[waterui::test(
    code_block,
    theme = hydrolysis_m3::Material3::with_colors(hydrolysis_m3::MaterialColorScheme::baseline_dark()),
    viewport = (480, 300),
    offscreen
)]
fn a_code_block_draws_from_the_dark_theme(app: &mut OffscreenApp) {
    assert_code_block_semantics(app);
    let _ = app.capture_snapshot("material3-preview", "code-block", "dark");
}
