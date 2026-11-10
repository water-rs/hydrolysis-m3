//! Material Design 3 icon buttons composed from `WaterUI` primitives.

use core::fmt::{self, Debug};
use core::marker::PhantomData;
use hydrolysis::theme::{ButtonLook, ButtonSpec, CornerShape, Corners, Surface};
use waterui_graphics::draw::kurbo::Size;

use waterui::accessibility::{AccessibilityChildren, AccessibilityRole};
use waterui::border::Border;
use waterui::color::Color;
use waterui::shape::{Circle, ShapeExt as _};
use waterui::{Environment, Str, View, ViewExt as _};
use waterui_core::handler::{Handler, boxed_action};

use crate::button;
use crate::color::{
    InverseOnSurface, InverseSurface, OnPrimary, OnSecondaryContainer, OnSurface, OnSurfaceVariant,
    OutlineVariant, Primary, SecondaryContainer,
};
use crate::semantics::interaction_chrome;
use crate::theme::colors::DISABLED_CONTAINER_OPACITY;
use waterui_controls::ControlSize;
use waterui_controls::button::ButtonStyle;

/// `IconButtonTokens.StateLayerSize`: the size of the visible state layer.
const ICON_BUTTON_STATE_LAYER_SIZE: f32 = 40.0;
/// `IconButtonTokens.IconSize`.
const ICON_BUTTON_ICON_SIZE: f32 = 24.0;
/// Compose wraps every icon button in `minimumInteractiveComponentSize()`,
/// documented as "an overall minimum touch target size of 48 x 48dp, to meet
/// accessibility guidelines". Compose can expand the touch target without
/// growing the layout; `WaterUI` has no such split, so the button occupies the
/// full target and draws its 40dp state layer centered inside.
const ICON_BUTTON_TOUCH_TARGET_SIZE: f32 = 48.0;
const ICON_BUTTON_OUTLINE_WIDTH: f32 = 1.0;

/// The geometry of one icon-button size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconButtonSizeTokens {
    /// The box the button occupies, which is also its hit area.
    pub container: f32,
    /// The state layer drawn inside that box.
    pub state_layer: f32,
    /// `IconSize`.
    pub icon: f32,
    /// `OutlinedOutlineWidth`.
    pub outline_width: f32,
}

/// How large an icon button is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IconButtonSize {
    /// `md.comp.icon-button.xsmall`: a 32dp state layer inside the 48dp
    /// minimum touch target.
    ExtraSmall,
    /// `md.comp.icon-button.small`: a 40dp state layer inside the 48dp target.
    #[default]
    Small,
    /// `md.comp.icon-button.medium`.
    Medium,
    /// `md.comp.icon-button.large`.
    Large,
    /// `md.comp.icon-button.xlarge`.
    ExtraLarge,
}

impl IconButtonSize {
    /// The token set for this size.
    #[must_use]
    pub const fn tokens(self) -> IconButtonSizeTokens {
        match self {
            Self::ExtraSmall => IconButtonSizeTokens {
                // md.comp.icon-button.xsmall.container.size = 32
                container: ICON_BUTTON_TOUCH_TARGET_SIZE,
                state_layer: 32.0,
                icon: 20.0,
                outline_width: ICON_BUTTON_OUTLINE_WIDTH,
            },
            Self::Small => IconButtonSizeTokens {
                container: ICON_BUTTON_TOUCH_TARGET_SIZE,
                state_layer: ICON_BUTTON_STATE_LAYER_SIZE,
                icon: ICON_BUTTON_ICON_SIZE,
                outline_width: ICON_BUTTON_OUTLINE_WIDTH,
            },
            // md.comp.icon-button.medium. At this size the container *is* the
            // state layer — it is already well past the 48dp minimum.
            Self::Medium => IconButtonSizeTokens {
                container: 56.0,
                state_layer: 56.0,
                icon: 24.0,
                outline_width: 1.0,
            },
            // md.comp.icon-button.large.
            Self::Large => IconButtonSizeTokens {
                container: 96.0,
                state_layer: 96.0,
                icon: 32.0,
                outline_width: 2.0,
            },
            // md.comp.icon-button.xlarge.
            Self::ExtraLarge => IconButtonSizeTokens {
                container: 136.0,
                state_layer: 136.0,
                icon: 40.0,
                outline_width: 3.0,
            },
        }
    }
}

/// The look of a semantic `Button` whose label is `IconOnly`.
///
/// Every style except `Link` reserves the icon-button touch target — 48dp
/// at the small size — with no text padding, and the layout bounds are the
/// hit area; the container is the size's state-layer circle centred inside
/// them, as Compose draws it. Filled and tonal variants fill it, the
/// outlined variant strokes it, and the standard variant — like a link —
/// draws no container at all. `Automatic` resolves to the standard
/// variant, the Material default for icon buttons. A link keeps no
/// minimum box.
///
/// # Panics
///
/// Panics on a `ButtonStyle` or `ControlSize` variant this theme does not
/// implement.
#[must_use]
pub fn look(spec: &ButtonSpec, env: &Environment) -> ButtonLook {
    let style = spec.style();
    let tokens = icon_button_size(spec.size()).tokens();
    let shape = CornerShape::new(button::container_corners(pressed_corner_radius));
    let surface = match style {
        // md.comp.icon-button.filled.container.color = primary;
        // md.comp.icon-button.filled.disabled.container.opacity = 0.12.
        ButtonStyle::BorderedProminent => Surface::new(
            shape,
            button::disabled_paint(
                Primary,
                OnSurface.with_opacity(DISABLED_CONTAINER_OPACITY),
                env,
            ),
        ),
        // md.comp.icon-button.outlined.outline.color = outline-variant in
        // every state, disabled included.
        ButtonStyle::Bordered => {
            Surface::clear(shape).border(button::outline(OutlineVariant, outline_width, env))
        }
        ButtonStyle::Automatic
        | ButtonStyle::Plain
        | ButtonStyle::Borderless
        | ButtonStyle::Link => Surface::clear(shape),
        _ => panic!("hydrolysis ButtonStyle variant is not implemented"),
    };
    // md.comp.icon-button.filled.*.state-layer.color = on-primary;
    // standard and outlined both use on-surface-variant.
    let layer = match style {
        ButtonStyle::BorderedProminent => Color::from(OnPrimary),
        _ => Color::from(OnSurfaceVariant),
    };
    let side = f64::from(tokens.state_layer);
    let chrome = button::interactive(surface, layer).centered(Size::new(side, side));
    let look = ButtonLook::new(chrome, button::label(*spec));
    if style == ButtonStyle::Link {
        look
    } else {
        let container = f64::from(tokens.container);
        look.minimum_size(Size::new(container, container))
    }
}

/// `md.comp.icon-button.<size>.pressed.container.shape` by the state
/// layer's side: corner-small up to 40dp, corner-medium to 56,
/// corner-large beyond.
fn pressed_corner_radius(side: f64) -> f64 {
    if side <= f64::from(ICON_BUTTON_STATE_LAYER_SIZE) {
        8.0 // md.sys.shape.corner-small
    } else if side <= 56.0 {
        12.0 // md.sys.shape.corner-medium
    } else {
        16.0 // md.sys.shape.corner-large
    }
}

/// `md.comp.icon-button.<size>.outlined.outline-width`: 1dp through medium,
/// 2dp at large, 3dp at xlarge.
fn outline_width(side: f64) -> f64 {
    if side <= 56.0 {
        1.0
    } else if side <= 96.0 {
        2.0
    } else {
        3.0
    }
}

const fn icon_button_size(size: ControlSize) -> IconButtonSize {
    match size {
        ControlSize::ExtraSmall => IconButtonSize::ExtraSmall,
        ControlSize::Small => IconButtonSize::Small,
        ControlSize::Medium => IconButtonSize::Medium,
        ControlSize::Large => IconButtonSize::Large,
        ControlSize::ExtraLarge => IconButtonSize::ExtraLarge,
        _ => panic!("hydrolysis ControlSize variant is not implemented"),
    }
}

/// Color token set for an icon button variant.
pub trait IconButtonVariantTokens: Default + 'static {
    /// Container color.
    fn container_color() -> Color;

    /// Icon/content color.
    fn icon_color() -> Color;

    /// Border color.
    ///
    /// `OutlinedIconButtonTokens.OutlineColor` is `outlineVariant` — the
    /// softer variant role, same as the outlined `Button`.
    #[must_use]
    fn outline_color() -> Color {
        OutlineVariant.into()
    }

    /// Border width.
    #[must_use]
    fn outline_width() -> f32 {
        0.0
    }
}

/// Standard icon button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardIconButton;

/// Filled icon button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct FilledIconButton;

/// Filled tonal icon button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct FilledTonalIconButton;

/// Outlined icon button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct OutlinedIconButton;

impl IconButtonVariantTokens for StandardIconButton {
    fn container_color() -> Color {
        crate::color::Surface.with_opacity(0.0).into()
    }

    fn icon_color() -> Color {
        OnSurfaceVariant.into()
    }
}

impl IconButtonVariantTokens for FilledIconButton {
    fn container_color() -> Color {
        Primary.into()
    }

    fn icon_color() -> Color {
        OnPrimary.into()
    }
}

impl IconButtonVariantTokens for FilledTonalIconButton {
    fn container_color() -> Color {
        SecondaryContainer.into()
    }

    fn icon_color() -> Color {
        OnSecondaryContainer.into()
    }
}

impl IconButtonVariantTokens for OutlinedIconButton {
    fn container_color() -> Color {
        crate::color::Surface.with_opacity(0.0).into()
    }

    fn icon_color() -> Color {
        OnSurfaceVariant.into()
    }

    fn outline_width() -> f32 {
        ICON_BUTTON_OUTLINE_WIDTH
    }
}

/// Selected standard icon button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct SelectedStandardIconButton;

/// Selected outlined icon button tokens.
#[derive(Debug, Clone, Copy, Default)]
pub struct SelectedOutlinedIconButton;

impl IconButtonVariantTokens for SelectedStandardIconButton {
    fn container_color() -> Color {
        crate::color::Surface.with_opacity(0.0).into()
    }

    fn icon_color() -> Color {
        Primary.into()
    }
}

impl IconButtonVariantTokens for SelectedOutlinedIconButton {
    fn container_color() -> Color {
        InverseSurface.into()
    }

    fn icon_color() -> Color {
        InverseOnSurface.into()
    }
}

/// A Material Design 3 icon button.
pub struct IconButton<Content, Action = fn(&Environment), Tokens = StandardIconButton> {
    accessibility_label: Str,
    content: Content,
    action: Action,
    size: IconButtonSize,
    tokens: PhantomData<Tokens>,
}

impl<Content, Action, Tokens> Debug for IconButton<Content, Action, Tokens> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IconButton")
            .field("accessibility_label", &self.accessibility_label)
            .finish_non_exhaustive()
    }
}

impl<Content> IconButton<Content, fn(&Environment), StandardIconButton> {
    /// Creates a standard icon button with arbitrary visual icon content.
    #[must_use]
    pub fn new(accessibility_label: impl Into<Str>, content: Content) -> Self {
        Self {
            accessibility_label: accessibility_label.into(),
            content,
            action: noop,
            size: IconButtonSize::default(),
            tokens: PhantomData,
        }
    }
}

impl<Content, Action, Tokens> IconButton<Content, Action, Tokens> {
    /// Uses filled icon button tokens.
    #[must_use]
    pub fn filled(self) -> IconButton<Content, Action, FilledIconButton> {
        self.with_variant()
    }

    /// Uses filled tonal icon button tokens.
    #[must_use]
    pub fn filled_tonal(self) -> IconButton<Content, Action, FilledTonalIconButton> {
        self.with_variant()
    }

    /// Uses outlined icon button tokens.
    #[must_use]
    pub fn outlined(self) -> IconButton<Content, Action, OutlinedIconButton> {
        self.with_variant()
    }

    /// Uses selected standard icon button tokens.
    #[must_use]
    pub fn selected(self) -> IconButton<Content, Action, SelectedStandardIconButton> {
        self.with_variant()
    }

    /// Uses selected outlined icon button tokens.
    #[must_use]
    pub fn selected_outlined(self) -> IconButton<Content, Action, SelectedOutlinedIconButton> {
        self.with_variant()
    }

    fn with_variant<NewTokens>(self) -> IconButton<Content, Action, NewTokens> {
        IconButton {
            accessibility_label: self.accessibility_label,
            content: self.content,
            action: self.action,
            size: self.size,
            tokens: PhantomData,
        }
    }

    /// Sets how large the icon button is drawn.
    #[must_use]
    pub const fn size(mut self, size: IconButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the action performed when the icon button is tapped.
    #[must_use]
    pub fn action<F, Args>(self, action: F) -> IconButton<Content, impl FnMut(&Environment), Tokens>
    where
        F: Handler<Args, ()> + 'static,
    {
        IconButton {
            accessibility_label: self.accessibility_label,
            content: self.content,
            action: boxed_action(action),
            size: self.size,
            tokens: PhantomData,
        }
    }
}

impl<Content, Action, Tokens> View for IconButton<Content, Action, Tokens>
where
    Content: View + 'static,
    Action: FnMut(&Environment) + 'static,
    Tokens: IconButtonVariantTokens,
{
    fn body(self, _env: &Environment) -> impl View {
        let mut action = self.action;
        let size = self.size.tokens();

        self.content
            .foreground(Tokens::icon_color())
            .size(size.icon, size.icon)
            .padding_with((size.state_layer - size.icon) * 0.5)
            .size(size.state_layer, size.state_layer)
            .background(Circle.fill(Tokens::container_color()))
            .border_with(
                Border::new(Tokens::outline_color(), Tokens::outline_width())
                    .corner_radius(size.state_layer * 0.5),
            )
            .size(size.container, size.container)
            .on_tap(move |env: Environment| action(&env))
            .a11y_label(self.accessibility_label)
            .a11y_role(AccessibilityRole::Button)
            .a11y_children(AccessibilityChildren::ExcludeDescendants)
            // The state layer is the container circle centred in the touch
            // target.
            .install(
                interaction_chrome(Tokens::icon_color(), Corners::Full).centered(Size::new(
                    f64::from(size.state_layer),
                    f64::from(size.state_layer),
                )),
            )
    }
}

const fn noop(_env: &Environment) {}

/// Creates a standard icon button with arbitrary visual icon content.
#[must_use]
pub fn icon_button<Content>(
    accessibility_label: impl Into<Str>,
    content: Content,
) -> IconButton<Content>
where
    Content: View + 'static,
{
    IconButton::new(accessibility_label, content)
}

/// Creates a filled icon button with arbitrary visual icon content.
#[must_use]
pub fn filled_icon_button<Content>(
    accessibility_label: impl Into<Str>,
    content: Content,
) -> IconButton<Content, fn(&Environment), FilledIconButton>
where
    Content: View + 'static,
{
    icon_button(accessibility_label, content).filled()
}

/// Creates a filled tonal icon button with arbitrary visual icon content.
#[must_use]
pub fn filled_tonal_icon_button<Content>(
    accessibility_label: impl Into<Str>,
    content: Content,
) -> IconButton<Content, fn(&Environment), FilledTonalIconButton>
where
    Content: View + 'static,
{
    icon_button(accessibility_label, content).filled_tonal()
}

/// Creates an outlined icon button with arbitrary visual icon content.
#[must_use]
pub fn outlined_icon_button<Content>(
    accessibility_label: impl Into<Str>,
    content: Content,
) -> IconButton<Content, fn(&Environment), OutlinedIconButton>
where
    Content: View + 'static,
{
    icon_button(accessibility_label, content).outlined()
}

#[cfg(test)]
mod tests {
    use super::{
        ICON_BUTTON_ICON_SIZE, ICON_BUTTON_OUTLINE_WIDTH, ICON_BUTTON_STATE_LAYER_SIZE,
        ICON_BUTTON_TOUCH_TARGET_SIZE, IconButtonSize, IconButtonVariantTokens, OutlinedIconButton,
        look,
    };
    use hydrolysis::Style as _;
    use hydrolysis::theme::{ButtonSpec, Fill, Inputs};
    use waterui::interaction::InteractionState;
    use waterui_controls::ControlSize;
    use waterui_controls::button::ButtonStyle;
    use waterui_core::layout::LayoutDirection;
    use waterui_graphics::draw::kurbo::{Rect, Size, Vec2};

    fn icon_look(style: ButtonStyle, size: ControlSize) -> hydrolysis::theme::ButtonLook {
        let mut env = waterui::Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        look(
            &ButtonSpec::new(style, size, true, LayoutDirection::LeftToRight),
            &env,
        )
    }

    /// A semantic icon-only button lays out at the icon-button touch
    /// target — 48dp at the small size — with no text padding, and the
    /// layout bounds are the hit area; its container is the size's state
    /// layer centred inside. A link keeps no minimum box.
    #[test]
    fn icon_button_look_lays_out_at_the_touch_target() {
        let target = f64::from(ICON_BUTTON_TOUCH_TARGET_SIZE);
        let automatic = icon_look(ButtonStyle::Automatic, ControlSize::Small);
        assert_eq!(automatic.get_minimum_size(), Size::new(target, target));
        assert_eq!(automatic.get_padding(), Vec2::ZERO);
        assert_eq!(
            automatic
                .chrome()
                .frame(Rect::new(0.0, 0.0, target, target)),
            Rect::new(4.0, 4.0, 44.0, 44.0),
            "the 40dp state layer sits centred in the 48dp target"
        );

        let extra_small = icon_look(ButtonStyle::Automatic, ControlSize::ExtraSmall);
        assert_eq!(
            extra_small
                .chrome()
                .frame(Rect::new(0.0, 0.0, target, target))
                .size(),
            Size::new(32.0, 32.0),
            "the spec carries the size, so the extra-small layer is 32dp"
        );

        let link = icon_look(ButtonStyle::Link, ControlSize::Small);
        assert_eq!(link.get_minimum_size(), Size::ZERO);
    }

    /// `Automatic` is the standard icon button: no container fill, and the
    /// on-surface-variant label; `BorderedProminent` fills its container.
    #[test]
    fn automatic_icon_button_is_standard() {
        let colors = crate::MaterialColorScheme::baseline_light();
        let automatic = icon_look(ButtonStyle::Automatic, ControlSize::Small);
        let Fill::Paint(fill) = automatic.chrome().surface().fill() else {
            panic!("the icon button's surface is a paint")
        };
        let rest = Inputs::at_rest(
            Rect::new(0.0, 0.0, 40.0, 40.0),
            InteractionState::empty(),
            &(),
        );
        assert_eq!(
            fill.target(&rest),
            waterui_graphics::draw::Paint::from(waterui_graphics::draw::WorkingColor::TRANSPARENT,)
        );
        let mut env = waterui::Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        let label = automatic
            .label()
            .get_color()
            .expect("the icon button colours its label")
            .target(&rest);
        assert_eq!(
            waterui::Signal::snapshot(&label.resolve(&env)).components,
            colors.on_surface_variant.working().components
        );

        let prominent = icon_look(ButtonStyle::BorderedProminent, ControlSize::Small);
        let Fill::Paint(fill) = prominent.chrome().surface().fill() else {
            panic!("the icon button's surface is a paint")
        };
        assert_eq!(
            fill.target(&rest),
            waterui_graphics::draw::Paint::from(colors.primary.working())
        );
    }

    /// `md.comp.icon-button.{xsmall..xlarge}`. Past the small size the
    /// container has outgrown the 48dp minimum, so the state layer fills it
    /// rather than sitting inside a larger touch target.
    #[test]
    fn icon_button_size_scale_matches_compose_icon_button_tokens() {
        let extra_small = IconButtonSize::ExtraSmall.tokens();
        assert_eq!(extra_small.container, 48.0);
        assert_eq!(extra_small.state_layer, 32.0);
        assert_eq!(extra_small.icon, 20.0);
        assert_eq!(extra_small.outline_width, 1.0);

        let small = IconButtonSize::Small.tokens();
        assert_eq!(small.container, 48.0);
        assert_eq!(small.state_layer, 40.0);
        assert_eq!(small.icon, 24.0);

        let medium = IconButtonSize::Medium.tokens();
        assert_eq!(medium.container, 56.0);
        assert_eq!(medium.icon, 24.0);
        assert_eq!(medium.outline_width, 1.0);

        let large = IconButtonSize::Large.tokens();
        assert_eq!(large.container, 96.0);
        assert_eq!(large.icon, 32.0);
        assert_eq!(large.outline_width, 2.0);

        let extra_large = IconButtonSize::ExtraLarge.tokens();
        assert_eq!(extra_large.container, 136.0);
        assert_eq!(extra_large.icon, 40.0);
        assert_eq!(extra_large.outline_width, 3.0);

        // Every size clears the 48dp minimum touch target, and no state layer
        // ever spills outside the box that receives the taps.
        for size in [extra_small, small, medium, large, extra_large] {
            assert!(size.container >= 48.0);
            assert!(size.state_layer <= size.container);
            assert!(size.icon < size.state_layer);
        }
    }

    #[test]
    fn icon_button_tokens_match_compose_icon_button_tokens() {
        assert_eq!(ICON_BUTTON_STATE_LAYER_SIZE, 40.0);
        assert_eq!(ICON_BUTTON_ICON_SIZE, 24.0);
    }

    #[test]
    fn outlined_icon_button_tokens_match_compose_icon_button_tokens() {
        assert_eq!(
            OutlinedIconButton::outline_width(),
            ICON_BUTTON_OUTLINE_WIDTH
        );
    }

    /// `OutlinedIconButtonTokens.OutlineColor` is `outlineVariant` — the
    /// softer variant role — not `outline`.
    #[test]
    fn outlined_icon_button_border_uses_outline_variant() {
        use super::{IconButtonVariantTokens, SelectedOutlinedIconButton};
        use crate::{Material3, theme::colors::MaterialColorScheme};
        use hydrolysis::Style as _;
        use waterui::{Environment, Signal};
        use waterui_graphics::draw::WorkingColor;

        fn assert_resolves_to(actual: WorkingColor, expected: WorkingColor) {
            assert_eq!(
                actual.components.map(f32::to_bits),
                expected.components.map(f32::to_bits)
            );
        }

        let scheme = MaterialColorScheme::baseline_light();
        let mut env = Environment::new();
        Material3::with_colors(scheme).install_tokens(&mut env);

        for color in [
            OutlinedIconButton::outline_color(),
            SelectedOutlinedIconButton::outline_color(),
        ] {
            assert_resolves_to(
                color.resolve(&env).snapshot(),
                scheme.outline_variant.working(),
            );
        }
    }
}
