//! Material Design 3 color role tokens.
//!
//! These tokens resolve against the `MaterialColorScheme` installed by a
//! `hydrolysis_m3::Material3` style and can be used anywhere a `WaterUI`
//! color is accepted.

use cherenkov::WorkingColor;
use waterui::View;
use waterui::color::{Color, WithOpacity};
use waterui::reactive::{Computed, Signal, SignalExt as _, impl_constant};
use waterui::theme::{ColorScheme, current_color_scheme};
use waterui_core::{Environment, resolve::Resolvable};

use crate::theme::colors::{MaterialColorScheme, MaterialColorSchemes, MaterialRoleColor};

fn resolve_role(
    env: &Environment,
    token: &'static str,
    role: fn(&MaterialColorScheme) -> MaterialRoleColor,
) -> Computed<WorkingColor> {
    if let Some(schemes) = env.get::<MaterialColorSchemes>() {
        let schemes = schemes.clone();
        return current_color_scheme(env)
            .map(move |mode| role(&material_scheme_for_color_scheme(&schemes, mode)).working())
            .computed();
    }

    let scheme = env.get::<MaterialColorScheme>().unwrap_or_else(|| {
        panic!(
            "hydrolysis_m3::color::{token} requires a hydrolysis_m3::Material3 style's tokens in the environment"
        )
    });
    Computed::constant(role(scheme).working())
}

const fn material_scheme_for_color_scheme(
    schemes: &MaterialColorSchemes,
    mode: ColorScheme,
) -> MaterialColorScheme {
    match mode {
        ColorScheme::Light => schemes.light(),
        ColorScheme::Dark => schemes.dark(),
    }
}

macro_rules! define_material_color_tokens {
    ($(
        $(#[$meta:meta])*
        $name:ident => $field:ident;
    )+) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, Default)]
            pub struct $name;

            impl $name {
                /// Creates this Material color role with the specified opacity applied.
                #[must_use]
                pub const fn with_opacity(self, opacity: f32) -> WithOpacity<Self> {
                    WithOpacity::new(self, opacity)
                }
            }

            impl Resolvable for $name {
                type Resolved = WorkingColor;

                fn resolve(&self, env: &Environment) -> impl Signal<Output = Self::Resolved> {
                    resolve_role(env, stringify!($name), |scheme| scheme.$field)
                }
            }

            impl_constant!($name);

            impl View for $name {
                fn body(self, _env: &Environment) -> impl View {
                    Color::new(self)
                }
            }
        )+
    };
}

define_material_color_tokens! {
    /// Material `background` role.
    Background => background;
    /// Material `on-background` role.
    OnBackground => on_background;
    /// Material `surface` role.
    Surface => surface;
    /// Material `surface-dim` role.
    SurfaceDim => surface_dim;
    /// Material `surface-bright` role.
    SurfaceBright => surface_bright;
    /// Material `surface-container-lowest` role.
    SurfaceContainerLowest => surface_container_lowest;
    /// Material `surface-container-low` role.
    SurfaceContainerLow => surface_container_low;
    /// Material `surface-container` role.
    SurfaceContainer => surface_container;
    /// Material `surface-container-high` role.
    SurfaceContainerHigh => surface_container_high;
    /// Material `surface-container-highest` role.
    SurfaceContainerHighest => surface_container_highest;
    /// Material `on-surface` role.
    OnSurface => on_surface;
    /// Material `surface-variant` role.
    SurfaceVariant => surface_variant;
    /// Material `on-surface-variant` role.
    OnSurfaceVariant => on_surface_variant;
    /// Material `inverse-surface` role.
    InverseSurface => inverse_surface;
    /// Material `inverse-on-surface` role.
    InverseOnSurface => inverse_on_surface;
    /// Material `outline` role.
    Outline => outline;
    /// Material `outline-variant` role.
    OutlineVariant => outline_variant;
    /// Material `shadow` role.
    Shadow => shadow;
    /// Material `scrim` role.
    Scrim => scrim;
    /// Material `surface-tint` role.
    SurfaceTint => surface_tint;
    /// Material `primary` role.
    Primary => primary;
    /// Material `on-primary` role.
    OnPrimary => on_primary;
    /// Material `primary-container` role.
    PrimaryContainer => primary_container;
    /// Material `on-primary-container` role.
    OnPrimaryContainer => on_primary_container;
    /// Material `inverse-primary` role.
    InversePrimary => inverse_primary;
    /// Material `secondary` role.
    Secondary => secondary;
    /// Material `on-secondary` role.
    OnSecondary => on_secondary;
    /// Material `secondary-container` role.
    SecondaryContainer => secondary_container;
    /// Material `on-secondary-container` role.
    OnSecondaryContainer => on_secondary_container;
    /// Material `tertiary` role.
    Tertiary => tertiary;
    /// Material `on-tertiary` role.
    OnTertiary => on_tertiary;
    /// Material `tertiary-container` role.
    TertiaryContainer => tertiary_container;
    /// Material `on-tertiary-container` role.
    OnTertiaryContainer => on_tertiary_container;
    /// Material `error` role.
    Error => error;
    /// Material `on-error` role.
    OnError => on_error;
    /// Material `error-container` role.
    ErrorContainer => error_container;
    /// Material `on-error-container` role.
    OnErrorContainer => on_error_container;
    /// Material `primary-fixed` role.
    PrimaryFixed => primary_fixed;
    /// Material `primary-fixed-dim` role.
    PrimaryFixedDim => primary_fixed_dim;
    /// Material `on-primary-fixed` role.
    OnPrimaryFixed => on_primary_fixed;
    /// Material `on-primary-fixed-variant` role.
    OnPrimaryFixedVariant => on_primary_fixed_variant;
    /// Material `secondary-fixed` role.
    SecondaryFixed => secondary_fixed;
    /// Material `secondary-fixed-dim` role.
    SecondaryFixedDim => secondary_fixed_dim;
    /// Material `on-secondary-fixed` role.
    OnSecondaryFixed => on_secondary_fixed;
    /// Material `on-secondary-fixed-variant` role.
    OnSecondaryFixedVariant => on_secondary_fixed_variant;
    /// Material `tertiary-fixed` role.
    TertiaryFixed => tertiary_fixed;
    /// Material `tertiary-fixed-dim` role.
    TertiaryFixedDim => tertiary_fixed_dim;
    /// Material `on-tertiary-fixed` role.
    OnTertiaryFixed => on_tertiary_fixed;
    /// Material `on-tertiary-fixed-variant` role.
    OnTertiaryFixedVariant => on_tertiary_fixed_variant;
}

#[cfg(test)]
mod tests {
    use material_color_utils::utils::color_utils::Argb;
    use waterui::Plugin as _;
    use waterui::prelude::theme_color;
    use waterui::reactive::Computed;
    use waterui::theme::{
        ColorScheme, Theme, current_color_scheme, installed_color_scheme, installed_color_signal,
    };

    use super::*;
    use crate::{Material3, MaterialColorMode};
    use hydrolysis::Style as _;

    fn assert_resolved_color_eq(actual: WorkingColor, expected: WorkingColor) {
        assert_eq!(
            actual.components.map(f32::to_bits),
            expected.components.map(f32::to_bits)
        );
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "test helper; passing assertion inputs by value reads clearest"
    )]
    fn assert_resolves_to(
        env: &Environment,
        color: impl Resolvable<Resolved = WorkingColor>,
        expected: MaterialRoleColor,
    ) {
        assert_resolved_color_eq(color.resolve(env).snapshot(), expected.working());
    }

    #[test]
    fn material_role_tokens_resolve_from_installed_scheme() {
        let scheme = MaterialColorScheme::baseline_light();
        let mut env = Environment::new();
        Material3::with_colors(scheme).install_tokens(&mut env);

        macro_rules! assert_role {
            ($token:expr, $field:ident) => {
                assert_resolves_to(&env, $token, scheme.$field);
            };
        }

        assert_role!(Background, background);
        assert_role!(OnBackground, on_background);
        assert_role!(Surface, surface);
        assert_role!(SurfaceDim, surface_dim);
        assert_role!(SurfaceBright, surface_bright);
        assert_role!(SurfaceContainerLowest, surface_container_lowest);
        assert_role!(SurfaceContainerLow, surface_container_low);
        assert_role!(SurfaceContainer, surface_container);
        assert_role!(SurfaceContainerHigh, surface_container_high);
        assert_role!(SurfaceContainerHighest, surface_container_highest);
        assert_role!(OnSurface, on_surface);
        assert_role!(SurfaceVariant, surface_variant);
        assert_role!(OnSurfaceVariant, on_surface_variant);
        assert_role!(InverseSurface, inverse_surface);
        assert_role!(InverseOnSurface, inverse_on_surface);
        assert_role!(Outline, outline);
        assert_role!(OutlineVariant, outline_variant);
        assert_role!(Shadow, shadow);
        assert_role!(Scrim, scrim);
        assert_role!(SurfaceTint, surface_tint);
        assert_role!(Primary, primary);
        assert_role!(OnPrimary, on_primary);
        assert_role!(PrimaryContainer, primary_container);
        assert_role!(OnPrimaryContainer, on_primary_container);
        assert_role!(InversePrimary, inverse_primary);
        assert_role!(Secondary, secondary);
        assert_role!(OnSecondary, on_secondary);
        assert_role!(SecondaryContainer, secondary_container);
        assert_role!(OnSecondaryContainer, on_secondary_container);
        assert_role!(Tertiary, tertiary);
        assert_role!(OnTertiary, on_tertiary);
        assert_role!(TertiaryContainer, tertiary_container);
        assert_role!(OnTertiaryContainer, on_tertiary_container);
        assert_role!(Error, error);
        assert_role!(OnError, on_error);
        assert_role!(ErrorContainer, error_container);
        assert_role!(OnErrorContainer, on_error_container);
        assert_role!(PrimaryFixed, primary_fixed);
        assert_role!(PrimaryFixedDim, primary_fixed_dim);
        assert_role!(OnPrimaryFixed, on_primary_fixed);
        assert_role!(OnPrimaryFixedVariant, on_primary_fixed_variant);
        assert_role!(SecondaryFixed, secondary_fixed);
        assert_role!(SecondaryFixedDim, secondary_fixed_dim);
        assert_role!(OnSecondaryFixed, on_secondary_fixed);
        assert_role!(OnSecondaryFixedVariant, on_secondary_fixed_variant);
        assert_role!(TertiaryFixed, tertiary_fixed);
        assert_role!(TertiaryFixedDim, tertiary_fixed_dim);
        assert_role!(OnTertiaryFixed, on_tertiary_fixed);
        assert_role!(OnTertiaryFixedVariant, on_tertiary_fixed_variant);
    }

    #[test]
    fn defaults_bind_a_light_scheme_on_an_empty_environment() {
        let mut env = Environment::new();
        Material3::defaults().install_tokens(&mut env);

        assert_eq!(current_color_scheme(&env).snapshot(), ColorScheme::Light);
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::Accent>(&env)
                .expect("default accent token should be installed")
                .snapshot(),
            MaterialColorScheme::baseline_light().primary.working(),
        );
    }

    #[test]
    fn a_clone_binds_to_the_first_environment_it_is_installed_into() {
        let original = Material3::defaults();
        let mut env = Environment::new();
        original.install_tokens(&mut env);

        let clone = original.clone();
        let mut env2 = Environment::new();
        Theme::new()
            .color_scheme(Computed::constant(ColorScheme::Dark))
            .install(&mut env2);
        clone.install_tokens(&mut env2);

        assert_eq!(clone.colors().mode, MaterialColorMode::Dark);
        assert_eq!(original.colors().mode, MaterialColorMode::Light);
    }

    #[test]
    fn a_bound_scheme_replaces_the_signal_of_later_environments() {
        let style = Material3::defaults();
        let mut env = Environment::new();
        style.install_tokens(&mut env);

        let mut env2 = Environment::new();
        Theme::new()
            .color_scheme(Computed::constant(ColorScheme::Dark))
            .install(&mut env2);
        style.install_tokens(&mut env2);

        assert_eq!(style.colors().mode, MaterialColorMode::Light);
        assert_eq!(
            installed_color_scheme(&env2)
                .expect("the bound scheme signal should be installed")
                .snapshot(),
            ColorScheme::Light
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::Accent>(&env2)
                .expect("default accent token should be installed")
                .snapshot(),
            MaterialColorScheme::baseline_light().primary.working(),
        );
    }

    #[test]
    fn custom_seed_roles_drive_waterui_theme_tokens() {
        let scheme = MaterialColorScheme::from_seed(
            Argb::from_rgb(0x00, 0x6a, 0x6a),
            MaterialColorMode::Dark,
        );
        let mut env = Environment::new();
        Material3::with_colors(scheme).install_tokens(&mut env);

        assert_eq!(current_color_scheme(&env).snapshot(), ColorScheme::Dark);
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::Accent>(&env)
                .expect("accent token should be installed")
                .snapshot(),
            scheme.primary.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::AccentContainer>(&env)
                .expect("accent-container token should be installed")
                .snapshot(),
            scheme.primary_container.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::Tertiary>(&env)
                .expect("tertiary token should be installed")
                .snapshot(),
            scheme.tertiary.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::TertiaryContainer>(&env)
                .expect("tertiary-container token should be installed")
                .snapshot(),
            scheme.tertiary_container.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::SelectionContainer>(&env)
                .expect("selection-container token should be installed")
                .snapshot(),
            scheme.secondary_container.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::SelectionForeground>(&env)
                .expect("selection-foreground token should be installed")
                .snapshot(),
            scheme.on_secondary_container.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::Error>(&env)
                .expect("error token should be installed")
                .snapshot(),
            scheme.error.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::ErrorForeground>(&env)
                .expect("error-foreground token should be installed")
                .snapshot(),
            scheme.on_error.working(),
        );
        assert_resolved_color_eq(
            installed_color_signal::<theme_color::Surface>(&env)
                .expect("surface token should be installed")
                .snapshot(),
            scheme.surface.working(),
        );
        assert_resolves_to(&env, Primary, scheme.primary);
        assert_resolves_to(&env, Surface, scheme.surface);
    }
}
