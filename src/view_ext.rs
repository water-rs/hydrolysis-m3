//! Crate-private view-composition modifiers backed directly by foundation crates,
//! so `hydrolysis-m3` does not depend on the `waterui` facade.

use nami::{
    Binding, Signal,
    signal::{IntoComputed, IntoSignal},
};
use waterui_accessibility::{
    AccessibilityChildren, AccessibilityHidden, AccessibilityLabel, AccessibilityRole,
    AccessibilityState, AccessibilityStateSignal,
};
use waterui_controls::{
    anchored_overlay::{AnchoredOverlay, AnchoredOverlayView},
    theme,
};
use waterui_core::{
    Angle, IntoSignalF32, Metadata, Str, View,
    env::Installed,
    handler::{EventHandler, Handler},
    plugin::Plugin,
};
use waterui_graphics::color::Color;
#[cfg(feature = "list")]
use waterui_interaction::interaction::Selected;
use waterui_interaction::{
    event::{OnChange, OnEvent},
    gesture::{GestureObserver, GestureType, LongPressGesture, TapGesture},
    interaction::{Hittable, InteractionReport, InteractionState},
};
use waterui_layout::{
    frame::Frame,
    padding::{EdgeInsets, Padding},
};
use waterui_shape::{
    Floating,
    background::IntoBackground,
    border::Border,
    filter::Opacity,
    style::{Anchor, Offset, Rotation, Scale, Shadow},
};

pub trait ViewExt: View + Sized {
    fn opacity(self, amount: impl IntoSignalF32) -> Metadata<Opacity, Self> {
        Metadata::new(self, Opacity::new(amount))
    }

    fn background<B: IntoBackground<M>, M>(self, background: B) -> B::Output<Self> {
        background.apply_background(self)
    }

    fn foreground(self, color: impl IntoSignal<Color> + 'static) -> impl View {
        self.install(theme::ForegroundOverride::new(color))
    }

    fn anchored_overlay<V: View>(
        self,
        overlay: AnchoredOverlay<V>,
    ) -> AnchoredOverlayView<Self, V> {
        AnchoredOverlayView::new(self, overlay)
    }

    fn width(self, width: impl IntoSignalF32 + 'static) -> Frame<Self> {
        Frame::new(self).width(width)
    }

    fn height(self, height: impl IntoSignalF32 + 'static) -> Frame<Self> {
        Frame::new(self).height(height)
    }

    fn min_width(self, width: impl IntoSignalF32 + 'static) -> Frame<Self> {
        Frame::new(self).min_width(width)
    }

    fn max_width(self, width: impl IntoSignalF32 + 'static) -> Frame<Self> {
        Frame::new(self).max_width(width)
    }

    fn size(
        self,
        width: impl IntoSignalF32 + 'static,
        height: impl IntoSignalF32 + 'static,
    ) -> Frame<Self> {
        Frame::new(self).width(width).height(height)
    }

    fn padding_with(self, edge: impl IntoComputed<EdgeInsets>) -> Padding<Self> {
        Padding::new(edge, self)
    }

    fn gesture<G, H, Args>(self, gesture: G, action: H) -> Metadata<GestureObserver, Self>
    where
        G: GestureType,
        H: EventHandler<G::Event, Args>,
    {
        Metadata::new(self, GestureObserver::new(gesture, action))
    }

    fn on_tap<H, Args>(self, action: H) -> Metadata<GestureObserver, Self>
    where
        H: Handler<Args, ()>,
    {
        Metadata::new(
            self,
            GestureObserver::without_event(TapGesture::new(), action),
        )
    }

    fn on_long_press<H, Args>(self, action: H) -> Metadata<GestureObserver, Self>
    where
        H: Handler<Args, ()>,
    {
        Metadata::new(
            self,
            GestureObserver::without_event(LongPressGesture::new(), action),
        )
    }

    fn on_change<C, H, A>(self, source: &C, handler: H) -> OnChange<Self>
    where
        C: Signal,
        C::Output: PartialEq + Clone,
        H: EventHandler<C::Output, A>,
    {
        OnChange::new(self, source, handler)
    }

    fn on_hover_enter<H, Args>(self, handler: H) -> Metadata<OnEvent, Self>
    where
        H: Handler<Args, ()>,
    {
        Metadata::new(self, OnEvent::hover_enter(handler))
    }

    fn on_hover_exit<H, Args>(self, handler: H) -> Metadata<OnEvent, Self>
    where
        H: Handler<Args, ()>,
    {
        Metadata::new(self, OnEvent::hover_exit(handler))
    }

    fn interaction_state(self, state: &Binding<InteractionState>) -> impl View + use<Self> {
        self.install(InteractionReport(state.clone()))
    }

    #[cfg(feature = "list")]
    fn selected(self, is_selected: impl IntoComputed<bool>) -> Metadata<Selected, Self> {
        Metadata::new(self, Selected(is_selected.into_computed()))
    }

    fn shadow(self, shadow: impl Into<Shadow>) -> Metadata<Shadow, Self> {
        Metadata::new(self, shadow.into())
    }

    fn floating(self) -> Metadata<Floating, Self> {
        Metadata::new(self, Floating)
    }

    fn border_with(self, border: Border) -> Metadata<Border, Self> {
        Metadata::new(self, border)
    }

    fn scale_from(self, factor: impl IntoSignalF32, anchor: Anchor) -> Metadata<Scale, Self> {
        Metadata::new(self, Scale::uniform_from(factor, anchor))
    }

    fn rotation(self, angle: impl IntoComputed<Angle>) -> Metadata<Rotation, Self> {
        Metadata::new(self, Rotation::new(angle))
    }

    fn offset(self, x: impl IntoSignalF32, y: impl IntoSignalF32) -> Metadata<Offset, Self> {
        Metadata::new(self, Offset::new(x, y))
    }

    fn install(self, plugin: impl Plugin) -> impl View {
        Installed::new(self, plugin)
    }

    fn hittable(self, enabled: impl IntoComputed<bool>) -> Metadata<Hittable, Self> {
        Metadata::new(self, Hittable::new(enabled))
    }

    fn a11y_label(self, label: impl IntoComputed<Str>) -> Metadata<AccessibilityLabel, Self> {
        Metadata::new(self, AccessibilityLabel::new(label))
    }

    fn a11y_role(self, role: AccessibilityRole) -> Metadata<AccessibilityRole, Self> {
        Metadata::new(self, role)
    }

    fn a11y_state(
        self,
        state: impl IntoComputed<AccessibilityState>,
    ) -> Metadata<AccessibilityStateSignal, Self> {
        Metadata::new(self, AccessibilityStateSignal::new(state))
    }

    fn a11y_children(
        self,
        children: AccessibilityChildren,
    ) -> Metadata<AccessibilityChildren, Self> {
        Metadata::new(self, children)
    }

    fn a11y_hidden(self, hidden: bool) -> Metadata<AccessibilityHidden, Self> {
        Metadata::new(self, AccessibilityHidden::new(hidden))
    }
}

impl<V: View + Sized> ViewExt for V {}
