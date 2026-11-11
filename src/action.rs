//! The action a Material control stores when none was configured.

use waterui::Environment;
use waterui_core::handler::ExtractionError;

/// The stored action's signature: the framework's button action, which fails
/// when one of the handler's parameters cannot be extracted from the
/// environment.
pub use waterui_controls::button::NoAction;

/// The action of a control with none configured: it does nothing and cannot
/// fail.
#[expect(
    clippy::unnecessary_wraps,
    reason = "the signature is the stored action's"
)]
pub const fn noop(_env: &Environment) -> Result<(), ExtractionError> {
    Ok(())
}
