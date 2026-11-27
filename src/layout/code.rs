//! The Material 3 code-block look from the surface container, outline, and
//! typography tokens.

use hydrolysis_style::draw::{Paint, WorkingColor};
use hydrolysis_style::theme::{
    Border, CodeLook, CodeMetrics, CornerShape, Driven, Inputs, Surface,
};
use waterui_core::Environment;
use waterui_graphics::color::Color;
use waterui_text::font::{Body, Caption, Font};

use crate::color::{OnSurfaceVariant, OutlineVariant, Primary, SurfaceContainerHighest};
use crate::controls::button::solid_paint;

/// Inner padding on all four edges of a Material 3 code block container, in dp.
const CODE_CONTENT_PADDING: f32 = 16.0;
/// Vertical gap between the language header row and the source body, in dp.
const CODE_HEADER_SPACING: f32 = 8.0;
/// Medium corner radius (`md.sys.shape.corner.medium`), in dp.
const CODE_CORNER_RADIUS: f32 = 12.0;
/// Hairline outline stroke width (`outline-variant`), in dp.
const CODE_OUTLINE_WIDTH: f32 = 1.0;

fn outline_border(env: &Environment) -> Driven<Border> {
    Driven::follow(
        Color::from(OutlineVariant).resolve(env),
        |outline: &WorkingColor, _inputs: &Inputs<'_, ()>| {
            Border::new(CODE_OUTLINE_WIDTH, Paint::from(*outline))
        },
    )
}

/// The Material 3 code-block look: `surface-container-highest` fill with a
/// 1 dp `outline-variant` stroke, 12 dp rounded corners, `on-surface-variant`
/// header caption, `primary` copy action tint, and monospaced `Body` source.
#[must_use]
pub fn look(env: &Environment) -> CodeLook {
    let shape = CornerShape::radius(CODE_CORNER_RADIUS);
    let surface = Surface::new(shape, solid_paint::<()>(SurfaceContainerHighest, env))
        .border(outline_border(env));
    CodeLook::new(
        surface,
        CodeMetrics::uniform(CODE_CONTENT_PADDING, CODE_HEADER_SPACING),
    )
    .header_font(Font::from(Caption))
    .header_color(Color::from(OnSurfaceVariant))
    .copy_color(Color::from(Primary))
    .copied_color(Color::from(Primary))
    .body_font(Font::from(Body).monospaced(true))
}

#[cfg(test)]
mod tests {
    use hydrolysis_style::Style as _;
    use hydrolysis_style::draw::kurbo::Rect;
    use hydrolysis_style::theme::{Corners, Inputs, WidgetTheme as _};
    use waterui_core::Environment;
    use waterui_interaction::interaction::InteractionState;

    use super::{
        CODE_CONTENT_PADDING, CODE_CORNER_RADIUS, CODE_HEADER_SPACING, CODE_OUTLINE_WIDTH, look,
    };
    use crate::Material3;

    #[test]
    fn code_look_uses_material3_surface_and_metrics() {
        let mut env = Environment::new();
        let style = Material3::defaults();
        style.install_tokens(&mut env);
        let frame = Rect::new(0.0, 0.0, 320.0, 140.0);
        let rest = Inputs::at_rest(frame, InteractionState::empty(), &());

        let code = style.code(&env);
        assert_eq!(code.metrics().horizontal_padding, CODE_CONTENT_PADDING);
        assert_eq!(code.metrics().vertical_padding, CODE_CONTENT_PADDING);
        assert_eq!(code.metrics().header_spacing, CODE_HEADER_SPACING);
        assert_eq!(
            code.surface().shape().corners().target(&rest),
            Corners::from(CODE_CORNER_RADIUS)
        );
        let border = code
            .surface()
            .get_border()
            .expect("Material 3 code block carries an outline-variant border")
            .target(&rest);
        assert_eq!(border.width, CODE_OUTLINE_WIDTH);
        assert!(code.surface().get_elevation().target(&rest).is_none());

        let direct = look(&env);
        assert_eq!(direct.metrics(), code.metrics());
    }
}
