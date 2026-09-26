use crate::{PressWave, PressWaves, WidgetInteractionState};
use cherenkov::kurbo::{Circle, RoundedRect};
use cherenkov::kurbo::{Point, Rect, RoundedRectRadii};
use cherenkov::{Draw as _, Paint, Recorder, WorkingColor};

/// Minimum ripple diameter (Material Web `MINIMUM_PRESS_DIAMETER`): small
/// targets still produce a ripple at least this wide.
const RIPPLE_MINIMUM_DIAMETER: f64 = 48.0;

/// `StateTokens.HoverStateLayerOpacity`.
pub const HOVER_STATE_LAYER_OPACITY: f32 = 0.08;
/// `StateTokens.FocusStateLayerOpacity`.
pub const FOCUS_STATE_LAYER_OPACITY: f32 = 0.1;
/// `StateTokens.PressedStateLayerOpacity`.
pub const PRESSED_STATE_LAYER_OPACITY: f32 = 0.1;
/// `StateTokens.DraggedStateLayerOpacity`.
pub const DRAGGED_STATE_LAYER_OPACITY: f32 = 0.16;

/// The dominant resting state-layer opacity for `state`: the renderer-sampled
/// animated value when one is in flight, otherwise the MD3 token for the
/// active boolean state (focus outranks hover).
fn resolved_state_layer_opacity(state: WidgetInteractionState) -> f32 {
    if state.focus_visible {
        state.focus_progress * FOCUS_STATE_LAYER_OPACITY
    } else if state.state_layer_opacity > 0.0 {
        state.state_layer_opacity
    } else if state.hovered {
        HOVER_STATE_LAYER_OPACITY
    } else {
        0.0
    }
}

/// The press waves to draw for `state`: the renderer-sampled waves when any
/// are in flight, otherwise — for states constructed without renderer
/// animation (static previews, tests) — a single full-coverage wave at the
/// MD3 pressed token while the state reads as pressed.
fn resolved_press_waves(state: WidgetInteractionState) -> PressWaves {
    if !state.press_waves.is_empty() {
        return state.press_waves;
    }
    let mut waves = PressWaves::EMPTY;
    if state.pressed {
        waves.push(PressWave {
            origin: None,
            progress: 1.0,
            opacity: PRESSED_STATE_LAYER_OPACITY,
        });
    }
    waves
}

/// Initial ripple size as a fraction of the full diameter: a wave starts at
/// this scale over its press point and grows to full size as the renderer's
/// press-grow animation advances the wave's progress from 0 to 1.
const RIPPLE_INITIAL_SCALE: f64 = 0.4;

/// The full Material ripple diameter for a target: the surface diagonal,
/// floored at [`RIPPLE_MINIMUM_DIAMETER`]. Waves are drawn fresh each frame
/// from the sampled interaction state — [`ripple_kinematics`] scales each
/// from the initial fraction up to full size while its center drifts from its
/// press point to the surface center.
pub fn ripple_diameter(bounds: Rect) -> f64 {
    bounds
        .width()
        .hypot(bounds.height())
        .max(RIPPLE_MINIMUM_DIAMETER)
}

/// Material ripple kinematics for one frame: given the wave's convergence
/// `target` center and full `diameter`, returns the wave's current center and
/// radius.
///
/// The center drifts linearly from the wave's press point (already mapped by
/// the renderer into the same coordinate space; `target` when absent) to
/// `target`, and the radius grows from [`RIPPLE_INITIAL_SCALE`] of
/// `diameter` to the full size, both driven by the wave's progress.
fn ripple_kinematics(target: Point, diameter: f64, wave: PressWave) -> (Point, f64) {
    let progress = f64::from(wave.progress.clamp(0.0, 1.0));
    let origin = wave.origin.unwrap_or(target);
    let center = Point::new(
        (target.x - origin.x).mul_add(progress, origin.x),
        (target.y - origin.y).mul_add(progress, origin.y),
    );
    let scale = (1.0 - RIPPLE_INITIAL_SCALE).mul_add(progress, RIPPLE_INITIAL_SCALE);
    (center, diameter * 0.5 * scale)
}

/// Fills every visible wave as a solid circle at its current animated
/// geometry. The waves are ordered oldest to newest, so overlapping ripples
/// from rapid re-presses composite naturally (Material semantics: older waves keep
/// fading while the newest grows).
fn fill_waves(
    draw: &mut Recorder,
    target: Point,
    diameter: f64,
    color: WorkingColor,
    waves: PressWaves,
) {
    for wave in waves.iter() {
        if wave.opacity <= 0.0 {
            continue;
        }
        let (center, radius) = ripple_kinematics(target, diameter, wave);
        let brush = Paint::from(color.with_alpha(wave.opacity.clamp(0.0, 1.0)));
        draw.fill(Circle::new(center, radius), brush);
    }
}

/// Draws the resting state layer (hover/focus tint) into `bounds`.
fn draw_state_tint_rounded(
    draw: &mut Recorder,
    bounds: Rect,
    radii: RoundedRectRadii,
    color: WorkingColor,
    state_opacity: f32,
) {
    if state_opacity > 0.0 {
        draw.fill(
            RoundedRect::from_rect(bounds, radii),
            color.with_alpha(color.components[3] * state_opacity),
        );
    }
}

pub fn draw_bounded(
    draw: &mut Recorder,
    bounds: Rect,
    radii: RoundedRectRadii,
    color: WorkingColor,
    state: WidgetInteractionState,
) {
    draw_state_tint_rounded(
        draw,
        bounds,
        radii,
        color,
        resolved_state_layer_opacity(state),
    );

    let waves = resolved_press_waves(state);
    if waves.is_empty() {
        return;
    }

    // Solid Material ripple waves, drawn fresh each frame at their current
    // animated geometry: the retained tree re-encodes the scene every frame
    // while press animations run, so each wave's sampled progress/origin
    // drives its growth and drift directly.
    draw.clip(RoundedRect::from_rect(bounds, radii), |draw| {
        fill_waves(draw, bounds.center(), ripple_diameter(bounds), color, waves);
    });
}

pub fn draw_unbounded_circle(
    draw: &mut Recorder,
    center: Point,
    radius: f64,
    color: WorkingColor,
    state: WidgetInteractionState,
) {
    let state_opacity = resolved_state_layer_opacity(state);
    if state_opacity > 0.0 {
        draw.fill(
            Circle::new(center, radius),
            color.with_alpha(color.components[3] * state_opacity),
        );
    }

    let waves = resolved_press_waves(state);
    if waves.is_empty() {
        return;
    }
    // An unbounded halo caps the wave at exactly `radius` — not the disc's
    // diagonal and not the bounded-ripple minimum diameter — so the wave
    // target is the requested diameter directly.
    fill_waves(draw, center, radius * 2.0, color, waves);
}

#[cfg(test)]
mod tests {
    use super::{PRESSED_STATE_LAYER_OPACITY, RIPPLE_MINIMUM_DIAMETER, ripple_diameter};
    use crate::{PressWave, PressWaves, WidgetInteractionState, theme::state_layer};
    use cherenkov::kurbo::{Point, Rect, RoundedRect, RoundedRectRadii};
    use cherenkov::{Command, Content, Draw as _, Paint, Recorder, ShapeData, WorkingColor};
    use std::path::Path;

    #[test]
    fn material_ripple_diameter_spans_diagonal_with_minimum() {
        // Large surface: ripple diameter is the diagonal so the solid circle
        // covers the whole target at full scale.
        let wide = Rect::new(0.0, 0.0, 100.0, 40.0);
        assert!((ripple_diameter(wide) - 100.0_f64.hypot(40.0)).abs() < 1e-6);

        // Tiny surface: floored at the Material minimum press diameter.
        let tiny = Rect::new(0.0, 0.0, 20.0, 20.0);
        assert_eq!(ripple_diameter(tiny), RIPPLE_MINIMUM_DIAMETER);
    }

    fn pressed_state(waves: &[PressWave]) -> WidgetInteractionState {
        let mut press_waves = PressWaves::EMPTY;
        for wave in waves {
            press_waves.push(*wave);
        }
        WidgetInteractionState {
            pressed: true,
            press_waves,
            ..WidgetInteractionState::NONE
        }
    }

    #[test]
    fn material_ripple_animates_from_press_point_to_full_coverage() {
        // Mid-press: the ripple sits between the press point and the bounds
        // center, at a radius between the initial fraction and full size.
        let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);
        let origin = Point::new(10.0, 12.0);
        let mut recorder = Recorder::new();
        state_layer::draw_bounded(
            &mut recorder,
            bounds,
            8.0.into(),
            WorkingColor::new([1.0, 1.0, 1.0, 1.0]),
            pressed_state(&[PressWave {
                origin: Some(origin),
                progress: 0.5,
                opacity: 0.12,
            }]),
        );
        let circle = single_circle(recorder).expect("mid-press ripple must fill a solid circle");
        assert_eq!(
            circle.center,
            Point::new(30.0, 16.0),
            "center drifts halfway from the press point to the bounds center"
        );
        let full_radius = ripple_diameter(bounds) * 0.5;
        assert!(
            (circle.radius - full_radius * 0.7).abs() < 1e-6,
            "radius is halfway between the 0.4 initial fraction and full size"
        );
    }

    #[test]
    fn material_ripple_press_layer_is_a_solid_circle() {
        // At full progress the ripple covers the target: a solid circle (no
        // soft-edge gradient) centered in bounds at the full ripple diameter.
        let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);
        let mut recorder = Recorder::new();
        state_layer::draw_bounded(
            &mut recorder,
            bounds,
            8.0.into(),
            WorkingColor::new([1.0, 1.0, 1.0, 1.0]),
            pressed_state(&[PressWave {
                origin: Some(Point::new(10.0, 12.0)),
                progress: 1.0,
                opacity: 0.12,
            }]),
        );
        let circle = single_circle(recorder).expect("press ripple must fill a solid circle");
        assert_eq!(circle.center, Point::new(50.0, 20.0), "circle is centered");
        assert!(
            (circle.radius - ripple_diameter(bounds) * 0.5).abs() < 1e-6,
            "circle radius is half the ripple diameter"
        );
        assert!(
            matches!(circle.brush, RecordedBrush::Solid(alpha) if (alpha - 0.12).abs() < 1e-6),
            "ripple is a solid color at the press opacity, not a gradient"
        );
    }

    #[test]
    fn unbounded_circle_ripple_converges_to_the_requested_radius() {
        // Compose caps an unbounded ripple at `StateLayerSize / 2`: the wave
        // converges to the halo radius itself, not the disc's diagonal.
        let center = Point::new(50.0, 30.0);
        let radius = 20.0;
        let mut recorder = Recorder::new();
        state_layer::draw_unbounded_circle(
            &mut recorder,
            center,
            radius,
            WorkingColor::new([1.0, 1.0, 1.0, 1.0]),
            pressed_state(&[PressWave {
                origin: None,
                progress: 1.0,
                opacity: 0.12,
            }]),
        );
        let circle = single_circle(recorder).expect("press ripple must fill a solid circle");
        assert_eq!(circle.center, center, "wave converges on the halo center");
        assert!(
            (circle.radius - radius).abs() < 1e-6,
            "wave radius {} must converge to the halo radius {}",
            circle.radius,
            radius
        );
    }

    #[test]
    fn overlapping_press_waves_draw_oldest_to_newest() {
        // Rapid re-press: the older, released wave keeps fading at full size
        // while the fresh wave grows from its own press point — both are
        // filled the same frame, oldest first.
        let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);
        let mut recorder = Recorder::new();
        state_layer::draw_bounded(
            &mut recorder,
            bounds,
            8.0.into(),
            WorkingColor::new([1.0, 1.0, 1.0, 1.0]),
            pressed_state(&[
                PressWave {
                    origin: Some(Point::new(10.0, 12.0)),
                    progress: 1.0,
                    opacity: 0.06,
                },
                PressWave {
                    origin: Some(Point::new(80.0, 30.0)),
                    progress: 0.0,
                    opacity: 0.12,
                },
            ]),
        );
        let circles = filled_circles(recorder);
        assert_eq!(circles.len(), 2, "both waves must be filled");
        assert_eq!(
            circles[0].center,
            Point::new(50.0, 20.0),
            "the older wave is centered at full coverage"
        );
        assert!(
            matches!(circles[0].brush, RecordedBrush::Solid(alpha) if (alpha - 0.06).abs() < 1e-6),
            "the older wave fades at its own sampled opacity"
        );
        assert_eq!(
            circles[1].center,
            Point::new(80.0, 30.0),
            "the fresh wave starts over its own press point"
        );
        let full_radius = ripple_diameter(bounds) * 0.5;
        assert!(
            (circles[1].radius - full_radius * 0.4).abs() < 1e-6,
            "the fresh wave starts at the initial scale fraction"
        );
    }

    #[test]
    fn static_pressed_state_synthesizes_a_full_coverage_wave() {
        // A pressed state constructed without renderer animation (static
        // previews, tests) still renders a press layer: one centered,
        // full-coverage wave at the MD3 pressed token.
        let bounds = Rect::new(0.0, 0.0, 100.0, 40.0);
        let mut recorder = Recorder::new();
        state_layer::draw_bounded(
            &mut recorder,
            bounds,
            8.0.into(),
            WorkingColor::new([1.0, 1.0, 1.0, 1.0]),
            WidgetInteractionState {
                pressed: true,
                ..WidgetInteractionState::NONE
            },
        );
        let circle = single_circle(recorder).expect("static pressed state must fill a press layer");
        assert_eq!(circle.center, Point::new(50.0, 20.0));
        assert!(
            matches!(
                circle.brush,
                RecordedBrush::Solid(alpha)
                    if (alpha - PRESSED_STATE_LAYER_OPACITY).abs() < 1e-6
            ),
            "synthesized wave uses StateTokens.PressedStateLayerOpacity"
        );
    }

    #[derive(Debug, Clone, Copy)]
    enum RecordedBrush {
        Solid(f32),
        Gradient,
    }

    #[derive(Debug, Clone, Copy)]
    struct RecordedCircle {
        center: Point,
        radius: f64,
        brush: RecordedBrush,
    }

    fn record_brush(brush: &Paint) -> RecordedBrush {
        match brush {
            Paint::Solid(color) => RecordedBrush::Solid(color.components[3]),
            _ => RecordedBrush::Gradient,
        }
    }

    /// The solid circles the recorder filled, in draw order.
    fn filled_circles(recorder: Recorder) -> Vec<RecordedCircle> {
        let mut content: Content = recorder.finish();
        content
            .snapshot()
            .commands()
            .iter()
            .filter_map(|command| match command {
                Command::Fill {
                    shape: ShapeData::Circle(circle),
                    paint,
                } => Some(RecordedCircle {
                    center: circle.center,
                    radius: circle.radius,
                    brush: record_brush(paint),
                }),
                _ => None,
            })
            .collect()
    }

    fn single_circle(recorder: Recorder) -> Option<RecordedCircle> {
        let circles = filled_circles(recorder);
        assert!(
            circles.len() <= 1,
            "expected at most one filled circle, recorded {}",
            circles.len()
        );
        circles.first().copied()
    }

    #[test]
    #[ignore = "writes a visual acceptance PNG for direct image review"]
    fn material_ripple_visual_snapshot() {
        use hydrolysis::SurfaceProvider as _;

        let surface = hydrolysis::OffscreenSurface::new(240, 120);
        let bounds = Rect::new(24.0, 24.0, 216.0, 96.0);
        let picture = cherenkov::Picture::record(|draw| {
            draw.fill(
                RoundedRect::from_rect(bounds, RoundedRectRadii::from_single_radius(20.0)),
                WorkingColor::new([0.40, 0.31, 0.64, 1.0]),
            );
        });
        let mut recorder = Recorder::new();
        let mut press_waves = PressWaves::EMPTY;
        press_waves.push(PressWave {
            origin: Some(Point::new(56.0, 48.0)),
            progress: 0.72,
            opacity: 0.30,
        });
        state_layer::draw_bounded(
            &mut recorder,
            bounds,
            20.0.into(),
            WorkingColor::new([1.0, 1.0, 1.0, 1.0]),
            WidgetInteractionState {
                pressed: true,
                press_waves,
                ..WidgetInteractionState::NONE
            },
        );
        let ripple = recorder.finish().into_picture();

        let target = surface.surface();
        target.clear_color(WorkingColor::WHITE);
        target.update(|tx| {
            tx[target.root()].content(cherenkov::Picture::record(|draw| {
                draw.picture(&picture, cherenkov::kurbo::Affine::IDENTITY);
                draw.picture(&ripple, cherenkov::kurbo::Affine::IDENTITY);
            }));
        });
        surface
            .engine()
            .render(cherenkov::FrameTime::at(std::time::Instant::now()))
            .expect("ripple visual render failed");
        let rgba8 = surface.readback_rgba8();

        let output_path = Path::new("target/hydrolysis-m3-visual/material-ripple-solid.png");
        std::fs::create_dir_all(
            output_path
                .parent()
                .expect("ripple visual output path must have parent"),
        )
        .expect("failed to create ripple visual output directory");
        image::RgbaImage::from_raw(240, 120, rgba8)
            .expect("readback must be 240x120 RGBA8")
            .save(output_path)
            .expect("failed to save ripple visual output");
    }
}
