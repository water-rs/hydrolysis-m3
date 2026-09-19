//! Visual acceptance for the `Avatar` composer under Material 3.
//!
//! Avatar is a Rust-side composition (Framework Design Principle #2) — a
//! stack, a frame, a clip and theme tokens, with no FFI type of its own — so
//! what the renderer draws under this theme is exactly what these captures
//! review. The PNG-producing tests are ignored by default and reviewed by eye.

use core::cell::Cell;
use core::time::Duration;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use hydrolysis_m3::Material3;
use waterui::media::photo::Event as PhotoEvent;
use waterui::prelude::*;
use waterui::shape::RoundedRectangle;
use waterui::widget::avatar::Avatar;
use waterui_testing::{OffscreenApp, Styled, UiBuilder};

/// A 240×240 test portrait: four saturated quadrants.
///
/// Every feature is chosen to be read off the rendered avatar directly. The
/// quadrant boundaries show whether the picture is centred and undistorted,
/// and the silhouette's edge is the boundary between a saturated colour and
/// the page — which reads in both colour schemes, unlike a light or dark
/// border, either of which disappears into one of the two backgrounds and
/// makes a circular clip look like an octagon.
///
/// The portrait is wider than it is tall and carries a white disc at its
/// centre: an avatar that stretched the picture would show that disc as an
/// ellipse, one that covers and crops keeps it round. Quadrants alone could
/// not tell the two apart, since both leave their boundaries at the centre.
fn write_test_portrait(dir: &Path) -> PathBuf {
    const WIDTH: u32 = 320;
    const HEIGHT: u32 = 200;
    const DISC_RADIUS: f64 = 60.0;
    let path = dir.join("portrait.png");
    let mut pixels = image::RgbaImage::new(WIDTH, HEIGHT);
    let (centre_x, centre_y) = (f64::from(WIDTH) / 2.0, f64::from(HEIGHT) / 2.0);
    for (x, y, pixel) in pixels.enumerate_pixels_mut() {
        let (dx, dy) = (f64::from(x) + 0.5 - centre_x, f64::from(y) + 0.5 - centre_y);
        *pixel = if dx.hypot(dy) <= DISC_RADIUS {
            image::Rgba([255, 255, 255, 255])
        } else {
            match (x < WIDTH / 2, y < HEIGHT / 2) {
                (true, true) => image::Rgba([220, 40, 40, 255]),
                (false, true) => image::Rgba([40, 160, 60, 255]),
                (true, false) => image::Rgba([40, 80, 220, 255]),
                (false, false) => image::Rgba([230, 190, 40, 255]),
            }
        };
    }
    std::fs::create_dir_all(dir).expect("test portrait directory");
    pixels.save(&path).expect("test portrait should encode");
    path
}

fn portrait_url() -> Url {
    let dir = std::env::temp_dir().join("waterui-avatar-tests");
    Url::from_file_path_str(write_test_portrait(&dir).to_string_lossy().into_owned())
}

/// One row of the gallery a reviewer reads: every silhouette, the ring, the
/// monogram and a real picture.
///
/// `observed` is the picture whose load the capture waits on; it is attached to
/// one avatar per gallery so the wait is on a real completion event rather than
/// a guessed duration.
fn row(url: Url, side: f32, observed: Option<Rc<Cell<bool>>>) -> impl View {
    let watched = url.clone();
    hstack((
        avatar("Ada Lovelace").size(side),
        avatar("山田 太郎")
            .size(side)
            .shape(RoundedRectangle::new(0.25)),
        avatar("Katherine Johnson")
            .size(side)
            .ring(Color::new(theme_color::Accent), side / 20.0),
        avatar("Grace Hopper").image(url).size(side),
        observed.map(|observed| {
            Avatar::new("Grace Hopper", move || {
                let observed = Rc::clone(&observed);
                Photo::new(watched.clone())
                    .resizable()
                    .content_mode(ContentMode::Fill)
                    .on_event(move |event: PhotoEvent| {
                        if matches!(event, PhotoEvent::Loaded) {
                            observed.set(true);
                        }
                    })
            })
            .size(side)
            .ring(Color::new(theme_color::Accent), side / 20.0)
        }),
    ))
    .spacing(16.0)
}

/// The gallery: a realistic list-row size on top, and the same avatars at a
/// size where the clip's edge, the ring's concentricity and the monogram's
/// optical centring can be judged by eye.
fn gallery(url: Url, loaded: Rc<Cell<bool>>) -> impl View {
    vstack((row(url.clone(), 40.0, None), row(url, 128.0, Some(loaded))))
        .spacing(20.0)
        .padding_with(16.0)
}

fn capture(ui: UiBuilder<Styled<Material3>>, stage: &str) {
    let url = portrait_url();
    let loaded = Rc::new(Cell::new(false));
    let observer = Rc::clone(&loaded);
    let mut app: OffscreenApp =
        ui.mount_offscreen(move || gallery(url.clone(), Rc::clone(&observer)));

    assert!(
        app.pump_until(Duration::from_secs(5), || loaded.get()),
        "the test portrait must decode before the gallery is captured"
    );
    let _ = app.capture_snapshot("material3-preview", "avatar-gallery", stage);
}

// Moved from waterui's tests/avatar.rs.
#[ignore = "writes a visual acceptance PNG for direct image review"]
#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (820, 236))]
fn avatar_gallery_light(ui: UiBuilder<Styled<Material3>>) {
    capture(ui, "light");
}

// Moved from waterui's tests/avatar.rs.
#[ignore = "writes a visual acceptance PNG for direct image review"]
#[waterui::test(
    theme = hydrolysis_m3::Material3::with_colors(hydrolysis_m3::MaterialColorScheme::baseline_dark()),
    viewport = (820, 236)
)]
fn avatar_gallery_dark(ui: UiBuilder<Styled<Material3>>) {
    capture(ui, "dark");
}
