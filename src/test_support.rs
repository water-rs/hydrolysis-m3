//! Inspects what a theme draw recorded: the display list a [`Recorder`]
//! produced, sorted by command and shape so tests assert on geometry and paint
//! without a renderer.

use cherenkov::kurbo::{Circle, Line, Rect, RoundedRectRadii};
use cherenkov::{Command, Paint, Recorder, Shadow, ShapeData, WorkingColor};

/// The commands a theme draw recorded, split by shape.
#[derive(Debug, Default)]
#[allow(clippy::redundant_pub_crate, reason = "test-only module")]
pub(crate) struct Recorded {
    pub(crate) rect_fills: Vec<(Rect, Paint)>,
    pub(crate) rounded_fills: Vec<(Rect, RoundedRectRadii, Paint)>,
    pub(crate) rounded_strokes: Vec<(Rect, RoundedRectRadii, Paint, f64)>,
    pub(crate) circle_fills: Vec<(Circle, Paint)>,
    pub(crate) circle_strokes: Vec<(Circle, Paint, f64)>,
    pub(crate) line_strokes: Vec<(Line, Paint, f64)>,
    pub(crate) path_fills: usize,
    pub(crate) path_strokes: usize,
    pub(crate) shadows: Vec<Shadow>,
    /// Transform scopes opened.
    pub(crate) transforms: usize,
}

impl Recorded {
    /// Finishes `recorder` and sorts its commands.
    pub(crate) fn from(source: Recorder) -> Self {
        let mut content = source.finish();
        let mut recorded = Self::default();
        for command in content.snapshot().commands() {
            match command {
                Command::Fill { shape, paint } => match shape {
                    ShapeData::Rect(rect) => recorded.rect_fills.push((*rect, paint.clone())),
                    ShapeData::RoundedRect(rounded) => recorded.rounded_fills.push((
                        rounded.rect(),
                        rounded.radii(),
                        paint.clone(),
                    )),
                    ShapeData::Circle(circle) => {
                        recorded.circle_fills.push((*circle, paint.clone()));
                    }
                    ShapeData::Path { .. } => recorded.path_fills += 1,
                    ShapeData::Continuous(_) | ShapeData::Ellipse(_) | ShapeData::Line(_) => {}
                },
                Command::Stroke {
                    shape,
                    stroke,
                    paint,
                } => match shape {
                    ShapeData::RoundedRect(rounded) => recorded.rounded_strokes.push((
                        rounded.rect(),
                        rounded.radii(),
                        paint.clone(),
                        stroke.width,
                    )),
                    ShapeData::Circle(circle) => {
                        recorded
                            .circle_strokes
                            .push((*circle, paint.clone(), stroke.width));
                    }
                    ShapeData::Line(line) => {
                        recorded
                            .line_strokes
                            .push((*line, paint.clone(), stroke.width));
                    }
                    ShapeData::Path { .. } => recorded.path_strokes += 1,
                    ShapeData::Rect(_) | ShapeData::Continuous(_) | ShapeData::Ellipse(_) => {}
                },
                Command::Shadow { shadow, .. } => recorded.shadows.push(*shadow),
                Command::BeginTransform { .. } => recorded.transforms += 1,
                Command::Glyphs { .. }
                | Command::Image { .. }
                | Command::Picture { .. }
                | Command::BeginClip { .. }
                | Command::BeginGroup { .. }
                | Command::End => {}
            }
        }
        recorded
    }
}

/// The colour of a solid paint.
///
/// # Panics
/// Panics when the paint is a gradient or image.
#[allow(clippy::redundant_pub_crate, reason = "test-only module")]
pub(crate) fn solid(paint: &Paint) -> WorkingColor {
    let Paint::Solid(color) = paint else {
        panic!("expected a solid paint, recorded {paint:?}");
    };
    *color
}
