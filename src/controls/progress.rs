//! The Material 3 progress looks: the linear bar with its track gap and stop
//! indicator, the circular indicator, and the expressive loading indicator.
//!
//! A determinate indicator's parts are functions of its value, so a value
//! change moves their recorded operands. An indeterminate indicator's motion
//! is keyframes the engine samples on its own clock, each segment along its
//! own cubic-bezier or spring; the look declares the tracks and records them
//! once, so a running indicator does no work per frame.

use core::f64::consts::{FRAC_PI_2, TAU};
use core::time::Duration;

use hydrolysis::draw::kurbo::{
    Affine, Arc, BezPath, Circle, Point, Rect, RoundedRect, RoundedRectRadii, Shape as _, Size,
    Stroke,
};
use hydrolysis::draw::{
    Curve, Draw as _, Easing, Fixed, Keyframes, Live, Paint, Recorder, Repeat, ShapeData, Spring,
    WorkingColor,
};
use hydrolysis::theme::{
    Custom, Driven, IndicatorLook, Inputs, LinearProgressLook, ProgressControl, ProgressSpec,
};
use waterui_core::Environment;
use waterui_core::SignalExt as _;
use waterui_graphics::color::Color;

use crate::color::{
    Primary, PrimaryContainer, SurfaceContainerHighest, Tertiary, TertiaryContainer,
};
use crate::controls::button::solid_paint;
use crate::dimensions::{
    PROGRESS_CIRCULAR_DIAMETER, PROGRESS_CIRCULAR_STROKE_WIDTH,
    PROGRESS_CIRCULAR_TRACK_ACTIVE_SPACE, PROGRESS_LINEAR_BAR_HEIGHT,
    PROGRESS_LINEAR_BAR_HORIZONTAL_INSET, PROGRESS_LINEAR_BAR_TOP_OFFSET,
    PROGRESS_LINEAR_LABEL_HEIGHT, PROGRESS_LINEAR_MIN_TRACK_WIDTH, PROGRESS_LINEAR_STOP_SIZE,
    PROGRESS_LINEAR_TRACK_ACTIVE_SPACE, PROGRESS_LINEAR_VALUE_LABEL_TOP_SPACING,
};
use crate::material_shapes::{material_shape_sequence, radii_to_path};
use crate::theme::motion;
use crate::theme::motion::tokens::{self, easing};

/// `LinearAnimationDuration` in Compose's `ProgressIndicator.kt`.
const LINEAR_INDETERMINATE_CYCLE: Duration = Duration::from_millis(1_750);
/// `CircularAnimationProgressDuration`: one full indeterminate cycle.
const CIRCULAR_CYCLE_DURATION: Duration = Duration::from_secs(6);
/// `CircularAnimationAdditionalRotationDuration`: how long each 90-degree
/// kick of the additional rotation takes.
const CIRCULAR_ADDITIONAL_ROTATION_DURATION: Duration = Duration::from_millis(300);
/// `CircularAnimationAdditionalRotationDelay`: the spacing between those kicks.
const CIRCULAR_ADDITIONAL_ROTATION_DELAY: Duration = Duration::from_millis(1_500);
/// `CircularGlobalRotationDegreesTarget`: three turns per cycle, linear.
const CIRCULAR_GLOBAL_ROTATION_DEGREES: f64 = 1_080.0;
/// `CircularAdditionalRotationDegreesTarget`, reached in four 90-degree steps.
const CIRCULAR_ADDITIONAL_ROTATION_DEGREES: f64 = 360.0;
/// How many kicks the additional rotation makes per cycle.
const CIRCULAR_ADDITIONAL_ROTATION_STEPS: u32 = 4;
/// `CircularIndeterminateMinProgress`: the shortest the arc gets.
const CIRCULAR_INDETERMINATE_MIN_PROGRESS: f64 = 0.1;
/// `CircularIndeterminateMaxProgress`: the longest it grows to, half a cycle in.
const CIRCULAR_INDETERMINATE_MAX_PROGRESS: f64 = 0.87;

/// `LoadingIndicatorTokens.ContainerWidth` / `ContainerHeight`.
pub const LOADING_CONTAINER_SIZE: f64 = 48.0;
/// `LoadingIndicatorTokens.ActiveSize`: the shape inside that container.
pub const LOADING_INDICATOR_SIZE: f64 = 38.0;

// The loading indicator's timing is Compose's `LoadingIndicator.kt`
// (androidx.compose.material3): the morph loop waits `MorphIntervalMillis`
// between morphs, `globalRotation` runs `GlobalRotationDurationMillis` per
// turn, and the shape is drawn rotated by
// `progress * 90 + morphRotationTargetAngle + globalRotation`, where
// `progress` is the morph's spring and `morphRotationTargetAngle` advances
// by `QuarterRotation` per morph.

/// `MorphIntervalMillis`: how long each morph, and the rest after it, lasts.
const LOADING_MORPH_INTERVAL_MS: u32 = 650;
/// `GlobalRotationDurationMillis`: one turn of the steady rotation, linear.
const LOADING_GLOBAL_ROTATION_MS: u32 = 4_666;
/// `QuarterRotation`: each morph also kicks the shape a quarter turn on.
const LOADING_QUARTER_ROTATION: f64 = FRAC_PI_2;
/// The morph's `spring(dampingRatio = 0.6f, stiffness = 200f)`.
const LOADING_MORPH_SPRING_DAMPING_RATIO: f64 = 0.6;
const LOADING_MORPH_SPRING_STIFFNESS: f64 = 200.0;

#[derive(Debug, Clone, Copy)]
struct Segment {
    at: f64,
    value: f64,
    easing: Option<(f32, f32, f32, f32)>,
}

const PRIMARY_SCALE: &[Segment] = &[
    Segment {
        at: 0.0,
        value: 0.08,
        easing: None,
    },
    Segment {
        at: 0.3665,
        value: 0.08,
        easing: Some((0.334_731, 0.12482, 0.785_844, 1.0)),
    },
    Segment {
        at: 0.6915,
        value: 0.661_479,
        easing: Some((0.06, 0.11, 0.6, 1.0)),
    },
    Segment {
        at: 1.0,
        value: 0.08,
        easing: None,
    },
];
const PRIMARY_TRANSLATE: &[Segment] = &[
    Segment {
        at: 0.0,
        value: 0.0,
        easing: None,
    },
    Segment {
        at: 0.20,
        value: 0.0,
        easing: Some((0.5, 0.0, 0.701_732, 0.495_819)),
    },
    Segment {
        at: 0.5915,
        value: 83.6714,
        easing: Some((0.302_435, 0.381_352, 0.55, 0.956_352)),
    },
    Segment {
        at: 1.0,
        value: 200.611,
        easing: None,
    },
];
const SECONDARY_SCALE: &[Segment] = &[
    Segment {
        at: 0.0,
        value: 0.08,
        easing: Some((0.205_028, 0.057_051, 0.57661, 0.453_971)),
    },
    Segment {
        at: 0.1915,
        value: 0.457_104,
        easing: Some((0.152_313, 0.196_432, 0.648_374, 1.00432)),
    },
    Segment {
        at: 0.4415,
        value: 0.72796,
        easing: Some((0.257_759, -0.003_163, 0.211_762, 1.38179)),
    },
    Segment {
        at: 1.0,
        value: 0.08,
        easing: None,
    },
];
const SECONDARY_TRANSLATE: &[Segment] = &[
    Segment {
        at: 0.0,
        value: 0.0,
        easing: Some((0.15, 0.0, 0.515_058, 0.409_685)),
    },
    Segment {
        at: 0.25,
        value: 37.6519,
        easing: Some((0.31033, 0.284_058, 0.8, 0.733_712)),
    },
    Segment {
        at: 0.4835,
        value: 84.3862,
        easing: Some((0.4, 0.627_035, 0.6, 0.902_026)),
    },
    Segment {
        at: 1.0,
        value: 160.278,
        easing: None,
    },
];
/// Where each indeterminate bar starts, in percent of the track's width.
const PRIMARY_INITIAL_INSET: f64 = -145.167;
const SECONDARY_INITIAL_INSET: f64 = -54.8889;
/// How far a bar's rounded half reaches past its end, in track widths: more
/// than any bar is long.
const BAR_REACH: f64 = 4.0;

/// The look of a linear progress indicator.
pub fn linear(spec: &ProgressSpec, env: &Environment) -> LinearProgressLook {
    let rtl = spec.direction().is_right_to_left();
    let control = ProgressControl::new(
        Size::new(PROGRESS_LINEAR_MIN_TRACK_WIDTH, PROGRESS_LINEAR_BAR_HEIGHT),
        motion::progress_linear(),
    );
    let control = if spec.determinate() {
        control.part(linear_determinate(spec, rtl, env))
    } else {
        control.part(linear_indeterminate(spec, rtl, env))
    };
    LinearProgressLook::new(control, PROGRESS_LINEAR_BAR_HORIZONTAL_INSET)
        .label_band(PROGRESS_LINEAR_LABEL_HEIGHT, PROGRESS_LINEAR_BAR_TOP_OFFSET)
        .value_label_band(PROGRESS_LINEAR_VALUE_LABEL_TOP_SPACING)
}

/// The look of a circular progress indicator.
pub fn circular(spec: &ProgressSpec, env: &Environment) -> IndicatorLook {
    let control = ProgressControl::new(
        Size::new(PROGRESS_CIRCULAR_DIAMETER, PROGRESS_CIRCULAR_DIAMETER),
        motion::progress_circular(),
    );
    IndicatorLook::new(if spec.determinate() {
        control.part(circular_determinate(spec, env))
    } else {
        control.part(circular_indeterminate(spec, env))
    })
}

/// The look of the expressive loading indicator.
pub fn loading(spec: &ProgressSpec, env: &Environment) -> IndicatorLook {
    IndicatorLook::new(
        ProgressControl::new(
            Size::new(LOADING_CONTAINER_SIZE, LOADING_CONTAINER_SIZE),
            motion::progress_circular(),
        )
        .part(loading_part(spec, env)),
    )
}

// ---------------------------------------------------------------------------
// Paints
// ---------------------------------------------------------------------------

/// The indicator's active colour: the tint, or the four accent colours in
/// turn over `cycle` when the spec asks for them. The four follow their
/// theme tokens: a colour-scheme change starts the cycle again from the new
/// colours.
fn active_paint(spec: &ProgressSpec, cycle: Duration, env: &Environment) -> Driven<Paint, f64> {
    if !spec.four_color() {
        return solid_paint(spec.tint(), env);
    }
    let [primary, primary_container, tertiary, tertiary_container] = [
        Color::from(Primary),
        Color::from(PrimaryContainer),
        Color::from(Tertiary),
        Color::from(TertiaryContainer),
    ]
    .map(|color| color.resolve(env));
    Driven::repeating_following(
        primary
            .zip(&primary_container)
            .zip(&tertiary)
            .zip(&tertiary_container),
        move |(((primary, primary_container), tertiary), tertiary_container): &(
            ((WorkingColor, WorkingColor), WorkingColor),
            WorkingColor,
        ),
              _frame| {
            four_colors(
                &[
                    Paint::from(*primary),
                    Paint::from(*primary_container),
                    Paint::from(*tertiary),
                    Paint::from(*tertiary_container),
                ],
                cycle,
            )
        },
        Repeat::Forever,
    )
}

/// The palette's colours held for a stretch of the cycle each, and blended
/// into the next over a tenth of it.
fn four_colors(palette: &[Paint; 4], cycle: Duration) -> Keyframes<Paint> {
    let curve = Curve::linear(cycle);
    let [first, second, third, fourth] = palette;
    Keyframes::new(cycle, first.clone())
        .then(0.15, curve, first.clone())
        .then(0.25, curve, second.clone())
        .then(0.40, curve, second.clone())
        .then(0.50, curve, third.clone())
        .then(0.65, curve, third.clone())
        .then(0.75, curve, fourth.clone())
        .then(0.90, curve, fourth.clone())
        .then(1.0, curve, first.clone())
}

// ---------------------------------------------------------------------------
// Keyframes
// ---------------------------------------------------------------------------

/// The cubic-bezier easing token as a curve of a `cycle`-long track, whose
/// own length the engine ignores: a segment's share of the cycle sets it.
fn curve(cycle: Duration, token: tokens::Easing) -> Curve {
    Curve::bezier(
        cycle,
        f64::from(token.x1),
        f64::from(token.y1),
        f64::from(token.x2),
        f64::from(token.y2),
    )
}

/// `token` run backwards: the easing of a move that is another one in
/// reverse, `1 - ease(1 - t)`.
fn reversed(token: tokens::Easing) -> tokens::Easing {
    tokens::Easing::bezier(
        1.0 - token.x2,
        1.0 - token.y2,
        1.0 - token.x1,
        1.0 - token.y1,
    )
}

/// The track of `segments` over `cycle`: each reaches the next keyframe
/// along its own cubic-bezier easing, or linearly when it has none. `value`
/// builds the track's value from a segment's.
fn segment_track<T>(
    cycle: Duration,
    segments: &[Segment],
    value: impl Fn(f64) -> T,
) -> Keyframes<T> {
    let (first, rest) = segments
        .split_first()
        .expect("indeterminate progress segments must not be empty");
    rest.iter().zip(segments).fold(
        Keyframes::new(cycle, value(first.value)),
        |keyframes, (end, start)| {
            let easing = start.easing.map_or_else(
                || Curve::linear(cycle),
                |(x1, y1, x2, y2)| curve(cycle, tokens::Easing::bezier(x1, y1, x2, y2)),
            );
            keyframes.then(end.at, easing, value(end.value))
        },
    )
}

// ---------------------------------------------------------------------------
// Linear
// ---------------------------------------------------------------------------

/// A pill `[x0, x1]` wide across `frame`'s height: `CornerFull` ends.
fn pill(frame: Rect, x0: f64, x1: f64) -> RoundedRect {
    RoundedRect::from_rect(
        Rect::new(x0, frame.y0, x1.max(x0), frame.y1),
        RoundedRectRadii::from_single_radius(frame.height() / 2.0),
    )
}

/// A pill `[x0, x1]` wide across `frame`'s height, mirrored for a
/// right-to-left indicator.
fn bar(frame: Rect, x0: f64, x1: f64, rtl: bool) -> ShapeData {
    let (x0, x1) = if rtl {
        (frame.x0 + frame.x1 - x1, frame.x0 + frame.x1 - x0)
    } else {
        (x0, x1)
    };
    ShapeData::of(&pill(frame, x0, x1))
}

/// The three pieces of a determinate linear bar at `value`: the track, kept
/// clear of the active indicator by `TrackActiveSpace`, the active indicator,
/// and the stop indicator at the track's far end, which is gone once the
/// active indicator reaches it.
fn linear_pieces(frame: Rect, value: f64, rtl: bool) -> (ShapeData, ShapeData, ShapeData) {
    let end = frame.width().mul_add(value, frame.x0);
    let start = (end + PROGRESS_LINEAR_TRACK_ACTIVE_SPACE).min(frame.x1);
    let stop_center = frame.x1 - PROGRESS_LINEAR_STOP_SIZE / 2.0;
    let stop = Circle::new(
        Point::new(
            if rtl {
                frame.x0 + frame.x1 - stop_center
            } else {
                stop_center
            },
            frame.center().y,
        ),
        if stop_center > start {
            PROGRESS_LINEAR_STOP_SIZE / 2.0
        } else {
            0.0
        },
    );
    (
        bar(frame, start, frame.x1, rtl),
        bar(frame, frame.x0, end, rtl),
        ShapeData::of(&stop),
    )
}

/// A determinate linear bar: three shapes, each with its paint.
type LinearBar = Custom<
    f64,
    (
        Driven<ShapeData, f64>,
        Driven<Paint, f64>,
        Driven<ShapeData, f64>,
        Driven<Paint, f64>,
        Driven<ShapeData, f64>,
        Driven<Paint, f64>,
    ),
>;

/// An indeterminate linear bar: the track's paint, then for each of the two
/// moving bars the translation of its start, the translation that lengthens
/// it, and its paint.
type MovingBars = Custom<
    f64,
    (
        Driven<Paint, f64>,
        Driven<Affine, f64>,
        Driven<Affine, f64>,
        Driven<Paint, f64>,
        Driven<Affine, f64>,
        Driven<Affine, f64>,
        Driven<Paint, f64>,
    ),
>;

/// A circular indicator: the track and the arc, each a stroke with its paint.
type Ring = Custom<
    f64,
    (
        Driven<Stroke, f64>,
        Driven<Paint, f64>,
        Driven<Stroke, f64>,
        Driven<Paint, f64>,
    ),
>;

/// The loading indicator: one shape with its paint.
type Blob = Custom<f64, (Driven<ShapeData, f64>, Driven<Paint, f64>)>;

fn linear_determinate(spec: &ProgressSpec, rtl: bool, env: &Environment) -> LinearBar {
    let piece = |pick: fn((ShapeData, ShapeData, ShapeData)) -> ShapeData| {
        Driven::new(move |inputs: &Inputs<'_, f64>| {
            pick(linear_pieces(inputs.frame, *inputs.value, rtl))
        })
    };
    Custom::new(
        (
            piece(|(track, ..)| track),
            solid_paint(SurfaceContainerHighest, env),
            piece(|(_, active, _)| active),
            solid_paint(spec.tint(), env),
            piece(|(.., stop)| stop),
            solid_paint(spec.tint(), env),
        ),
        |recorder, _frame, (track, track_paint, active, active_paint, stop, stop_paint)| {
            recorder.fill(track, track_paint);
            recorder.fill(active, active_paint);
            recorder.fill(stop, stop_paint);
        },
    )
}

/// How far a bar's start travels across `frame`, as the translation of the
/// layer it is drawn in.
fn bar_start(frame: Rect, translate: &[Segment]) -> Keyframes<Affine> {
    segment_track(LINEAR_INDETERMINATE_CYCLE, translate, |percent| {
        Affine::translate((frame.width() * percent / 100.0, 0.0))
    })
}

/// How far a bar's end travels from its start across `frame`: the bar's
/// length, `scale` of the track's width.
fn bar_length(frame: Rect, scale: &[Segment]) -> Keyframes<Affine> {
    segment_track(LINEAR_INDETERMINATE_CYCLE, scale, |share| {
        Affine::translate((frame.width() * share, 0.0))
    })
}

/// One indeterminate bar. It starts `initial_inset` percent of the track's
/// width in and spans from its start, which travels by `start`, to its end,
/// which travels from there by `length`: a pill reaching right from the
/// start, clipped by one reaching left from the end. The two travel with
/// their own tracks, each segment on its own easing, so the bar's length
/// and its position are both exact.
fn record_bar(
    recorder: &mut Recorder,
    frame: Rect,
    initial_inset: f64,
    start: Live<Affine>,
    length: Live<Affine>,
    paint: Live<Paint>,
) {
    let origin = frame.width().mul_add(initial_inset / 100.0, frame.x0);
    let reach = frame.width() * BAR_REACH;
    recorder.transform(start, |recorder| {
        recorder.clip(pill(frame, origin, origin + reach), |recorder| {
            recorder.transform(length, |recorder| {
                recorder.fill(pill(frame, origin - reach, origin), paint);
            });
        });
    });
}

fn linear_indeterminate(spec: &ProgressSpec, rtl: bool, env: &Environment) -> MovingBars {
    // The two bars share one colour cycle, twice the bars' own.
    let paint = active_paint(spec, LINEAR_INDETERMINATE_CYCLE * 2, env);
    Custom::new(
        (
            solid_paint(SurfaceContainerHighest, env),
            Driven::repeating(|frame| bar_start(frame, PRIMARY_TRANSLATE), Repeat::Forever),
            Driven::repeating(|frame| bar_length(frame, PRIMARY_SCALE), Repeat::Forever),
            paint.clone(),
            Driven::repeating(
                |frame| bar_start(frame, SECONDARY_TRANSLATE),
                Repeat::Forever,
            ),
            Driven::repeating(|frame| bar_length(frame, SECONDARY_SCALE), Repeat::Forever),
            paint,
        ),
        move |recorder,
              frame,
              (
            track_paint,
            primary_start,
            primary_length,
            primary_paint,
            secondary_start,
            secondary_length,
            secondary_paint,
        )| {
            let draw = |recorder: &mut Recorder| {
                recorder.fill(pill(frame, frame.x0, frame.x1), track_paint);
                recorder.clip(frame, |recorder| {
                    record_bar(
                        recorder,
                        frame,
                        PRIMARY_INITIAL_INSET,
                        primary_start,
                        primary_length,
                        primary_paint,
                    );
                    record_bar(
                        recorder,
                        frame,
                        SECONDARY_INITIAL_INSET,
                        secondary_start,
                        secondary_length,
                        secondary_paint,
                    );
                });
            };
            if rtl {
                let mirror = Affine::translate((frame.x0 + frame.x1, 0.0))
                    * Affine::scale_non_uniform(-1.0, 1.0);
                recorder.transform(Fixed(mirror), draw);
            } else {
                draw(recorder);
            }
        },
    )
}

// ---------------------------------------------------------------------------
// Circular
// ---------------------------------------------------------------------------

/// The ring a circular indicator strokes, in `frame`, starting at the top
/// and running clockwise, and its circumference.
fn ring(frame: Rect) -> (BezPath, f64) {
    let radius =
        (frame.width().min(frame.height()) - PROGRESS_CIRCULAR_STROKE_WIDTH).max(0.0) / 2.0;
    (
        Arc::new(frame.center(), (radius, radius), -FRAC_PI_2, TAU, 0.0).into_path(0.1),
        TAU * radius,
    )
}

/// The dashes that stroke the active arc covering `value` of the ring.
fn active_stroke(circumference: f64, value: f64) -> Stroke {
    Stroke::new(PROGRESS_CIRCULAR_STROKE_WIDTH)
        .with_dashes(0.0, [value * circumference, circumference])
}

/// The dashes that stroke the track around an active arc covering `value` of
/// the ring: the rest of it, clear of the arc by `TrackActiveSpace` at both
/// ends. The pattern is one dash and one ring of gap, shifted so the dash
/// begins where the gap after the arc ends.
fn track_stroke(circumference: f64, value: f64) -> Stroke {
    let gap = PROGRESS_CIRCULAR_TRACK_ACTIVE_SPACE;
    let active = value * circumference;
    let dash = gap.mul_add(-2.0, circumference - active).max(0.0);
    let period = dash + circumference;
    Stroke::new(PROGRESS_CIRCULAR_STROKE_WIDTH).with_dashes(
        (period - (active + gap)).rem_euclid(period),
        [dash, circumference],
    )
}

fn circular_determinate(spec: &ProgressSpec, env: &Environment) -> Ring {
    Custom::new(
        (
            Driven::new(|inputs: &Inputs<'_, f64>| {
                let (_, circumference) = ring(inputs.frame);
                track_stroke(circumference, *inputs.value)
            }),
            solid_paint(SurfaceContainerHighest, env),
            Driven::new(|inputs: &Inputs<'_, f64>| {
                active_stroke(ring(inputs.frame).1, *inputs.value)
            }),
            solid_paint(spec.tint(), env),
        ),
        |recorder, frame, (track, track_paint, active, active_paint)| {
            let (path, _) = ring(frame);
            recorder.stroke(path.clone(), track, track_paint);
            recorder.stroke(path, active, active_paint);
        },
    )
}

/// The arc's dashes when it covers `share` of a ring of `circumference`: one
/// dash as long as the arc, and the rest of the ring as gap.
fn arc_stroke(circumference: f64, share: f64) -> Stroke {
    let arc = share * circumference;
    Stroke::new(PROGRESS_CIRCULAR_STROKE_WIDTH).with_dashes(0.0, [arc, circumference - arc])
}

/// The arc's length over a cycle: growing from `CircularIndeterminateMinProgress`
/// to `CircularIndeterminateMaxProgress` by mid-cycle along the standard
/// easing, then shrinking back along the same one in reverse.
fn arc_keyframes(circumference: f64) -> Keyframes<Stroke> {
    let cycle = CIRCULAR_CYCLE_DURATION;
    Keyframes::new(
        cycle,
        arc_stroke(circumference, CIRCULAR_INDETERMINATE_MIN_PROGRESS),
    )
    .then(
        0.5,
        curve(cycle, easing::STANDARD),
        arc_stroke(circumference, CIRCULAR_INDETERMINATE_MAX_PROGRESS),
    )
    .then(
        1.0,
        curve(cycle, reversed(easing::STANDARD)),
        arc_stroke(circumference, CIRCULAR_INDETERMINATE_MIN_PROGRESS),
    )
}

/// The ring's steady global rotation over a cycle, in radians: three turns
/// linearly. It is the outer stage of the ring's rotation.
fn circular_turn() -> Keyframes<f64> {
    let cycle = CIRCULAR_CYCLE_DURATION;
    Keyframes::new(cycle, 0.0).then(
        1.0,
        Curve::linear(cycle),
        CIRCULAR_GLOBAL_ROTATION_DEGREES.to_radians(),
    )
}

/// The ring's additional rotation over a cycle, in radians: four 90-degree
/// kicks, each eased with the emphasized-decelerate curve over its 300 ms and
/// held until the next one 1.5 s after it began, a whole turn in all. It is
/// the inner stage of the ring's rotation.
fn circular_kicks() -> Keyframes<f64> {
    let cycle = CIRCULAR_CYCLE_DURATION;
    let total = cycle.as_secs_f64();
    let delay = CIRCULAR_ADDITIONAL_ROTATION_DELAY.as_secs_f64();
    let kick = CIRCULAR_ADDITIONAL_ROTATION_DURATION.as_secs_f64();
    let steps = CIRCULAR_ADDITIONAL_ROTATION_STEPS;
    let step = CIRCULAR_ADDITIONAL_ROTATION_DEGREES / f64::from(steps);
    let kick_curve = curve(cycle, easing::EMPHASIZED_DECELERATE);
    let hold_curve = Curve::linear(cycle);
    (0..steps).fold(Keyframes::new(cycle, 0.0), |keyframes, index| {
        let turned = (step * f64::from(index + 1)).to_radians();
        let next = if index + 1 == steps {
            1.0
        } else {
            f64::from(index + 1) * delay / total
        };
        keyframes
            .then(
                f64::from(index).mul_add(delay, kick) / total,
                kick_curve,
                turned,
            )
            .then(next, hold_curve, turned)
    })
}

fn circular_indeterminate(spec: &ProgressSpec, env: &Environment) -> Ring {
    Custom::new(
        (
            Driven::new(|_inputs: &Inputs<'_, f64>| Stroke::new(PROGRESS_CIRCULAR_STROKE_WIDTH)),
            solid_paint(SurfaceContainerHighest, env),
            Driven::repeating(|frame| arc_keyframes(ring(frame).1), Repeat::Forever),
            active_paint(spec, CIRCULAR_CYCLE_DURATION, env),
        ),
        |recorder, frame, (track, track_paint, arc, arc_paint)| {
            let (path, _) = ring(frame);
            recorder.stroke(path.clone(), track, track_paint);
            recorder.stroke(path, arc, arc_paint);
        },
    )
    // The whole ring turns as one layer, so the arc keeps its place in the
    // layer and only its length is keyframed.
    .rotating(circular_kicks(), Repeat::Forever)
    .rotating(circular_turn(), Repeat::Forever)
}

// ---------------------------------------------------------------------------
// Loading
// ---------------------------------------------------------------------------

/// The spring Compose morphs the loading indicator's shapes with.
fn morph_spring() -> Spring {
    Spring {
        response: TAU / LOADING_MORPH_SPRING_STIFFNESS.sqrt(),
        damping: LOADING_MORPH_SPRING_DAMPING_RATIO,
    }
}

/// The outlines the loading indicator morphs through, in `frame`: the shape
/// is drawn at `ActiveSize` inside a container box, so it never touches the
/// container's edge.
fn loading_outlines(frame: Rect) -> Vec<ShapeData> {
    let scale = LOADING_INDICATOR_SIZE.min(frame.width().min(frame.height())) / 2.0;
    material_shape_sequence()
        .iter()
        .map(|radii| ShapeData::of(&radii_to_path(radii, frame.center(), scale)))
        .collect()
}

/// The loop through every shape in turn and back to the first, each morph a
/// spring over its interval.
fn loading_morph(frame: Rect) -> Keyframes<ShapeData> {
    let outlines = loading_outlines(frame);
    let count = outlines.len();
    let steps = u32::try_from(count).expect("the shape sequence is seven long");
    let spring = Easing::Spring(morph_spring());
    let first = outlines[0].clone();
    outlines
        .into_iter()
        .cycle()
        .skip(1)
        .take(count)
        .enumerate()
        .fold(
            Keyframes::new(
                Duration::from_millis(u64::from(LOADING_MORPH_INTERVAL_MS)) * steps,
                first,
            ),
            |keyframes, (index, outline)| {
                let at = f64::from(u32::try_from(index + 1).expect("bounded by the shape count"))
                    / f64::from(steps);
                keyframes.then(at, spring, outline)
            },
        )
}

/// How many morphs make the quarter-turn kicks a whole turn.
const LOADING_KICKS_PER_TURN: u32 = 4;

/// The loading indicator's steady rotation, in radians: `globalRotation`,
/// one turn linearly over `GlobalRotationDurationMillis`. It is the outer
/// stage of the rotation, repeating on its own period.
fn loading_turn() -> Keyframes<f64> {
    let period = Duration::from_millis(u64::from(LOADING_GLOBAL_ROTATION_MS));
    Keyframes::new(period, 0.0).then(1.0, Curve::linear(period), TAU)
}

/// The loading indicator's kicks, in radians: Compose's `progress * 90 +
/// morphRotationTargetAngle`, whose target starts at a quarter turn and
/// gains one per morph while `progress` is the morph's spring. Four morphs
/// turn it a whole turn, so the track is those four and repeats on that
/// period. It is the inner stage of the rotation.
fn loading_kicks() -> Keyframes<f64> {
    let spring = Easing::Spring(morph_spring());
    let period = Duration::from_millis(u64::from(
        LOADING_KICKS_PER_TURN * LOADING_MORPH_INTERVAL_MS,
    ));
    (1..=LOADING_KICKS_PER_TURN).fold(
        Keyframes::new(period, LOADING_QUARTER_ROTATION),
        |keyframes, kick| {
            keyframes.then(
                f64::from(kick) / f64::from(LOADING_KICKS_PER_TURN),
                spring,
                LOADING_QUARTER_ROTATION * f64::from(kick + 1),
            )
        },
    )
}

fn loading_part(spec: &ProgressSpec, env: &Environment) -> Blob {
    Custom::new(
        (
            Driven::repeating(loading_morph, Repeat::Forever),
            active_paint(
                spec,
                Duration::from_millis(u64::from(LOADING_GLOBAL_ROTATION_MS)),
                env,
            ),
        ),
        |recorder, _frame, (outline, paint)| {
            recorder.fill(outline, paint);
        },
    )
    // Two rotating stages, the kicks inside the steady turn, each repeating
    // on its own period.
    .rotating(loading_kicks(), Repeat::Forever)
    .rotating(loading_turn(), Repeat::Forever)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydrolysis::Style as _;
    use hydrolysis::draw::curve_value;
    use hydrolysis::theme::SpecTint;
    use waterui_core::layout::LayoutDirection;

    fn env() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    fn spec(style: waterui::component::progress::ProgressStyle, determinate: bool) -> ProgressSpec {
        ProgressSpec::new(
            style,
            determinate,
            LayoutDirection::LeftToRight,
            SpecTint::default(),
        )
    }

    fn track() -> Rect {
        Rect::new(8.0, 0.0, 328.0, 4.0)
    }

    fn rounded(shape: &ShapeData) -> (Rect, RoundedRectRadii) {
        let ShapeData::RoundedRect(rounded) = shape else {
            panic!("expected a rounded rect, got {shape:?}")
        };
        (rounded.rect(), rounded.radii())
    }

    /// The value of `segments` at `phase` of their cycle, read directly
    /// off the table: the oracle the keyframes are checked against.
    fn sample_segments(segments: &[Segment], phase: f64) -> f64 {
        for window in segments.windows(2) {
            let (start, end) = (window[0], window[1]);
            if phase < start.at || phase > end.at {
                continue;
            }
            let local = ((phase - start.at) / (end.at - start.at)).clamp(0.0, 1.0);
            let eased = start.easing.map_or(local, |(x1, y1, x2, y2)| {
                curve_value(
                    &Curve::bezier(
                        Duration::from_secs(1),
                        f64::from(x1),
                        f64::from(y1),
                        f64::from(x2),
                        f64::from(y2),
                    ),
                    local,
                )
            });
            return (end.value - start.value).mul_add(eased, start.value);
        }
        segments.last().expect("segments").value
    }

    /// The value of `keyframes` at `phase` of its cycle.
    macro_rules! at_phase {
        ($keyframes:expr, $phase:expr) => {
            $keyframes
                .sample($keyframes.duration().mul_f64($phase), Repeat::Forever)
                .expect("a repeating track has a value")
        };
    }

    /// The indeterminate circular motion follows Compose's
    /// `CircularProgressIndicator`: a 6s cycle whose arc grows from
    /// `CircularIndeterminateMinProgress` to `CircularIndeterminateMaxProgress`
    /// at the halfway point and shrinks back.
    #[test]
    fn circular_indeterminate_sweep_follows_the_compose_cycle() {
        let circumference = 100.0;
        let keyframes = arc_keyframes(circumference);
        let share = |phase: f64| at_phase!(keyframes, phase).dash_pattern[0] / circumference;
        assert!(
            (share(0.0) - CIRCULAR_INDETERMINATE_MIN_PROGRESS).abs() < 1e-6,
            "the cycle starts at the minimum arc, was {}",
            share(0.0)
        );
        assert!(
            (share(0.5) - CIRCULAR_INDETERMINATE_MAX_PROGRESS).abs() < 1e-6,
            "the arc peaks at the half cycle, was {}",
            share(0.5)
        );
        // The cycle is closed: it ends where it began, so a repeat does not jump.
        assert!(
            (share(0.999_999) - CIRCULAR_INDETERMINATE_MIN_PROGRESS).abs() < 1e-3,
            "the cycle returns to the minimum arc, was {}",
            share(0.999_999)
        );
        // Each half follows its easing exactly: the standard curve growing,
        // the same curve in reverse shrinking.
        let span = CIRCULAR_INDETERMINATE_MAX_PROGRESS - CIRCULAR_INDETERMINATE_MIN_PROGRESS;
        for step in 0..=120 {
            let phase = f64::from(step) / 120.0 * 0.999_999;
            let half = if phase <= 0.5 {
                phase * 2.0
            } else {
                (1.0 - phase) * 2.0
            };
            let expected = span.mul_add(
                curve_value(&curve(CIRCULAR_CYCLE_DURATION, easing::STANDARD), half),
                CIRCULAR_INDETERMINATE_MIN_PROGRESS,
            );
            assert!(
                (share(phase) - expected).abs() < 1e-4,
                "the arc at {phase} is {}, not {expected}",
                share(phase)
            );
        }
    }

    /// The additional rotation reaches a full turn by the end of the cycle
    /// in four 90-degree steps.
    #[test]
    fn circular_additional_rotation_completes_a_turn_in_four_steps() {
        let rotation = circular_kicks();
        let additional = |seconds: f64| at_phase!(rotation, seconds / 6.0).to_degrees();
        assert!(additional(0.0).abs() < 1e-6);
        // Compose's keyframes ramp each step over 300ms and then hold it until
        // the next 1.5s boundary: 90 at 300ms, still 90 at 1.4s, 180 at 1.8s.
        assert!((additional(0.3) - 90.0).abs() < 1e-6, "{}", additional(0.3));
        assert!((additional(1.4) - 90.0).abs() < 1e-6, "{}", additional(1.4));
        assert!(
            (additional(1.8) - 180.0).abs() < 1e-6,
            "{}",
            additional(1.8)
        );
        assert!(
            (additional(5.999_999) - CIRCULAR_ADDITIONAL_ROTATION_DEGREES).abs() < 1e-3,
            "a full cycle turns the arc once more"
        );
        // Monotonic: the arc never rotates backwards.
        let mut previous = 0.0_f64;
        for step in 0..=600 {
            let value = additional(f64::from(step) / 100.0 * 0.999_999);
            assert!(
                value >= previous - 1e-9,
                "rotation went backwards at {step}"
            );
            previous = value;
        }
    }

    /// Each stage turns a whole number of times over its cycle, so the
    /// repeat does not jump, and the global stage is steady: three turns in
    /// six seconds.
    #[test]
    fn circular_rotation_stages_repeat_seamlessly() {
        let (kicks, turn) = (circular_kicks(), circular_turn());
        for stage in [&kicks, &turn] {
            let turns = at_phase!(stage, 0.999_999_9) / TAU;
            assert!(
                (turns - turns.round()).abs() < 1e-5,
                "the cycle ends a whole number of turns on, was {turns}"
            );
        }
        let third = at_phase!(turn, 1.0 / 3.0) / TAU;
        assert!((third - 1.0).abs() < 1e-9, "a turn every two seconds");
    }

    /// Both bars' tracks are the table's segments, each on its own easing:
    /// the start and the length at any phase equal the table evaluated
    /// directly.
    #[test]
    fn indeterminate_bars_follow_their_segment_tables() {
        let frame = track();
        for (translate, scale) in [
            (PRIMARY_TRANSLATE, PRIMARY_SCALE),
            (SECONDARY_TRANSLATE, SECONDARY_SCALE),
        ] {
            let start = bar_start(frame, translate);
            let length = bar_length(frame, scale);
            for step in 0..=200 {
                let phase = f64::from(step) / 200.0 * 0.999_999;
                let moved = at_phase!(start, phase).translation().x;
                let expected = frame.width() * sample_segments(translate, phase) / 100.0;
                assert!(
                    (moved - expected).abs() < 1e-3,
                    "start at {phase}: {moved} vs {expected}"
                );
                let long = at_phase!(length, phase).translation().x;
                let expected = frame.width() * sample_segments(scale, phase);
                assert!(
                    (long - expected).abs() < 1e-3,
                    "length at {phase}: {long} vs {expected}"
                );
            }
        }
    }

    /// Expressive gives every progress element `CornerFull` ends: the moving
    /// indeterminate segments cap at half the track height, same as the
    /// determinate fill.
    #[test]
    fn indeterminate_segments_draw_with_rounded_caps() {
        let frame = track();
        let (_, radii) = rounded(&bar(frame, 20.0, 120.0, false));
        assert_eq!(radii, RoundedRectRadii::from(frame.height() / 2.0));
    }

    /// The indeterminate bars start and end off the track, so the repeat
    /// does not show a segment appear.
    #[test]
    fn indeterminate_bars_leave_the_track_between_cycles() {
        let frame = track();
        let origin = frame
            .width()
            .mul_add(PRIMARY_INITIAL_INSET / 100.0, frame.x0);
        let (start, length) = (
            bar_start(frame, PRIMARY_TRANSLATE),
            bar_length(frame, PRIMARY_SCALE),
        );
        let (start, length) = (
            at_phase!(start, 0.0).translation().x,
            at_phase!(length, 0.0).translation().x,
        );
        assert!(
            origin + start + length < frame.x0,
            "the primary bar starts left of the track"
        );
    }

    /// The determinate bar leaves `TrackActiveSpace` between the active
    /// indicator and the track, and the stop indicator disappears under the
    /// active indicator.
    #[test]
    fn determinate_linear_pieces_follow_the_value() {
        let frame = track();
        let (track_shape, active, stop) = linear_pieces(frame, 0.5, false);
        let (active_rect, _) = rounded(&active);
        let (track_rect, _) = rounded(&track_shape);
        assert!((active_rect.x1 - 168.0).abs() < 1e-9);
        assert!(
            (track_rect.x0 - active_rect.x1 - PROGRESS_LINEAR_TRACK_ACTIVE_SPACE).abs() < 1e-9,
            "the track starts a gap after the active indicator"
        );
        let ShapeData::Circle(stop) = stop else {
            panic!("the stop indicator is a circle")
        };
        assert!((stop.radius - PROGRESS_LINEAR_STOP_SIZE / 2.0).abs() < 1e-9);
        assert!((stop.center.x - (frame.x1 - PROGRESS_LINEAR_STOP_SIZE / 2.0)).abs() < 1e-9);

        let (.., stop) = linear_pieces(frame, 1.0, false);
        let ShapeData::Circle(stop) = stop else {
            panic!("the stop indicator is a circle")
        };
        assert!(stop.radius.abs() < 1e-9, "a full bar covers the stop");
    }

    /// A right-to-left bar fills from the right edge.
    #[test]
    fn determinate_linear_pieces_mirror_for_right_to_left() {
        let frame = track();
        let (_, active, _) = linear_pieces(frame, 0.25, true);
        let (rect, _) = rounded(&active);
        assert!((rect.x1 - frame.x1).abs() < 1e-9);
        assert!((rect.x0 - (frame.x1 - 80.0)).abs() < 1e-9);
    }

    /// The active arc and the track together cover the ring, apart from
    /// `TrackActiveSpace` at each of the two seams between them.
    #[test]
    fn circular_strokes_leave_the_track_gap() {
        let circumference = 100.0;
        for value in [0.0, 0.25, 0.5, 0.9] {
            let active = active_stroke(circumference, value);
            let track = track_stroke(circumference, value);
            let covered = value * circumference;
            assert!((active.dash_pattern[0] - covered).abs() < 1e-9);
            let gap = PROGRESS_CIRCULAR_TRACK_ACTIVE_SPACE;
            assert!(
                (track.dash_pattern[0] - (circumference - covered - gap - gap)).abs() < 1e-9,
                "the track is what the arc and its two gaps leave"
            );
            // The track's dash begins a gap past the arc's end.
            let period = track.dash_pattern[0] + track.dash_pattern[1];
            let begins = (period - track.dash_offset).rem_euclid(period);
            assert!(
                (begins - (covered + gap)).abs() < 1e-9,
                "the track begins a gap after the arc, at {begins}"
            );
        }
    }

    /// The indicators have the Compose progress tokens' sizes.
    #[test]
    fn looks_match_compose_progress_tokens() {
        use waterui::component::progress::ProgressStyle;

        let env = env();
        let linear = linear(&spec(ProgressStyle::Linear, true), &env);
        // LinearProgressIndicatorTokens.Height / TrackThickness
        assert!((linear.control().size().height - 4.0).abs() < f64::EPSILON);
        assert!((linear.minimum_width() - (80.0 + 8.0 + 8.0)).abs() < f64::EPSILON);
        // CircularProgressIndicatorTokens.Size
        let circular = circular(&spec(ProgressStyle::Circular, true), &env);
        assert!((circular.control().size().width - 40.0).abs() < f64::EPSILON);
        // CircularProgressIndicatorTokens.ActiveThickness / TrackThickness
        assert!((PROGRESS_CIRCULAR_STROKE_WIDTH - 4.0).abs() < f64::EPSILON);
        // TrackActiveSpace on both indicators
        assert!((PROGRESS_CIRCULAR_TRACK_ACTIVE_SPACE - 4.0).abs() < f64::EPSILON);
        assert!((PROGRESS_LINEAR_TRACK_ACTIVE_SPACE - 4.0).abs() < f64::EPSILON);
        // LinearProgressIndicatorTokens.StopSize
        assert!((PROGRESS_LINEAR_STOP_SIZE - 4.0).abs() < f64::EPSILON);
        let loading = loading(&spec(ProgressStyle::Loading, false), &env);
        assert!((loading.control().size().width - LOADING_CONTAINER_SIZE).abs() < f64::EPSILON);
    }

    /// The loading indicator's timing is Compose's `LoadingIndicator.kt`
    /// (`MorphIntervalMillis` 650, `GlobalRotationDurationMillis` 4666,
    /// `QuarterRotation` 90 degrees, `spring(dampingRatio = 0.6f,
    /// stiffness = 200f)`).
    #[test]
    fn loading_timing_is_compose_s() {
        assert_eq!(LOADING_MORPH_INTERVAL_MS, 650);
        assert_eq!(LOADING_GLOBAL_ROTATION_MS, 4_666);
        assert!((LOADING_QUARTER_ROTATION.to_degrees() - 90.0).abs() < 1e-9);
        let spring = morph_spring();
        assert!((spring.damping - 0.6).abs() < f64::EPSILON);
        let omega = TAU / spring.response;
        assert!(omega.mul_add(omega, -200.0).abs() < 1e-9);
        // The seven shapes repeat every seven intervals.
        let morph = loading_morph(Rect::new(0.0, 0.0, 48.0, 48.0));
        assert_eq!(morph.duration(), Duration::from_millis(7 * 650));
        // The rotation is two stages, each on its own period: one steady
        // turn, and four quarter-turn kicks.
        assert_eq!(loading_turn().duration(), Duration::from_millis(4_666));
        assert_eq!(loading_kicks().duration(), Duration::from_millis(4 * 650));
    }

    /// Every shape the loading indicator morphs through has the same
    /// verbs, which is what lets the engine morph between them.
    #[test]
    fn loading_outlines_share_one_structure() {
        let outlines = loading_outlines(Rect::new(0.0, 0.0, 48.0, 48.0));
        assert_eq!(outlines.len(), 7);
        let verbs = |shape: &ShapeData| match shape {
            ShapeData::Path { elements, .. } => elements.len(),
            other => panic!("expected a path, got {other:?}"),
        };
        for outline in &outlines {
            assert_eq!(verbs(outline), verbs(&outlines[0]));
        }
        // The loop returns to its first shape.
        let morph = loading_morph(Rect::new(0.0, 0.0, 48.0, 48.0));
        assert_eq!(morph.first(), &outlines[0]);
    }

    /// The two rotation stages compose to Compose's sum at any time, however
    /// far in: the steady turn plus a quarter turn per morph, the last along
    /// the morph's spring. Each stage repeats on its own period, so the sum
    /// is the unwrapped reference up to whole turns.
    #[test]
    fn loading_rotation_stages_compose_the_steady_turn_and_the_springed_quarter_turns() {
        let (kicks, turn) = (loading_kicks(), loading_turn());
        let kick = Keyframes::new(Duration::from_millis(650), 0.0).then(1.0, morph_spring(), 1.0);
        let reference = |millis: u64| {
            let morphs = millis / 650;
            let into = Duration::from_millis(millis % 650);
            let eased = kick.sample(into, Repeat::Forever).expect("forever");
            TAU.mul_add(
                f64::from(u32::try_from(millis).expect("under the period"))
                    / f64::from(LOADING_GLOBAL_ROTATION_MS),
                LOADING_QUARTER_ROTATION
                    * (f64::from(u32::try_from(morphs).expect("a few morphs")) + 1.0 + eased),
            )
        };
        for millis in [
            0, 1, 40, 200, 649, 650, 651, 1_300, 123_456, 3_000_000, 6_065_000,
        ] {
            let at = Duration::from_millis(millis);
            let sampled = kicks.sample(at, Repeat::Forever).expect("forever")
                + turn.sample(at, Repeat::Forever).expect("forever");
            let apart = (sampled - reference(millis)).rem_euclid(TAU);
            assert!(
                // The steady turn is the engine's linear bezier, which its
                // solver resolves to about a thousandth of a milliradian.
                apart.min(TAU - apart) < 1e-5,
                "{millis} ms: {sampled} vs {}",
                reference(millis)
            );
        }
        // Each stage ends a whole number of turns past where it started, so
        // its repeat does not jump: a nanosecond before the period the last
        // spring has settled to within its residual (0.4% of a quarter turn)
        // of the last keyframe.
        for stage in [&kicks, &turn] {
            let end = stage
                .sample(
                    stage
                        .duration()
                        .checked_sub(Duration::from_nanos(1))
                        .expect("a period outlasts a nanosecond"),
                    Repeat::Forever,
                )
                .expect("forever");
            let turns = (end - stage.first()) / TAU;
            assert!(
                (turns - turns.round()).abs() < 2e-3,
                "ended {turns} turns on"
            );
        }
    }
}
