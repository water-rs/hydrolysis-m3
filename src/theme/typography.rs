use waterui::reactive::{Computed, Signal};
use waterui::text::font::{
    Body, Caption, Font, FontWeight, Footnote, Headline, ResolvedFont, Subheadline, Title,
};
use waterui::theme::install_font_signal;
use waterui_core::{Environment, resolve::Resolvable};

const MATERIAL_TYPEFACE: &str = "Roboto, sans-serif";

const fn font(
    size: f32,
    weight: FontWeight,
    line_height: f32,
    letter_spacing: f32,
) -> ResolvedFont {
    ResolvedFont::with_static_family(size, weight, MATERIAL_TYPEFACE)
        .with_typography_metrics(line_height, letter_spacing)
}

/// Installs Material's font for role `T` over whatever `env` carries.
///
/// The runtime hands a style an environment that already holds the
/// framework's default fonts and layers the application's own environment
/// over the style's tokens afterwards, so an unconditional install replaces
/// the framework default while an application's font for `T` still wins.
fn install<T: 'static>(env: &mut Environment, value: ResolvedFont) {
    install_font_signal::<T>(env, Computed::constant(value));
}

pub fn defaults(env: &mut Environment) {
    install::<Body>(env, font(16.0, FontWeight::Normal, 24.0, 0.5));
    install::<Title>(env, font(22.0, FontWeight::Normal, 28.0, 0.0));
    install::<Headline>(env, font(24.0, FontWeight::Normal, 32.0, 0.0));
    install::<Subheadline>(env, font(16.0, FontWeight::Medium, 24.0, 0.15));
    install::<Caption>(env, font(12.0, FontWeight::Normal, 16.0, 0.4));
    install::<Footnote>(env, font(11.0, FontWeight::Medium, 16.0, 0.5));
}

#[derive(Debug, Clone, Copy)]
pub struct LabelLarge;

impl Resolvable for LabelLarge {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(14.0, FontWeight::Medium, 20.0, 0.1))
    }
}

pub fn label_large() -> Font {
    Font::new(LabelLarge)
}

/// `md.sys.typescale.label-large-prominent`: label-large at `weight.bold`,
/// which the spec uses for the active item's label in navigation rails and
/// drawers.
#[derive(Debug, Clone, Copy)]
pub struct LabelLargeProminent;

impl Resolvable for LabelLargeProminent {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(14.0, FontWeight::Bold, 20.0, 0.1))
    }
}

pub fn label_large_prominent() -> Font {
    Font::new(LabelLargeProminent)
}

/// `md.comp.slider.value-indicator.label.label-text.*` per
/// m3.material.io/components/sliders/specs: the typescale's label-large
/// (14/20, weight 500 Medium, tracking 0.1).
#[derive(Debug, Clone, Copy)]
pub struct SliderValueIndicatorLabel;

impl Resolvable for SliderValueIndicatorLabel {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(14.0, FontWeight::Medium, 20.0, 0.1))
    }
}

pub fn slider_value_indicator_label() -> Font {
    Font::new(SliderValueIndicatorLabel)
}

#[derive(Debug, Clone, Copy)]
pub struct LabelMedium;

impl Resolvable for LabelMedium {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(12.0, FontWeight::Medium, 16.0, 0.5))
    }
}

pub fn label_medium() -> Font {
    Font::new(LabelMedium)
}

/// `md.sys.typescale.label-medium-prominent`: label-medium at `weight.bold`,
/// which the spec uses for the active item's label in a navigation bar.
#[derive(Debug, Clone, Copy)]
pub struct LabelMediumProminent;

impl Resolvable for LabelMediumProminent {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(12.0, FontWeight::Bold, 16.0, 0.5))
    }
}

pub fn label_medium_prominent() -> Font {
    Font::new(LabelMediumProminent)
}

#[derive(Debug, Clone, Copy)]
pub struct LabelSmall;

impl Resolvable for LabelSmall {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(11.0, FontWeight::Medium, 16.0, 0.5))
    }
}

pub fn label_small() -> Font {
    Font::new(LabelSmall)
}

#[derive(Debug, Clone, Copy)]
pub struct BodyLarge;

impl Resolvable for BodyLarge {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(16.0, FontWeight::Normal, 24.0, 0.5))
    }
}

pub fn body_large() -> Font {
    Font::new(BodyLarge)
}

#[derive(Debug, Clone, Copy)]
pub struct BodyMedium;

impl Resolvable for BodyMedium {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(14.0, FontWeight::Normal, 20.0, 0.25))
    }
}

pub fn body_medium() -> Font {
    Font::new(BodyMedium)
}

/// Line height of Material's Body Small role, also the floating label's type.
pub const BODY_SMALL_LINE_HEIGHT: f32 = 16.0;

#[derive(Debug, Clone, Copy)]
pub struct BodySmall;

impl Resolvable for BodySmall {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(12.0, FontWeight::Normal, BODY_SMALL_LINE_HEIGHT, 0.4))
    }
}

pub fn body_small() -> Font {
    Font::new(BodySmall)
}

#[derive(Debug, Clone, Copy)]
pub struct TitleSmall;

impl Resolvable for TitleSmall {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(14.0, FontWeight::Medium, 20.0, 0.1))
    }
}

pub fn title_small() -> Font {
    Font::new(TitleSmall)
}

#[derive(Debug, Clone, Copy)]
pub struct HeadlineSmall;

impl Resolvable for HeadlineSmall {
    type Resolved = ResolvedFont;

    fn resolve(&self, _env: &Environment) -> impl Signal<Output = Self::Resolved> {
        Computed::constant(font(24.0, FontWeight::Normal, 32.0, 0.0))
    }
}

pub fn headline_small() -> Font {
    Font::new(HeadlineSmall)
}

#[cfg(test)]
mod tests {
    use super::{
        MATERIAL_TYPEFACE, body_large, body_medium, body_small, defaults, headline_small,
        label_large, label_medium, label_small, title_small,
    };
    use waterui::Plugin as _;
    use waterui::env::Environment;
    use waterui::reactive::Signal as _;
    use waterui::text::font::{
        Body, Caption, FontWeight, Footnote, Headline, ResolvedFont, Subheadline, Title,
    };
    use waterui_core::resolve::Resolvable as _;

    #[allow(
        clippy::needless_pass_by_value,
        reason = "test helper; passing assertion inputs by value reads clearest"
    )]
    fn assert_material_font(
        font: waterui::text::font::ResolvedFont,
        expected_size: f32,
        expected_weight: FontWeight,
        expected_line_height: f32,
        expected_letter_spacing: f32,
    ) {
        assert_eq!(font.size, expected_size);
        assert_eq!(font.weight, expected_weight);
        assert_eq!(font.line_height, Some(expected_line_height));
        assert_eq!(font.letter_spacing, expected_letter_spacing);
        assert_eq!(font.family.as_deref(), Some(MATERIAL_TYPEFACE));
    }

    #[test]
    fn material_typeface_uses_roboto_then_script_aware_sans_fallbacks() {
        assert_eq!(MATERIAL_TYPEFACE, "Roboto, sans-serif");
    }

    #[test]
    fn font_slots_match_compose_type_scale_tokens() {
        let mut env = Environment::new();
        defaults(&mut env);

        assert_material_font(
            Body.resolve(&env).snapshot(),
            16.0,
            FontWeight::Normal,
            24.0,
            0.5,
        );
        assert_material_font(
            Title.resolve(&env).snapshot(),
            22.0,
            FontWeight::Normal,
            28.0,
            0.0,
        );
        assert_material_font(
            Headline.resolve(&env).snapshot(),
            24.0,
            FontWeight::Normal,
            32.0,
            0.0,
        );
        assert_material_font(
            Subheadline.resolve(&env).snapshot(),
            16.0,
            FontWeight::Medium,
            24.0,
            0.15,
        );
        assert_material_font(
            Caption.resolve(&env).snapshot(),
            12.0,
            FontWeight::Normal,
            16.0,
            0.4,
        );
        assert_material_font(
            Footnote.resolve(&env).snapshot(),
            11.0,
            FontWeight::Medium,
            16.0,
            0.5,
        );
    }

    #[test]
    fn material_fonts_replace_framework_defaults_and_yield_to_the_app() {
        // The runtime's assembly: the framework defaults, the application's
        // environment over them while the style installs, and the
        // application's environment over the style's tokens at the end.
        let mut framework = Environment::new();
        waterui::theme::Theme::new()
            .fonts(waterui::theme::FontSettings::default_scale())
            .install(&mut framework);
        let app_body = ResolvedFont::new(27.0, FontWeight::Bold);
        let mut app = Environment::new();
        waterui::theme::Theme::new()
            .fonts(waterui::theme::FontSettings::new().body(app_body.clone()))
            .install(&mut app);

        let mut styled = Environment::new().layered_on(&framework);
        defaults(&mut styled);
        assert_material_font(
            Body.resolve(&styled).snapshot(),
            16.0,
            FontWeight::Normal,
            24.0,
            0.5,
        );

        let mut styled = app.layered_on(&framework);
        defaults(&mut styled);
        let assembled = app.layered_on(&styled);
        let resolved_body = Body.resolve(&assembled).snapshot();
        assert_eq!(resolved_body.size, app_body.size);
        assert_eq!(resolved_body.weight, app_body.weight);
        assert_eq!(resolved_body.family, app_body.family);
        assert_material_font(
            Title.resolve(&assembled).snapshot(),
            22.0,
            FontWeight::Normal,
            28.0,
            0.0,
        );
    }

    #[test]
    fn label_large_matches_compose_type_scale() {
        let env = Environment::new();

        assert_material_font(
            label_large().resolve(&env).snapshot(),
            14.0,
            FontWeight::Medium,
            20.0,
            0.1,
        );
    }

    #[test]
    fn label_medium_matches_compose_type_scale() {
        let env = Environment::new();

        assert_material_font(
            label_medium().resolve(&env).snapshot(),
            12.0,
            FontWeight::Medium,
            16.0,
            0.5,
        );
    }

    #[test]
    fn label_small_matches_compose_type_scale() {
        let env = Environment::new();

        assert_material_font(
            label_small().resolve(&env).snapshot(),
            11.0,
            FontWeight::Medium,
            16.0,
            0.5,
        );
    }

    #[test]
    fn body_medium_matches_compose_type_scale() {
        let env = Environment::new();

        assert_material_font(
            body_medium().resolve(&env).snapshot(),
            14.0,
            FontWeight::Normal,
            20.0,
            0.25,
        );
    }

    #[test]
    fn body_large_matches_compose_type_scale() {
        let env = Environment::new();

        assert_material_font(
            body_large().resolve(&env).snapshot(),
            16.0,
            FontWeight::Normal,
            24.0,
            0.5,
        );
    }

    #[test]
    fn body_small_matches_compose_type_scale() {
        let env = Environment::new();

        assert_material_font(
            body_small().resolve(&env).snapshot(),
            12.0,
            FontWeight::Normal,
            16.0,
            0.4,
        );
    }

    #[test]
    fn title_small_matches_compose_type_scale() {
        let env = Environment::new();

        assert_material_font(
            title_small().resolve(&env).snapshot(),
            14.0,
            FontWeight::Medium,
            20.0,
            0.1,
        );
    }

    #[test]
    fn headline_small_matches_m3_reference_headline_small() {
        let env = Environment::new();

        assert_material_font(
            headline_small().resolve(&env).snapshot(),
            24.0,
            FontWeight::Normal,
            32.0,
            0.0,
        );
    }
}
