/// Minimum ripple diameter (Material Web `MINIMUM_PRESS_DIAMETER`): small
/// targets still produce a ripple at least this wide.
pub const RIPPLE_MINIMUM_DIAMETER: f64 = 48.0;

/// `StateTokens.HoverStateLayerOpacity`.
pub const HOVER_STATE_LAYER_OPACITY: f32 = 0.08;
/// `StateTokens.FocusStateLayerOpacity`.
pub const FOCUS_STATE_LAYER_OPACITY: f32 = 0.1;
/// `StateTokens.PressedStateLayerOpacity`.
pub const PRESSED_STATE_LAYER_OPACITY: f32 = 0.1;
/// `StateTokens.DraggedStateLayerOpacity`.
pub const DRAGGED_STATE_LAYER_OPACITY: f32 = 0.16;

/// Initial ripple size as a fraction of the full diameter: a wave starts at
/// this scale over its press point and grows to full size.
pub const RIPPLE_INITIAL_SCALE: f64 = 0.4;
