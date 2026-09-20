//! Visual acceptance for non-Latin text under Material 3.
//!
//! Hydrolysis shapes with `parley` over the font collection the host installs.
//! A collection that resolves a family but never asks the platform for a face
//! covering the run's script maps every cluster outside that family to
//! `.notdef` — glyph 0, which draws as an empty box — so an application whose
//! content is Chinese, Japanese, Korean, Arabic, Hebrew, Thai or Devanagari
//! renders as rows of tofu while Latin and Cyrillic look fine (waterui#426).
//!
//! The counted half of that statement is asserted without pixels, next to the
//! collection itself, in `hydrolysis`'s `runner::fonts` tests: no cluster of
//! any of these samples may shape to glyph 0. This is the half a reader has to
//! confirm, because a glyph id says nothing about whether the shape drawn is
//! the letter — so the gallery is ignored by default and reviewed by eye.

use hydrolysis_m3::Material3;
use waterui::component::{hstack, vstack};
use waterui::prelude::*;
use waterui_testing::{OffscreenApp, Styled, UiBuilder};

/// One line per script, each labelled in Latin so a reviewer can tell which
/// row failed without being able to read the row itself.
///
/// The samples are words, not lone codepoints: a script whose shaping depends
/// on its neighbours — Arabic's joining forms, Devanagari's conjunct and
/// reordered vowel, Thai's stacked marks — only shows a broken font stack when
/// there is more than one cluster to join.
const SAMPLES: &[(&str, &str)] = &[
    ("Latin", "Ada Lovelace"),
    ("Cyrillic", "Ольга Ладыженская"),
    ("Han (ja)", "山田 太郎"),
    ("Han (zh)", "北京欢迎你"),
    ("Hangul", "안녕하세요"),
    ("Arabic", "مرحبا بالعالم"),
    ("Hebrew", "שלום עולם"),
    ("Thai", "สวัสดีชาวโลก"),
    ("Devanagari", "नमस्ते दुनिया"),
];

fn gallery() -> impl View {
    vstack(
        SAMPLES
            .iter()
            .map(|(script, sample)| {
                hstack((
                    text(*script).width(120.0),
                    text(*sample).font(waterui::text::font::Title),
                ))
                .spacing(16.0)
            })
            .collect::<Vec<_>>(),
    )
    .spacing(8.0)
    .padding_with(16.0)
}

fn capture(ui: UiBuilder<Styled<Material3>>, stage: &str) {
    let mut app: OffscreenApp = ui.mount_offscreen(gallery);
    let _ = app.capture_snapshot("text-scripts", "gallery", stage);
}

// Moved from waterui's tests/text_scripts.rs.
#[ignore = "writes a visual acceptance PNG for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (520, 420))]
fn script_gallery_light(ui: UiBuilder<Styled<Material3>>) {
    capture(ui, "light");
}

// Moved from waterui's tests/text_scripts.rs.
#[ignore = "writes a visual acceptance PNG for direct image review"]
#[waterui::test(
    theme = hydrolysis_m3::Material3::with_colors(hydrolysis_m3::MaterialColorScheme::baseline_dark()),
    viewport = (520, 420)
)]
fn script_gallery_dark(ui: UiBuilder<Styled<Material3>>) {
    capture(ui, "dark");
}
