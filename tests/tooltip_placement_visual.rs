//! Visual acceptance captures for issue #93: a plain tooltip on a top-edge
//! target and a rich tooltip on a bottom-edge target, light and dark, at 2x.
//!
//! `WATERUI_TEST_ARTIFACTS_DIR` controls where the PNGs land; each capture is
//! a `before`/`after` pair so a future fix renders into the same layout.

use std::time::Duration;

use hydrolysis_m3::{Material3, MaterialColorScheme, plain_tooltip, rich_tooltip};
use waterui::prelude::{PositionExt as _, UnitPoint, absolute};
use waterui_controls::button;
use waterui_core::View;
use waterui_testing::UiBuilder;

const PLAIN_LABEL: &str = "Adds this item to your favorites";
const RICH_SUBHEAD: &str = "Grant permission";
const RICH_TEXT: &str = "Allow the app to add this item to your favorites";

fn scene(content: impl View, position: UnitPoint) -> impl View {
    absolute((content.position_in(position),))
}

fn capture<F, V>(theme: Material3, build: F, case: &str, stage: &str)
where
    F: Fn() -> V + 'static,
    V: View + 'static,
{
    let mut app = UiBuilder::new()
        .theme(theme)
        .scale_factor(2.0)
        .viewport(320, 180)
        .mount_offscreen(build);
    app.press_named_key("Tab");
    app.pump_for(Duration::from_millis(400));
    app.capture_snapshot("tooltip-placement", case, stage);
}

/// Before-state captures on `dev`: the plain tooltip above the top-edge
/// target is clipped by the scene frame and the rich tooltip below the
/// bottom-edge target spills past the frame. The targets sit a few logical
/// pixels inside the edges — like the issue's top tab strip repro — so the
/// clipped popup stays partially visible. Re-run these after the placement
/// fix lands.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn tooltip_placement_before_captures() {
    capture(
        Material3::defaults(),
        || {
            scene(
                plain_tooltip(PLAIN_LABEL).for_target(button("+").action(|| {})),
                UnitPoint::new(0.5, 0.1),
            )
        },
        "top-edge-plain-light",
        "before",
    );
    capture(
        Material3::with_colors(MaterialColorScheme::baseline_dark()),
        || {
            scene(
                plain_tooltip(PLAIN_LABEL).for_target(button("+").action(|| {})),
                UnitPoint::new(0.5, 0.1),
            )
        },
        "top-edge-plain-dark",
        "before",
    );
    capture(
        Material3::defaults(),
        || {
            scene(
                rich_tooltip(RICH_SUBHEAD, RICH_TEXT).for_target(button("+").action(|| {})),
                UnitPoint::new(0.5, 0.9),
            )
        },
        "bottom-edge-rich-light",
        "before",
    );
    capture(
        Material3::with_colors(MaterialColorScheme::baseline_dark()),
        || {
            scene(
                rich_tooltip(RICH_SUBHEAD, RICH_TEXT).for_target(button("+").action(|| {})),
                UnitPoint::new(0.5, 0.9),
            )
        },
        "bottom-edge-rich-dark",
        "before",
    );
}
