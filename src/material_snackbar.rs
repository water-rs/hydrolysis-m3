//! Material Design 3 snackbar convenience API.

use waterui::Str;
use waterui::snackbar::Snackbar;

/// A Material Design 3 snackbar configuration.
///
/// Rendering, queueing, dismissal, and motion are owned by `WaterUI`'s
/// `SnackbarManager`; this alias keeps the Material package API explicit while
/// preserving the existing runtime contract.
pub type MaterialSnackbar = Snackbar;

/// Creates a Material Design 3 snackbar.
#[must_use]
pub fn material_snackbar(message: impl Into<Str>) -> MaterialSnackbar {
    Snackbar::new(message.into())
}

#[cfg(test)]
mod tests {
    use waterui_core::Environment;

    use crate::layout::snackbar;

    #[test]
    fn material_snackbar_tokens_match_compose_snackbar_tokens() {
        let look = snackbar::look(&Environment::new());

        assert_eq!(look.single_line_min_height, 48.0);
        assert_eq!(look.corner_radius, 4.0);
        assert_eq!(look.content_padding.top(), 12.0);
        assert_eq!(look.content_padding.leading(), 16.0);
        assert_eq!(look.content_spacing, 12.0);
        assert_eq!(look.shadow_radius, 5.0);
        assert_eq!(look.shadow_offset_y, 1.25);
        assert_eq!(look.motion_offset_y, 20.0);
    }
}
