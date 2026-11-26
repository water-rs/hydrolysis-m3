//! The Material 3 switch and checkbox looks from the `md.comp.switch.*` and
//! `md.comp.checkbox.*` tokens.
//!
//! Every part of a toggle is a [`Custom`] part over the toggle's `bool`:
//! its [`Driven`] values read the value, the interaction state and, for a
//! switch, the drag, so the thumb's slide, its size, the track's colour, the
//! outline's width, the check glyph and the state layer are live operands
//! the engine animates. A toggle, hover, press or drag records nothing.

use hydrolysis_style::draw::kurbo::{
    Affine, BezPath, Circle, PathEl, Point, Rect, RoundedRect, RoundedRectRadii, Size, Stroke,
};
use hydrolysis_style::draw::{Draw as _, Paint, Shadow as ShadowCast, WorkingColor};
use hydrolysis_style::theme::{
    CheckboxLook, Custom, Driven, Inputs, SwitchDrag, SwitchLook, ToggleControl, ToggleSpec,
    ValueMotion,
};
use waterui::animation::Animation;
use waterui::interaction::InteractionState;
use waterui_core::Environment;
use waterui_core::layout::LayoutDirection;
use waterui_graphics::color::Color;

use super::selection::{
    STATE_LAYER_RADIUS, ShapePart, Toggled, centered_ripple, centered_state_layer, disabled, faded,
    layer_paint, narrow, ripple_motion, ripple_opacity, ripple_radius, state_layer_opacity,
    toggle_motion,
};
use crate::semantics::gather;

use crate::color::{
    OnPrimary, OnPrimaryContainer, OnSurface, OnSurfaceVariant, Outline, PrimaryContainer, Shadow,
    Surface, SurfaceContainerHighest,
};
use crate::dimensions::{
    TOGGLE_CHECKBOX_CONTAINER_SHAPE, TOGGLE_CHECKBOX_OUTLINE_WIDTH,
    TOGGLE_CHECKBOX_SELECTED_SCALE_START, TOGGLE_CHECKBOX_SIZE, TOGGLE_LABEL_SPACING,
    TOGGLE_SWITCH_HEIGHT, TOGGLE_SWITCH_ICON_SCALE_START, TOGGLE_SWITCH_ICON_SIZE,
    TOGGLE_SWITCH_OUTLINE_WIDTH, TOGGLE_SWITCH_PRESSED_HANDLE_SIZE,
    TOGGLE_SWITCH_SELECTED_HANDLE_SIZE, TOGGLE_SWITCH_THUMB_PADDING,
    TOGGLE_SWITCH_UNSELECTED_HANDLE_SIZE, TOGGLE_SWITCH_WIDTH,
};
use crate::elevation::{MaterialElevationLevel, chrome_elevation};
use crate::theme::colors::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY};
use crate::theme::motion::tokens::duration;
use crate::theme::motion::{self};

/// The distance the switch thumb travels between its resting centres.
const SWITCH_TRAVEL: f32 = 2.0f32.mul_add(
    -(TOGGLE_SWITCH_THUMB_PADDING + SWITCH_SLOT_HALF),
    TOGGLE_SWITCH_WIDTH,
);
/// Half the slot the thumb centres in at each end of the track: Compose
/// reserves the *selected* handle's width, so growing or shrinking the thumb
/// never moves it sideways.
const SWITCH_SLOT_HALF: f32 = TOGGLE_SWITCH_SELECTED_HANDLE_SIZE / 2.0;

/// The switch outline: its shape, stroke and paint.
type OutlinePart = Custom<bool, (Toggled<RoundedRect>, Toggled<Stroke>, Toggled<Paint>)>;
/// The thumb's key and ambient shadows, each over its own copy of the
/// thumb's shape.
type ShadowPart = Custom<
    bool,
    (
        Toggled<Circle>,
        Toggled<ShadowCast>,
        Toggled<Circle>,
        Toggled<ShadowCast>,
    ),
>;
/// The checked container: where it scales to, its fill and its check.
type ContainerPart = Custom<bool, (Toggled<Affine>, Toggled<Paint>, Toggled<Paint>)>;

fn lerp(start: f64, end: f64, t: f64) -> f64 {
    (end - start).mul_add(t, start)
}

fn blend(start: WorkingColor, end: WorkingColor, t: f64) -> WorkingColor {
    crate::lerp_color(start, end, narrow(t))
}

/// Where a toggle's operands are between their ends: the value's end, moved
/// by a switch's drag.
#[derive(Clone, Copy)]
struct Progress {
    drag: SwitchDrag,
    direction: LayoutDirection,
}

impl Progress {
    fn at(self, inputs: &Inputs<'_, bool>) -> f64 {
        self.drag.progress(*inputs.value, inputs.drag)
    }

    /// The thumb's centre at `inputs`.
    fn thumb_center(self, inputs: &Inputs<'_, bool>) -> Point {
        let frame = inputs.frame;
        let slot = f64::from(TOGGLE_SWITCH_THUMB_PADDING + SWITCH_SLOT_HALF);
        let (off, on) = match self.direction {
            LayoutDirection::LeftToRight => (frame.x0 + slot, frame.x1 - slot),
            LayoutDirection::RightToLeft => (frame.x1 - slot, frame.x0 + slot),
        };
        Point::new(lerp(off, on, self.at(inputs)), frame.center().y)
    }

    /// The thumb's radius: the pressed handle while pressed, otherwise
    /// growing from the unselected to the selected handle.
    fn thumb_radius(self, inputs: &Inputs<'_, bool>) -> f64 {
        if inputs.state.contains(InteractionState::PRESSED) {
            TOGGLE_SWITCH_PRESSED_HANDLE_SIZE / 2.0
        } else {
            lerp(
                TOGGLE_SWITCH_UNSELECTED_HANDLE_SIZE,
                f64::from(TOGGLE_SWITCH_SELECTED_HANDLE_SIZE),
                self.at(inputs),
            ) / 2.0
        }
    }

    fn thumb(self) -> Toggled<Circle> {
        Driven::new(move |inputs: &Inputs<'_, bool>| {
            Circle::new(self.thumb_center(inputs), self.thumb_radius(inputs))
        })
    }
}

/// The look of a switch in `spec`.
pub fn switch(spec: &ToggleSpec, env: &Environment) -> SwitchLook {
    let drag = SwitchDrag::new(SWITCH_TRAVEL, spec.direction());
    let progress = Progress {
        drag,
        direction: spec.direction(),
    };
    let tint = spec.tint();
    let control = ToggleControl::new(
        Size::new(
            f64::from(TOGGLE_SWITCH_WIDTH),
            f64::from(TOGGLE_SWITCH_HEIGHT),
        ),
        toggle_motion(),
    )
    .label_spacing(TOGGLE_LABEL_SPACING)
    .reach(STATE_LAYER_RADIUS)
    .hit(motion::hit_region())
    .part(switch_track(progress, &tint, env))
    .part(switch_outline(progress, env))
    .part(thumb_shadow(progress, env))
    .part(thumb(progress, env))
    .part(thumb_icon(progress, env))
    .part(switch_state_layer(progress, &tint, env))
    .part(switch_ripple(progress, &tint, env));
    SwitchLook::new(control, drag)
}

/// The look of a checkbox in `spec`.
pub fn checkbox(spec: &ToggleSpec, env: &Environment) -> CheckboxLook {
    // A checkbox has no drag: its operands sit at the value's end.
    let progress = Progress {
        drag: SwitchDrag::new(1.0, spec.direction()),
        direction: spec.direction(),
    };
    let tint = spec.tint();
    let control = ToggleControl::new(
        Size::new(TOGGLE_CHECKBOX_SIZE, TOGGLE_CHECKBOX_SIZE),
        toggle_motion(),
    )
    .label_spacing(TOGGLE_LABEL_SPACING)
    .reach(STATE_LAYER_RADIUS)
    .hit(motion::hit_region())
    .part(checkbox_outline(progress, env))
    .part(checkbox_container(progress, &tint, env))
    .part(centered_state_layer(&tint, env))
    .part(centered_ripple(&tint, env));
    CheckboxLook::new(control)
}

/// The track: surface-container-highest to the tint, to the disabled
/// containers at 12% while disabled.
fn track_paint(progress: Progress, tint: &Color, env: &Environment) -> Toggled<Paint> {
    Driven::follow(
        gather(
            [
                SurfaceContainerHighest.into(),
                tint.clone(),
                SurfaceContainerHighest
                    .with_opacity(DISABLED_CONTAINER_OPACITY)
                    .into(),
                OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY).into(),
            ],
            env,
        ),
        move |&[off, on, off_disabled, on_disabled]: &[WorkingColor; 4],
              inputs: &Inputs<'_, bool>| {
            let t = progress.at(inputs);
            Paint::from(if disabled(inputs) {
                blend(off_disabled, on_disabled, t)
            } else {
                blend(off, on, t)
            })
        },
    )
}

fn switch_track(
    progress: Progress,
    tint: &Color,
    env: &Environment,
) -> Custom<bool, (Toggled<Paint>,)> {
    Custom::new(
        (track_paint(progress, tint, env),),
        |recorder, frame, (paint,)| {
            recorder.fill(
                RoundedRect::from_rect(
                    frame,
                    RoundedRectRadii::from_single_radius(frame.height() / 2.0),
                ),
                paint,
            );
        },
    )
}

/// The unselected outline's width: 2dp shrinking to nothing as the switch
/// turns on (the border-width animates, not its opacity), with the border
/// inside the track's box.
fn outline_width(progress: Progress, inputs: &Inputs<'_, bool>) -> f64 {
    TOGGLE_SWITCH_OUTLINE_WIDTH * (1.0 - progress.at(inputs))
}

fn switch_outline(progress: Progress, env: &Environment) -> OutlinePart {
    let shape = Driven::new(move |inputs: &Inputs<'_, bool>| {
        let inset = outline_width(progress, inputs) / 2.0;
        RoundedRect::from_rect(
            inputs.frame.inflate(-inset, -inset),
            RoundedRectRadii::from_single_radius(inputs.frame.height() / 2.0 - inset),
        )
    });
    let stroke =
        Driven::new(move |inputs: &Inputs<'_, bool>| Stroke::new(outline_width(progress, inputs)));
    let paint = Driven::follow(
        gather(
            [
                Outline.into(),
                OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY).into(),
            ],
            env,
        ),
        |&[rest, dim]: &[WorkingColor; 2], inputs: &Inputs<'_, bool>| {
            Paint::from(if disabled(inputs) { dim } else { rest })
        },
    );
    Custom::new(
        (shape, stroke, paint),
        |recorder, _frame, (shape, stroke, paint)| {
            recorder.stroke(shape, stroke, paint);
        },
    )
}

/// `md.comp.switch.handle.elevation`: level1, level0 while disabled.
fn thumb_shadow(progress: Progress, env: &Environment) -> ShadowPart {
    let cast = |key: bool| {
        Driven::follow(
            Color::from(Shadow).resolve(env),
            move |shadow: &WorkingColor, inputs: &Inputs<'_, bool>| {
                let elevation = if disabled(inputs) {
                    hydrolysis_style::theme::Elevation::NONE
                } else {
                    chrome_elevation(MaterialElevationLevel::LEVEL1, *shadow)
                };
                if key {
                    elevation.key
                } else {
                    elevation.ambient
                }
            },
        )
    };
    Custom::new(
        (progress.thumb(), cast(true), progress.thumb(), cast(false)),
        |recorder, _frame, (key_shape, key, ambient_shape, ambient)| {
            recorder.shadow(key_shape, key);
            recorder.shadow(ambient_shape, ambient);
        },
    )
}

fn thumb(progress: Progress, env: &Environment) -> ShapePart {
    let paint = Driven::follow(
        gather(
            [
                OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
                Surface.into(),
                PrimaryContainer.into(),
                OnSurfaceVariant.into(),
                Outline.into(),
                OnPrimary.into(),
            ],
            env,
        ),
        move |&[
            disabled_off,
            disabled_on,
            selected,
            unselected,
            rest_off,
            rest_on,
        ]: &[WorkingColor; 6],
              inputs: &Inputs<'_, bool>| {
            let t = progress.at(inputs);
            Paint::from(if disabled(inputs) {
                blend(disabled_off, disabled_on, t)
            } else if inputs.state.intersects(
                InteractionState::PRESSED | InteractionState::HOVERED | InteractionState::FOCUSED,
            ) {
                if *inputs.value { selected } else { unselected }
            } else {
                blend(rest_off, rest_on, t)
            })
        },
    );
    Custom::new(
        (progress.thumb(), paint),
        |recorder, _frame, (circle, paint)| {
            recorder.fill(circle, paint);
        },
    )
}

/// How the thumb icon fades, per the M3 reference's staged linear
/// transitions over the 200ms value animation: entering lags behind the
/// slide (the reference waits 50ms, then ramps over 150ms) and exiting is
/// gone within the first 50ms. An animation has no delay, so entering is an
/// ease-in over the whole 200ms.
const fn icon_fade(on: bool) -> ValueMotion {
    ValueMotion::Own(if on {
        Animation::ease_in(duration::SHORT_4)
    } else {
        Animation::linear(duration::SHORT_1)
    })
}

/// The default checked icon: a 16dp checkmark on the thumb in
/// on-primary-container, scaling in from 0.92 as it fades with the switch
/// turning on.
fn thumb_icon(
    progress: Progress,
    env: &Environment,
) -> Custom<bool, (Toggled<Affine>, Toggled<Paint>)> {
    let place = Driven::new(move |inputs: &Inputs<'_, bool>| {
        let scale = lerp(TOGGLE_SWITCH_ICON_SCALE_START, 1.0, progress.at(inputs));
        let center = progress.thumb_center(inputs);
        Affine::translate(center.to_vec2()) * Affine::scale(scale)
    });
    let paint = Driven::follow(
        gather(
            [
                OnPrimaryContainer.into(),
                OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
            ],
            env,
        ),
        move |&[rest, dim]: &[WorkingColor; 2], inputs: &Inputs<'_, bool>| {
            Paint::from(faded(
                if disabled(inputs) { dim } else { rest },
                progress.at(inputs),
            ))
        },
    )
    .value_motion(|on| icon_fade(*on));
    Custom::new((place, paint), |recorder, _frame, (place, paint)| {
        recorder.transform(place, |recorder| {
            recorder.stroke(
                check_glyph_path(Rect::from_center_size(
                    Point::ORIGIN,
                    (TOGGLE_SWITCH_ICON_SIZE, TOGGLE_SWITCH_ICON_SIZE),
                )),
                Stroke::new(TOGGLE_CHECKBOX_OUTLINE_WIDTH),
                paint,
            );
        });
    })
}

/// The Material `check` glyph as a stroked path filling `bounds`.
fn check_glyph_path(bounds: Rect) -> BezPath {
    BezPath::from_vec(vec![
        PathEl::MoveTo(Point::new(
            bounds.width().mul_add(0.25, bounds.x0),
            bounds.height().mul_add(0.55, bounds.y0),
        )),
        PathEl::LineTo(Point::new(
            bounds.width().mul_add(0.45, bounds.x0),
            bounds.height().mul_add(0.75, bounds.y0),
        )),
        PathEl::LineTo(Point::new(
            bounds.width().mul_add(0.78, bounds.x0),
            bounds.height().mul_add(0.3, bounds.y0),
        )),
    ])
}

fn switch_state_layer(progress: Progress, tint: &Color, env: &Environment) -> ShapePart {
    let circle = Driven::new(move |inputs: &Inputs<'_, bool>| {
        Circle::new(progress.thumb_center(inputs), f64::from(STATE_LAYER_RADIUS))
    });
    Custom::new(
        (circle, layer_paint(tint, env, state_layer_opacity)),
        |recorder, _frame, (circle, paint)| recorder.fill(circle, paint),
    )
}

fn switch_ripple(progress: Progress, tint: &Color, env: &Environment) -> ShapePart {
    let circle = Driven::new(move |inputs: &Inputs<'_, bool>| {
        Circle::new(progress.thumb_center(inputs), ripple_radius(inputs))
    })
    .motion(ripple_motion());
    let paint = layer_paint(tint, env, ripple_opacity).motion(ripple_motion());
    Custom::new((circle, paint), |recorder, _frame, (circle, paint)| {
        recorder.fill(circle, paint);
    })
}

/// The unchecked outline: on-surface-variant (on-surface at 38% while
/// disabled), fading out as the box checks.
fn checkbox_outline_paint(progress: Progress, env: &Environment) -> Toggled<Paint> {
    Driven::follow(
        gather(
            [
                OnSurfaceVariant.into(),
                OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
            ],
            env,
        ),
        move |&[rest, dim]: &[WorkingColor; 2], inputs: &Inputs<'_, bool>| {
            Paint::from(faded(
                if disabled(inputs) { dim } else { rest },
                1.0 - progress.at(inputs),
            ))
        },
    )
}

fn checkbox_outline(progress: Progress, env: &Environment) -> Custom<bool, (Toggled<Paint>,)> {
    Custom::new(
        (checkbox_outline_paint(progress, env),),
        |recorder, frame, (paint,)| {
            recorder.stroke(
                container_shape(frame),
                Stroke::new(TOGGLE_CHECKBOX_OUTLINE_WIDTH),
                paint,
            );
        },
    )
}

fn container_shape(frame: Rect) -> RoundedRect {
    RoundedRect::from_rect(
        frame,
        RoundedRectRadii::from_single_radius(TOGGLE_CHECKBOX_CONTAINER_SHAPE),
    )
}

/// The checked container's fill and its check, fading in with the box
/// checking: the tint with an on-primary check, on-surface at 38% with a
/// surface check while disabled.
fn checkbox_paints(
    progress: Progress,
    tint: &Color,
    env: &Environment,
) -> (Toggled<Paint>, Toggled<Paint>) {
    let paint = |container: fn(&[WorkingColor; 4]) -> (WorkingColor, WorkingColor)| {
        Driven::follow(
            gather(
                [
                    tint.clone(),
                    OnPrimary.into(),
                    OnSurface.with_opacity(DISABLED_CONTENT_OPACITY).into(),
                    Surface.into(),
                ],
                env,
            ),
            move |colors: &[WorkingColor; 4], inputs: &Inputs<'_, bool>| {
                let (rest, dim) = container(colors);
                Paint::from(faded(
                    if disabled(inputs) { dim } else { rest },
                    progress.at(inputs),
                ))
            },
        )
    };
    (
        paint(|&[primary, _, dim, _]| (primary, dim)),
        paint(|&[_, on_primary, _, surface]| (on_primary, surface)),
    )
}

/// The checked container and its check, scaling in from 0.6 as they fade
/// in.
fn checkbox_container(progress: Progress, tint: &Color, env: &Environment) -> ContainerPart {
    let place = Driven::new(move |inputs: &Inputs<'_, bool>| {
        let center = inputs.frame.center().to_vec2();
        let scale = lerp(
            TOGGLE_CHECKBOX_SELECTED_SCALE_START,
            1.0,
            progress.at(inputs),
        );
        Affine::translate(center) * Affine::scale(scale) * Affine::translate(-center)
    });
    let (fill, check) = checkbox_paints(progress, tint, env);
    Custom::new(
        (place, fill, check),
        |recorder, frame, (place, fill, check)| {
            recorder.transform(place, |recorder| {
                recorder.fill(container_shape(frame), fill);
                recorder.stroke(
                    check_glyph_path(frame),
                    Stroke::new(TOGGLE_CHECKBOX_OUTLINE_WIDTH),
                    check,
                );
            });
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{checkbox, icon_fade, switch};
    use crate::color::Primary;
    use crate::dimensions::{
        TOGGLE_LABEL_SPACING, TOGGLE_SWITCH_HEIGHT, TOGGLE_SWITCH_PRESSED_HANDLE_SIZE,
        TOGGLE_SWITCH_WIDTH,
    };
    use crate::theme::motion::tokens::duration;
    use hydrolysis_style::Style as _;
    use hydrolysis_style::draw::Paint;
    use hydrolysis_style::draw::kurbo::{Circle, Point, Rect, Vec2};
    use hydrolysis_style::theme::ValueMotion;
    use hydrolysis_style::theme::{Inputs, SpecTint, SwitchDrag, SwitchLook, ToggleSpec};
    use waterui::interaction::InteractionState;
    use waterui_controls::ControlSize;
    use waterui_controls::toggle::ToggleStyle;
    use waterui_core::Environment;
    use waterui_core::layout::LayoutDirection;
    use waterui_graphics::color::Color;

    fn env() -> Environment {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        env
    }

    fn spec(style: ToggleStyle, direction: LayoutDirection) -> ToggleSpec {
        ToggleSpec::new(style, ControlSize::Small, direction, SpecTint::default())
    }

    /// The `Primary` token as the untinted toggles of these tests draw it.
    fn untinted() -> Color {
        Color::from(Primary)
    }

    fn frame() -> Rect {
        Rect::from_origin_size(
            (0.0, 0.0),
            (
                f64::from(TOGGLE_SWITCH_WIDTH),
                f64::from(TOGGLE_SWITCH_HEIGHT),
            ),
        )
    }

    fn at(value: &bool, state: InteractionState, drag: Option<Vec2>) -> Inputs<'_, bool> {
        Inputs {
            frame: frame(),
            state,
            value,
            drag,
            press_point: None,
            velocity: None,
        }
    }

    fn progress(direction: LayoutDirection) -> super::Progress {
        let look = switch(&spec(ToggleStyle::Switch, direction), &env());
        super::Progress {
            drag: *look.drag(),
            direction,
        }
    }

    #[test]
    fn the_thumb_icon_leaves_within_the_first_50ms() {
        let (ValueMotion::Own(entering), ValueMotion::Own(exiting)) =
            (icon_fade(true), icon_fade(false))
        else {
            panic!("the icon fade owns its animation");
        };
        assert_eq!(entering.duration(), duration::SHORT_4);
        assert_eq!(exiting.duration(), duration::SHORT_1);
    }

    #[test]
    fn toggle_sizes_and_spacing_are_the_material_tokens() {
        let look: SwitchLook = switch(
            &spec(ToggleStyle::Switch, LayoutDirection::LeftToRight),
            &env(),
        );
        assert_eq!(
            look.control().size(),
            kurbo_size(
                f64::from(TOGGLE_SWITCH_WIDTH),
                f64::from(TOGGLE_SWITCH_HEIGHT),
            )
        );
        assert_eq!(look.control().get_label_spacing(), TOGGLE_LABEL_SPACING);
        assert_eq!(TOGGLE_LABEL_SPACING, 8.0);
        let checkbox = checkbox(
            &spec(ToggleStyle::Checkbox, LayoutDirection::LeftToRight),
            &env(),
        );
        assert_eq!(checkbox.control().size(), kurbo_size(18.0, 18.0));
    }

    fn kurbo_size(width: f64, height: f64) -> hydrolysis_style::draw::kurbo::Size {
        hydrolysis_style::draw::kurbo::Size::new(width, height)
    }

    /// Compose's `ThumbPadding` is `(32 - 24) / 2 = 4`, and the thumb sits in a
    /// 24dp slot at each end regardless of the size it currently draws at. The
    /// rest centres are therefore 4 + 12 = 16dp from either edge of the 52dp
    /// track, which is what keeps the thumb still while it changes size.
    #[test]
    fn switch_thumb_rests_at_compose_centers() {
        let progress = progress(LayoutDirection::LeftToRight);
        let off = progress
            .thumb()
            .target(&at(&false, InteractionState::empty(), None));
        let on = progress
            .thumb()
            .target(&at(&true, InteractionState::empty(), None));
        assert_eq!(off.center, Point::new(16.0, 16.0));
        assert_eq!(on.center, Point::new(36.0, 16.0));
        // Pressing grows the thumb in place.
        let pressed = progress
            .thumb()
            .target(&at(&false, InteractionState::PRESSED, None));
        assert_eq!(pressed.center, off.center);
        assert_eq!(pressed.radius, TOGGLE_SWITCH_PRESSED_HANDLE_SIZE / 2.0);
    }

    #[test]
    fn switch_thumb_mirrors_in_a_right_to_left_layout() {
        let progress = progress(LayoutDirection::RightToLeft);
        let off = progress
            .thumb()
            .target(&at(&false, InteractionState::empty(), None));
        let on = progress
            .thumb()
            .target(&at(&true, InteractionState::empty(), None));
        assert_eq!(off.center, Point::new(36.0, 16.0));
        assert_eq!(on.center, Point::new(16.0, 16.0));
    }

    #[test]
    fn a_drag_moves_the_thumb_between_its_centres() {
        let progress = progress(LayoutDirection::LeftToRight);
        let halfway: Circle = progress.thumb().target(&at(
            &false,
            InteractionState::DRAGGED,
            Some(Vec2::new(10.0, 0.0)),
        ));
        assert_eq!(halfway.center, Point::new(26.0, 16.0));
    }

    /// The border-width animates 2dp to 0 with the value transition, not the
    /// outline's opacity.
    #[test]
    fn switch_outline_width_shrinks_with_progress() {
        let progress = progress(LayoutDirection::LeftToRight);
        let rest = &false;
        assert_eq!(
            super::outline_width(progress, &at(rest, InteractionState::empty(), None)),
            2.0
        );
        assert_eq!(
            super::outline_width(
                progress,
                &at(
                    &true,
                    InteractionState::DRAGGED,
                    Some(Vec2::new(-10.0, 0.0))
                )
            ),
            1.0
        );
        assert_eq!(
            super::outline_width(progress, &at(&true, InteractionState::empty(), None)),
            0.0
        );
    }

    #[test]
    fn disabled_material_switch_uses_disabled_palette() {
        use crate::color::{OnSurface, SurfaceContainerHighest};
        use crate::theme::colors::DISABLED_CONTAINER_OPACITY;
        use waterui_core::Signal as _;
        use waterui_graphics::color::Color;

        let env = env();
        let progress = progress(LayoutDirection::LeftToRight);
        let paint = super::track_paint(progress, &untinted(), &env);
        let disabled = InteractionState::DISABLED;
        // Unchecked: surface-container-highest at 12%; checked: on-surface
        // at 12%.
        assert_close(
            &paint.target(&at(&false, disabled, None)),
            &Paint::from(
                Color::from(SurfaceContainerHighest.with_opacity(DISABLED_CONTAINER_OPACITY))
                    .resolve(&env)
                    .snapshot(),
            ),
        );
        assert_close(
            &paint.target(&at(&true, disabled, None)),
            &Paint::from(
                Color::from(OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY))
                    .resolve(&env)
                    .snapshot(),
            ),
        );
        // Enabled and on, the track is primary.
        assert_close(
            &paint.target(&at(&true, InteractionState::empty(), None)),
            &Paint::from(Color::from(Primary).resolve(&env).snapshot()),
        );
    }

    /// A tinted subtree's switch track and checkbox container draw in the
    /// tint; an untinted one's draw in `Primary`, the `Accent` token's value.
    #[test]
    fn toggles_draw_with_the_subtrees_tint() {
        use hydrolysis_style::draw::WorkingColor;
        use waterui::theme::Tint;
        use waterui_core::Signal as _;
        use waterui_graphics::color::Working;

        let tint = WorkingColor::new([0.9, 0.1, 0.2, 1.0]);
        let env = env().extending(Tint::new(Color::new(Working(tint))));
        let spec = ToggleSpec::new(
            ToggleStyle::Switch,
            ControlSize::Small,
            LayoutDirection::LeftToRight,
            SpecTint::resolve(&env),
        );
        let progress = progress(LayoutDirection::LeftToRight);
        let on = InteractionState::empty();
        assert_close(
            &super::track_paint(progress, &spec.tint(), &env).target(&at(&true, on, None)),
            &Paint::from(tint),
        );
        let (fill, _) = super::checkbox_paints(progress, &spec.tint(), &env);
        assert_close(&fill.target(&at(&true, on, None)), &Paint::from(tint));

        let plain = Paint::from(Color::from(Primary).resolve(&env).snapshot());
        assert_ne!(plain, Paint::from(tint));
    }

    /// Two solid paints equal to the blend's float rounding.
    fn assert_close(got: &Paint, want: &Paint) {
        let (Paint::Solid(got), Paint::Solid(want)) = (got, want) else {
            panic!("a Material toggle paints solid colours, not {got:?} and {want:?}");
        };
        for (channel, expected) in got.components.iter().zip(want.components) {
            assert!((channel - expected).abs() < 1e-5, "{got:?} is not {want:?}");
        }
    }

    fn alpha(paint: Paint) -> f32 {
        match paint {
            Paint::Solid(color) => color.components[3],
            other => panic!("a Material checkbox paints solid colours, not {other:?}"),
        }
    }

    /// The unchecked outline fades out as the container fades in.
    #[test]
    fn checkbox_outline_and_container_trade_places() {
        let env = env();
        let progress = super::Progress {
            drag: SwitchDrag::new(1.0, LayoutDirection::LeftToRight),
            direction: LayoutDirection::LeftToRight,
        };
        let outline = super::checkbox_outline_paint(progress, &env);
        let (fill, check) = super::checkbox_paints(progress, &untinted(), &env);
        let rest = InteractionState::empty();
        assert_eq!(alpha(outline.target(&at(&false, rest, None))), 1.0);
        assert_eq!(alpha(outline.target(&at(&true, rest, None))), 0.0);
        assert_eq!(alpha(fill.target(&at(&false, rest, None))), 0.0);
        assert_eq!(alpha(fill.target(&at(&true, rest, None))), 1.0);
        assert_eq!(alpha(check.target(&at(&true, rest, None))), 1.0);
    }

    /// Disabled, the unchecked outline drops to on-surface at 38% and the
    /// checked container to on-surface at 38% under a surface check.
    #[test]
    fn disabled_material_checkbox_uses_disabled_palette() {
        use crate::color::{OnSurface, Surface};
        use crate::theme::colors::DISABLED_CONTENT_OPACITY;
        use waterui_core::Signal as _;
        use waterui_graphics::color::Color;

        let env = env();
        let progress = super::Progress {
            drag: SwitchDrag::new(1.0, LayoutDirection::LeftToRight),
            direction: LayoutDirection::LeftToRight,
        };
        let dim = Paint::from(
            Color::from(OnSurface.with_opacity(DISABLED_CONTENT_OPACITY))
                .resolve(&env)
                .snapshot(),
        );
        let surface = Paint::from(Color::from(Surface).resolve(&env).snapshot());
        let disabled = InteractionState::DISABLED;
        let outline = super::checkbox_outline_paint(progress, &env);
        assert_eq!(outline.target(&at(&false, disabled, None)), dim);
        let (fill, check) = super::checkbox_paints(progress, &untinted(), &env);
        assert_eq!(fill.target(&at(&true, disabled, None)), dim);
        assert_eq!(check.target(&at(&true, disabled, None)), surface);
    }
}
