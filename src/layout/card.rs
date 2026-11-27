//! The Material 3 card look from the `md.comp.*-card.*` tokens.

use hydrolysis_style::draw::{Paint, WorkingColor};
use hydrolysis_style::theme::{
    Border, CardLayout, CardLook, CornerShape, Driven, Elevation, Inputs, Surface,
};
use waterui_controls::card::CardStyle;
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{
    OutlineVariant, Shadow, Surface as SurfaceRole, SurfaceContainerHighest, SurfaceContainerLow,
};
use crate::controls::button::solid_paint;
use crate::elevation::{MaterialElevationLevel, chrome_elevation};

const CARD_CONTENT_PADDING: f32 = 16.0;
const CARD_CONTENT_SPACING: f32 = 4.0;
const CARD_CORNER_RADIUS: f32 = 12.0;
const CARD_OUTLINE_WIDTH: f32 = 1.0;

fn card_elevation(level: MaterialElevationLevel, env: &Environment) -> Driven<Elevation> {
    Driven::follow(
        Color::from(Shadow).resolve(env),
        move |shadow: &WorkingColor, _inputs: &Inputs<'_, ()>| chrome_elevation(level, *shadow),
    )
}

fn outline_border(env: &Environment) -> Driven<Border> {
    Driven::follow(
        Color::from(OutlineVariant).resolve(env),
        |outline: &WorkingColor, _inputs: &Inputs<'_, ()>| {
            Border::new(CARD_OUTLINE_WIDTH, Paint::from(*outline))
        },
    )
}

/// The Material 3 card look for `style`.
///
/// - `Automatic` / `Filled`: `surface-container-highest` at level 0, 12 dp corners.
/// - `Elevated`: `surface-container-low` at level 1 (`md.comp.elevated-card.container.elevation`).
/// - `Outlined`: `surface` at level 0 with a 1 dp `outline-variant` stroke.
#[must_use]
pub fn look(style: CardStyle, env: &Environment) -> CardLook {
    let shape = CornerShape::radius(CARD_CORNER_RADIUS);
    let surface = match style {
        CardStyle::Elevated => Surface::new(shape, solid_paint::<()>(SurfaceContainerLow, env))
            .elevation(card_elevation(MaterialElevationLevel::LEVEL1, env)),
        CardStyle::Outlined => {
            Surface::new(shape, solid_paint::<()>(SurfaceRole, env)).border(outline_border(env))
        }
        CardStyle::Automatic | CardStyle::Filled | _ => {
            Surface::new(shape, solid_paint::<()>(SurfaceContainerHighest, env))
        }
    };
    CardLook::new(
        surface,
        CardLayout::uniform(CARD_CONTENT_PADDING, CARD_CONTENT_SPACING),
    )
}

#[cfg(test)]
mod tests {
    use hydrolysis_style::Style as _;
    use hydrolysis_style::draw::kurbo::Rect;
    use hydrolysis_style::theme::{Corners, Inputs};
    use waterui_controls::card::CardStyle;
    use waterui_core::Environment;
    use waterui_interaction::interaction::InteractionState;

    use super::{CARD_CONTENT_PADDING, CARD_CONTENT_SPACING, CARD_CORNER_RADIUS, look};
    use crate::Material3;

    #[test]
    fn card_look_matches_compose_card_tokens() {
        let mut env = Environment::new();
        Material3::defaults().install_tokens(&mut env);
        let frame = Rect::new(0.0, 0.0, 200.0, 100.0);
        let rest = Inputs::at_rest(frame, InteractionState::empty(), &());

        let filled = look(CardStyle::Filled, &env);
        assert_eq!(filled.layout().horizontal_padding, CARD_CONTENT_PADDING);
        assert_eq!(filled.layout().vertical_padding, CARD_CONTENT_PADDING);
        assert_eq!(filled.layout().content_spacing, CARD_CONTENT_SPACING);
        assert_eq!(
            filled.surface().shape().corners().target(&rest),
            Corners::from(CARD_CORNER_RADIUS)
        );
        assert!(filled.surface().get_border().is_none());
        assert!(filled.surface().get_elevation().target(&rest).is_none());

        let elevated = look(CardStyle::Elevated, &env);
        assert!(elevated.surface().get_border().is_none());
        let elev = elevated.surface().get_elevation().target(&rest);
        assert!(!elev.is_none());
        assert_eq!(elev.key.sigma, 1.5);
        assert_eq!(elev.key.offset.y, 0.5);
        assert_eq!(elev.ambient.sigma, 1.0);
        assert_eq!(elev.ambient.offset.y, 0.0);

        let outlined = look(CardStyle::Outlined, &env);
        let border = outlined
            .surface()
            .get_border()
            .expect("outlined card carries a border")
            .target(&rest);
        assert_eq!(border.width, 1.0);
        assert!(outlined.surface().get_elevation().target(&rest).is_none());
    }
}
