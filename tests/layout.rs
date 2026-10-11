//! Layout assertions that only the rendered runtime under Material 3 can
//! carry: the M3 text field's ideal width, and the retained-sub-view
//! re-measure contract.
//!
//! Moved out of hydrolysis's own suite under water-rs/hydrolysis-m3#113: the
//! theme-independent stack geometry stayed in waterui-layout as unstyled
//! semantic tests; what remains here is either an M3 metric or a property
//! only a rendered, retained runtime can observe.

use waterui::layout::scroll::scroll_horizontal;
use waterui::prelude::*;
use waterui::{Binding, Str};
use waterui_controls::field;
use waterui_testing::{OffscreenApp, Role, Styled, UiBuilder};

/// With no width proposal on its stretch axis the field keeps its intrinsic
/// width — the theme's ideal `min_width` (280 under Material 3), not a hard
/// floor. A horizontal scroll is the container that measures content at
/// `None` on the scrolling axis; a viewport narrower than 280 keeps the field
/// at its ideal and lets it overflow into scrollable content.
fn lone_text_field_view() -> impl View {
    let value = Binding::container(Str::from(""));
    scroll_horizontal(field("Field", &value))
}

#[waterui::test(lone_text_field_view, theme = hydrolysis_m3::Material3::defaults(), offscreen, viewport = (200, 120))]
fn lone_text_field_keeps_ideal_width(app: &mut OffscreenApp) {
    let field = app
        .query()
        .role(Role::TEXT_INPUT)
        .label("Field")
        .single()
        .bounds();
    assert!(
        (field.width() - 280.0).abs() <= 1.0,
        "an unproposed field should keep the 280-pt ideal width: {field:?}"
    );
}

/// A text whose string changes after mount is measured again inside a
/// retained sub-view, so the box layout gives it, and the place of the view
/// beside it, follow the new string. Before waterui#1872 the sub-view kept the
/// box the first string measured, and the flush wrapped the new string inside
/// it, over the next view. Covered where a sub-view retains its tree: a
/// navigation page and a split view's detail column, and a lazy row inside a
/// page, which its own flush places after the page's layout at a rect the new
/// string does not change.
const LONG: &str = "A label long enough to wrap into several lines at this width";

#[waterui::test(theme = hydrolysis_m3::Material3::defaults(), viewport = (900, 600))]
fn a_changed_text_is_measured_again_inside_a_retained_sub_view(
    ui: UiBuilder<Styled<hydrolysis_m3::Material3>>,
) {
    use waterui::id::SelfId;
    use waterui::navigation::{NavigationSplitView, NavigationStack, NavigationView};

    fn status_column(status: &Binding<String>) -> impl View {
        vstack((
            text(status.clone()).a11y_label("status").width(150.0),
            text("next").a11y_label("next"),
        ))
    }

    /// Mounts the status column in one kind of retained sub-view.
    type Mount = fn(&Binding<String>) -> AnyView;

    fn page(status: &Binding<String>) -> AnyView {
        AnyView::new(NavigationStack::new(NavigationView::new(
            "Status",
            status_column(status),
        )))
    }

    fn split_detail(status: &Binding<String>) -> AnyView {
        let status = status.clone();
        let selection = Binding::container(Some(1_i32));
        AnyView::new(NavigationSplitView::new(
            &selection,
            || text("sidebar"),
            move |_| NavigationView::new("Detail", status_column(&status)),
        ))
    }

    fn lazy_row(status: &Binding<String>) -> AnyView {
        let status = status.clone();
        AnyView::new(NavigationStack::new(NavigationView::new(
            "Status",
            VStack::for_each(vec![SelfId::new(0_usize)], move |_| {
                hstack((
                    text(status.clone()).a11y_label("status"),
                    spacer(),
                    text("next").a11y_label("next"),
                ))
            }),
        )))
    }

    let cases: [(&str, Mount); 3] = [
        ("navigation page", page),
        ("split detail", split_detail),
        ("lazy row in a page", lazy_row),
    ];
    for (case, mount) in cases {
        let mut settled = ui
            .clone()
            .mount_offscreen(move || mount(&Binding::container(LONG.to_owned())));
        let expected = settled.query().label("status").single().bounds();

        let status = Binding::container(String::from("Al"));
        let mut app = ui.clone().mount_offscreen({
            let status = status.clone();
            move || mount(&status)
        });
        status.set(LONG.to_owned());
        app.settle();

        let measured = app.query().label("status").single().bounds();
        let next = app.query().label("next").single().bounds();
        assert!(
            (measured.width() - expected.width()).abs() < 0.5
                && (measured.height() - expected.height()).abs() < 0.5,
            "{case}: the changed text keeps the box it measured before the change: \
             {measured:?}, while the same string mounted directly measures {expected:?}"
        );
        let apart = next.y() >= measured.y() + measured.height() - 0.5
            || next.x() >= measured.x() + measured.width() - 0.5;
        assert!(
            apart,
            "{case}: the next view must be placed clear of the re-measured text: \
             {next:?} vs {measured:?}"
        );
    }
}

/// A text field is a `Horizontal` leaf: inside an hstack offered 340 wide,
/// two fields split the row instead of overflowing at the 280-pt intrinsic
/// floor. water-rs/hydrolysis#107.
fn text_field_row_view() -> impl View {
    let first = Binding::container(Str::from(""));
    let last = Binding::container(Str::from(""));
    hstack((field("First name", &first), field("Last name", &last))).size(340.0, 60.0)
}

#[waterui::test(text_field_row_view, theme = hydrolysis_m3::Material3::defaults(), offscreen, viewport = (400, 120))]
fn text_fields_share_a_340_row(app: &mut OffscreenApp) {
    let first = app
        .query()
        .role(Role::TEXT_INPUT)
        .label("First name")
        .single()
        .bounds();
    let last = app
        .query()
        .role(Role::TEXT_INPUT)
        .label("Last name")
        .single()
        .bounds();
    let half = (340.0_f32 - 10.0) / 2.0;
    assert!(
        (first.width() - half).abs() <= 1.0 && (last.width() - half).abs() <= 1.0,
        "each field should get about half of the row: first={first:?} last={last:?}"
    );
    assert!(
        first.x() + first.width() <= last.x(),
        "fields should share the row without overlapping: first={first:?} last={last:?}"
    );
    assert!(
        last.x() + last.width() - first.x() <= 340.0,
        "fields should stay inside the 340-wide row: first={first:?} last={last:?}"
    );
}

/// In a 600-wide container the field answers the proposal and fills it.
fn text_field_in_container_view() -> impl View {
    let value = Binding::container(Str::from(""));
    field("Field", &value).size(600.0, 60.0)
}

#[waterui::test(text_field_in_container_view, theme = hydrolysis_m3::Material3::defaults(), offscreen, viewport = (640, 120))]
fn text_field_fills_wide_container(app: &mut OffscreenApp) {
    let field = app
        .query()
        .role(Role::TEXT_INPUT)
        .label("Field")
        .single()
        .bounds();
    assert!(
        (field.width() - 600.0).abs() <= 1.0,
        "a field should fill its 600-wide container: {field:?}"
    );
}

/// The webview address row from water-rs/waterui#2275. A text field is a
/// `Horizontal` leaf that answers the proposal width, so typing a URL wider
/// than the row leaves the field, the `Go` button beside it, and the row
/// below at the frames the empty field gave them; the text scrolls inside
/// the field. Its height is its intrinsic one-line height, which an empty
/// field measures too, so nothing below moves either.
fn wide_url_field_view() -> impl View {
    let url = Binding::container(Str::from(""));
    vstack((
        hstack((field("Address", &url), button("Go").action(|| {}))),
        hstack((text("Allow redirects"), spacer(), text("100%"))),
    ))
    .spacing(12.0)
    .padding_with(20.0)
    .background(Srgb::BLACK)
}

#[waterui::test(wide_url_field_view, theme = hydrolysis_m3::Material3::defaults(), offscreen, viewport = (360, 200))]
fn wide_field_text_keeps_every_frame(app: &mut OffscreenApp) {
    // `bounds()` reads the snapshot taken at query time, so each pass
    // re-queries.
    let frames = |app: &mut OffscreenApp| {
        [
            app.query().role(Role::TEXT_INPUT).label("Address").single(),
            app.query().role(Role::BUTTON).label("Go").single(),
            app.query()
                .role(Role::LABEL)
                .label("Allow redirects")
                .single(),
            app.query().role(Role::LABEL).label("100%").single(),
        ]
        .map(|element| element.bounds())
    };
    let empty = frames(app);

    app.query()
        .role(Role::TEXT_INPUT)
        .label("Address")
        .single()
        .set_text(
            app,
            "https://example.com/a/path/long/enough/to/overflow/the/address/field",
        );
    app.settle();

    assert_eq!(
        frames(app),
        empty,
        "field, Go, and the row below must keep their frames once the field holds wide text"
    );
}

/// The conversation row from water-rs/waterui#1219: a one-line preview that
/// truncates in the second row. The truncated leaf reports the width it
/// drew — its laid-out line including the ellipsis — so what it declines
/// reaches the row's spacer and the trailing badge ends on the row's
/// trailing edge, where the timestamp in the row above already lands.
fn compressed_row_view() -> impl View {
    hstack((
        text("RC").padding_with(44.0),
        vstack((
            hstack((
                text("Ada").a11y_label("title"),
                spacer(),
                text("14:32").a11y_label("stamp"),
            ))
            .spacing(4.0),
            hstack((
                text("On my way — see you at the gate in about ten minutes")
                    .caption()
                    .line_limit(core::num::NonZeroUsize::MIN)
                    .a11y_label("preview"),
                spacer(),
                text("2").caption().a11y_label("badge"),
            ))
            .spacing(4.0),
        ))
        .spacing(2.0)
        .leading(),
    ))
    .spacing(10.0)
    .padding_with((6.0, 10.0))
}

#[waterui::test(compressed_row_view, theme = hydrolysis_m3::Material3::defaults(), offscreen, viewport = (340, 200))]
fn a_compressed_limited_text_leaves_its_spacer_nothing(app: &mut OffscreenApp) {
    let stamp = app.query().label("stamp").single().bounds();
    let badge = app.query().label("badge").single().bounds();
    // The outer padding insets horizontally by 10, so the trailing edge of
    // either stretched row lands at 340 - 10.
    let trailing = 330.0;
    assert!(
        (stamp.x() + stamp.width() - trailing).abs() <= 1.0,
        "the timestamp should end on the row's trailing edge: {stamp:?}"
    );
    assert!(
        (badge.x() + badge.width() - trailing).abs() <= 1.0,
        "the badge should end on the row's trailing edge: {badge:?}"
    );
}
