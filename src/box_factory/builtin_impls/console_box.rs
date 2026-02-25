/*!
 * Builtin ConsoleBox Implementation (Phase 151: Selfhost Support Fallback)
 *
 * ⚠️ PLUGIN-PREFERRED: nyash-console-plugin is the primary implementation
 * 🎯 Phase 151: Builtin fallback added to support selfhost Stage-3 pipeline
 *
 * Context: When running through selfhost compiler, plugins may not be properly
 * initialized, so we provide a builtin fallback to ensure ConsoleBox is available.
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;
use crate::boxes::ConsoleBox;

/// Create builtin ConsoleBox instance
///
/// Primary: nyash-console-plugin
/// Fallback: This builtin implementation (selfhost support)
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    // Phase 151: Quiet fallback (no deprecation warning - this is intentional selfhost support)
    Ok(Box::new(ConsoleBox::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_console_box_creation() {
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<ConsoleBox>().is_some());
    }
}
