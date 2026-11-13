//! Material Design 3 divider composed from the `WaterUI` divider primitive.

use waterui::widget::Divider;

/// A Material Design 3 divider.
///
/// Divider orientation follows the surrounding stack axis, matching the
/// `WaterUI` divider primitive. Hydrolysis draws it at 1 logical unit in the
/// `Border` token, which this theme maps to the `outline` role.
pub type MaterialDivider = Divider;

/// Creates a Material Design 3 divider.
pub const fn material_divider() -> MaterialDivider {
    Divider
}
