use crate::dimensions::{
    PICKER_HORIZONTAL_INSET, PICKER_INDICATOR_SPACE, PICKER_LABEL_SPACING, PICKER_MENU_MIN_WIDTH,
    PICKER_MENU_POPUP_CORNER_RADIUS, PICKER_MENU_POPUP_ROW_HEIGHT, PICKER_MENU_POPUP_TOP_SPACING,
    PICKER_MIN_HEIGHT, PICKER_RADIO_INDICATOR_SIZE, PICKER_RADIO_INNER_DOT_RADIUS,
    PICKER_RADIO_LABEL_SPACING, PICKER_RADIO_MIN_WIDTH, PICKER_RADIO_OUTER_RING_WIDTH,
    PICKER_RADIO_ROW_SPACING, PICKER_VERTICAL_INSET,
};
use crate::elevation::MaterialElevationLevel;
use crate::theme::colors::{MaterialColorScheme, MaterialRoleColor};
use crate::theme::state_layer;
use crate::{PickerMetrics, RadioIndicatorState, WidgetInteractionState};
use hydrolysis::draw::kurbo::{Circle, RoundedRect, RoundedRectRadii, Stroke};
use hydrolysis::draw::{Draw as _, Recorder, WorkingColor};
use waterui::contract_violation;
use waterui_form::picker::PickerStyle;

/// The metrics of `style`. A `PickerStyle` this theme does not implement is
/// a contract violation; a release build uses the menu metrics, the
/// automatic style's.
pub fn metrics(style: PickerStyle) -> PickerMetrics {
    match style {
        PickerStyle::Automatic | PickerStyle::Menu => menu_metrics(),
        PickerStyle::Radio => radio_metrics(),
        // A segmented picker is measured by its look
        // ([`super::segmented::look`]), not by picker metrics.
        _ => {
            contract_violation!(
                "hydrolysis-m3 does not implement PickerStyle {style:?}; using the menu metrics"
            );
            menu_metrics()
        }
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
    }
}

pub fn draw_indicator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    bounds: hydrolysis::draw::kurbo::Rect,
) {
    let center_x = PICKER_INDICATOR_SPACE.mul_add(-0.5, bounds.x1 - PICKER_HORIZONTAL_INSET);
    let center_y = bounds.height().mul_add(0.5, bounds.y0);
    let chevron = hydrolysis::draw::kurbo::BezPath::from_vec(vec![
        hydrolysis::draw::kurbo::PathEl::MoveTo(hydrolysis::draw::kurbo::Point::new(
            center_x - 4.0,
            center_y - 2.0,
        )),
        hydrolysis::draw::kurbo::PathEl::LineTo(hydrolysis::draw::kurbo::Point::new(
            center_x,
            center_y + 2.0,
        )),
        hydrolysis::draw::kurbo::PathEl::LineTo(hydrolysis::draw::kurbo::Point::new(
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
    bounds: hydrolysis::draw::kurbo::Rect,
    state: WidgetInteractionState,
) {
    // md.comp.filled-select menu field: container shape is
    // corner-extra-small-top — 4dp on top, square below the active indicator.
    state_layer::draw_bounded(
        draw,
        bounds,
        RoundedRectRadii::new(4.0, 4.0, 0.0, 0.0),
        colors.on_surface.working(),
        state,
    );
}

pub fn draw_popup(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    popup_rect: hydrolysis::draw::kurbo::Rect,
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
    row_rect: hydrolysis::draw::kurbo::Rect,
    selected: bool,
) {
    if !selected {
        return;
    }
    let inset = hydrolysis::draw::kurbo::Rect::new(
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
    row_rect: hydrolysis::draw::kurbo::Rect,
    _selected: bool,
    state: WidgetInteractionState,
) {
    let inset = hydrolysis::draw::kurbo::Rect::new(
        row_rect.x0 + 2.0,
        row_rect.y0 + 1.0,
        row_rect.x1 - 2.0,
        row_rect.y1 - 1.0,
    );
    // md.comp.filled-select.menu.item.selected.container.color =
    // surface-container-highest, so a selected row takes on-surface like
    // every other row.
    state_layer::draw_bounded(draw, inset, 0.0.into(), colors.on_surface.working(), state);
}

pub fn draw_separator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    separator: hydrolysis::draw::kurbo::Rect,
) {
    // md.comp.filled-select.menu.divider.color = surface-variant.
    draw.fill(separator, colors.surface_variant.working());
}

pub fn draw_radio_indicator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    center: hydrolysis::draw::kurbo::Point,
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
    center: hydrolysis::draw::kurbo::Point,
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

#[cfg(test)]
mod tests {
    use hydrolysis::draw::kurbo::{Point, Rect, RoundedRectRadii};
    use hydrolysis::draw::{
        Command, Content, LayoutSize, Paint, Recorder, ShapeData, WorkingColor as Color,
    };

    use super::{
        MaterialColorScheme, RadioIndicatorState, blend_role_color, draw_popup_row_background,
        draw_radio_indicator, draw_separator, menu_metrics, radio_metrics,
    };
    use crate::dimensions::{
        PICKER_LABEL_SPACING, PICKER_MENU_POPUP_CORNER_RADIUS, PICKER_MENU_POPUP_ROW_HEIGHT,
        PICKER_RADIO_INDICATOR_SIZE, PICKER_RADIO_INNER_DOT_RADIUS, PICKER_RADIO_OUTER_RING_WIDTH,
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

    /// Records `body` on a fresh recorder and sorts its commands.
    fn recorded(body: impl FnOnce(&mut Recorder)) -> Recorded {
        let mut content = Content::record(&LayoutSize::new(), body);
        Recorded::from(&mut content)
    }

    impl Recorded {
        fn from(content: &mut Content) -> Self {
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

        let unselected = recorded(|draw| {
            draw_radio_indicator(
                &colors,
                draw,
                center,
                10.0,
                RadioIndicatorState {
                    selected: false,
                    outer_selected_progress: 0.0,
                    inner_scale: 1.0,
                    inner_opacity: 0.0,
                },
            );
        });

        let selected = recorded(|draw| {
            draw_radio_indicator(
                &colors,
                draw,
                center,
                10.0,
                RadioIndicatorState {
                    selected: true,
                    outer_selected_progress: 1.0,
                    inner_scale: 1.0,
                    inner_opacity: 1.0,
                },
            );
        });

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
        let draw = recorded(|draw| {
            draw_radio_indicator(
                &colors,
                draw,
                center,
                10.0,
                RadioIndicatorState {
                    selected: true,
                    outer_selected_progress: 1.0,
                    inner_scale: 0.4,
                    inner_opacity: 0.25,
                },
            );
        });

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
        let draw = recorded(|draw| {
            draw_radio_indicator(
                &colors,
                draw,
                center,
                10.0,
                RadioIndicatorState {
                    selected: true,
                    outer_selected_progress: 0.5,
                    inner_scale: 0.0,
                    inner_opacity: 0.0,
                },
            );
        });

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
        let draw = recorded(|draw| {
            draw_popup_row_background(&colors, draw, Rect::new(0.0, 0.0, 120.0, 48.0), true);
            draw_separator(&colors, draw, Rect::new(0.0, 48.0, 120.0, 49.0));
        });

        assert_eq!(
            draw.rect_fills,
            vec![
                colors.surface_container_highest.working(),
                colors.surface_variant.working(),
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
    /// `MenuTokens.ContainerElevation` is `ElevationTokens.Level2`: the popup
    /// casts the level-2 key and ambient shadows and carries no outline.
    fn popup_casts_the_level_two_elevation_shadows() {
        use super::draw_popup;

        let colors = MaterialColorScheme::baseline_light();
        let draw = recorded(|draw| {
            draw_popup(&colors, draw, Rect::new(0.0, 0.0, 112.0, 96.0));
        });

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
