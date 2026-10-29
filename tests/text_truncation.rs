//! Line-limited text under Material 3: a squeezed line ends in a real
//! ellipsis — three separated baseline ink dots — rather than a mid-glyph
//! clip (water-rs/hydrolysis#141), a limit reserves height only for the
//! lines it keeps, and button labels fold to one truncated line instead of
//! wrapping.
//!
//! Moved out of hydrolysis's own suite under water-rs/hydrolysis-m3#113: the
//! checks read rendered pixels or text-line metrics, so they live beside the
//! theme that draws the text. The pixel heuristic is font-independent — it
//! looks for the ellipsis's low-ink column pattern, not a particular face's
//! metrics.

mod support {
    /// Converts a `f32` coordinate to `usize` with `as` saturating truncation.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "test coordinates are non-negative and within usize range"
    )]
    pub const fn f32_as_usize(v: f32) -> usize {
        v as usize
    }
}

use waterui::prelude::*;
use waterui_controls::button;
use waterui_testing::{OffscreenApp, Styled, UiBuilder};

const LONG: &str = "A label long enough to wrap into several lines at this width";

/// A limited text reserves height for its visible lines only.
///
/// Origin: waterui `tests/text_line_limit.rs`.
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (600, 600))]
fn a_line_limit_caps_the_reserved_height(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let mut app = ui.mount_offscreen(|| {
        vstack((
            text(LONG).a11y_label("unlimited").width(150.0),
            text(LONG)
                .line_limit(core::num::NonZeroUsize::MIN)
                .a11y_label("limited")
                .width(150.0),
        ))
    });

    let unlimited = app.query().label("unlimited").single().bounds();
    let limited = app.query().label("limited").single().bounds();
    assert!(
        unlimited.height() > limited.height() * 2.0,
        "the unlimited text wraps ({}) while the limited one stays one line ({})",
        unlimited.height(),
        limited.height(),
    );
}

/// Compressed buttons keep their labels on one line instead of folding them
/// into paragraphs — the webview example's toolbar rendered "Back" as
/// "Bac / k" before button labels defaulted to a single truncated line.
///
/// Origin: waterui `tests/text_line_limit.rs`.
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (600, 600))]
fn compressed_button_labels_stay_on_one_line(ui: UiBuilder<Styled<hydrolysis_m3::Material3>>) {
    let mut app = ui.mount_offscreen(|| {
        hstack((
            button("Back").action(|| {}),
            button("Forward").action(|| {}),
            button("Reload").action(|| {}),
            button("Stop").action(|| {}),
        ))
        .width(250.0)
    });

    let mut heights = Vec::new();
    for label in ["Back", "Forward", "Reload", "Stop"] {
        heights.push(app.query().label(label).single().bounds().height());
    }
    let min = heights.iter().copied().fold(f32::INFINITY, f32::min);
    let max = heights.iter().copied().fold(0.0_f32, f32::max);
    assert!(
        (max - min) < 1.0,
        "buttons disagree about line count, so a label folded: {heights:?}"
    );
}

fn truncated_row_view() -> impl View {
    vstack((
        hstack((
            text("On my way — see you at the gate in about ten minutes")
                .line_limit(core::num::NonZeroUsize::MIN)
                .a11y_label("preview"),
            text("2").a11y_label("tight"),
        ))
        .spacing(4.0),
        hstack((text("x").a11y_label("short"), text("2").a11y_label("free"))).spacing(4.0),
    ))
    .spacing(8.0)
    .leading()
    .padding_with((6.0, 10.0))
}

#[waterui::test(truncated_row_view, theme = hydrolysis_m3::Material3::defaults(), offscreen, viewport = (220, 120))]
fn a_limited_text_truncates_with_an_ellipsis_beside_its_sibling(app: &mut OffscreenApp) {
    let preview = app.query().label("preview").single().bounds();
    let tight = app.query().label("tight").single().bounds();
    let free = app.query().label("free").single().bounds();

    assert!(
        (tight.width() - free.width()).abs() <= 1.0,
        "the sibling keeps the width it takes unconstrained: tight={tight:?} free={free:?}"
    );
    assert!(
        preview.x() + preview.width() <= tight.x(),
        "text and sibling share the row without overlapping: preview={preview:?} tight={tight:?}"
    );

    // The respelled line ends in "…" — three separated ink dots riding the
    // baseline. Walk left from the leaf's last inked column while columns
    // carry ink only below the midline; a clipped line instead ends
    // mid-glyph with tall ink at its trailing edge.
    let snapshot = app.snapshot();
    let x0 = support::f32_as_usize(preview.x().max(0.0));
    let x1 = (support::f32_as_usize(preview.x() + preview.width())).min(snapshot.width() as usize);
    let y0 = support::f32_as_usize(preview.y().max(0.0));
    let y1 =
        (support::f32_as_usize(preview.y() + preview.height())).min(snapshot.height() as usize);
    let mid = y0 + (y1 - y0) / 2;
    let bg: [u8; 4] = snapshot.rgba8()
        [(y0 * snapshot.width() as usize + x0.saturating_sub(6)) * 4..][..4]
        .try_into()
        .unwrap();
    let ink = |x: usize, y: usize| -> bool {
        let px: [u8; 4] = snapshot.rgba8()[(y * snapshot.width() as usize + x) * 4..][..4]
            .try_into()
            .unwrap();
        (0..3).any(|c| px[c].abs_diff(bg[c]) > 48)
    };

    let last_ink_x = (x0..x1)
        .rev()
        .find(|&x| (y0..y1).any(|y| ink(x, y)))
        .expect("the truncated text drew ink");
    let tall = |x: usize| (y0..mid).any(|y| ink(x, y));
    let low = |x: usize| (mid..y1).any(|y| ink(x, y));
    assert!(
        !tall(last_ink_x),
        "the leaf's trailing ink should sit on the baseline — a tall trailing column is a clipped glyph: {preview:?}"
    );
    let zone_x0 = (x0..last_ink_x)
        .rev()
        .find(|&x| tall(x))
        .map_or(x0, |x| x + 1);

    let mut runs = 0usize;
    let mut in_run = false;
    for x in zone_x0..=last_ink_x {
        if low(x) && !in_run {
            runs += 1;
        }
        in_run = low(x);
    }
    assert!(
        runs >= 3,
        "the truncated line should end in a '…' — {runs} baseline ink runs in its tail: {preview:?}"
    );
}
