//! The Material 3 progress looks: the linear bar with its track gap and stop
//! indicator, the circular indicator, and the expressive loading indicator.
//!
//! A determinate indicator's parts are functions of its value, so a value
//! change moves their recorded operands. An indeterminate indicator's motion
//! is keyframes the engine samples on its own clock; the look declares the
//! tracks and records them once, so a running indicator does no work per
//! frame.

use core::f64::consts::{FRAC_PI_2, TAU};
use core::time::Duration;

use hydrolysis::theme::{
    Custom, Driven, IndicatorLook, Inputs, LinearProgressLook, ProgressControl, ProgressSpec,
};
use waterui_core::Environment;
use waterui_core::Signal as _;
use waterui_graphics::color::Color;
use waterui_graphics::draw::kurbo::{
    Arc, BezPath, Circle, Point, Rect, RoundedRect, RoundedRectRadii, Shape as _, Size, Stroke,
};
use waterui_graphics::draw::{
    Curve, Draw as _, Easing, Keyframes, Paint, Repeat, ShapeData, Spring, WorkingColor,
    curve_value,
};

use crate::color::{
    Primary, PrimaryContainer, SurfaceContainerHighest, Tertiary, TertiaryContainer,
};
use crate::dimensions::{
    PROGRESS_CIRCULAR_DIAMETER, PROGRESS_CIRCULAR_STROKE_WIDTH,
    PROGRESS_CIRCULAR_TRACK_ACTIVE_SPACE, PROGRESS_LINEAR_BAR_HEIGHT,
    PROGRESS_LINEAR_BAR_HORIZONTAL_INSET, PROGRESS_LINEAR_BAR_TOP_OFFSET,
    PROGRESS_LINEAR_LABEL_HEIGHT, PROGRESS_LINEAR_MIN_TRACK_WIDTH, PROGRESS_LINEAR_STOP_SIZE,
    PROGRESS_LINEAR_TRACK_ACTIVE_SPACE, PROGRESS_LINEAR_VALUE_LABEL_TOP_SPACING,
};
use crate::material_shapes::{material_shape_sequence, radii_to_path};
use crate::theme::motion;

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
/// `CircularIndeterminateMinProgress`: the shortest the arc gets.
const CIRCULAR_INDETERMINATE_MIN_PROGRESS: f64 = 0.1;
/// `CircularIndeterminateMaxProgress`: the longest it grows to, half a cycle in.
const CIRCULAR_INDETERMINATE_MAX_PROGRESS: f64 = 0.87;

/// `LoadingIndicatorTokens.ContainerWidth` / `ContainerHeight`.
pub const LOADING_CONTAINER_SIZE: f64 = 48.0;
/// `LoadingIndicatorTokens.ActiveSize`: the shape inside that container.
pub const LOADING_INDICATOR_SIZE: f64 = 38.0;
/// `MorphIntervalMillis`: how long the indicator rests on each shape.
const LOADING_MORPH_INTERVAL: Duration = Duration::from_millis(650);
/// The nearest whole number of morph intervals to `GlobalRotationDurationMillis`
/// (4666 ms): one turn of the steady rotation, linear. Whole intervals put the
/// turns and the morphs back in phase after five turns, which is what lets the
/// engine repeat the rotation seamlessly.
const LOADING_GLOBAL_ROTATION: Duration = Duration::from_millis(4_680);
/// How many global turns the rotation track runs before it repeats.
const LOADING_ROTATION_TURNS: u32 = 5;
/// `QuarterRotation`: each morph also kicks the shape a quarter turn on.
const LOADING_QUARTER_ROTATION: f64 = FRAC_PI_2;
/// Compose morphs between shapes with `spring(dampingRatio = 0.6f, stiffness = 200f)`.
const LOADING_MORPH_SPRING_DAMPING_RATIO: f64 = 0.6;
const LOADING_MORPH_SPRING_STIFFNESS: f64 = 200.0;

/// How finely a motion with no closed keyframe form is sampled: about one
/// segment per frame at 60 Hz. The engine interpolates linearly between
/// samples.
const SAMPLE_STEP: Duration = Duration::from_millis(16);

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

/// `color` as a paint that follows it.
fn solid(color: impl Into<Color>, env: &Environment) -> Driven<Paint, f64> {
    Driven::follow(
        color.into().resolve(env),
        |color: &WorkingColor, _inputs: &Inputs<'_, f64>| Paint::from(*color),
    )
}

/// The indicator's active colour: the tint, or the four accent colours in
/// turn over `cycle` when the spec asks for them.
fn active_paint(spec: &ProgressSpec, cycle: Duration, env: &Environment) -> Driven<Paint, f64> {
    if !spec.four_color() {
        return solid(spec.tint(), env);
    }
    let palette = [
        Color::from(Primary),
        Color::from(PrimaryContainer),
        Color::from(Tertiary),
        Color::from(TertiaryContainer),
    ]
    .map(|color| Paint::from(color.resolve(env).snapshot()));
    Driven::repeating(move |_frame| four_colors(&palette, cycle), Repeat::Forever)
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

/// A track of `cycle` whose value at each phase `0.0..=1.0` is `value`,
/// sampled once per [`SAMPLE_STEP`]. `value` must come back to its start at
/// phase one, or the repeat jumps.
fn sampled<T>(cycle: Duration, value: impl Fn(f64) -> T) -> Keyframes<T> {
    let steps = u32::try_from(cycle.as_millis().div_ceil(SAMPLE_STEP.as_millis()))
        .expect("a progress cycle is a few seconds");
    let curve = Curve::linear(cycle);
    (1..=steps).fold(Keyframes::new(cycle, value(0.0)), |keyframes, step| {
        let phase = f64::from(step) / f64::from(steps);
        keyframes.then(phase, curve, value(phase))
    })
}

// ---------------------------------------------------------------------------
// Linear
// ---------------------------------------------------------------------------

/// A bar `[x0, x1]` wide across `frame`'s height with `CornerFull` ends,
/// mirrored for a right-to-left indicator.
fn bar(frame: Rect, x0: f64, x1: f64, rtl: bool) -> ShapeData {
    let (x0, x1) = if rtl {
        (frame.x0 + frame.x1 - x1, frame.x0 + frame.x1 - x0)
    } else {
        (x0, x1)
    };
    ShapeData::of(&RoundedRect::from_rect(
        Rect::new(x0, frame.y0, x1.max(x0), frame.y1),
        RoundedRectRadii::from_single_radius(frame.height() / 2.0),
    ))
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

/// A linear bar: three shapes, each with its paint.
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
            solid(SurfaceContainerHighest, env),
            piece(|(_, active, _)| active),
            solid(spec.tint(), env),
            piece(|(.., stop)| stop),
            solid(spec.tint(), env),
        ),
        |recorder, _frame, (track, track_paint, active, active_paint, stop, stop_paint)| {
            recorder.fill(track, track_paint);
            recorder.fill(active, active_paint);
            recorder.fill(stop, stop_paint);
        },
    )
}

/// The value of `segments` at `phase` of their cycle.
fn sample_segments(segments: &[Segment], phase: f64) -> f64 {
    for window in segments.windows(2) {
        let (start, end) = (window[0], window[1]);
        if phase < start.at || phase > end.at {
            continue;
        }
        if (end.at - start.at).abs() <= f64::EPSILON {
            return end.value;
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
    segments
        .last()
        .expect("indeterminate progress segments must not be empty")
        .value
}

/// One indeterminate bar's keyframes across `frame`: it starts
/// `initial_inset` percent of the track's width in, travels by `translate`
/// and is `scale` of the width long.
fn indeterminate_bar(
    frame: Rect,
    rtl: bool,
    initial_inset: f64,
    translate: &'static [Segment],
    scale: &'static [Segment],
) -> Keyframes<ShapeData> {
    sampled(LINEAR_INDETERMINATE_CYCLE, |phase| {
        let x = frame.width().mul_add(
            (initial_inset + sample_segments(translate, phase)) / 100.0,
            frame.x0,
        );
        let width = (frame.width() * sample_segments(scale, phase)).max(0.0);
        bar(frame, x, x + width, rtl)
    })
}

fn linear_indeterminate(spec: &ProgressSpec, rtl: bool, env: &Environment) -> LinearBar {
    // The two bars share one colour cycle, twice the bars' own.
    let paint = active_paint(spec, LINEAR_INDETERMINATE_CYCLE * 2, env);
    Custom::new(
        (
            Driven::new(move |inputs: &Inputs<'_, f64>| {
                bar(inputs.frame, inputs.frame.x0, inputs.frame.x1, rtl)
            }),
            solid(SurfaceContainerHighest, env),
            Driven::repeating(
                move |frame| {
                    indeterminate_bar(
                        frame,
                        rtl,
                        PRIMARY_INITIAL_INSET,
                        PRIMARY_TRANSLATE,
                        PRIMARY_SCALE,
                    )
                },
                Repeat::Forever,
            ),
            paint.clone(),
            Driven::repeating(
                move |frame| {
                    indeterminate_bar(
                        frame,
                        rtl,
                        SECONDARY_INITIAL_INSET,
                        SECONDARY_TRANSLATE,
                        SECONDARY_SCALE,
                    )
                },
                Repeat::Forever,
            ),
            paint,
        ),
        |recorder,
         frame,
         (track, track_paint, primary, primary_paint, secondary, secondary_paint)| {
            recorder.fill(track, track_paint);
            recorder.clip(frame, |recorder| {
                recorder.fill(primary, primary_paint);
                recorder.fill(secondary, secondary_paint);
            });
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
            solid(SurfaceContainerHighest, env),
            Driven::new(|inputs: &Inputs<'_, f64>| {
                active_stroke(ring(inputs.frame).1, *inputs.value)
            }),
            solid(spec.tint(), env),
        ),
        |recorder, frame, (track, track_paint, active, active_paint)| {
            let (path, _) = ring(frame);
            recorder.stroke(path.clone(), track, track_paint);
            recorder.stroke(path, active, active_paint);
        },
    )
}

/// The arc's length at `cycle`, as a share of the ring: growing to
/// `CircularIndeterminateMaxProgress` by mid-cycle, then shrinking back to
/// `CircularIndeterminateMinProgress`.
fn circular_indeterminate_progress(cycle: f64) -> f64 {
    let span = CIRCULAR_INDETERMINATE_MAX_PROGRESS - CIRCULAR_INDETERMINATE_MIN_PROGRESS;
    let half = if cycle <= 0.5 {
        cycle * 2.0
    } else {
        (1.0 - cycle) * 2.0
    };
    let eased = f64::from(crate::theme::motion::tokens::easing::STANDARD.ease(phase_f32(half)));
    span.mul_add(eased, CIRCULAR_INDETERMINATE_MIN_PROGRESS)
}

fn phase_f32(value: f64) -> f32 {
    num_traits::ToPrimitive::to_f32(&value).expect("progress phase must be representable as f32")
}

/// The additional rotation, in degrees, at `cycle`: four 90-degree steps,
/// each eased with the emphasized-decelerate curve and held until the next
/// one begins.
fn circular_additional_rotation(cycle: f64) -> f64 {
    const STEPS: f64 = 4.0;
    let cycle_secs = CIRCULAR_CYCLE_DURATION.as_secs_f64();
    let delay_secs = CIRCULAR_ADDITIONAL_ROTATION_DELAY.as_secs_f64();
    let step_secs = CIRCULAR_ADDITIONAL_ROTATION_DURATION.as_secs_f64();
    let elapsed_secs = cycle * cycle_secs;
    let completed = (elapsed_secs / delay_secs).floor().min(STEPS);
    let into_step = completed.mul_add(-delay_secs, elapsed_secs);
    let step_progress = if step_secs > 0.0 {
        (into_step / step_secs).clamp(0.0, 1.0)
    } else {
        1.0
    };
    let eased = f64::from(
        crate::theme::motion::tokens::easing::EMPHASIZED_DECELERATE.ease(phase_f32(step_progress)),
    );
    (completed + eased).min(STEPS) / STEPS * CIRCULAR_ADDITIONAL_ROTATION_DEGREES
}

/// How far the whole ring has turned at `cycle` of the indeterminate cycle,
/// in radians: the steady global rotation plus the additional rotation's kicks.
fn circular_rotation(cycle: f64) -> f64 {
    cycle
        .mul_add(
            CIRCULAR_GLOBAL_ROTATION_DEGREES,
            circular_additional_rotation(cycle),
        )
        .to_radians()
}

/// The arc's dashes at `cycle`: one dash as long as the arc, and the rest of
/// the ring as gap.
fn arc_stroke(circumference: f64, cycle: f64) -> Stroke {
    let arc = circular_indeterminate_progress(cycle) * circumference;
    Stroke::new(PROGRESS_CIRCULAR_STROKE_WIDTH).with_dashes(0.0, [arc, circumference - arc])
}

fn circular_indeterminate(spec: &ProgressSpec, env: &Environment) -> Ring {
    Custom::new(
        (
            Driven::new(|_inputs: &Inputs<'_, f64>| Stroke::new(PROGRESS_CIRCULAR_STROKE_WIDTH)),
            solid(SurfaceContainerHighest, env),
            Driven::repeating(
                |frame| {
                    let (_, circumference) = ring(frame);
                    sampled(CIRCULAR_CYCLE_DURATION, |cycle| {
                        arc_stroke(circumference, cycle)
                    })
                },
                Repeat::Forever,
            ),
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
    .rotating(
        sampled(CIRCULAR_CYCLE_DURATION, circular_rotation),
        Repeat::Forever,
    )
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
/// spring.
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
            Keyframes::new(LOADING_MORPH_INTERVAL * steps, first),
            |keyframes, (index, outline)| {
                let at = f64::from(u32::try_from(index + 1).expect("bounded by the shape count"))
                    / f64::from(steps);
                keyframes.then(at, spring, outline)
            },
        )
}

/// How far the loading indicator has turned `elapsed` into its rotation
/// cycle, in radians: a steady rotation, plus the quarter turn each morph
/// adds, which runs with the morph's own spring.
fn loading_rotation(kick: &Keyframes<f64>, elapsed: Duration) -> f64 {
    let turns = elapsed.as_secs_f64() / LOADING_GLOBAL_ROTATION.as_secs_f64();
    // Which morph is counted in whole nanoseconds, so a rounding error at a
    // boundary cannot land on the wrong one; only the share within a morph
    // needs to be continuous.
    let interval = LOADING_MORPH_INTERVAL.as_nanos();
    let morphs = f64::from(
        u32::try_from(elapsed.as_nanos() / interval).expect("a rotation cycle is a few morphs"),
    );
    let into_morph = Duration::from_nanos(
        u64::try_from(elapsed.as_nanos() % interval).expect("a morph lasts under a second"),
    );
    let eased = kick
        .sample(into_morph, Repeat::Forever)
        .expect("a repeating track always has a value");
    TAU.mul_add(turns, LOADING_QUARTER_ROTATION * (morphs + eased))
}

/// One morph's quarter turn: from nothing to a whole, along the morph's spring.
fn loading_kick() -> Keyframes<f64> {
    Keyframes::new(LOADING_MORPH_INTERVAL, 0.0).then(1.0, morph_spring(), 1.0)
}

fn loading_part(spec: &ProgressSpec, env: &Environment) -> Blob {
    let kick = loading_kick();
    let rotation_cycle = LOADING_GLOBAL_ROTATION * LOADING_ROTATION_TURNS;
    Custom::new(
        (
            Driven::repeating(loading_morph, Repeat::Forever),
            active_paint(spec, LOADING_GLOBAL_ROTATION, env),
        ),
        |recorder, _frame, (outline, paint)| {
            recorder.fill(outline, paint);
        },
    )
    .rotating(
        sampled(rotation_cycle, |phase| {
            loading_rotation(&kick, rotation_cycle.mul_f64(phase))
        }),
        Repeat::Forever,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use hydrolysis::Style as _;
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

    /// The indeterminate circular motion follows Compose's
    /// `CircularProgressIndicator`: a 6s cycle whose arc grows from
    /// `CircularIndeterminateMinProgress` to `CircularIndeterminateMaxProgress`
    /// at the halfway point and shrinks back.
    #[test]
    fn circular_indeterminate_sweep_follows_the_compose_cycle() {
        let start = circular_indeterminate_progress(0.0);
        assert!(
            (start - CIRCULAR_INDETERMINATE_MIN_PROGRESS).abs() < 1e-6,
            "the cycle starts at the minimum arc, was {start}"
        );
        let middle = circular_indeterminate_progress(0.5);
        assert!(
            (middle - CIRCULAR_INDETERMINATE_MAX_PROGRESS).abs() < 1e-6,
            "the arc peaks at the half cycle, was {middle}"
        );
        // The cycle is closed: it ends where it began, so a repeat does not jump.
        let end = circular_indeterminate_progress(1.0);
        assert!(
            (end - CIRCULAR_INDETERMINATE_MIN_PROGRESS).abs() < 1e-6,
            "the cycle returns to the minimum arc, was {end}"
        );
        // Never inverted or past a full turn at any point in the cycle.
        for step in 0..=120 {
            let turns = circular_indeterminate_progress(f64::from(step) / 120.0);
            assert!(
                (CIRCULAR_INDETERMINATE_MIN_PROGRESS - 1e-6
                    ..=CIRCULAR_INDETERMINATE_MAX_PROGRESS + 1e-6)
                    .contains(&turns),
                "sweep left its range at step {step}: {turns}"
            );
        }
    }

    /// The additional rotation reaches a full turn by the end of the cycle, in
    /// four 90-degree steps.
    #[test]
    fn circular_additional_rotation_completes_a_turn_in_four_steps() {
        assert!(circular_additional_rotation(0.0).abs() < 1e-6);
        assert!(
            (circular_additional_rotation(1.0) - CIRCULAR_ADDITIONAL_ROTATION_DEGREES).abs() < 1e-6,
            "a full cycle turns the arc once more"
        );
        // Compose's keyframes ramp each step over 300ms and then hold it until
        // the next 1.5s boundary: 90 at 300ms, still 90 at 1.4s, 180 at 1.8s.
        let first_step = circular_additional_rotation(300.0 / 6_000.0);
        assert!(
            (first_step - 90.0).abs() < 1e-6,
            "the first step completes at 300ms, was {first_step}"
        );
        let holding = circular_additional_rotation(1_400.0 / 6_000.0);
        assert!(
            (holding - 90.0).abs() < 1e-6,
            "the step holds until the next delay boundary, was {holding}"
        );
        let second_step = circular_additional_rotation(1_800.0 / 6_000.0);
        assert!(
            (second_step - 180.0).abs() < 1e-6,
            "the second step completes at 1.8s, was {second_step}"
        );
        // Monotonic: the arc never rotates backwards.
        let mut previous = 0.0_f64;
        for step in 0..=200 {
            let value = circular_additional_rotation(f64::from(step) / 200.0);
            assert!(
                value >= previous - 1e-9,
                "rotation went backwards at {step}"
            );
            previous = value;
        }
    }

    /// The ring turns a whole number of times over the cycle, so the repeat
    /// does not jump.
    #[test]
    fn circular_rotation_repeats_seamlessly() {
        let turns = circular_rotation(1.0) / TAU;
        assert!(
            (turns - turns.round()).abs() < 1e-9,
            "the cycle ends a whole number of turns on, was {turns}"
        );
    }

    /// Expressive gives every progress element `CornerFull` ends: the moving
    /// indeterminate segments cap at half the track height, same as the
    /// determinate fill.
    #[test]
    fn indeterminate_segments_draw_with_rounded_caps() {
        let frame = track();
        for keyframes in [
            indeterminate_bar(
                frame,
                false,
                PRIMARY_INITIAL_INSET,
                PRIMARY_TRANSLATE,
                PRIMARY_SCALE,
            ),
            indeterminate_bar(
                frame,
                false,
                SECONDARY_INITIAL_INSET,
                SECONDARY_TRANSLATE,
                SECONDARY_SCALE,
            ),
        ] {
            // Mid-cycle: both segments are on screen with real width.
            let shape = keyframes
                .sample(LINEAR_INDETERMINATE_CYCLE / 2, Repeat::Forever)
                .expect("a repeating track has a value");
            let (rect, radii) = rounded(&shape);
            assert!(rect.width() > 0.0, "segment {rect:?} has no width");
            assert_eq!(
                radii,
                RoundedRectRadii::from(frame.height() / 2.0),
                "segment {rect:?} did not cap at half the track height"
            );
        }
    }

    /// The indeterminate bars start and end off the track, so the repeat
    /// does not show a segment appear.
    #[test]
    fn indeterminate_bars_leave_the_track_between_cycles() {
        let frame = track();
        let primary = indeterminate_bar(
            frame,
            false,
            PRIMARY_INITIAL_INSET,
            PRIMARY_TRANSLATE,
            PRIMARY_SCALE,
        );
        let start = primary
            .sample(Duration::ZERO, Repeat::Forever)
            .expect("a repeating track has a value");
        assert!(
            rounded(&start).0.x1 < frame.x0,
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

    /// The morph's quarter turn settles within its interval, so the rotation
    /// is continuous across morph boundaries.
    #[test]
    fn loading_rotation_is_continuous_across_morphs() {
        let kick = loading_kick();
        let nudge = Duration::from_micros(10);
        let before = loading_rotation(
            &kick,
            LOADING_MORPH_INTERVAL
                .checked_sub(nudge)
                .expect("an interval outlasts a nudge"),
        );
        let after = loading_rotation(&kick, LOADING_MORPH_INTERVAL + nudge);
        assert!(
            (after - before).abs() < 1e-2,
            "rotation jumped from {before} to {after} at a morph"
        );
        // The rotation track repeats seamlessly: it ends whole turns on.
        let cycle = LOADING_GLOBAL_ROTATION * LOADING_ROTATION_TURNS;
        let turns = loading_rotation(&kick, cycle) / TAU;
        assert!(
            (turns - turns.round()).abs() < 1e-6,
            "ended {turns} turns on"
        );
    }
}
