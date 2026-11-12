//! The Material 3 menu look from the `md.comp.menu.*` tokens.
//!
//! One look serves the picker popup, the text context menu and the popup
//! window a menu button opens. Hydrolysis measures and places a menu from
//! its layout and records the parts below once, at rest: a menu has no live
//! state of its own, its rows are buttons with looks of their own.

use hydrolysis::draw::kurbo::{Rect, RoundedRect, RoundedRectRadii};
use hydrolysis::draw::{Draw as _, Paint, Shadow as ShadowCast};
use hydrolysis::theme::{Custom, Driven, MenuLayout, MenuLook, MenuSpec};
use waterui_core::Environment;

use crate::color::{SurfaceContainer, SurfaceContainerHighest, SurfaceVariant};
use crate::controls::button::solid_paint;
use crate::dimensions::{
    MENU_CONTAINER_SHAPE, MENU_HORIZONTAL_PADDING, MENU_MAX_WIDTH, MENU_MIN_WIDTH, MENU_ROW_HEIGHT,
    MENU_SELECTED_ROW_HORIZONTAL_INSET, MENU_SELECTED_ROW_VERTICAL_INSET,
    MENU_SEPARATOR_HORIZONTAL_INSET, MENU_SEPARATOR_THICKNESS, MENU_VERTICAL_PADDING,
    MENU_WIDTH_PER_CHAR,
};
use crate::elevation::{MaterialElevationLevel, cast_shadows};

/// The measures of a menu: rows 48 tall, padded 12 by 8, 112 to 280 wide,
/// on a container of corner 4 (`md.comp.menu.list-item.container.height`,
/// `.leading-space`, `md.comp.menu.container.shape` and the Compose menu's
/// `DropdownMenuItemDefaultMinWidth` / `DropdownMenuItemDefaultMaxWidth`).
pub const fn layout() -> MenuLayout {
    MenuLayout {
        row_height: MENU_ROW_HEIGHT,
        horizontal_padding: MENU_HORIZONTAL_PADDING,
        vertical_padding: MENU_VERTICAL_PADDING,
        min_width: MENU_MIN_WIDTH,
        max_width: MENU_MAX_WIDTH,
        width_per_char: MENU_WIDTH_PER_CHAR,
        corner_radius: MENU_CONTAINER_SHAPE,
        separator_horizontal_inset: MENU_SEPARATOR_HORIZONTAL_INSET,
        separator_thickness: MENU_SEPARATOR_THICKNESS,
    }
}

/// The look of a menu in `spec`.
///
/// The panel is `surface-container` on corner 4, raised to level 2
/// (`md.comp.menu.container.*`) and carrying no outline; the separator is
/// `surface-variant` (`md.comp.menu.divider.color`); a picker popup's
/// selected row is `surface-container-highest`
/// (`md.comp.filled-select.menu.list-item.selected.container.color`).
pub fn look(_spec: MenuSpec, env: &Environment) -> MenuLook {
    MenuLook::new(layout(), panel(env), separator(env)).selected_row(selected_row(env))
}

/// The panel's operands: its level-2 key and ambient shadows, then its
/// container paint.
type PanelOperands = (Driven<ShadowCast>, Driven<ShadowCast>, Driven<Paint>);

/// The panel: its level-2 key and ambient shadows, then its container.
fn panel(env: &Environment) -> Custom<(), PanelOperands> {
    let (key, ambient) = cast_shadows(MaterialElevationLevel::LEVEL2, env);
    Custom::new(
        (key, ambient, solid_paint(SurfaceContainer, env)),
        |recorder, frame, (key, ambient, paint)| {
            let shape = RoundedRect::from_rect(
                frame,
                RoundedRectRadii::from_single_radius(MENU_CONTAINER_SHAPE),
            );
            recorder.shadow(shape, key);
            recorder.shadow(shape, ambient);
            recorder.fill(shape, paint);
        },
    )
}

/// The separator, filling the line a divider row leaves for it.
fn separator(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (solid_paint(SurfaceVariant, env),),
        |recorder, frame, (paint,)| recorder.fill(frame, paint),
    )
}

/// The rect a picker popup's selected row fills inside its `row`.
fn selected_row_rect(row: Rect) -> Rect {
    row.inset((
        -MENU_SELECTED_ROW_HORIZONTAL_INSET,
        -MENU_SELECTED_ROW_VERTICAL_INSET,
    ))
}

/// A picker popup's selected row, inset from its row.
fn selected_row(env: &Environment) -> Custom<(), (Driven<Paint>,)> {
    Custom::new(
        (solid_paint(SurfaceContainerHighest, env),),
        |recorder, frame, (paint,)| recorder.fill(selected_row_rect(frame), paint),
    )
}

#[cfg(test)]
mod tests {
    use super::{layout, look, selected_row_rect};
    use crate::dimensions::{
        MENU_CONTAINER_SHAPE, MENU_HORIZONTAL_PADDING, MENU_MAX_WIDTH, MENU_MIN_WIDTH,
        MENU_ROW_HEIGHT, MENU_VERTICAL_PADDING,
    };
    use hydrolysis::Style as _;
    use hydrolysis::draw::kurbo::Rect;
    use hydrolysis::theme::MenuSpec;
    use waterui_core::Environment;
    use waterui_core::layout::LayoutDirection;

    #[test]
    fn menu_layout_matches_compose_menu_tokens() {
        let layout = layout();

        assert_eq!(layout.row_height, MENU_ROW_HEIGHT);
        assert_eq!(layout.row_height, 48.0);
        assert_eq!(layout.horizontal_padding, MENU_HORIZONTAL_PADDING);
        assert_eq!(layout.horizontal_padding, 12.0);
        assert_eq!(layout.vertical_padding, MENU_VERTICAL_PADDING);
        assert_eq!(layout.vertical_padding, 8.0);
        assert_eq!(layout.min_width, MENU_MIN_WIDTH);
        assert_eq!(layout.min_width, 112.0);
        assert_eq!(layout.max_width, MENU_MAX_WIDTH);
        assert_eq!(layout.max_width, 280.0);
        assert_eq!(layout.corner_radius, MENU_CONTAINER_SHAPE);
        assert_eq!(layout.corner_radius, 4.0);
        assert_eq!(layout.separator_thickness, 1.0);
    }

    #[test]
    fn the_look_measures_by_the_layout_and_draws_its_selected_row() {
        let mut env = Environment::new();
        crate::Material3::defaults().install_tokens(&mut env);
        let look = look(MenuSpec::new(LayoutDirection::LeftToRight), &env);

        assert_eq!(look.layout(), layout());
        assert!(look.has_selected_row());
    }

    /// The selected row leaves the row's edges to the rows around it.
    #[test]
    fn the_selected_row_sits_inside_its_row() {
        assert_eq!(
            selected_row_rect(Rect::new(0.0, 0.0, 120.0, 48.0)),
            Rect::new(2.0, 1.0, 118.0, 47.0)
        );
    }
}
