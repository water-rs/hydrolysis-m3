use crate::dimensions::{
    TOGGLE_CHECKBOX_CONTAINER_SHAPE, TOGGLE_CHECKBOX_OUTLINE_WIDTH,
    TOGGLE_CHECKBOX_SELECTED_SCALE_START, TOGGLE_CHECKBOX_SIZE, TOGGLE_LABEL_SPACING,
    TOGGLE_SWITCH_HEIGHT, TOGGLE_SWITCH_ICON_SCALE_START, TOGGLE_SWITCH_ICON_SIZE,
    TOGGLE_SWITCH_OUTLINE_WIDTH, TOGGLE_SWITCH_PRESSED_HANDLE_SIZE,
    TOGGLE_SWITCH_SELECTED_HANDLE_SIZE, TOGGLE_SWITCH_THUMB_PADDING,
    TOGGLE_SWITCH_UNSELECTED_HANDLE_SIZE, TOGGLE_SWITCH_WIDTH,
};
use crate::theme::colors::MaterialColorScheme;
use crate::theme::state_layer;
use crate::{ToggleMetrics, WidgetInteractionState, lerp_color};
use cherenkov::kurbo::{Affine, BezPath, PathEl, Point, Rect};
use cherenkov::kurbo::{Circle, RoundedRect, RoundedRectRadii, Stroke};
use cherenkov::{Draw as _, Recorder};
use waterui_controls::toggle::ToggleStyle;

pub fn metrics(style: ToggleStyle) -> ToggleMetrics {
    match style {
        ToggleStyle::Automatic | ToggleStyle::Switch => ToggleMetrics::new(
            TOGGLE_SWITCH_WIDTH,
            TOGGLE_SWITCH_HEIGHT,
            TOGGLE_LABEL_SPACING,
        ),
        ToggleStyle::Checkbox => ToggleMetrics::new(
            TOGGLE_CHECKBOX_SIZE,
            TOGGLE_CHECKBOX_SIZE,
            TOGGLE_LABEL_SPACING,
        ),
        _ => panic!("hydrolysis ToggleStyle variant is not implemented"),
    }
}

/// The switch thumb's center for the given animated `progress`.
///
/// Compose reserves a slot the width of the *selected* handle at each end,
/// inset by `ThumbPadding`, and centres whatever size the thumb currently is
/// inside it. Both rest centres therefore sit half a selected handle in from
/// the padding, and growing or shrinking the thumb — on press, or when the
/// value changes — never moves it sideways.
fn switch_thumb_center(bounds: Rect, progress: f32) -> Point {
    let slot_half = TOGGLE_SWITCH_SELECTED_HANDLE_SIZE / 2.0;
    let unselected_x = bounds.x0 + TOGGLE_SWITCH_THUMB_PADDING + slot_half;
    let selected_x = bounds.x1 - (TOGGLE_SWITCH_THUMB_PADDING + slot_half);
    Point::new(
        crate::lerp_f64(unselected_x, selected_x, progress),
        bounds.y0 + bounds.height() / 2.0,
    )
}

/// The thumb icon's opacity for one frame, per the M3 reference's staged linear
/// transitions over the 200ms value animation: entering waits 50ms then ramps
/// over 150ms (progress 0.25→1.0); exiting ramps out during the first 50ms
/// (progress 1.0→0.75).
fn switch_icon_opacity(progress: f32, selected: bool) -> f32 {
    if selected {
        ((progress - 0.25) / 0.75).clamp(0.0, 1.0)
    } else {
        ((progress - 0.75) / 0.25).clamp(0.0, 1.0)
    }
}

pub fn draw_switch(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: Rect,
    progress: f32,
    selected: bool,
    state: WidgetInteractionState,
) {
    let progress = progress.clamp(0.0, 1.0);
    // the disabled Material switch: the track drops to surface-container-highest at
    // 12% (unchecked) or on-surface at 12% (checked), the outline to
    // on-surface at 12%, the thumb to on-surface at 38% (unchecked) or opaque
    // surface (checked), and the checked icon to on-surface at 38%.
    let track_color = if state.disabled {
        lerp_color(
            colors
                .surface_container_highest
                .working_disabled_container(),
            colors.on_surface.working_disabled_container(),
            progress,
        )
    } else {
        lerp_color(
            colors.surface_container_highest.working(),
            colors.primary.working(),
            progress,
        )
    };
    let handle_size = if state.pressed {
        TOGGLE_SWITCH_PRESSED_HANDLE_SIZE
    } else {
        crate::lerp_f64(
            TOGGLE_SWITCH_UNSELECTED_HANDLE_SIZE,
            TOGGLE_SWITCH_SELECTED_HANDLE_SIZE,
            progress,
        )
    };
    let handle_radius = handle_size / 2.0;
    let thumb_center = switch_thumb_center(bounds, progress);
    let track_radius = bounds.height() / 2.0;
    draw.fill(
        RoundedRect::from_rect(bounds, RoundedRectRadii::from_single_radius(track_radius)),
        track_color,
    );
    // the unselected outline's border-width animates from 2dp to 0 (not
    // its opacity), with the border inside the 52x32 box (border-box sizing).
    let outline_width = TOGGLE_SWITCH_OUTLINE_WIDTH * f64::from(1.0 - progress);
    if outline_width > 0.01 {
        let inset = outline_width / 2.0;
        let outline_bounds = bounds.inflate(-inset, -inset);
        let outline_color = if state.disabled {
            colors.on_surface.working_disabled_container()
        } else {
            colors.outline.working()
        };
        draw.stroke(
            RoundedRect::from_rect(
                outline_bounds,
                RoundedRectRadii::from_single_radius(track_radius - inset),
            ),
            Stroke::new(outline_width),
            outline_color,
        );
    }
    let thumb_color = if state.disabled {
        lerp_color(
            colors.on_surface.working_disabled_content(),
            colors.surface.working(),
            progress,
        )
    } else if state.pressed || state.hovered || state.focus_visible {
        if selected {
            colors.primary_container.working()
        } else {
            colors.on_surface_variant.working()
        }
    } else {
        lerp_color(
            colors.outline.working(),
            colors.on_primary.working(),
            progress,
        )
    };
    draw.fill(Circle::new(thumb_center, handle_radius), thumb_color);

    // the default checked icon: a 16dp checkmark on the thumb, colored
    // on-primary-container, scaling in from 0.92 as it fades.
    let icon_opacity = switch_icon_opacity(progress, selected);
    if icon_opacity > 0.0 {
        let icon_scale = crate::lerp_f64(TOGGLE_SWITCH_ICON_SCALE_START, 1.0, icon_opacity);
        let icon_bounds = Rect::from_center_size(
            thumb_center,
            (
                TOGGLE_SWITCH_ICON_SIZE * icon_scale,
                TOGGLE_SWITCH_ICON_SIZE * icon_scale,
            ),
        );
        let icon_color = if state.disabled {
            colors.on_surface.working_disabled_content()
        } else {
            colors.on_primary_container.working()
        };
        draw.stroke(
            check_glyph_path(icon_bounds),
            Stroke::new(TOGGLE_CHECKBOX_OUTLINE_WIDTH),
            icon_color.with_alpha(icon_color.components[3] * icon_opacity),
        );
    }
}

pub fn draw_switch_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: Rect,
    progress: f32,
    selected: bool,
    state: WidgetInteractionState,
) {
    let center = switch_thumb_center(bounds, progress);
    // the Material switches the thumb's state-layer color with the checked attribute.
    let color = if selected {
        colors.primary.working()
    } else {
        colors.on_surface.working()
    };
    state_layer::draw_unbounded_circle(draw, center, 20.0, color, state);
}

pub fn draw_checkbox(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: Rect,
    progress: f32,
    state: WidgetInteractionState,
) {
    let progress = progress.clamp(0.0, 1.0);
    let outline_opacity = 1.0 - progress;
    if outline_opacity > 0.0 {
        // MD3 disabled checkbox: the unchecked outline drops to on-surface at
        // the 38% disabled-content opacity.
        let outline_color = if state.disabled {
            colors.on_surface.working_disabled_content()
        } else {
            colors.on_surface_variant.working()
        };
        draw.stroke(
            RoundedRect::from_rect(
                bounds,
                RoundedRectRadii::from_single_radius(TOGGLE_CHECKBOX_CONTAINER_SHAPE),
            ),
            Stroke::new(TOGGLE_CHECKBOX_OUTLINE_WIDTH),
            outline_color.with_alpha(outline_color.components[3] * outline_opacity),
        );
    }
    if progress <= 0.0 {
        return;
    }

    let selected_scale = crate::lerp_f64(TOGGLE_CHECKBOX_SELECTED_SCALE_START, 1.0, progress);
    let selected_transform = Affine::translate((bounds.center().x, bounds.center().y))
        * Affine::scale(selected_scale)
        * Affine::translate((-bounds.center().x, -bounds.center().y));
    // MD3 disabled checkbox (checked): container on-surface at 38%, checkmark
    // in the surface color.
    let (container_color, check_color) = if state.disabled {
        (
            colors.on_surface.working_disabled_content(),
            colors.surface.working(),
        )
    } else {
        (colors.primary.working(), colors.on_primary.working())
    };
    draw.transform(selected_transform, |draw| {
        draw.fill(
            RoundedRect::from_rect(
                bounds,
                RoundedRectRadii::from_single_radius(TOGGLE_CHECKBOX_CONTAINER_SHAPE),
            ),
            container_color.with_alpha(container_color.components[3] * progress),
        );
        draw.stroke(
            check_glyph_path(bounds),
            Stroke::new(TOGGLE_CHECKBOX_OUTLINE_WIDTH),
            check_color.with_alpha(check_color.components[3] * progress),
        );
    });
}

/// The Material `check` glyph as a stroked path filling `bounds`.
fn check_glyph_path(bounds: Rect) -> BezPath {
    BezPath::from_vec(vec![
        PathEl::MoveTo(Point::new(
            bounds.width().mul_add(0.25, bounds.x0),
            bounds.height().mul_add(0.55, bounds.y0),
        )),
        PathEl::LineTo(Point::new(
            bounds.width().mul_add(0.45, bounds.x0),
            bounds.height().mul_add(0.75, bounds.y0),
        )),
        PathEl::LineTo(Point::new(
            bounds.width().mul_add(0.78, bounds.x0),
            bounds.height().mul_add(0.3, bounds.y0),
        )),
    ])
}

pub fn draw_checkbox_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: Rect,
    progress: f32,
    state: WidgetInteractionState,
) {
    let center = Point::new(
        bounds.x0 + bounds.width() / 2.0,
        bounds.y0 + bounds.height() / 2.0,
    );
    let color = if progress > 0.0 {
        colors.primary.working()
    } else {
        colors.on_surface.working()
    };
    state_layer::draw_unbounded_circle(draw, center, 20.0, color, state);
}

#[cfg(test)]
mod tests {
    use crate::test_support::{Recorded, solid};
    use cherenkov::kurbo::{Point, Rect};
    use cherenkov::{Recorder, WorkingColor};

    use super::{
        MaterialColorScheme, WidgetInteractionState, draw_checkbox, draw_switch,
        switch_icon_opacity,
    };
    use crate::dimensions::{TOGGLE_LABEL_SPACING, TOGGLE_SWITCH_PRESSED_HANDLE_SIZE};

    /// The toggle chrome a draw recorded, in the shape the assertions read.
    struct Chrome {
        rounded_stroke_count: usize,
        rounded_stroke_widths: Vec<f64>,
        rounded_stroke_brushes: Vec<WorkingColor>,
        rounded_fill_count: usize,
        rounded_fill_brushes: Vec<WorkingColor>,
        path_stroke_count: usize,
        circle_centers: Vec<Point>,
        circle_radii: Vec<f64>,
        circle_brushes: Vec<WorkingColor>,
    }

    impl Chrome {
        fn from(source: Recorder) -> Self {
            let recorded = Recorded::from(source);
            Self {
                rounded_stroke_count: recorded.rounded_strokes.len(),
                rounded_stroke_widths: recorded
                    .rounded_strokes
                    .iter()
                    .map(|(_, _, _, width)| *width)
                    .collect(),
                rounded_stroke_brushes: recorded
                    .rounded_strokes
                    .iter()
                    .map(|(_, _, paint, _)| solid(paint))
                    .collect(),
                rounded_fill_count: recorded.rounded_fills.len(),
                rounded_fill_brushes: recorded
                    .rounded_fills
                    .iter()
                    .map(|(_, _, paint)| solid(paint))
                    .collect(),
                path_stroke_count: recorded.path_strokes,
                circle_centers: recorded
                    .circle_fills
                    .iter()
                    .map(|(circle, _)| circle.center)
                    .collect(),
                circle_radii: recorded
                    .circle_fills
                    .iter()
                    .map(|(circle, _)| circle.radius)
                    .collect(),
                circle_brushes: recorded
                    .circle_fills
                    .iter()
                    .map(|(_, paint)| solid(paint))
                    .collect(),
            }
        }
    }

    #[test]
    fn selected_material_switch_track_has_no_outline() {
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (52.0, 32.0));

        let mut unselected = Recorder::new();
        draw_switch(
            &colors,
            &mut unselected,
            bounds,
            0.0,
            false,
            WidgetInteractionState::NONE,
        );

        let mut selected = Recorder::new();
        draw_switch(
            &colors,
            &mut selected,
            bounds,
            1.0,
            true,
            WidgetInteractionState::NONE,
        );
        let unselected = Chrome::from(unselected);
        let selected = Chrome::from(selected);

        assert_eq!(unselected.rounded_stroke_count, 1);
        assert_eq!(selected.rounded_stroke_count, 0);
    }

    #[test]
    fn switch_outline_width_shrinks_with_progress() {
        // the border-width animates 2dp → 0 with the value transition, not
        // the outline's opacity.
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (52.0, 32.0));

        let mut mid = Recorder::new();
        draw_switch(
            &colors,
            &mut mid,
            bounds,
            0.5,
            true,
            WidgetInteractionState::NONE,
        );

        let mid = Chrome::from(mid);

        assert_eq!(mid.rounded_stroke_widths, vec![1.0]);
    }

    #[test]
    /// Compose's `ThumbPadding` is `(32 - 24) / 2 = 4`, and the thumb sits in a
    /// 24dp slot at each end regardless of the size it currently draws at. The
    /// rest centres are therefore 4 + 12 = 16dp from either edge of the 52dp
    /// track, which is what keeps the thumb still while it changes size.
    fn switch_thumb_rests_at_compose_centers() {
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (52.0, 32.0));

        let mut unselected = Recorder::new();
        draw_switch(
            &colors,
            &mut unselected,
            bounds,
            0.0,
            false,
            WidgetInteractionState::NONE,
        );
        let unselected = Chrome::from(unselected);
        assert_eq!(unselected.circle_centers, vec![Point::new(16.0, 16.0)]);

        let mut selected = Recorder::new();
        draw_switch(
            &colors,
            &mut selected,
            bounds,
            1.0,
            true,
            WidgetInteractionState::NONE,
        );
        let selected = Chrome::from(selected);
        assert_eq!(selected.circle_centers, vec![Point::new(36.0, 16.0)]);
    }

    #[test]
    fn switch_checked_icon_stages_with_material_timing() {
        // Entering: 50ms delay then a 150ms linear ramp over the 200ms value
        // animation. Exiting: gone within the first 50ms.
        assert_eq!(switch_icon_opacity(0.25, true), 0.0);
        assert!((switch_icon_opacity(0.625, true) - 0.5).abs() < 1e-6);
        assert_eq!(switch_icon_opacity(1.0, true), 1.0);
        assert!((switch_icon_opacity(0.9, false) - 0.6).abs() < 1e-6);
        assert_eq!(switch_icon_opacity(0.75, false), 0.0);
        assert_eq!(switch_icon_opacity(0.2, false), 0.0);
    }

    #[test]
    fn selected_switch_draws_checkmark_icon_on_thumb() {
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (52.0, 32.0));

        let mut selected = Recorder::new();
        draw_switch(
            &colors,
            &mut selected,
            bounds,
            1.0,
            true,
            WidgetInteractionState::NONE,
        );
        let selected = Chrome::from(selected);
        assert_eq!(
            selected.path_stroke_count, 1,
            "checked thumb shows the check glyph"
        );

        let mut unselected = Recorder::new();
        draw_switch(
            &colors,
            &mut unselected,
            bounds,
            0.0,
            false,
            WidgetInteractionState::NONE,
        );
        let unselected = Chrome::from(unselected);
        assert_eq!(
            unselected.path_stroke_count, 0,
            "unselected thumb has no icon"
        );
    }

    #[test]
    fn toggle_metrics_include_material_label_spacing() {
        let metrics = super::metrics(waterui_controls::toggle::ToggleStyle::Switch);

        assert_eq!(metrics.label_spacing, TOGGLE_LABEL_SPACING);
        assert_eq!(TOGGLE_LABEL_SPACING, 8.0);
    }

    #[test]
    fn disabled_material_switch_uses_disabled_palette() {
        // the disabled Material switch: unchecked track surface-container-highest at
        // 12% with an on-surface 12% outline and an on-surface 38% thumb;
        // checked track on-surface at 12% with an opaque surface thumb.
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (52.0, 32.0));
        let disabled = WidgetInteractionState {
            disabled: true,
            ..WidgetInteractionState::NONE
        };

        let mut unchecked = Recorder::new();
        draw_switch(&colors, &mut unchecked, bounds, 0.0, false, disabled);
        let unchecked = Chrome::from(unchecked);
        assert_eq!(
            unchecked.rounded_fill_brushes,
            vec![crate::lerp_color(
                colors
                    .surface_container_highest
                    .working_disabled_container(),
                colors.on_surface.working_disabled_container(),
                0.0,
            )],
            "disabled unchecked track drops to surface-container-highest at 12%"
        );
        assert_eq!(
            unchecked.rounded_stroke_brushes,
            vec![colors.on_surface.working_disabled_container()],
            "disabled unchecked outline drops to on-surface at 12%"
        );
        assert_eq!(
            unchecked.circle_brushes,
            vec![crate::lerp_color(
                colors.on_surface.working_disabled_content(),
                colors.surface.working(),
                0.0,
            )],
            "disabled unchecked thumb drops to on-surface at 38%"
        );

        let mut checked = Recorder::new();
        draw_switch(&colors, &mut checked, bounds, 1.0, true, disabled);
        let checked = Chrome::from(checked);
        assert_eq!(
            checked.rounded_fill_brushes,
            vec![crate::lerp_color(
                colors
                    .surface_container_highest
                    .working_disabled_container(),
                colors.on_surface.working_disabled_container(),
                1.0,
            )],
            "disabled checked track drops to on-surface at 12%"
        );
        assert_eq!(
            checked.circle_brushes,
            vec![crate::lerp_color(
                colors.on_surface.working_disabled_content(),
                colors.surface.working(),
                1.0,
            )],
            "disabled checked thumb is the opaque surface color"
        );
    }

    #[test]
    fn disabled_material_checkbox_uses_disabled_palette() {
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (18.0, 18.0));
        let disabled = WidgetInteractionState {
            disabled: true,
            ..WidgetInteractionState::NONE
        };

        let mut unchecked = Recorder::new();
        draw_checkbox(&colors, &mut unchecked, bounds, 0.0, disabled);
        let unchecked = Chrome::from(unchecked);
        assert_eq!(
            unchecked.rounded_stroke_brushes,
            vec![colors.on_surface.working_disabled_content()],
            "disabled unchecked outline drops to on-surface at 38%"
        );

        let mut checked = Recorder::new();
        draw_checkbox(&colors, &mut checked, bounds, 1.0, disabled);
        let checked = Chrome::from(checked);
        assert_eq!(
            checked.rounded_fill_brushes,
            vec![colors.on_surface.working_disabled_content()],
            "disabled checked container drops to on-surface at 38%"
        );
    }

    #[test]
    fn pressed_material_switch_uses_large_handle() {
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (52.0, 32.0));
        let mut draw = Recorder::new();

        draw_switch(
            &colors,
            &mut draw,
            bounds,
            0.0,
            false,
            WidgetInteractionState {
                pressed: true,
                ..WidgetInteractionState::NONE
            },
        );

        let draw = Chrome::from(draw);

        assert_eq!(
            draw.circle_radii,
            vec![TOGGLE_SWITCH_PRESSED_HANDLE_SIZE / 2.0]
        );
    }

    #[test]
    fn selected_material_checkbox_has_no_outline() {
        let colors = MaterialColorScheme::baseline_light();
        let bounds = Rect::from_origin_size((0.0, 0.0), (18.0, 18.0));

        let mut unselected = Recorder::new();
        draw_checkbox(
            &colors,
            &mut unselected,
            bounds,
            0.0,
            WidgetInteractionState::NONE,
        );

        let mut selected = Recorder::new();
        draw_checkbox(
            &colors,
            &mut selected,
            bounds,
            1.0,
            WidgetInteractionState::NONE,
        );

        let unselected = Chrome::from(unselected);

        let selected = Chrome::from(selected);

        assert_eq!(unselected.rounded_stroke_count, 1);
        assert_eq!(unselected.rounded_fill_count, 0);
        assert_eq!(selected.rounded_stroke_count, 0);
        assert_eq!(selected.rounded_fill_count, 1);
        assert_eq!(selected.path_stroke_count, 1);
    }
}
