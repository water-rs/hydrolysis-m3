use hydrolysis_m3::Material3;
use waterui_testing::OffscreenApp;

fn view() -> impl waterui::View {
    use waterui::prelude::*;
    vstack((text!("Hello"), button("Auto"), button(label("Prominent").icon(()).icon_only()).bordered_prominent())).padding_with(16.0).background(waterui::graphics::color::Srgb::WHITE)
}

#[waterui::test(view, theme = Material3::defaults(), viewport = (360, 160), offscreen)]
fn dump(app: &mut OffscreenApp) {
    let s = app.snapshot();
    s.save_png("/tmp/m3art/dump.png").unwrap();
    let mut hist = std::collections::BTreeMap::new();
    for p in s.rgba8.chunks(4) { *hist.entry(p.to_vec()).or_insert(0usize) += 1; }
    let mut v: Vec<_> = hist.into_iter().collect(); v.sort_by_key(|x| std::cmp::Reverse(x.1));
    for (k,c) in v.iter().take(6) { eprintln!("{k:?} x{c}"); }
}
