//! Inspects what a theme draw recorded: the display list a [`Recorder`]
//! produced, sorted by command and shape so tests assert on geometry and paint
//! without a renderer.
//!
//! A [`Recorder`] is handed out only inside [`Content::record`], so tests draw
//! through [`recorded`]: `recorded(|draw| draw_chrome(&colors, draw, …))` is
//! the whole harness.

use waterui_graphics::draw::kurbo::{Circle, Line, Rect, RoundedRectRadii};
use waterui_graphics::draw::{
    Command, Content, LayoutSize, Paint, Recorder, Shadow, ShapeData, WorkingColor,
};

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
    /// Snapshots `content` and sorts its commands.
    pub(crate) fn from(content: &mut Content) -> Self {
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
                | Command::Text { .. }
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

/// Records `body` on a fresh [`Recorder`] and sorts the commands it produced.
#[allow(clippy::redundant_pub_crate, reason = "test-only module")]
pub(crate) fn recorded(body: impl FnOnce(&mut Recorder)) -> Recorded {
    let mut content = Content::record(&LayoutSize::new(), body);
    Recorded::from(&mut content)
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
