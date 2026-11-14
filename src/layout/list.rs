//! The Material 3 list look: rows, separators, the reorder lift, the
//! trailing controls of an edited list and the swipe-to-dismiss background.

use hydrolysis::draw::kurbo::{Point, Rect, RoundedRect, RoundedRectRadii, Size, Vec2};
use hydrolysis::draw::{Draw as _, Paint, Shadow as ShadowCast, ShapeData};
use hydrolysis::theme::{
    Chrome, CornerShape, Custom, Driven, Inputs, ListControls, ListDividerMetrics, ListGestures,
    ListLook, ListMetrics, ListRowMetrics, ListSectionMetrics, ListSpec,
    ListTrailingControlMetrics, Surface,
};
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{
    ErrorContainer, OnError, OnErrorContainer, OnSurface, OnSurfaceVariant, Outline,
    Surface as SurfaceRole, TertiaryContainer,
};
use crate::controls::button::{interactive, solid_paint};
use crate::dimensions::{
    LIST_CONTROL_ICON_SIZE, LIST_CONTROL_STATE_LAYER_SIZE, LIST_DELETE_CONTROL_WIDTH,
    LIST_DIVIDER_LEADING_INSET, LIST_DIVIDER_TRAILING_INSET, LIST_HORIZONTAL_INSET,
    LIST_MOVE_CONTROL_WIDTH, LIST_MOVE_STATE_LAYER_SIZE, LIST_ONE_LINE_ROW_HEIGHT,
    LIST_REORDER_THRESHOLD, LIST_ROW_DRAG_SLOP, LIST_SECTION_FOOTER_HEIGHT,
    LIST_SECTION_HEADER_HEIGHT, LIST_SWIPE_DISMISS_THRESHOLD, LIST_SWIPE_ICON_EDGE_INSET,
    LIST_SWIPE_ICON_MIN_SCALE, LIST_SWIPE_ICON_SIZE, LIST_TRAILING_CONTROL_SPACING,
    LIST_TRAILING_CONTROL_VERTICAL_INSET, LIST_VERTICAL_INSET, SHAPE_CORNER_LARGE,
};
use crate::elevation::{MaterialElevationLevel, cast_shadows};
use crate::icon_paths::{self, IconGrid};
use crate::theme::motion;

/// The icon a trailing control draws, over its chrome's frame.
type Icon = Custom<(), (Driven<Paint>,)>;

/// The swipe background's operands: the container's paint, the delete
/// glyph's shape and the glyph's paint.
type SwipeBackground = Custom<
    f64,
    (
        Driven<Paint, f64>,
        Driven<ShapeData, f64>,
        Driven<Paint, f64>,
    ),
>;

/// The lifted row's operands: its level-4 key and ambient shadows, then its
/// container paint.
type LiftedRow = Custom<(), (Driven<ShadowCast>, Driven<ShadowCast>, Driven<Paint>)>;

pub const fn metrics() -> ListMetrics {
    ListMetrics::new(
        ListRowMetrics::new(
            LIST_ONE_LINE_ROW_HEIGHT,
            LIST_HORIZONTAL_INSET,
            LIST_VERTICAL_INSET,
        ),
        ListDividerMetrics::new(LIST_DIVIDER_LEADING_INSET, LIST_DIVIDER_TRAILING_INSET),
        ListTrailingControlMetrics::new(
            LIST_MOVE_CONTROL_WIDTH,
            LIST_DELETE_CONTROL_WIDTH,
            LIST_TRAILING_CONTROL_SPACING,
            LIST_TRAILING_CONTROL_VERTICAL_INSET,
        ),
        ListSectionMetrics::new(LIST_SECTION_HEADER_HEIGHT, LIST_SECTION_FOOTER_HEIGHT),
    )
}

/// How a row's swipe and reorder drag are recognised and settle: a swipe
/// dismisses at half the row's width (`SwipeToDismissBoxDefaults`), a
/// reorder steps at a slot's midpoint, a drag starts past the touch slop,
/// and a released row settles on the fast spatial spring.
fn gestures() -> ListGestures {
    ListGestures {
        swipe_threshold: LIST_SWIPE_DISMISS_THRESHOLD,
        reorder_threshold: LIST_REORDER_THRESHOLD,
        slop: LIST_ROW_DRAG_SLOP,
        settle: motion::fast_spatial(),
    }
}

/// The look of a list in `spec`.
///
/// A row is the `surface` colour; its separator is `outline`
/// (`md.comp.list.divider`); the row a reorder drag lifts is a
/// tertiary-container item at corner-large casting level 4
/// (`md.comp.list.reorder.list-item`); a swipe uncovers the error
/// container with a delete glyph.
pub fn look(_spec: ListSpec, env: &Environment) -> ListLook {
    ListLook::new(
        metrics(),
        gestures(),
        fill(SurfaceRole, env),
        fill(Outline, env),
        ListControls {
            move_up: move_half(Half::Upper, env),
            move_down: move_half(Half::Lower, env),
            delete: delete_control(env),
        },
    )
    .lifted_row(lifted_row(env))
    .swipe_background(swipe_background(env))
}

/// A part filling its frame with `color`.
fn fill(color: impl Into<Color>, env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new((solid_paint(color, env),), |recorder, frame, (paint,)| {
        recorder.fill(frame, paint);
    })
}

/// Which half of the reorder grip's control a chrome covers.
#[derive(Clone, Copy)]
enum Half {
    Upper,
    Lower,
}

/// The reorder grip's half: Material's `drag_handle` icon on the bare row,
/// centred on the seam between the halves.
///
/// Each half draws the whole icon, so a row at the list's boundary, which
/// has one half only, still shows the complete grip; where both exist the
/// two draws coincide. Material gives a list's trailing affordance an icon
/// and a state layer, not a bordered container of its own: an outlined box
/// would read as a stepper control rather than something to drag. The
/// halves touch, so each takes touches only inside its own rect, and its
/// circular state layer hugs the seam.
fn move_half(half: Half, env: &Environment) -> Chrome {
    let alignment_y = match half {
        Half::Upper => 1.0,
        Half::Lower => 0.0,
    };
    let side = LIST_MOVE_STATE_LAYER_SIZE;
    control(
        OnSurface,
        Size::new(side, side),
        Vec2::new(0.5, alignment_y),
    )
    .hit(motion::hit_region().minimum(Size::ZERO))
    .custom(icon(
        OnSurfaceVariant,
        move |frame| {
            let seam = match half {
                Half::Upper => frame.y1,
                Half::Lower => frame.y0,
            };
            icon_paths::drag_handle(IconGrid::centered(
                Point::new(frame.center().x, seam),
                LIST_CONTROL_ICON_SIZE,
            ))
        },
        env,
    ))
}

/// The delete affordance: Material's `delete` icon tinted `error`.
///
/// A solid error-coloured slab was never the Material treatment: the row
/// keeps its own surface and the destructive action is carried by the
/// icon's colour.
fn delete_control(env: &Environment) -> Chrome {
    let side = LIST_CONTROL_STATE_LAYER_SIZE;
    control(OnError, Size::new(side, side), Vec2::new(0.5, 0.5)).custom(icon(
        crate::color::Error,
        |frame| icon_paths::delete(IconGrid::centered(frame.center(), LIST_CONTROL_ICON_SIZE)),
        env,
    ))
}

/// A trailing control's chrome: no container, a circular state layer of
/// `size` in `layer`, placed at `alignment` in the control's rect, with the
/// Material ripple, focus indicator and touch target.
fn control(layer: impl Into<Color>, size: Size, alignment: Vec2) -> Chrome {
    interactive(Surface::clear(CornerShape::full()), layer.into()).sized(size, alignment)
}

/// A control's icon, `shape` over its frame, filled with `color`.
fn icon(
    color: impl Into<Color>,
    shape: impl Fn(Rect) -> hydrolysis::draw::kurbo::BezPath + 'static,
    env: &Environment,
) -> Icon {
    Custom::new(
        (solid_paint(color, env),),
        move |recorder, frame, (paint,)| {
            recorder.fill(shape(frame), paint);
        },
    )
}

/// Lifted treatment for a row being dragged to a new position:
/// `md.comp.list.reorder.list-item` — a tertiary-container item at
/// corner-large casting the dragged item's level-4 elevation.
fn lifted_row(env: &Environment) -> LiftedRow {
    let (key, ambient) = cast_shadows(MaterialElevationLevel::LEVEL4, env);
    Custom::new(
        (key, ambient, solid_paint(TertiaryContainer, env)),
        |recorder, frame, (key, ambient, paint)| {
            let shape = RoundedRect::from_rect(
                frame,
                RoundedRectRadii::from_single_radius(SHAPE_CORNER_LARGE),
            );
            recorder.shadow(shape, key);
            recorder.shadow(shape, ambient);
            recorder.fill(shape, paint);
        },
    )
}

/// Background revealed behind a row being swiped away: the error
/// container, with a delete glyph pinned to the edge the row is
/// uncovering.
///
/// The glyph grows in with the swipe's progress, so crossing the dismiss
/// threshold is legible before the finger lifts.
fn swipe_background(env: &Environment) -> SwipeBackground {
    Custom::new(
        (
            solid_paint(ErrorContainer, env),
            Driven::new(|inputs: &Inputs<'_, f64>| {
                ShapeData::of(&swipe_glyph(inputs.frame, *inputs.value))
            }),
            solid_paint(OnErrorContainer, env),
        ),
        |recorder, frame, (container, glyph, paint)| {
            recorder.fill(frame, container);
            recorder.fill(glyph, paint);
        },
    )
}

/// The swipe's delete glyph on `frame` at `progress`, negative while the row
/// travels toward the physical left edge, which uncovers the right one.
///
/// The glyph grows from a reduced size to full as the swipe approaches its
/// threshold.
fn swipe_glyph(frame: Rect, progress: f64) -> hydrolysis::draw::kurbo::BezPath {
    let inset = LIST_SWIPE_ICON_EDGE_INSET + LIST_SWIPE_ICON_SIZE / 2.0;
    let center_x = if progress < 0.0 {
        frame.x1 - inset
    } else {
        frame.x0 + inset
    };
    let scale = (1.0 - LIST_SWIPE_ICON_MIN_SCALE)
        .mul_add(progress.abs().clamp(0.0, 1.0), LIST_SWIPE_ICON_MIN_SCALE);
    icon_paths::delete(IconGrid::centered(
        Point::new(center_x, frame.center().y),
        LIST_SWIPE_ICON_SIZE * scale,
    ))
}

#[cfg(test)]
mod tests {
    use super::{gestures, metrics, swipe_glyph};
    use crate::dimensions::{
        LIST_DELETE_CONTROL_WIDTH, LIST_DIVIDER_LEADING_INSET, LIST_DIVIDER_TRAILING_INSET,
        LIST_HORIZONTAL_INSET, LIST_MOVE_CONTROL_WIDTH, LIST_MOVE_STATE_LAYER_SIZE,
        LIST_ONE_LINE_ROW_HEIGHT, LIST_TRAILING_CONTROL_SPACING,
        LIST_TRAILING_CONTROL_VERTICAL_INSET, LIST_VERTICAL_INSET,
    };
    use hydrolysis::draw::kurbo::{Rect, Shape as _};

    #[test]
    fn list_metrics_match_compose_list_tokens() {
        let metrics = metrics();

        assert_eq!(metrics.one_line_row_height, LIST_ONE_LINE_ROW_HEIGHT);
        assert_eq!(metrics.horizontal_inset, LIST_HORIZONTAL_INSET);
        assert_eq!(metrics.vertical_inset, LIST_VERTICAL_INSET);
        assert_eq!(metrics.divider_leading_inset, LIST_DIVIDER_LEADING_INSET);
        assert_eq!(metrics.divider_trailing_inset, LIST_DIVIDER_TRAILING_INSET);
        assert_eq!(metrics.move_control_width, LIST_MOVE_CONTROL_WIDTH);
        assert_eq!(metrics.delete_control_width, LIST_DELETE_CONTROL_WIDTH);
        assert_eq!(
            metrics.trailing_control_spacing,
            LIST_TRAILING_CONTROL_SPACING
        );
        assert_eq!(
            metrics.trailing_control_vertical_inset,
            LIST_TRAILING_CONTROL_VERTICAL_INSET
        );
        assert_eq!(LIST_ONE_LINE_ROW_HEIGHT, 56.0);
        assert_eq!(LIST_HORIZONTAL_INSET, 16.0);
        assert_eq!(LIST_VERTICAL_INSET, 10.0);
        assert_eq!(LIST_DIVIDER_LEADING_INSET, 16.0);
        assert_eq!(LIST_DIVIDER_TRAILING_INSET, 16.0);
    }

    /// The renderer's constants before the look carried them: a swipe
    /// dismisses at half a row, a reorder steps at a slot's midpoint, a
    /// drag starts past 8 points.
    #[test]
    fn gestures_keep_the_renderers_thresholds() {
        let gestures = gestures();

        assert_eq!(gestures.swipe_threshold, 0.5);
        assert_eq!(gestures.reorder_threshold, 0.5);
        assert_eq!(gestures.slop, 8.0);
    }

    /// A one-line row's 48dp move control splits into halves whose circles
    /// are the half's inscribed one.
    #[test]
    fn move_half_state_layer_is_half_a_one_line_controls_height() {
        assert_eq!(LIST_MOVE_STATE_LAYER_SIZE, 24.0);
    }

    /// The glyph sits on the edge the row uncovers: the right one while the
    /// row travels left, the left one while it travels right.
    #[test]
    fn swipe_glyph_follows_the_uncovered_edge() {
        let frame = Rect::new(0.0, 0.0, 300.0, 56.0);

        assert!(swipe_glyph(frame, -1.0).bounding_box().x0 > frame.center().x);
        assert!(swipe_glyph(frame, 1.0).bounding_box().x1 < frame.center().x);
    }

    /// The glyph grows as the swipe nears its threshold.
    #[test]
    fn swipe_glyph_grows_with_progress() {
        let frame = Rect::new(0.0, 0.0, 300.0, 56.0);

        assert!(
            swipe_glyph(frame, 1.0).bounding_box().width()
                > swipe_glyph(frame, 0.1).bounding_box().width()
        );
    }
}
