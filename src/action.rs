//! The action a Material control stores when none was configured.

use waterui::Environment;
use waterui_core::handler::ExtractionError;

/// The stored action's signature: a handler that runs against the
/// environment and fails when one of its parameters cannot be extracted.
pub type NoAction = fn(&Environment) -> Result<(), ExtractionError>;

/// The action of a control with none configured: it does nothing and cannot
/// fail.
#[expect(
    clippy::unnecessary_wraps,
    reason = "the signature is the stored action's"
)]
pub const fn noop(_env: &Environment) -> Result<(), ExtractionError> {
    Ok(())
}
