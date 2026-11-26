//! The Material 3 table look.
//!
//! Material ships no table component, so these values have no token to
//! match: the `surface` background, the `outline-variant` hairlines and the
//! 56dp header / 52dp row rhythm of the Material data-table spec are
//! `WaterUI`'s own choice.

use hydrolysis_style::draw::kurbo::{Line, Stroke};
use hydrolysis_style::draw::{Draw as _, Paint};
use hydrolysis_style::theme::{Custom, Driven, TableLook, TableMetrics};
use waterui_core::Environment;

use crate::color::{OutlineVariant, Surface};
use crate::controls::button::solid_paint;
use crate::dimensions::TABLE_OUTLINE_WIDTH;

pub const fn metrics() -> TableMetrics {
    TableMetrics {
        min_column_width: 72.0,
        cell_horizontal_padding: 32.0,
        cell_vertical_inset: 16.0,
        header_height: 56.0,
        row_height: 52.0,
        outline_width: 1.0,
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
    use crate::dimensions::TABLE_OUTLINE_WIDTH;

    #[test]
    /// Compose Material 3 ships no table component, so these metrics have no
    /// upstream token to match. They follow the Material data-table spec's
    /// 56dp header / 52dp row rhythm and are `WaterUI`'s own choice.
    fn table_metrics_are_waterui_specific() {
        let metrics = metrics();

        assert_eq!(metrics.min_column_width, 72.0);
        assert_eq!(metrics.cell_horizontal_padding, 32.0);
        assert_eq!(metrics.cell_vertical_inset, 16.0);
        assert_eq!(metrics.header_height, 56.0);
        assert_eq!(metrics.row_height, 52.0);
        assert_eq!(metrics.outline_width, 1.0);
        assert_eq!(TABLE_OUTLINE_WIDTH, 1.0);
    }
}
