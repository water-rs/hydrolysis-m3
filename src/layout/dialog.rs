//! The Material 3 dialog look from `DialogTokens`.

use hydrolysis_style::theme::DialogLook;
use waterui_core::Environment;
use waterui_layout::Alignment;
use waterui_layout::padding::EdgeInsets;

use crate::color::{Error, OnSurface, OnSurfaceVariant, Primary, SurfaceContainerHigh};

/// M3 `DialogTokens.ContainerMinWidth` (280 dp).
const DIALOG_MIN_WIDTH: f32 = 280.0;
/// M3 `DialogTokens.ContainerMaxWidth` (560 dp).
const DIALOG_MAX_WIDTH: f32 = 560.0;
/// M3 `DialogTokens.ContainerShape` (extra-large corner radius, 28 dp).
const DIALOG_CORNER_RADIUS: f32 = 28.0;
/// M3 dialog container padding (24 dp).
const DIALOG_CONTENT_PADDING: f32 = 24.0;
/// M3 headline-to-body spacing (16 dp).
const DIALOG_CONTENT_SPACING: f32 = 16.0;
/// M3 extra spacing above the action row (24 dp total gap = 16 dp + 8 dp).
const DIALOG_ACTIONS_TOP_PADDING: f32 = 8.0;
/// M3 spacing between adjacent dialog action buttons (8 dp).
const DIALOG_ACTIONS_SPACING: f32 = 8.0;

#[must_use]
pub fn look(_env: &Environment) -> DialogLook {
    DialogLook {
        container_color: SurfaceContainerHigh.into(),
        title_color: OnSurface.into(),
        message_color: OnSurfaceVariant.into(),
        primary_action_color: Primary.into(),
        destructive_action_color: Error.into(),
        default_action_color: OnSurface.into(),
        corner_radius: DIALOG_CORNER_RADIUS,
        content_padding: EdgeInsets::all(DIALOG_CONTENT_PADDING),
        min_width: DIALOG_MIN_WIDTH,
        max_width: DIALOG_MAX_WIDTH,
        content_spacing: DIALOG_CONTENT_SPACING,
        actions_top_padding: DIALOG_ACTIONS_TOP_PADDING,
        actions_spacing: DIALOG_ACTIONS_SPACING,
        actions_alignment: Alignment::Trailing,
    }
}

#[cfg(test)]
mod tests {
    use waterui_core::Environment;
    use waterui_layout::Alignment;

    use super::{
        DIALOG_ACTIONS_SPACING, DIALOG_ACTIONS_TOP_PADDING, DIALOG_CONTENT_PADDING,
        DIALOG_CONTENT_SPACING, DIALOG_CORNER_RADIUS, DIALOG_MAX_WIDTH, DIALOG_MIN_WIDTH, look,
    };

    #[test]
    fn dialog_look_matches_material_dialog_tokens() {
        let dialog_look = look(&Environment::new());

        assert_eq!(dialog_look.min_width, DIALOG_MIN_WIDTH);
        assert_eq!(dialog_look.max_width, DIALOG_MAX_WIDTH);
        assert_eq!(dialog_look.corner_radius, DIALOG_CORNER_RADIUS);
        assert_eq!(dialog_look.content_padding.top(), DIALOG_CONTENT_PADDING);
        assert_eq!(
            dialog_look.content_padding.leading(),
            DIALOG_CONTENT_PADDING
        );
        assert_eq!(dialog_look.content_spacing, DIALOG_CONTENT_SPACING);
        assert_eq!(dialog_look.actions_top_padding, DIALOG_ACTIONS_TOP_PADDING);
        assert_eq!(dialog_look.actions_spacing, DIALOG_ACTIONS_SPACING);
        assert_eq!(dialog_look.actions_alignment, Alignment::Trailing);
    }
}
