use crate::TableMetrics;
use crate::dimensions::{
    TABLE_CELL_HORIZONTAL_PADDING, TABLE_CELL_VERTICAL_INSET, TABLE_HEADER_HEIGHT,
    TABLE_MIN_COLUMN_WIDTH, TABLE_OUTLINE_WIDTH, TABLE_ROW_HEIGHT,
};
use crate::theme::colors::MaterialColorScheme;
use cherenkov::kurbo::{Line, Stroke};
use cherenkov::kurbo::{Point, Rect};
use cherenkov::{Draw as _, Recorder};

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

pub fn draw_background(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    draw.fill(bounds, colors.surface.working());
}

pub fn draw_header_background(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    draw.fill(bounds, colors.surface.working());
}

pub fn draw_cell_border(colors: &MaterialColorScheme, draw: &mut Recorder, bounds: Rect) {
    draw.stroke(
        bounds,
        Stroke::new(TABLE_OUTLINE_WIDTH),
        colors.outline_variant.working(),
    );
}

pub fn draw_column_separator(
    colors: &MaterialColorScheme,
    draw: &mut Recorder,
    from: Point,
    to: Point,
) {
    draw.stroke(
        Line::new(from, to),
        Stroke::new(TABLE_OUTLINE_WIDTH),
        colors.outline_variant.working(),
    );
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
