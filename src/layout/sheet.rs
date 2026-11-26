//! The Material 3 modal bottom-sheet look from `SheetBottomTokens`.

use hydrolysis_style::theme::SheetLook;
use waterui_core::Environment;
use waterui_graphics::color::Color;

use crate::color::{OnSurfaceVariant, SurfaceContainerLow};

/// M3 `SheetBottomTokens.DockedContainerMaxWidth` (640 dp).
const SHEET_MAX_WIDTH: f32 = 640.0;
/// M3 `SheetBottomTokens.DockedContainerShape` (extra-large corner radius, 28 dp).
const SHEET_CORNER_RADIUS: f32 = 28.0;
/// M3 `SheetBottomTokens.DockedDragHandleWidth` (32 dp).
const DRAG_HANDLE_WIDTH: f32 = 32.0;
/// M3 `SheetBottomTokens.DockedDragHandleHeight` (4 dp).
const DRAG_HANDLE_HEIGHT: f32 = 4.0;
/// Vertical padding above and below the M3 bottom-sheet drag handle (12 dp).
const DRAG_HANDLE_VERTICAL_PADDING: f32 = 12.0;
/// M3 `SheetBottomTokens.DockedDragHandleOpacity` (0.4).
const DRAG_HANDLE_OPACITY: f32 = 0.4;

#[must_use]
pub fn look(_env: &Environment) -> SheetLook {
    SheetLook {
        container_color: SurfaceContainerLow.into(),
        max_width: SHEET_MAX_WIDTH,
        corner_radius: SHEET_CORNER_RADIUS,
        show_drag_handle: true,
        drag_handle_color: Color::from(OnSurfaceVariant).with_opacity(DRAG_HANDLE_OPACITY),
        drag_handle_width: DRAG_HANDLE_WIDTH,
        drag_handle_height: DRAG_HANDLE_HEIGHT,
        drag_handle_vertical_padding: DRAG_HANDLE_VERTICAL_PADDING,
    }
}

#[cfg(test)]
mod tests {
    use waterui_core::Environment;

    use super::{
        DRAG_HANDLE_HEIGHT, DRAG_HANDLE_VERTICAL_PADDING, DRAG_HANDLE_WIDTH, SHEET_CORNER_RADIUS,
        SHEET_MAX_WIDTH, look,
    };

    #[test]
    fn sheet_look_matches_material_sheet_tokens() {
        let sheet_look = look(&Environment::new());

        assert_eq!(sheet_look.max_width, SHEET_MAX_WIDTH);
        assert_eq!(sheet_look.corner_radius, SHEET_CORNER_RADIUS);
        assert!(sheet_look.show_drag_handle);
        assert_eq!(sheet_look.drag_handle_width, DRAG_HANDLE_WIDTH);
        assert_eq!(sheet_look.drag_handle_height, DRAG_HANDLE_HEIGHT);
        assert_eq!(
            sheet_look.drag_handle_vertical_padding,
            DRAG_HANDLE_VERTICAL_PADDING
        );
    }
}
