use crate::dimensions::{
    PICKER_HORIZONTAL_INSET, PICKER_INDICATOR_SPACE, PICKER_LABEL_SPACING, PICKER_MENU_MIN_WIDTH,
    PICKER_MENU_POPUP_CORNER_RADIUS, PICKER_MENU_POPUP_ROW_HEIGHT, PICKER_MENU_POPUP_TOP_SPACING,
    PICKER_MIN_HEIGHT, PICKER_RADIO_INDICATOR_SIZE, PICKER_RADIO_INNER_DOT_RADIUS,
    PICKER_RADIO_LABEL_SPACING, PICKER_RADIO_MIN_WIDTH, PICKER_RADIO_OUTER_RING_WIDTH,
    PICKER_RADIO_ROW_SPACING, PICKER_SEGMENTED_CONTAINER_RADIUS, PICKER_SEGMENTED_HORIZONTAL_INSET,
    PICKER_SEGMENTED_MIN_HEIGHT, PICKER_SEGMENTED_MIN_WIDTH, PICKER_SEGMENTED_OUTLINE_WIDTH,
    PICKER_VERTICAL_INSET,
};
use crate::elevation::MaterialElevationLevel;
use crate::theme::colors::{MaterialColorScheme, MaterialRoleColor};
use crate::theme::state_layer;
use crate::{PickerMetrics, RadioIndicatorState, WidgetInteractionState};
use cherenkov::kurbo::{Circle, Line, RoundedRect, RoundedRectRadii, Stroke};
use cherenkov::{Draw as _, Recorder, WorkingColor};
use num_traits::ToPrimitive;
use waterui_form::picker::PickerStyle;

pub fn metrics(style: PickerStyle) -> PickerMetrics {
    match style {
        PickerStyle::Automatic | PickerStyle::Menu => menu_metrics(),
        PickerStyle::Radio => radio_metrics(),
        PickerStyle::Segmented => segmented_metrics(),
        _ => panic!("hydrolysis PickerStyle variant is not implemented"),
    }
}

const fn menu_metrics() -> PickerMetrics {
    PickerMetrics {
        min_width: PICKER_MENU_MIN_WIDTH,
        min_height: PICKER_MIN_HEIGHT,
        horizontal_inset: PICKER_HORIZONTAL_INSET,
        vertical_inset: PICKER_VERTICAL_INSET,
        label_spacing: PICKER_LABEL_SPACING,
        indicator_space: PICKER_INDICATOR_SPACE,
        radio_indicator_size: PICKER_RADIO_INDICATOR_SIZE,
        radio_label_spacing: PICKER_RADIO_LABEL_SPACING,
        radio_row_spacing: PICKER_RADIO_ROW_SPACING,
        popup_top_spacing: PICKER_MENU_POPUP_TOP_SPACING,
        popup_row_height: PICKER_MENU_POPUP_ROW_HEIGHT,
        popup_corner_radius: PICKER_MENU_POPUP_CORNER_RADIUS,
        segment_min_width: 0.0,
    }
}

const fn radio_metrics() -> PickerMetrics {
    PickerMetrics {
        min_width: PICKER_RADIO_MIN_WIDTH,
        min_height: PICKER_MIN_HEIGHT,
        horizontal_inset: PICKER_HORIZONTAL_INSET,
        vertical_inset: PICKER_VERTICAL_INSET,
        label_spacing: PICKER_LABEL_SPACING,
        indicator_space: PICKER_INDICATOR_SPACE,
        radio_indicator_size: PICKER_RADIO_INDICATOR_SIZE,
        radio_label_spacing: PICKER_RADIO_LABEL_SPACING,
        radio_row_spacing: PICKER_RADIO_ROW_SPACING,
        popup_top_spacing: PICKER_MENU_POPUP_TOP_SPACING,
        popup_row_height: PICKER_MENU_POPUP_ROW_HEIGHT,
        popup_corner_radius: PICKER_MENU_POPUP_CORNER_RADIUS,
        segment_min_width: 0.0,
    }
}

const fn segmented_metrics() -> PickerMetrics {
    PickerMetrics {
        min_width: PICKER_SEGMENTED_MIN_WIDTH,
        min_height: PICKER_SEGMENTED_MIN_HEIGHT,
        horizontal_inset: PICKER_SEGMENTED_HORIZONTAL_INSET,
        vertical_inset: 0.0,
        label_spacing: PICKER_LABEL_SPACING,
        indicator_space: 0.0,
        radio_indicator_size: PICKER_RADIO_INDICATOR_SIZE,
        radio_label_spacing: PICKER_RADIO_LABEL_SPACING,
        radio_row_spacing: PICKER_RADIO_ROW_SPACING,
        popup_top_spacing: PICKER_MENU_POPUP_TOP_SPACING,
        popup_row_height: PICKER_MENU_POPUP_ROW_HEIGHT,
        popup_corner_radius: PICKER_MENU_POPUP_CORNER_RADIUS,
        segment_min_width: PICKER_SEGMENTED_MIN_WIDTH,
    }
}

pub fn draw_indicator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: cherenkov::kurbo::Rect,
) {
    let center_x = PICKER_INDICATOR_SPACE.mul_add(-0.5, bounds.x1 - PICKER_HORIZONTAL_INSET);
    let center_y = bounds.height().mul_add(0.5, bounds.y0);
    let chevron = cherenkov::kurbo::BezPath::from_vec(vec![
        cherenkov::kurbo::PathEl::MoveTo(cherenkov::kurbo::Point::new(
            center_x - 4.0,
            center_y - 2.0,
        )),
        cherenkov::kurbo::PathEl::LineTo(cherenkov::kurbo::Point::new(center_x, center_y + 2.0)),
        cherenkov::kurbo::PathEl::LineTo(cherenkov::kurbo::Point::new(
            center_x + 4.0,
            center_y - 2.0,
        )),
    ]);
    draw.stroke(
        chevron,
        Stroke::new(1.5),
        colors.on_surface_variant.working(),
    );
}

pub fn draw_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: cherenkov::kurbo::Rect,
    state: WidgetInteractionState,
) {
    state_layer::draw_bounded(draw, bounds, 4.0.into(), colors.on_surface.working(), state);
}

pub fn draw_popup(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    popup_rect: cherenkov::kurbo::Rect,
) {
    let radii = PICKER_MENU_POPUP_CORNER_RADIUS.into();
    // `MenuTokens.ContainerElevation` is `ElevationTokens.Level2`; Material
    // menus carry no outline.
    crate::elevation::draw_shadows(
        draw,
        popup_rect,
        radii,
        MaterialElevationLevel::LEVEL2,
        colors,
    );
    draw.fill(
        RoundedRect::from_rect(popup_rect, radii),
        colors.surface_container.working(),
    );
}

pub fn draw_popup_row_background(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    row_rect: cherenkov::kurbo::Rect,
    selected: bool,
) {
    if !selected {
        return;
    }
    let inset = cherenkov::kurbo::Rect::new(
        row_rect.x0 + 2.0,
        row_rect.y0 + 1.0,
        row_rect.x1 - 2.0,
        row_rect.y1 - 1.0,
    );
    draw.fill(inset, colors.surface_container_highest.working());
}

pub fn draw_popup_row_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    row_rect: cherenkov::kurbo::Rect,
    selected: bool,
    state: WidgetInteractionState,
) {
    let inset = cherenkov::kurbo::Rect::new(
        row_rect.x0 + 2.0,
        row_rect.y0 + 1.0,
        row_rect.x1 - 2.0,
        row_rect.y1 - 1.0,
    );
    state_layer::draw_bounded(
        draw,
        inset,
        0.0.into(),
        if selected {
            colors.on_secondary_container.working()
        } else {
            colors.on_surface.working()
        },
        state,
    );
}

pub fn draw_separator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    separator: cherenkov::kurbo::Rect,
) {
    draw.fill(separator, colors.outline_variant.working());
}

pub fn draw_radio_indicator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    center: cherenkov::kurbo::Point,
    radius: f64,
    state: RadioIndicatorState,
) {
    let outer_selected_progress = state.outer_selected_progress.clamp(0.0, 1.0);
    let inner_scale = state.inner_scale.clamp(0.0, 1.0);
    let inner_opacity = state.inner_opacity.clamp(0.0, 1.0);
    let outer_ring_center_radius = radius - PICKER_RADIO_OUTER_RING_WIDTH / 2.0;
    draw.stroke(
        Circle::new(center, outer_ring_center_radius),
        Stroke::new(PICKER_RADIO_OUTER_RING_WIDTH),
        blend_role_color(
            colors.on_surface_variant,
            colors.primary,
            outer_selected_progress,
        ),
    );
    let inner_radius = PICKER_RADIO_INNER_DOT_RADIUS * f64::from(inner_scale);
    if inner_radius > 0.0 && inner_opacity > 0.0 {
        draw.fill(
            Circle::new(center, inner_radius),
            colors.primary.working().with_alpha(inner_opacity),
        );
    }
}

fn blend_role_color(from: MaterialRoleColor, to: MaterialRoleColor, progress: f32) -> WorkingColor {
    crate::lerp_color(from.working(), to.working(), progress)
}

pub fn draw_radio_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    center: cherenkov::kurbo::Point,
    _radius: f64,
    selected: bool,
    state: WidgetInteractionState,
) {
    state_layer::draw_unbounded_circle(
        draw,
        center,
        20.0,
        if selected {
            colors.primary.working()
        } else {
            colors.on_surface.working()
        },
        state,
    );
}

pub fn segmented_label_color(
    colors: &MaterialColorScheme,
    selected: bool,
) -> waterui_graphics::color::Color {
    if selected {
        colors.on_secondary_container.view_color()
    } else {
        colors.on_surface.view_color()
    }
}

pub fn draw_segmented_container(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: cherenkov::kurbo::Rect,
    segment_count: usize,
) {
    draw.stroke(
        RoundedRect::from_rect(
            bounds,
            RoundedRectRadii::from_single_radius(PICKER_SEGMENTED_CONTAINER_RADIUS),
        ),
        Stroke::new(PICKER_SEGMENTED_OUTLINE_WIDTH),
        colors.outline.working(),
    );
    if segment_count <= 1 {
        return;
    }
    let segment_width = bounds.width()
        / segment_count
            .to_f64()
            .expect("picker segment count must be representable as f64");
    for index in 1..segment_count {
        let x = segment_width.mul_add(
            index
                .to_f64()
                .expect("picker segment index must be representable as f64"),
            bounds.x0,
        );
        draw.stroke(
            Line::new(
                cherenkov::kurbo::Point::new(x, bounds.y0),
                cherenkov::kurbo::Point::new(x, bounds.y1),
            ),
            Stroke::new(PICKER_SEGMENTED_OUTLINE_WIDTH),
            colors.outline.working(),
        );
    }
}

/// One segment's item shape, per Compose `itemShape(index, count)`: only the
/// outside edge of each end segment takes the group's full rounding — the
/// first item rounds its leading corners, the last its trailing corners, and
/// middle items stay square against the separator strokes.
const fn segment_radii(is_first: bool, is_last: bool) -> cherenkov::kurbo::RoundedRectRadii {
    let radius = PICKER_SEGMENTED_CONTAINER_RADIUS;
    match (is_first, is_last) {
        (true, true) => cherenkov::kurbo::RoundedRectRadii::new(radius, radius, radius, radius),
        (true, false) => cherenkov::kurbo::RoundedRectRadii::new(radius, 0.0, 0.0, radius),
        (false, true) => cherenkov::kurbo::RoundedRectRadii::new(0.0, radius, radius, 0.0),
        (false, false) => cherenkov::kurbo::RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0),
    }
}

pub fn draw_segmented_segment(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: cherenkov::kurbo::Rect,
    selected: bool,
    is_first: bool,
    is_last: bool,
) {
    if !selected {
        return;
    }
    draw.fill(
        RoundedRect::from_rect(bounds, segment_radii(is_first, is_last)),
        colors.secondary_container.working(),
    );
}

pub fn draw_segmented_state_layer(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: cherenkov::kurbo::Rect,
    selected: bool,
    is_first: bool,
    is_last: bool,
    state: WidgetInteractionState,
) {
    state_layer::draw_bounded(
        draw,
        bounds,
        segment_radii(is_first, is_last),
        if selected {
            colors.on_secondary_container.working()
        } else {
            colors.on_surface.working()
        },
        state,
    );
}

#[cfg(test)]
mod tests {
    use cherenkov::kurbo::{Point, Rect, RoundedRectRadii};
    use cherenkov::{Command, Paint, Recorder, ShapeData, WorkingColor as Color};

    use super::{
        MaterialColorScheme, RadioIndicatorState, blend_role_color, draw_popup_row_background,
        draw_radio_indicator, draw_segmented_container, draw_segmented_segment, draw_separator,
        menu_metrics, radio_metrics, segment_radii, segmented_metrics,
    };
    use crate::dimensions::{
        PICKER_LABEL_SPACING, PICKER_MENU_POPUP_CORNER_RADIUS, PICKER_MENU_POPUP_ROW_HEIGHT,
        PICKER_RADIO_INDICATOR_SIZE, PICKER_RADIO_INNER_DOT_RADIUS, PICKER_RADIO_OUTER_RING_WIDTH,
        PICKER_SEGMENTED_CONTAINER_RADIUS, PICKER_SEGMENTED_HORIZONTAL_INSET,
        PICKER_SEGMENTED_MIN_HEIGHT,
    };

    /// The solid-colour commands a theme draw recorded, sorted by shape.
    #[derive(Default)]
    struct Recorded {
        circle_fills: Vec<(f64, Color)>,
        circle_strokes: Vec<(f64, f64, Color)>,
        rect_fills: Vec<Color>,
        rounded_fills: Vec<(RoundedRectRadii, Color)>,
        rounded_strokes: Vec<(RoundedRectRadii, Color, f64)>,
        line_strokes: Vec<(Color, f64)>,
        shadows: Vec<(f64, f64, Color)>,
    }

    fn solid(paint: &Paint) -> Color {
        let Paint::Solid(color) = paint else {
            panic!("Material picker token paints must be solid colors");
        };
        *color
    }

    impl Recorded {
        fn from(source: Recorder) -> Self {
            let mut content = source.finish();
            let mut recorded = Self::default();
            for command in content.snapshot().commands() {
                match command {
                    Command::Fill { shape, paint } => match shape {
                        ShapeData::Circle(circle) => {
                            recorded.circle_fills.push((circle.radius, solid(paint)));
                        }
                        ShapeData::Rect(_) => recorded.rect_fills.push(solid(paint)),
                        ShapeData::RoundedRect(rounded) => {
                            recorded.rounded_fills.push((rounded.radii(), solid(paint)));
                        }
                        _ => {}
                    },
                    Command::Stroke {
                        shape,
                        stroke,
                        paint,
                    } => match shape {
                        ShapeData::Circle(circle) => {
                            recorded.circle_strokes.push((
                                circle.radius,
                                stroke.width,
                                solid(paint),
                            ));
                        }
                        ShapeData::RoundedRect(rounded) => {
                            recorded.rounded_strokes.push((
                                rounded.radii(),
                                solid(paint),
                                stroke.width,
                            ));
                        }
                        ShapeData::Line(_) => {
                            recorded.line_strokes.push((solid(paint), stroke.width));
                        }
                        _ => {}
                    },
                    Command::Shadow { shadow, .. } => {
                        recorded
                            .shadows
                            .push((shadow.sigma, shadow.offset.y, shadow.color));
                    }
                    _ => {}
                }
            }
            recorded
        }
    }

    #[test]
    /// Values from `androidx.compose.material3.tokens.RadioButtonTokens` and
    /// the private dimensions in `RadioButton.kt`.
    fn radio_metrics_match_compose_radio_tokens() {
        let metrics = radio_metrics();

        assert_eq!(metrics.radio_indicator_size, PICKER_RADIO_INDICATOR_SIZE);
        assert_eq!(metrics.label_spacing, PICKER_LABEL_SPACING);
        assert_eq!(PICKER_RADIO_INDICATOR_SIZE, 20.0);
        assert_eq!(PICKER_RADIO_OUTER_RING_WIDTH, 2.0);
        // RadioButtonDotSize is 12dp across, so half of it here.
        assert_eq!(PICKER_RADIO_INNER_DOT_RADIUS, 6.0);
        assert_eq!(metrics.popup_row_height, PICKER_MENU_POPUP_ROW_HEIGHT);
        assert_eq!(metrics.popup_corner_radius, PICKER_MENU_POPUP_CORNER_RADIUS);
        assert_eq!(PICKER_MENU_POPUP_ROW_HEIGHT, 48.0);
        assert_eq!(PICKER_MENU_POPUP_CORNER_RADIUS, 4.0);
    }

    #[test]
    fn radio_indicator_draws_the_compose_donut_icon() {
        let colors = MaterialColorScheme::baseline_light();
        let center = Point::new(10.0, 10.0);

        let mut unselected = Recorder::new();
        draw_radio_indicator(
            &colors,
            &mut unselected,
            center,
            10.0,
            RadioIndicatorState {
                selected: false,
                outer_selected_progress: 0.0,
                inner_scale: 1.0,
                inner_opacity: 0.0,
            },
        );

        let mut selected = Recorder::new();
        draw_radio_indicator(
            &colors,
            &mut selected,
            center,
            10.0,
            RadioIndicatorState {
                selected: true,
                outer_selected_progress: 1.0,
                inner_scale: 1.0,
                inner_opacity: 1.0,
            },
        );

        let unselected = Recorded::from(unselected);

        let selected = Recorded::from(selected);

        assert_eq!(unselected.circle_fills, Vec::<(f64, Color)>::new());
        assert_eq!(
            unselected.circle_strokes,
            vec![(9.0, 2.0, colors.on_surface_variant.working())]
        );
        assert_eq!(
            selected.circle_strokes,
            vec![(9.0, 2.0, colors.primary.working())]
        );
        assert_eq!(selected.circle_fills, vec![(6.0, colors.primary.working())]);
    }

    #[test]
    fn radio_indicator_inner_dot_scales_and_fades() {
        let colors = MaterialColorScheme::baseline_light();
        let center = Point::new(10.0, 10.0);
        let mut draw = Recorder::new();

        draw_radio_indicator(
            &colors,
            &mut draw,
            center,
            10.0,
            RadioIndicatorState {
                selected: true,
                outer_selected_progress: 1.0,
                inner_scale: 0.4,
                inner_opacity: 0.25,
            },
        );

        let draw = Recorded::from(draw);

        assert_eq!(draw.circle_fills.len(), 1);
        assert!(
            PICKER_RADIO_INNER_DOT_RADIUS
                .mul_add(-0.4, draw.circle_fills[0].0)
                .abs()
                < 0.000_001
        );
        assert_eq!(
            draw.circle_fills[0].1,
            colors.primary.working().with_alpha(0.25)
        );
    }

    #[test]
    fn radio_indicator_outer_ring_color_interpolates() {
        let colors = MaterialColorScheme::baseline_light();
        let center = Point::new(10.0, 10.0);
        let mut draw = Recorder::new();

        draw_radio_indicator(
            &colors,
            &mut draw,
            center,
            10.0,
            RadioIndicatorState {
                selected: true,
                outer_selected_progress: 0.5,
                inner_scale: 0.0,
                inner_opacity: 0.0,
            },
        );

        let draw = Recorded::from(draw);

        assert_eq!(
            draw.circle_strokes,
            vec![(
                9.0,
                2.0,
                blend_role_color(colors.on_surface_variant, colors.primary, 0.5)
            )]
        );
    }

    #[test]
    fn menu_selected_row_and_divider_use_filled_select_tokens() {
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = Recorder::new();

        draw_popup_row_background(&colors, &mut draw, Rect::new(0.0, 0.0, 120.0, 48.0), true);
        draw_separator(&colors, &mut draw, Rect::new(0.0, 48.0, 120.0, 49.0));

        let draw = Recorded::from(draw);

        assert_eq!(
            draw.rect_fills,
            vec![
                colors.surface_container_highest.working(),
                colors.outline_variant.working(),
            ]
        );
    }

    #[test]
    fn menu_picker_min_width_matches_text_field() {
        // The menu style is filled-TextField chrome; `ExposedDropdownMenuBox`
        // inherits `TextFieldDefaults.MinWidth` = 280dp.
        assert_eq!(menu_metrics().min_width, 280.0);
        assert_eq!(radio_metrics().min_width, 72.0);
    }

    #[test]
    fn segmented_metrics_match_compose_segmented_button_tokens() {
        let metrics = segmented_metrics();

        assert_eq!(metrics.min_height, PICKER_SEGMENTED_MIN_HEIGHT);
        assert_eq!(metrics.horizontal_inset, PICKER_SEGMENTED_HORIZONTAL_INSET);
        assert_eq!(metrics.segment_min_width, 58.0);
        assert_eq!(PICKER_SEGMENTED_MIN_HEIGHT, 40.0);
        assert_eq!(PICKER_SEGMENTED_CONTAINER_RADIUS, 20.0);
    }

    /// Compose `itemShape(index, count)`: only the outside edge of each end
    /// segment rounds; middle segments are square.
    #[test]
    fn segmented_item_shape_rounds_only_the_outside_edge() {
        let radius = PICKER_SEGMENTED_CONTAINER_RADIUS;
        let zero = RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0);

        assert_eq!(
            segment_radii(true, false),
            RoundedRectRadii::new(radius, 0.0, 0.0, radius)
        );
        assert_eq!(
            segment_radii(false, true),
            RoundedRectRadii::new(0.0, radius, radius, 0.0)
        );
        assert_eq!(segment_radii(false, false), zero);
        assert_eq!(
            segment_radii(true, true),
            RoundedRectRadii::new(radius, radius, radius, radius)
        );
    }

    #[test]
    fn segmented_container_and_selected_segment_use_material_tokens() {
        let colors = MaterialColorScheme::baseline_light();
        let mut draw = Recorder::new();

        draw_segmented_segment(
            &colors,
            &mut draw,
            Rect::new(0.0, 0.0, 80.0, 40.0),
            true,
            false,
            false,
        );
        draw_segmented_container(&colors, &mut draw, Rect::new(0.0, 0.0, 240.0, 40.0), 3);

        let draw = Recorded::from(draw);

        assert_eq!(
            draw.rounded_fills,
            vec![(
                RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0),
                colors.secondary_container.working()
            )]
        );
        assert_eq!(draw.rounded_strokes.len(), 1);
        assert_eq!(draw.rounded_strokes[0].1, colors.outline.working());
        assert_eq!(draw.rounded_strokes[0].2, 1.0);
        assert_eq!(
            draw.line_strokes,
            vec![
                (colors.outline.working(), 1.0),
                (colors.outline.working(), 1.0)
            ]
        );
    }

    #[test]
    /// `MenuTokens.ContainerElevation` is `ElevationTokens.Level2`: the popup
    /// casts the level-2 key and ambient shadows and carries no outline.
    fn popup_casts_the_level_two_elevation_shadows() {
        use super::draw_popup;

        let colors = MaterialColorScheme::baseline_light();
        let mut draw = Recorder::new();

        draw_popup(&colors, &mut draw, Rect::new(0.0, 0.0, 112.0, 96.0));

        let draw = Recorded::from(draw);

        assert_eq!(draw.shadows.len(), 2, "key then ambient shadow");
        let (key_blur, key_y, _) = draw.shadows[0];
        let (ambient_blur, ambient_y, _) = draw.shadows[1];
        assert_eq!(key_blur, f64::from(3.0f32));
        assert_eq!(key_y, f64::from(0.85f32));
        assert_eq!(ambient_blur, f64::from(1.0f32));
        assert_eq!(ambient_y, f64::from(0.25f32));
        assert!(
            draw.rounded_strokes.is_empty(),
            "Material menus carry no outline"
        );
    }
}
