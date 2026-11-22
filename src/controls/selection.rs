use crate::color::OnSurface;
use crate::semantics::gather;
use crate::theme::motion::{
    self, FOCUS_STATE_LAYER_OPACITY, HOVER_STATE_LAYER_OPACITY, PRESSED_STATE_LAYER_OPACITY,
    RIPPLE_INITIAL_SCALE,
};
use hydrolysis_style::draw::kurbo::Circle;
use hydrolysis_style::draw::{Draw as _, Paint, WorkingColor};
use hydrolysis_style::theme::{Custom, Driven, Inputs, Motion, Transition};
use waterui::interaction::InteractionState;
use waterui::reactive::Computed;
use waterui_core::Environment;
use waterui_graphics::color::Color;

/// A part drawing a shape and its paint.
pub type ShapePart = Custom<bool, (Toggled<Circle>, Toggled<Paint>)>;

/// A `bool`-valued [`Driven`].
pub type Toggled<T> = Driven<T, bool>;

/// `md.comp.switch.state-layer.size` / `md.comp.checkbox.state-layer.size`
/// over two: the unbounded state layer's radius.
pub const STATE_LAYER_RADIUS: f64 = 20.0;

/// The Material toggle's motion: the control chrome's, with the value
/// running the 200 ms standard easing whatever animation the change names.
pub fn toggle_motion() -> Motion {
    motion::value(motion::toggle_value())
}

pub const fn disabled(inputs: &Inputs<'_, bool>) -> bool {
    inputs.state.contains(InteractionState::DISABLED)
}

/// `t` as the `f32` the colour blend takes.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a blend factor in [0, 1] loses nothing visible in f32"
)]
pub(super) const fn narrow(t: f64) -> f32 {
    t as f32
}

/// `color` at `opacity` of its own alpha.
pub fn faded(color: WorkingColor, opacity: f64) -> WorkingColor {
    color.with_alpha(color.components[3] * narrow(opacity))
}

/// The state layer's opacity while hovered or focused; the press is the
/// ripple's.
pub(super) fn state_layer_opacity(inputs: &Inputs<'_, bool>) -> f64 {
    if disabled(inputs) {
        0.0
    } else if inputs.state.contains(InteractionState::FOCUSED) {
        f64::from(FOCUS_STATE_LAYER_OPACITY)
    } else if inputs.state.contains(InteractionState::HOVERED) {
        f64::from(HOVER_STATE_LAYER_OPACITY)
    } else {
        0.0
    }
}

/// The state-layer colour: the tint while on, on-surface while off.
fn layer_color(tint: &Color, env: &Environment) -> Computed<[WorkingColor; 2]> {
    gather([tint.clone(), OnSurface.into()], env)
}

pub(super) fn layer_paint(
    tint: &Color,
    env: &Environment,
    opacity: impl Fn(&Inputs<'_, bool>) -> f64 + 'static,
) -> Toggled<Paint> {
    Driven::follow(
        layer_color(tint, env),
        move |&[on, off]: &[WorkingColor; 2], inputs: &Inputs<'_, bool>| {
            Paint::from(faded(if *inputs.value { on } else { off }, opacity(inputs)))
        },
    )
}

/// The press ripple of an unbounded layer: it grows from the initial scale
/// to the layer's radius and fades in on press, and fades out on release.
pub(super) fn ripple_motion() -> Motion {
    let ripple = motion::ripple();
    let mut motion = toggle_motion();
    motion.press = Transition::new(ripple.grow, ripple.fade_out);
    motion
}

pub(super) fn ripple_radius(inputs: &Inputs<'_, bool>) -> f64 {
    if inputs.state.contains(InteractionState::PRESSED) {
        STATE_LAYER_RADIUS
    } else {
        STATE_LAYER_RADIUS * RIPPLE_INITIAL_SCALE
    }
}

pub(super) fn ripple_opacity(inputs: &Inputs<'_, bool>) -> f64 {
    if inputs.state.contains(InteractionState::PRESSED) && !disabled(inputs) {
        f64::from(PRESSED_STATE_LAYER_OPACITY)
    } else {
        0.0
    }
}

/// The hover and focus state layer of a toggle-like control: an unbounded
/// circle of [`STATE_LAYER_RADIUS`] centred on the control, in the tint while
/// on and on-surface while off.
pub fn centered_state_layer(tint: &Color, env: &Environment) -> ShapePart {
    let circle = Driven::new(|inputs: &Inputs<'_, bool>| {
        Circle::new(inputs.frame.center(), STATE_LAYER_RADIUS)
    });
    Custom::new(
        (circle, layer_paint(tint, env, state_layer_opacity)),
        |recorder, _frame, (circle, paint)| recorder.fill(circle, paint),
    )
}

/// The press ripple of a toggle-like control, over the same circle as
/// [`centered_state_layer`].
pub fn centered_ripple(tint: &Color, env: &Environment) -> ShapePart {
    let circle = Driven::new(|inputs: &Inputs<'_, bool>| {
        Circle::new(inputs.frame.center(), ripple_radius(inputs))
    })
    .motion(ripple_motion());
    let paint = layer_paint(tint, env, ripple_opacity).motion(ripple_motion());
    Custom::new((circle, paint), |recorder, _frame, (circle, paint)| {
        recorder.fill(circle, paint);
    })
}
