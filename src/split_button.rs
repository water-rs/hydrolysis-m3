//! Material Design 3 split button composed from `WaterUI` primitives.
//!
//! A split button is two buttons sharing one silhouette: a leading button that
//! performs the action and a trailing one that opens a menu of alternatives.
//! Their touching corners are tucked in while their outer corners stay fully
//! round, which is what makes the pair read as a single control with a seam
//! rather than as two adjacent buttons.

use core::fmt::{self, Debug};
use core::marker::PhantomData;

use crate::view_ext::ViewExt as _;
use nami::{SignalExt as _, binding};
use waterui_accessibility::{AccessibilityChildren, AccessibilityRole};
use waterui_controls::label::{IntoLabel, Label, LabelKind};
use waterui_core::dynamic::watch;
use waterui_core::handler::{ExtractionError, Handler, ReportExtraction as _, boxed_action};
use waterui_core::{AnyView, Environment, Str, View};
use waterui_graphics::color::Color;
use waterui_interaction::interaction::{InteractionState, StateValue};
use waterui_layout::padding::EdgeInsets;
use waterui_layout::stack::hstack;
use waterui_shape::{CornerRadii, FilledShape, Path, RoundedRectangle, ShapeExt as _};

use crate::action::{NoAction, noop};
use hydrolysis_style::draw::kurbo::RoundedRectRadii;

use crate::color::{OnPrimary, OnSecondaryContainer, Primary, SecondaryContainer};
use crate::semantics::interaction_chrome;

/// `SplitButtonSmallTokens.ContainerHeight`.
const CONTAINER_HEIGHT: f32 = 40.0;
/// `SplitButtonSmallTokens.BetweenSpace`: the seam between the two halves.
const BETWEEN_SPACE: f32 = 2.0;
/// `SplitButtonSmallTokens.LeadingButtonLeadingSpace`.
const LEADING_BUTTON_LEADING_SPACE: f32 = 16.0;
/// `SplitButtonSmallTokens.LeadingButtonTrailingSpace`.
const LEADING_BUTTON_TRAILING_SPACE: f32 = 12.0;
/// `SplitButtonSmallTokens.TrailingButtonLeadingSpace` / `TrailingSpace`.
const TRAILING_BUTTON_SPACE: f32 = 13.0;
/// `SplitButtonSmallTokens.TrailingIconSize`.
const TRAILING_ICON_SIZE: f32 = 22.0;
/// `SplitButtonSmallTokens.InnerCornerCornerSize`, `CornerValueExtraSmall`:
/// the radius on the two corners either side of the seam.
const INNER_CORNER_RADIUS: f32 = 4.0;
/// `SplitButtonSmallTokens.PressedInnerCornerCornerSize`,
/// `CornerValueMedium`: the seam corners soften while a half is hovered or
/// held — the hovered token's shape matches it.
const PRESSED_INNER_CORNER_RADIUS: f32 = 12.0;

/// The outer corners are `CornerFull`, which on this container is half its
/// height.
const OUTER_CORNER_RADIUS: f32 = CONTAINER_HEIGHT / 2.0;

/// Colour token set for a split button variant.
pub trait SplitButtonVariantTokens: Default + 'static {
    /// Container colour of both halves.
    fn container_color() -> Color;

    /// Label and icon colour.
    fn content_color() -> Color;
}

/// Filled split button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct FilledSplitButton;

/// Tonal split button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct TonalSplitButton;

impl SplitButtonVariantTokens for FilledSplitButton {
    fn container_color() -> Color {
        Primary.into()
    }

    fn content_color() -> Color {
        OnPrimary.into()
    }
}

impl SplitButtonVariantTokens for TonalSplitButton {
    fn container_color() -> Color {
        SecondaryContainer.into()
    }

    fn content_color() -> Color {
        OnSecondaryContainer.into()
    }
}

/// A Material Design 3 split button.
pub struct SplitButton<L, Action = NoAction, TrailingAction = NoAction, Tokens = FilledSplitButton>
{
    label: Label<L>,
    trailing_accessibility_label: Str,
    action: Action,
    trailing_action: TrailingAction,
    tokens: PhantomData<Tokens>,
}

impl<L: Debug, Action, TrailingAction, Tokens> Debug
    for SplitButton<L, Action, TrailingAction, Tokens>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitButton")
            .field("label", &self.label)
            .field(
                "trailing_accessibility_label",
                &self.trailing_accessibility_label,
            )
            .finish_non_exhaustive()
    }
}

impl<L: LabelKind> SplitButton<L> {
    /// Creates a split button.
    ///
    /// `trailing_accessibility_label` names what the trailing half opens; it
    /// has no visible text of its own, only a chevron, so assistive technology
    /// has nothing else to announce.
    #[must_use]
    pub fn new(
        label: impl IntoLabel<Kind = L>,
        trailing_accessibility_label: impl Into<Str>,
    ) -> Self {
        Self {
            label: label.into_label(),
            trailing_accessibility_label: trailing_accessibility_label.into(),
            action: noop,
            trailing_action: noop,
            tokens: PhantomData,
        }
    }
}

impl<L, Action, TrailingAction, Tokens> SplitButton<L, Action, TrailingAction, Tokens> {
    /// Uses tonal split button tokens.
    #[must_use]
    pub fn tonal(self) -> SplitButton<L, Action, TrailingAction, TonalSplitButton> {
        SplitButton {
            label: self.label,
            trailing_accessibility_label: self.trailing_accessibility_label,
            action: self.action,
            trailing_action: self.trailing_action,
            tokens: PhantomData,
        }
    }

    /// Sets the action the leading half performs.
    #[must_use]
    pub fn action<F, Args>(
        self,
        action: F,
    ) -> SplitButton<
        L,
        impl FnMut(&Environment) -> Result<(), ExtractionError>,
        TrailingAction,
        Tokens,
    >
    where
        F: Handler<Args, ()> + 'static,
    {
        SplitButton {
            label: self.label,
            trailing_accessibility_label: self.trailing_accessibility_label,
            action: boxed_action(action),
            trailing_action: self.trailing_action,
            tokens: PhantomData,
        }
    }

    /// Sets the action the trailing half performs.
    #[must_use]
    pub fn trailing_action<F, Args>(
        self,
        action: F,
    ) -> SplitButton<L, Action, impl FnMut(&Environment) -> Result<(), ExtractionError>, Tokens>
    where
        F: Handler<Args, ()> + 'static,
    {
        SplitButton {
            label: self.label,
            trailing_accessibility_label: self.trailing_accessibility_label,
            action: self.action,
            trailing_action: boxed_action(action),
            tokens: PhantomData,
        }
    }
}

impl<L, Action, TrailingAction, Tokens> View for SplitButton<L, Action, TrailingAction, Tokens>
where
    L: LabelKind,
    Action: FnMut(&Environment) -> Result<(), ExtractionError> + 'static,
    TrailingAction: FnMut(&Environment) -> Result<(), ExtractionError> + 'static,
    Tokens: SplitButtonVariantTokens,
{
    fn body(self, _env: &Environment) -> impl View {
        let mut action = self.action;
        let mut trailing_action = self.trailing_action;
        let outer = OUTER_CORNER_RADIUS;
        let inner = INNER_CORNER_RADIUS;
        let soft_inner = PRESSED_INNER_CORNER_RADIUS;

        // Round on the outside, tucked in against the seam. Each half reports
        // its own state through `.interaction_state`, and
        // `md.comp.split-button.*.{hovered,pressed}.inner-corner-radius`
        // (CornerValueMedium) softens its seam corners while it is hovered or
        // held.
        let leading_state = binding(InteractionState::empty());
        let leading_shape = leading_state.map(move |state: InteractionState| {
            let seam = if state.intersects(InteractionState::HOVERED | InteractionState::PRESSED) {
                soft_inner
            } else {
                inner
            };
            RoundedRectangle::new(CornerRadii::new(outer, seam, outer, seam))
        });
        let trailing_state = binding(InteractionState::empty());
        let trailing_shape = trailing_state.map(move |state: InteractionState| {
            let seam = if state.intersects(InteractionState::HOVERED | InteractionState::PRESSED) {
                soft_inner
            } else {
                inner
            };
            RoundedRectangle::new(CornerRadii::new(seam, outer, seam, outer))
        });

        let leading = AnyView::new(
            self.label
                .font(crate::theme::typography::label_large())
                .foreground(Tokens::content_color())
                .padding_with(EdgeInsets::new(
                    0.0,
                    LEADING_BUTTON_LEADING_SPACE,
                    0.0,
                    LEADING_BUTTON_TRAILING_SPACE,
                ))
                .height(CONTAINER_HEIGHT)
                .background(ReactiveSplitShape {
                    shape: leading_shape,
                    color: Tokens::container_color(),
                })
                .on_tap(move |env: Environment| {
                    action(&env).report_extraction();
                })
                .a11y_role(AccessibilityRole::Button)
                .interaction_state(&leading_state)
                .install(half_style(
                    Tokens::content_color(),
                    RoundedRectRadii::new(
                        f64::from(OUTER_CORNER_RADIUS),
                        f64::from(INNER_CORNER_RADIUS),
                        f64::from(INNER_CORNER_RADIUS),
                        f64::from(OUTER_CORNER_RADIUS),
                    ),
                    RoundedRectRadii::new(
                        f64::from(OUTER_CORNER_RADIUS),
                        f64::from(PRESSED_INNER_CORNER_RADIUS),
                        f64::from(PRESSED_INNER_CORNER_RADIUS),
                        f64::from(OUTER_CORNER_RADIUS),
                    ),
                )),
        );

        let chevron = ChevronIcon {
            color: Tokens::content_color(),
            size: TRAILING_ICON_SIZE,
        };
        let trailing = AnyView::new(
            chevron
                .padding_with(EdgeInsets::new(
                    0.0,
                    TRAILING_BUTTON_SPACE,
                    0.0,
                    TRAILING_BUTTON_SPACE,
                ))
                .height(CONTAINER_HEIGHT)
                .background(ReactiveSplitShape {
                    shape: trailing_shape,
                    color: Tokens::container_color(),
                })
                .on_tap(move |env: Environment| {
                    trailing_action(&env).report_extraction();
                })
                .a11y_label(self.trailing_accessibility_label)
                .a11y_role(AccessibilityRole::Button)
                .a11y_children(AccessibilityChildren::ExcludeDescendants)
                .interaction_state(&trailing_state)
                .install(half_style(
                    Tokens::content_color(),
                    RoundedRectRadii::new(
                        f64::from(INNER_CORNER_RADIUS),
                        f64::from(OUTER_CORNER_RADIUS),
                        f64::from(OUTER_CORNER_RADIUS),
                        f64::from(INNER_CORNER_RADIUS),
                    ),
                    RoundedRectRadii::new(
                        f64::from(PRESSED_INNER_CORNER_RADIUS),
                        f64::from(OUTER_CORNER_RADIUS),
                        f64::from(OUTER_CORNER_RADIUS),
                        f64::from(PRESSED_INNER_CORNER_RADIUS),
                    ),
                )),
        );

        hstack((leading, trailing)).spacing(BETWEEN_SPACE)
    }
}

/// One half's chrome: the state layer traces the resting corners and
/// softens to `CornerValueMedium` on the seam side while the half is
/// hovered or held.
fn half_style(
    state_layer_color: impl Into<Color>,
    resting: RoundedRectRadii,
    soft: RoundedRectRadii,
) -> hydrolysis_style::theme::Chrome {
    interaction_chrome(
        state_layer_color,
        StateValue::new(resting)
            .when(InteractionState::PRESSED, soft)
            .when(InteractionState::HOVERED, soft),
    )
}

/// The trailing half's chevron, sized and tinted by the variant.
#[derive(Debug, Clone)]
struct ChevronIcon {
    color: Color,
    size: f32,
}

impl View for ChevronIcon {
    fn body(self, _env: &Environment) -> impl View {
        // The outline is authored on Material's 24dp grid; `FilledShape` wants
        // unit-square commands, so it is normalized here rather than restated.
        let grid = crate::icon_paths::icon_grid_size();
        let normalize = |value: f64| {
            #[allow(
                clippy::cast_possible_truncation,
                reason = "icon grid coordinates are small and exact in f32"
            )]
            let normalized = (value / grid) as f32;
            normalized
        };
        let mut corners = crate::icon_paths::CHEVRON_DOWN_OUTLINE.into_iter();
        let (start_x, start_y) = corners.next().expect("chevron outline is non-empty");
        let path = corners
            .fold(
                Path::new().move_to(normalize(start_x), normalize(start_y)),
                |path, (x, y)| path.line_to(normalize(x), normalize(y)),
            )
            .close();
        FilledShape::new(path, self.color).size(self.size, self.size)
    }
}

/// One half's container, whose seam corners follow its pressed state.
#[derive(Debug)]
struct ReactiveSplitShape<S> {
    shape: S,
    color: Color,
}

impl<S> View for ReactiveSplitShape<S>
where
    S: nami::Signal<Output = RoundedRectangle> + 'static,
{
    fn body(self, _env: &Environment) -> impl View {
        let color = self.color;
        watch(self.shape, move |shape| shape.fill(color.clone()))
    }
}

/// Creates a Material Design 3 split button.
#[must_use]
pub fn split_button<L: IntoLabel>(
    label: L,
    trailing_accessibility_label: impl Into<Str>,
) -> SplitButton<L::Kind> {
    SplitButton::new(label, trailing_accessibility_label)
}

#[cfg(test)]
mod tests {
    use super::{
        BETWEEN_SPACE, CONTAINER_HEIGHT, INNER_CORNER_RADIUS, LEADING_BUTTON_LEADING_SPACE,
        LEADING_BUTTON_TRAILING_SPACE, OUTER_CORNER_RADIUS, TRAILING_BUTTON_SPACE,
        TRAILING_ICON_SIZE,
    };

    /// Values from `androidx.compose.material3.tokens.SplitButtonSmallTokens`.
    #[test]
    fn split_button_tokens_match_compose_split_button_tokens() {
        assert_eq!(CONTAINER_HEIGHT, 40.0);
        assert_eq!(BETWEEN_SPACE, 2.0);
        assert_eq!(LEADING_BUTTON_LEADING_SPACE, 16.0);
        assert_eq!(LEADING_BUTTON_TRAILING_SPACE, 12.0);
        assert_eq!(TRAILING_BUTTON_SPACE, 13.0);
        assert_eq!(TRAILING_ICON_SIZE, 22.0);
        // InnerCornerCornerSize is ShapeTokens.CornerValueExtraSmall.
        assert_eq!(INNER_CORNER_RADIUS, 4.0);
        // The outer corners are CornerFull on a 40dp container.
        assert_eq!(OUTER_CORNER_RADIUS, 20.0);
    }

    /// Corner radii stay within half the container height so tucked-in seam
    /// corners do not clamp and round as much as the outer ones.
    #[test]
    fn corner_radii_fit_within_half_container_height() {
        assert!(
            (OUTER_CORNER_RADIUS - CONTAINER_HEIGHT / 2.0).abs() < 1e-6,
            "outer corner is fully round"
        );
        const {
            assert!(
                INNER_CORNER_RADIUS > 0.0 && INNER_CORNER_RADIUS < OUTER_CORNER_RADIUS,
                "the seam is tucked in"
            );
        }
    }
}
