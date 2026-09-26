#![cfg_attr(
    test,
    allow(clippy::float_cmp, reason = "tests assert exact widget-metric values")
)]
//! Material Design 3 widget metrics and drawing primitives.
//!
//! This crate is a style package: [`Material3`] implements the backend-neutral
//! widget chrome contract from `waterui-backend-core` and the
//! [`hydrolysis::Style`] contract the rendered Hydrolysis runtime is
//! constructed with — `hydrolysis::run(app, Material3::defaults())`.

pub mod button_group;
pub mod chip;
pub mod color;
pub mod dialog;
pub mod drag_handle;
pub mod elevation;
pub mod fab;
pub mod fab_menu;
pub mod icon_button;
pub mod material_badge;
pub mod material_card;
pub mod material_divider;
pub mod material_list;
pub mod material_menu;
pub mod material_navigation;
pub mod material_snackbar;
pub mod material_tabs;
pub mod navigation_bar;
pub mod navigation_drawer;
pub mod navigation_rail;
pub mod segmented_button;
#[cfg(test)]
mod test_support;
pub mod toolbar;
pub mod tooltip;

#[cfg(test)]
mod layout_test_support;

mod controls;
mod icon_paths;
mod icons;
mod layout;
pub mod material_shapes;
mod navigation;
mod semantics;
pub mod split_button;
mod theme;

pub(crate) use controls::{button, input, picker, progress, slider, stepper, toggle};
pub(crate) use layout::{badge, card, divider, list, menu, scroll, snackbar, table};
pub(crate) use navigation::{navigation as navigation_chrome, tabs};
pub(crate) use theme::dimensions;

use cherenkov::{Paint, Recorder, WorkingColor};
use core::cell::OnceCell;

use cherenkov::kurbo::{BezPath, Point, Rect};
use waterui::Plugin as _;
use waterui::reactive::{Computed, Signal, SignalExt as _};
use waterui::text::font::Font;
use waterui::theme::{self as waterui_theme, ColorScheme, ColorSettings, Theme};
pub use waterui_backend_core::widget::{
    BadgeMetrics, ButtonMetrics, DividerMetrics, InputFieldMetrics, InteractionFocusBinding,
    InteractionMotion, InteractionStyle, ListDividerMetrics, ListMetrics, ListRowMetrics,
    ListSectionMetrics, ListTrailingControlMetrics, ModalInteraction, NavigationMetrics,
    NavigationMotion, PickerMetrics, PressWave, PressWaves, ProgressIndicatorStyle,
    ProgressMetrics, ProgressMotion, RadioIndicatorState, RadioSelectionMotion, SliderMetrics,
    StepperEnd, StepperMetrics, TableMetrics, TabsMetrics, TextCaretMotion, TextContextMenuMetrics,
    ToggleMetrics, WidgetInteractionState, WidgetTheme,
};
use waterui_controls::button::{ButtonSize, ButtonStyle};
use waterui_controls::toggle::ToggleStyle;
use waterui_core::Environment;
use waterui_form::picker::PickerStyle;
use waterui_graphics::color::Color;

pub use button_group::{
    ButtonGroup, ConnectedButton, ConnectedButtonGroup, GroupButton, button_group,
    connected_button, connected_button_group, group_button,
};
pub use chip::{
    AssistChip, FilterChip, InputChip, OutlinedChip, SuggestionChip, assist_chip, filter_chip,
    input_chip, suggestion_chip,
};
pub use dialog::{Dialog, DialogAction, dialog, dialog_action};
pub use drag_handle::{VerticalDragHandle, vertical_drag_handle};
pub use elevation::{MaterialElevation, MaterialElevationLevel, material_elevation};
pub use fab::{
    ExtendedFab, Fab, FabSize, FabSizeTokens, FabVariantTokens, PrimaryFab, SecondaryFab,
    SurfaceFab, TertiaryFab, extended_fab, fab,
};
pub use fab_menu::{FabMenuItem, FloatingActionButtonMenu, fab_menu, fab_menu_item};
pub use icon_button::{
    FilledIconButton, FilledTonalIconButton, IconButton, IconButtonSize, IconButtonSizeTokens,
    IconButtonVariantTokens, OutlinedIconButton, SelectedOutlinedIconButton,
    SelectedStandardIconButton, StandardIconButton, filled_icon_button, filled_tonal_icon_button,
    icon_button, outlined_icon_button,
};
pub use material_badge::{MaterialBadge, material_badge};
pub use material_card::{MaterialCard, material_card};
pub use material_color_utils::dynamic::{
    color_spec::{Platform as MaterialColorPlatform, SpecVersion as MaterialColorSpecVersion},
    variant::Variant as MaterialColorVariant,
};
/// The ARGB seed color the Material You generators take (`Argb(0xAARRGGBB)`),
/// re-exported so `Material3::with_seed` / `MaterialColorSource::new` are callable
/// without naming the color-utils crate directly.
pub use material_color_utils::utils::color_utils::Argb;
pub use material_divider::{MaterialDivider, material_divider};
pub use material_list::{MaterialList, MaterialListItem, material_list, material_list_item};
pub use material_menu::{
    MaterialMenu, MaterialMenuCommand, MaterialMenuItem, MaterialMenuItemBuilder, material_menu,
    material_menu_divider, material_menu_icon_label, material_menu_item, material_sub_menu,
};
pub use material_navigation::{
    MaterialNavigationView, material_large_top_app_bar, material_navigation_view,
    material_small_top_app_bar,
};
pub use material_snackbar::{MaterialSnackbar, material_snackbar};
pub use material_tabs::{MaterialTab, MaterialTabs, material_tab, material_tabs};
pub use navigation_bar::{NavigationBar, NavigationTab, navigation_bar, navigation_tab};
pub use navigation_drawer::{
    NavigationDrawer, NavigationDrawerItem, navigation_drawer, navigation_drawer_item,
};
pub use navigation_rail::{
    NavigationRail, NavigationRailItem, NavigationRailLayout, navigation_rail, navigation_rail_item,
};
pub use segmented_button::{
    OutlinedSegmentedButton, OutlinedSegmentedButtonSet, SegmentedButtonShape,
    outlined_segmented_button, outlined_segmented_button_set,
};
pub use split_button::{
    FilledSplitButton, SplitButton, SplitButtonVariantTokens, TonalSplitButton, split_button,
};
pub use theme::colors::{
    MaterialColorMode, MaterialColorScheme, MaterialColorSchemes, MaterialColorSource,
    MaterialContrastLevel, MaterialRoleColor,
};
/// The Material Design 3 motion token scale — the duration and easing tokens
/// this theme's component motion is built from, re-exported so app and backend
/// code can animate on the same scale instead of re-deriving the numbers.
pub use theme::motion::tokens as motion_tokens;
pub use toolbar::{
    DockedToolbar, FloatingToolbar, FloatingToolbarStyle, docked_toolbar, floating_toolbar,
};
pub use tooltip::{PlainTooltip, RichTooltip, TooltipAnchor, plain_tooltip, rich_tooltip};

#[derive(Debug)]
/// Material Design 3 style for the Hydrolysis runtime.
///
/// `Material3` is both the [`WidgetTheme`] the backend resolves widget chrome
/// through and the [`hydrolysis::Style`] whose `install_tokens` writes the
/// Material color and typography tokens into the environment:
/// `hydrolysis::run(app, Material3::defaults())`.
///
/// A dynamic `Material3` ([`Material3::defaults`]) binds to the color-scheme
/// signal of the first environment `install_tokens` installs it into and
/// follows that signal for the rest of its lifetime; every later environment
/// the same instance is installed into is rebound to that signal, so paint
/// and tokens can never diverge. Cloning produces a fresh, unbound style: the
/// clone binds to the first environment *it* is installed into, never to the
/// original's signal.
pub struct Material3 {
    colors: Material3Colors,
}

impl Clone for Material3 {
    fn clone(&self) -> Self {
        Self {
            colors: self.colors.clone(),
        }
    }
}

#[derive(Debug)]
#[allow(
    clippy::large_enum_variant,
    reason = "a `Material3` holds exactly one of these; the variant size difference is immaterial and boxing would add needless indirection"
)]
enum Material3Colors {
    /// One fixed color scheme, resolved at construction.
    Static(MaterialColorScheme),
    /// A light/dark pair that follows the environment's color-scheme signal;
    /// the signal binds on the first `install_tokens` and stays bound.
    Dynamic {
        light: MaterialColorScheme,
        dark: MaterialColorScheme,
        scheme: OnceCell<Computed<ColorScheme>>,
    },
}

impl Clone for Material3Colors {
    fn clone(&self) -> Self {
        match self {
            Self::Static(colors) => Self::Static(*colors),
            Self::Dynamic { light, dark, .. } => Self::Dynamic {
                light: *light,
                dark: *dark,
                scheme: OnceCell::new(),
            },
        }
    }
}

impl Material3 {
    /// The default Material Design 3 style: baseline light/dark color roles
    /// that follow the app's color-scheme signal.
    #[must_use]
    pub const fn defaults() -> Self {
        Self {
            colors: Material3Colors::Dynamic {
                light: MaterialColorScheme::baseline_light(),
                dark: MaterialColorScheme::baseline_dark(),
                scheme: OnceCell::new(),
            },
        }
    }

    /// The dark Material Design 3 baseline color scheme.
    #[must_use]
    pub const fn dark() -> Self {
        Self::with_colors(MaterialColorScheme::baseline_dark())
    }

    /// A Material Design 3 style from explicit Material color roles.
    #[must_use]
    pub const fn with_colors(colors: MaterialColorScheme) -> Self {
        Self {
            colors: Material3Colors::Static(colors),
        }
    }

    /// A Material Design 3 style generated from a Material You seed color.
    #[must_use]
    pub fn with_seed(seed: Argb) -> Self {
        Self::with_seed_mode(seed, MaterialColorMode::Light)
    }

    /// A Material Design 3 style generated from a seed color and mode.
    #[must_use]
    pub fn with_seed_mode(seed: Argb, mode: MaterialColorMode) -> Self {
        Self::with_source(MaterialColorSource::new(seed), mode)
    }

    /// A Material Design 3 style generated from a complete Material You source.
    #[must_use]
    pub fn with_source(source: MaterialColorSource, mode: MaterialColorMode) -> Self {
        Self::with_colors(source.scheme(mode))
    }

    /// A Material Design 3 style picking one scheme from a paired
    /// light/dark Material You scheme set.
    #[must_use]
    pub const fn with_color_schemes(
        schemes: &MaterialColorSchemes,
        mode: MaterialColorMode,
    ) -> Self {
        Self::with_colors(schemes.scheme(mode))
    }

    /// Return the Material color roles this style resolves to.
    ///
    /// For the dynamic constructors the roles follow the color-scheme signal
    /// bound during `install_tokens`; a `Material3` that was never installed
    /// has no signal to read and this method panics.
    #[must_use]
    pub fn colors(&self) -> MaterialColorScheme {
        self.colors.current()
    }
}

impl Material3Colors {
    fn current(&self) -> MaterialColorScheme {
        match self {
            Self::Static(colors) => *colors,
            Self::Dynamic {
                light,
                dark,
                scheme,
            } => material_scheme_for_color_scheme(
                *light,
                *dark,
                scheme
                    .get()
                    .expect("a `Material3` reads its color scheme only after `install_tokens`")
                    .snapshot(),
            ),
        }
    }
}

impl hydrolysis::Style for Material3 {
    /// Writes the Material color and typography tokens into `env`, over the
    /// framework defaults the runtime installs first.
    ///
    /// A dynamic scheme binds on first install: it takes `env`'s color-scheme
    /// signal if one is installed, else binds a constant [`ColorScheme::Light`]
    /// signal. On every install the bound signal is written into `env`, so an
    /// environment carrying a different signal is rebound to it and the tokens
    /// always match what the style paints.
    fn install_tokens(&self, env: &mut Environment) {
        match &self.colors {
            Material3Colors::Static(colors) => insert_static_tokens(env, *colors),
            Material3Colors::Dynamic {
                light,
                dark,
                scheme,
            } => {
                let scheme = scheme.get_or_init(|| {
                    waterui_theme::installed_color_scheme(env)
                        .unwrap_or_else(|| Computed::constant(ColorScheme::Light))
                });
                Theme::new().color_scheme(scheme.clone()).install(env);
                insert_dynamic_tokens(env, scheme, *light, *dark);
            }
        }
    }
}

/// Installs the `WaterUI` theme tokens and Material component themes for one
/// fixed color scheme.
fn insert_static_tokens(env: &mut Environment, colors: MaterialColorScheme) {
    let color_scheme = match colors.mode {
        MaterialColorMode::Light => ColorScheme::Light,
        MaterialColorMode::Dark => ColorScheme::Dark,
    };
    Theme::new()
        .color_scheme(color_scheme)
        .colors(
            ColorSettings::new()
                .background(colors.background.working())
                .surface(colors.surface.working())
                .surface_variant(colors.surface_variant.working())
                .border(colors.outline.working())
                .foreground(colors.on_surface.working())
                .muted_foreground(colors.on_surface_variant.working())
                .accent(colors.primary.working())
                .accent_foreground(colors.on_primary.working())
                .accent_container(colors.primary_container.working())
                .tertiary(colors.tertiary.working())
                .tertiary_container(colors.tertiary_container.working())
                .selection_container(colors.secondary_container.working())
                .selection_foreground(colors.on_secondary_container.working())
                .error(colors.error.working())
                .error_foreground(colors.on_error.working()),
        )
        .install(env);
    theme::typography::defaults(env);
    env.insert(colors);
    env.insert(card::theme(&colors));
    env.insert(fab::theme());
    env.insert(snackbar::theme(&colors));
}

/// Installs the paired light/dark schemes bound to `scheme`, the
/// environment's color-scheme signal.
fn insert_dynamic_tokens(
    env: &mut Environment,
    scheme: &Computed<ColorScheme>,
    light: MaterialColorScheme,
    dark: MaterialColorScheme,
) {
    project_color_tokens(env, scheme, light, dark);
    theme::typography::defaults(env);
    let initial = material_scheme_for_color_scheme(light, dark, scheme.snapshot());
    env.insert(initial);
    env.insert(MaterialColorSchemes::new(
        MaterialColorSource::default(),
        light,
        dark,
    ));
    env.insert(card::theme(&initial));
    env.insert(fab::theme());
    env.insert(snackbar::theme(&initial));
}

/// Projects each `WaterUI` color token onto the Material role that carries it,
/// as a signal that re-resolves when the color scheme flips.
fn project_color_tokens(
    env: &mut Environment,
    scheme: &Computed<ColorScheme>,
    light: MaterialColorScheme,
    dark: MaterialColorScheme,
) {
    macro_rules! project {
        ($($token:ident => $role:ident),* $(,)?) => {
            $(
                project_color_token::<waterui_theme::color::$token>(
                    env,
                    scheme,
                    light,
                    dark,
                    |colors| colors.$role,
                );
            )*
        };
    }

    project! {
        Background => background,
        Surface => surface,
        SurfaceVariant => surface_variant,
        Border => outline,
        Foreground => on_surface,
        MutedForeground => on_surface_variant,
        Accent => primary,
        AccentForeground => on_primary,
        AccentContainer => primary_container,
        Tertiary => tertiary,
        TertiaryContainer => tertiary_container,
        SelectionContainer => secondary_container,
        SelectionForeground => on_secondary_container,
        Error => error,
        ErrorForeground => on_error,
    }
}

fn project_color_token<T: 'static>(
    env: &mut Environment,
    scheme: &Computed<ColorScheme>,
    light: MaterialColorScheme,
    dark: MaterialColorScheme,
    role: fn(MaterialColorScheme) -> MaterialRoleColor,
) {
    let signal = scheme
        .map(move |scheme| role(material_scheme_for_color_scheme(light, dark, scheme)).working())
        .computed();
    waterui_theme::install_color_signal::<T>(env, signal);
}

const fn material_scheme_for_color_scheme(
    light: MaterialColorScheme,
    dark: MaterialColorScheme,
    scheme: ColorScheme,
) -> MaterialColorScheme {
    match scheme {
        ColorScheme::Light => light,
        ColorScheme::Dark => dark,
    }
}

impl Default for Material3 {
    fn default() -> Self {
        Self::defaults()
    }
}

impl WidgetTheme for Material3 {
    fn interaction_motion(&self) -> InteractionMotion {
        theme::motion::interaction()
    }

    fn progress_motion(&self) -> ProgressMotion {
        theme::motion::progress()
    }

    fn text_caret_motion(&self) -> TextCaretMotion {
        theme::motion::text_caret()
    }

    fn navigation_motion(&self) -> NavigationMotion {
        theme::motion::navigation()
    }

    fn button_metrics(&self, style: ButtonStyle, size: ButtonSize) -> ButtonMetrics {
        button::metrics(style, size)
    }

    fn icon_button_metrics(&self, style: ButtonStyle, size: ButtonSize) -> ButtonMetrics {
        icon_button::metrics(style, size)
    }

    fn button_label_color(&self, style: ButtonStyle, disabled: bool) -> Option<Color> {
        Some(button::label_color(&self.colors(), style, disabled))
    }

    fn button_label_font(&self, _style: ButtonStyle) -> Option<Font> {
        Some(theme::typography::label_large())
    }

    fn draw_button_chrome(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        style: ButtonStyle,
        icon_only: bool,
        state: WidgetInteractionState,
    ) {
        if icon_only {
            icon_button::draw_chrome(&self.colors(), draw, bounds, style, state);
        } else {
            button::draw_chrome(&self.colors(), draw, bounds, style, state);
        }
    }

    fn draw_button_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        style: ButtonStyle,
        icon_only: bool,
        state: WidgetInteractionState,
    ) {
        if icon_only {
            icon_button::draw_state_layer(&self.colors(), draw, bounds, style, state);
        } else {
            button::draw_state_layer(&self.colors(), draw, bounds, style, state);
        }
    }

    fn draw_interaction_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        radii: cherenkov::kurbo::RoundedRectRadii,
        color: WorkingColor,
        state: WidgetInteractionState,
    ) {
        theme::state_layer::draw_bounded(draw, bounds, radii, color, state);
    }

    fn toggle_metrics(&self, style: ToggleStyle) -> ToggleMetrics {
        toggle::metrics(style)
    }

    fn toggle_value_animation(&self) -> waterui::animation::Animation {
        theme::motion::toggle_value()
    }

    fn draw_toggle_switch(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        progress: f32,
        selected: bool,
        state: WidgetInteractionState,
    ) {
        toggle::draw_switch(&self.colors(), draw, bounds, progress, selected, state);
    }

    fn draw_toggle_switch_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        progress: f32,
        selected: bool,
        state: WidgetInteractionState,
    ) {
        toggle::draw_switch_state_layer(&self.colors(), draw, bounds, progress, selected, state);
    }

    fn draw_toggle_checkbox(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        progress: f32,
        state: WidgetInteractionState,
    ) {
        toggle::draw_checkbox(&self.colors(), draw, bounds, progress, state);
    }

    fn draw_toggle_checkbox_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        progress: f32,
        state: WidgetInteractionState,
    ) {
        toggle::draw_checkbox_state_layer(&self.colors(), draw, bounds, progress, state);
    }

    fn stepper_metrics(&self) -> StepperMetrics {
        stepper::metrics()
    }

    fn draw_stepper_button(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        end: StepperEnd,
        state: WidgetInteractionState,
    ) {
        stepper::draw_button(&self.colors(), draw, bounds, end, state);
    }

    fn draw_stepper_decrement_icon(&self, draw: &mut Recorder, bounds: Rect) {
        stepper::draw_decrement_icon(&self.colors(), draw, bounds);
    }

    fn draw_stepper_increment_icon(&self, draw: &mut Recorder, bounds: Rect) {
        stepper::draw_increment_icon(&self.colors(), draw, bounds);
    }

    fn draw_stepper_button_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        end: StepperEnd,
        state: WidgetInteractionState,
    ) {
        stepper::draw_button_state_layer(&self.colors(), draw, bounds, end, state);
    }

    fn input_field_metrics(&self) -> InputFieldMetrics {
        input::metrics()
    }

    fn input_placeholder_color(&self) -> Color {
        input::placeholder_color(&self.colors())
    }

    fn input_selection_brush(&self) -> Paint {
        input::selection_brush(&self.colors())
    }

    fn input_caret_brush(&self, opacity: f32) -> Paint {
        input::caret_brush(&self.colors(), opacity)
    }

    fn draw_input_field(&self, draw: &mut Recorder, bounds: Rect, state: WidgetInteractionState) {
        input::draw_field(&self.colors(), draw, bounds, state);
    }

    fn draw_input_field_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        state: WidgetInteractionState,
    ) {
        input::draw_state_layer(&self.colors(), draw, bounds, state);
    }

    fn text_context_menu_metrics(&self) -> TextContextMenuMetrics {
        menu::text_context_metrics()
    }

    fn draw_text_context_menu_panel(&self, draw: &mut Recorder, bounds: Rect) {
        menu::draw_text_context_panel(&self.colors(), draw, bounds);
    }

    fn draw_text_context_menu_separator(&self, draw: &mut Recorder, bounds: Rect) {
        menu::draw_text_context_separator(&self.colors(), draw, bounds);
    }

    fn picker_metrics(&self, style: PickerStyle) -> PickerMetrics {
        picker::metrics(style)
    }

    fn radio_selection_motion(&self) -> RadioSelectionMotion {
        theme::motion::radio_selection()
    }

    fn draw_picker_indicator(&self, draw: &mut Recorder, bounds: Rect) {
        picker::draw_indicator(&self.colors(), draw, bounds);
    }

    fn draw_picker_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        state: WidgetInteractionState,
    ) {
        picker::draw_state_layer(&self.colors(), draw, bounds, state);
    }

    fn draw_picker_popup(&self, draw: &mut Recorder, popup_rect: Rect) {
        picker::draw_popup(&self.colors(), draw, popup_rect);
    }

    fn draw_picker_popup_row_background(
        &self,
        draw: &mut Recorder,
        row_rect: Rect,
        selected: bool,
    ) {
        picker::draw_popup_row_background(&self.colors(), draw, row_rect, selected);
    }

    fn draw_picker_popup_row_state_layer(
        &self,
        draw: &mut Recorder,
        row_rect: Rect,
        selected: bool,
        state: WidgetInteractionState,
    ) {
        picker::draw_popup_row_state_layer(&self.colors(), draw, row_rect, selected, state);
    }

    fn draw_picker_separator(&self, draw: &mut Recorder, separator: Rect) {
        picker::draw_separator(&self.colors(), draw, separator);
    }

    fn draw_radio_indicator(
        &self,
        draw: &mut Recorder,
        center: Point,
        radius: f64,
        state: RadioIndicatorState,
    ) {
        picker::draw_radio_indicator(&self.colors(), draw, center, radius, state);
    }

    fn draw_radio_state_layer(
        &self,
        draw: &mut Recorder,
        center: Point,
        radius: f64,
        selected: bool,
        state: WidgetInteractionState,
    ) {
        picker::draw_radio_state_layer(&self.colors(), draw, center, radius, selected, state);
    }

    fn segmented_picker_label_color(&self, selected: bool) -> Option<Color> {
        Some(picker::segmented_label_color(&self.colors(), selected))
    }

    fn draw_segmented_picker_container(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        segment_count: usize,
    ) {
        picker::draw_segmented_container(&self.colors(), draw, bounds, segment_count);
    }

    fn draw_segmented_picker_segment(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        selected: bool,
        is_first: bool,
        is_last: bool,
    ) {
        picker::draw_segmented_segment(&self.colors(), draw, bounds, selected, is_first, is_last);
    }

    fn draw_segmented_picker_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        selected: bool,
        is_first: bool,
        is_last: bool,
        state: WidgetInteractionState,
    ) {
        picker::draw_segmented_state_layer(
            &self.colors(),
            draw,
            bounds,
            selected,
            is_first,
            is_last,
            state,
        );
    }

    fn slider_metrics(&self) -> SliderMetrics {
        slider::metrics()
    }

    fn draw_slider_track(
        &self,
        draw: &mut Recorder,
        track_rect: Rect,
        fill_rect: Rect,
        state: WidgetInteractionState,
    ) {
        slider::draw_track(&self.colors(), draw, track_rect, fill_rect, state);
    }

    fn draw_slider_thumb(
        &self,
        draw: &mut Recorder,
        center: Point,
        radius: f64,
        state: WidgetInteractionState,
    ) {
        slider::draw_thumb(&self.colors(), draw, center, radius, state);
    }

    fn draw_slider_thumb_state_layer(
        &self,
        draw: &mut Recorder,
        center: Point,
        radius: f64,
        state: WidgetInteractionState,
    ) {
        slider::draw_thumb_state_layer(&self.colors(), draw, center, radius, state);
    }

    fn progress_metrics(&self, style: ProgressIndicatorStyle) -> ProgressMetrics {
        progress::metrics(style)
    }

    fn draw_progress_linear_track(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        active_end: Option<f64>,
    ) {
        progress::draw_linear_track(&self.colors(), draw, bounds, active_end);
    }

    fn draw_progress_linear_fill(&self, draw: &mut Recorder, bounds: Rect) {
        progress::draw_linear_fill(&self.colors(), draw, bounds);
    }

    fn draw_progress_linear_indeterminate(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        elapsed: core::time::Duration,
        four_color: bool,
    ) {
        progress::draw_linear_indeterminate(&self.colors(), draw, bounds, elapsed, four_color);
    }

    fn draw_progress_circular_track(
        &self,
        draw: &mut Recorder,
        center: Point,
        radius: f64,
        width: f64,
        active_turns: Option<f64>,
    ) {
        progress::draw_circular_track(&self.colors(), draw, center, radius, width, active_turns);
    }

    fn draw_progress_circular_fill(&self, draw: &mut Recorder, path: &BezPath, width: f64) {
        progress::draw_circular_fill(&self.colors(), draw, path, width);
    }

    fn draw_progress_loading(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        elapsed: core::time::Duration,
        four_color: bool,
    ) {
        progress::draw_loading(&self.colors(), draw, bounds, elapsed, four_color);
    }

    fn draw_progress_circular_indeterminate(
        &self,
        draw: &mut Recorder,
        center: Point,
        radius: f64,
        width: f64,
        elapsed: core::time::Duration,
        four_color: bool,
    ) {
        progress::draw_circular_indeterminate(
            &self.colors(),
            draw,
            center,
            radius,
            width,
            elapsed,
            four_color,
        );
    }

    fn navigation_metrics(&self) -> NavigationMetrics {
        navigation_chrome::metrics()
    }

    fn draw_navigation_bar(&self, draw: &mut Recorder, bounds: Rect, background: &Paint) {
        navigation_chrome::draw_bar(draw, bounds, background);
    }

    fn draw_navigation_bar_separator(&self, draw: &mut Recorder, bounds: Rect) {
        navigation_chrome::draw_bar_separator(&self.colors(), draw, bounds);
    }

    fn draw_navigation_back_button(&self, draw: &mut Recorder, bounds: Rect) {
        navigation_chrome::draw_back_button(&self.colors(), draw, bounds);
    }

    fn tabs_metrics(&self) -> TabsMetrics {
        tabs::metrics()
    }

    fn draw_tabs_bar(&self, draw: &mut Recorder, bounds: Rect, top_edge: bool) {
        tabs::draw_bar(&self.colors(), draw, bounds, top_edge);
    }

    fn draw_tabs_highlight(&self, draw: &mut Recorder, bounds: Rect) {
        tabs::draw_highlight(&self.colors(), draw, bounds);
    }

    fn draw_tabs_button_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        selected: bool,
        state: WidgetInteractionState,
    ) {
        tabs::draw_button_state_layer(&self.colors(), draw, bounds, selected, state);
    }

    fn draw_scroll_indicator(&self, draw: &mut Recorder, bounds: Rect) {
        scroll::draw_indicator(&self.colors(), draw, bounds);
    }

    fn divider_metrics(&self) -> DividerMetrics {
        divider::metrics()
    }

    fn draw_divider(&self, draw: &mut Recorder, bounds: Rect) {
        divider::draw(&self.colors(), draw, bounds);
    }

    fn badge_metrics(&self) -> BadgeMetrics {
        badge::metrics()
    }

    fn badge_label_color(&self) -> Color {
        badge::label_color(&self.colors())
    }

    fn badge_label_font(&self) -> Font {
        theme::typography::label_small()
    }

    fn draw_badge_small(&self, draw: &mut Recorder, bounds: Rect) {
        badge::draw_small(&self.colors(), draw, bounds);
    }

    fn draw_badge_large(&self, draw: &mut Recorder, bounds: Rect) {
        badge::draw_large(&self.colors(), draw, bounds);
    }

    fn list_metrics(&self) -> ListMetrics {
        list::metrics()
    }

    fn draw_list_row_background(&self, draw: &mut Recorder, bounds: Rect, alternate: bool) {
        list::draw_row_background(&self.colors(), draw, bounds, alternate);
    }

    fn draw_list_move_control(&self, draw: &mut Recorder, bounds: Rect) {
        list::draw_move_control(&self.colors(), draw, bounds);
    }

    fn draw_list_move_control_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        state: WidgetInteractionState,
    ) {
        list::draw_move_control_state_layer(&self.colors(), draw, bounds, state);
    }

    fn draw_list_delete_control(&self, draw: &mut Recorder, bounds: Rect) {
        list::draw_delete_control(&self.colors(), draw, bounds);
    }

    fn draw_list_delete_control_state_layer(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        state: WidgetInteractionState,
    ) {
        list::draw_delete_control_state_layer(&self.colors(), draw, bounds, state);
    }

    fn draw_list_swipe_dismiss_background(
        &self,
        draw: &mut Recorder,
        bounds: Rect,
        progress: f64,
        toward_start: bool,
    ) {
        list::draw_swipe_dismiss_background(&self.colors(), draw, bounds, progress, toward_start);
    }

    fn draw_list_row_lifted(&self, draw: &mut Recorder, bounds: Rect, elevation: f64) {
        list::draw_row_lifted(&self.colors(), draw, bounds, elevation);
    }

    fn draw_list_separator(&self, draw: &mut Recorder, bounds: Rect) {
        list::draw_separator(&self.colors(), draw, bounds);
    }

    fn table_metrics(&self) -> TableMetrics {
        table::metrics()
    }

    fn draw_table_background(&self, draw: &mut Recorder, bounds: Rect) {
        table::draw_background(&self.colors(), draw, bounds);
    }

    fn draw_table_header_background(&self, draw: &mut Recorder, bounds: Rect) {
        table::draw_header_background(&self.colors(), draw, bounds);
    }

    fn draw_table_cell_border(&self, draw: &mut Recorder, bounds: Rect) {
        table::draw_cell_border(&self.colors(), draw, bounds);
    }

    fn draw_table_column_separator(&self, draw: &mut Recorder, from: Point, to: Point) {
        table::draw_column_separator(&self.colors(), draw, from, to);
    }
}

fn lerp_channel(start: f32, end: f32, t: f32) -> f32 {
    (end - start).mul_add(t, start)
}

fn lerp_color(start: WorkingColor, end: WorkingColor, t: f32) -> WorkingColor {
    let t = t.clamp(0.0, 1.0);
    let [r0, g0, b0, a0] = start.components;
    let [r1, g1, b1, a1] = end.components;
    WorkingColor::new([
        lerp_channel(r0, r1, t),
        lerp_channel(g0, g1, t),
        lerp_channel(b0, b1, t),
        lerp_channel(a0, a1, t),
    ])
}

fn lerp_f64(start: f64, end: f64, t: f32) -> f64 {
    (end - start).mul_add(f64::from(t.clamp(0.0, 1.0)), start)
}
