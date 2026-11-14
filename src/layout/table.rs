//! The Material 3 table look.
//!
//! Material ships no table component, so these values have no token to
//! match: the `surface` background, the `outline-variant` hairlines and the
//! 56dp header / 52dp row rhythm of the Material data-table spec are
//! `WaterUI`'s own choice.

use hydrolysis::draw::kurbo::{Line, Stroke};
use hydrolysis::draw::{Draw as _, Paint};
use hydrolysis::theme::{Custom, Driven, TableLook, TableMetrics};
use waterui_core::Environment;

use crate::color::{OutlineVariant, Surface};
use crate::controls::button::solid_paint;
use crate::dimensions::{
    TABLE_CELL_HORIZONTAL_PADDING, TABLE_CELL_VERTICAL_INSET, TABLE_HEADER_HEIGHT,
    TABLE_MIN_COLUMN_WIDTH, TABLE_OUTLINE_WIDTH, TABLE_ROW_HEIGHT,
};

pub const fn metrics() -> TableMetrics {
    TableMetrics {
        min_column_width: TABLE_MIN_COLUMN_WIDTH,
        cell_horizontal_padding: TABLE_CELL_HORIZONTAL_PADDING,
        cell_vertical_inset: TABLE_CELL_VERTICAL_INSET,
        header_height: TABLE_HEADER_HEIGHT,
        row_height: TABLE_ROW_HEIGHT,
        outline_width: TABLE_OUTLINE_WIDTH,
    }
}

/// The look of a table: `surface` behind the body and the header, cells
/// bordered and columns separated by `outline-variant` hairlines.
pub fn look(env: &Environment) -> TableLook {
    TableLook::new(
        metrics(),
        fill(env),
        fill(env),
        cell_border(env),
        column_separator(env),
    )
}

fn fill(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new((solid_paint(Surface, env),), |recorder, frame, (paint,)| {
        recorder.fill(frame, paint);
    })
}

fn cell_border(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (solid_paint(OutlineVariant, env),),
        |recorder, frame, (paint,)| {
            recorder.stroke(frame, Stroke::new(TABLE_OUTLINE_WIDTH), paint);
        },
    )
}

/// The line between the two ends of the zero-width rect it draws over.
fn column_separator(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (solid_paint(OutlineVariant, env),),
        |recorder, frame, (paint,)| {
            recorder.stroke(
                Line::new((frame.x0, frame.y0), (frame.x1, frame.y1)),
                Stroke::new(TABLE_OUTLINE_WIDTH),
                paint,
            );
        },
    )
}

#[cfg(test)]
mod tests {
    use super::metrics;
    use crate::dimensions::{
        TABLE_CELL_HORIZONTAL_PADDING, TABLE_CELL_VERTICAL_INSET, TABLE_HEADER_HEIGHT,
        TABLE_MIN_COLUMN_WIDTH, TABLE_OUTLINE_WIDTH, TABLE_ROW_HEIGHT,
    };

    #[test]
    /// Compose Material 3 ships no table component, so these metrics have no
    /// upstream token to match. They follow the Material data-table spec's
    /// 56dp header / 52dp row rhythm and are `WaterUI`'s own choice.
    fn table_metrics_are_waterui_specific() {
        let metrics = metrics();

        assert_eq!(metrics.min_column_width, TABLE_MIN_COLUMN_WIDTH);
        assert_eq!(
            metrics.cell_horizontal_padding,
            TABLE_CELL_HORIZONTAL_PADDING
        );
        assert_eq!(metrics.cell_vertical_inset, TABLE_CELL_VERTICAL_INSET);
        assert_eq!(metrics.header_height, TABLE_HEADER_HEIGHT);
        assert_eq!(metrics.row_height, TABLE_ROW_HEIGHT);
        assert_eq!(metrics.outline_width, TABLE_OUTLINE_WIDTH);
        assert_eq!(TABLE_HEADER_HEIGHT, 56.0);
        assert_eq!(TABLE_ROW_HEIGHT, 52.0);
        assert_eq!(TABLE_CELL_HORIZONTAL_PADDING, 32.0);
        assert_eq!(TABLE_CELL_VERTICAL_INSET, 16.0);
        assert_eq!(TABLE_OUTLINE_WIDTH, 1.0);
    }
}
