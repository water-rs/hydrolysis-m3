//! Visual acceptance captures for issue #93: a plain tooltip on a top-edge
//! target and a rich tooltip on a bottom-edge target, light and dark, at 2x,
//! plus the hydroterm-style repro — a `+` button in a top tab strip, hovered.
//!
//! `WATERUI_TEST_ARTIFACTS_DIR` controls where the PNGs land; `before` stages
//! show the unflipped placement on `dev`, `after` stages the anchored-overlay
//! placement.

use std::time::Duration;

use hydrolysis_m3::{Material3, MaterialColorScheme, icon_button, plain_tooltip, rich_tooltip};
use waterui::component::{hstack, spacer, text, vstack};
use waterui::layout::padding::EdgeInsets;
use waterui::prelude::{PositionExt as _, UnitPoint, ViewExt as _, absolute};
use waterui_controls::button;
use waterui_core::View;
use waterui_testing::{OffscreenApp, Role, UiBuilder};

const PLAIN_LABEL: &str = "Adds this item to your favorites";
const RICH_SUBHEAD: &str = "Grant permission";
const RICH_TEXT: &str = "Allow the app to add this item to your favorites";

fn scene(content: impl View, position: UnitPoint) -> impl View {
    absolute((content.position_in(position),))
}

fn mount<F, V>(theme: Material3, build: F) -> OffscreenApp
where
    F: Fn() -> V + 'static,
    V: View + 'static,
{
    UiBuilder::new()
        .theme(theme)
        .scale_factor(2.0)
        .viewport(320, 180)
        .mount_offscreen(build)
}

fn capture_hover(app: &mut OffscreenApp, target: &str, case: &str, stage: &str) {
    app.query().role(Role::BUTTON).label(target).hover();
    app.pump_for(Duration::from_millis(400));
    app.capture_snapshot("tooltip-placement", case, stage);
}

/// The edge cases from the issue: a plain tooltip above a top-edge target
/// (clipped before the fix, flipped below after) and a rich tooltip below a
/// bottom-edge target (spilling before, flipped above after). The targets
/// sit a few logical pixels inside the edges — like the issue's top tab
/// strip — so the clipped `before` popup stays partially visible.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn tooltip_placement_captures() {
    let light = Material3::defaults();
    let dark = Material3::with_colors(MaterialColorScheme::baseline_dark());
    for (theme, scheme) in [(light, "light"), (dark, "dark")] {
        let mut app = mount(theme.clone(), || {
            scene(
                plain_tooltip(PLAIN_LABEL).for_target(button("+").action(|| {})),
                UnitPoint::new(0.5, 0.1),
            )
        });
        capture_hover(&mut app, "+", &format!("top-edge-plain-{scheme}"), "after");

        let mut app = mount(theme.clone(), || {
            scene(
                rich_tooltip(RICH_SUBHEAD, RICH_TEXT).for_target(button("+").action(|| {})),
                UnitPoint::new(0.5, 0.9),
            )
        });
        capture_hover(
            &mut app,
            "+",
            &format!("bottom-edge-rich-{scheme}"),
            "after",
        );
    }
}

/// The issue's hydroterm repro: a `+` icon button in a tab strip pinned to
/// the top of the window, hovered — the plain tooltip must appear below the
/// button, inside the window.
#[test]
#[ignore = "writes visual acceptance PNG files for direct image review"]
fn tooltip_tab_strip_captures() {
    let light = Material3::defaults();
    let dark = Material3::with_colors(MaterialColorScheme::baseline_dark());
    for (theme, scheme) in [(light, "light"), (dark, "dark")] {
        let mut app = mount(theme.clone(), || {
            vstack((
                hstack((
                    text("Tab 1"),
                    text("Tab 2"),
                    plain_tooltip("New tab")
                        .for_target(icon_button("New tab", text("+")).action(|| {})),
                ))
                .spacing(8.0)
                .padding_with(EdgeInsets::all(8.0)),
                spacer(),
            ))
            .spacing(0.0)
        });
        capture_hover(&mut app, "New tab", &format!("tab-strip-{scheme}"), "after");
    }
}
