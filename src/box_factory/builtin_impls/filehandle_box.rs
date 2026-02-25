/*!
 * Builtin FileHandleBox Implementation (Phase 113: Nyash API)
 *
 * 🎯 Phase 113: Exposes FileHandleBox methods to Nyash (.hako) code
 * 🛡️ Ring0-aware: Respects RuntimeProfile (Default/NoFs)
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;
use crate::boxes::file::FileHandleBox;

/// Create builtin FileHandleBox instance
///
/// This provides FileHandleBox with ny_* methods accessible from Nyash.
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    if crate::config::env::cli_verbose_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug("[FileHandleBox] Creating FileHandleBox instance (Phase 113)");
    }

    // FileHandleBox::new() will automatically use the global Ring0 registry
    // which respects the RuntimeProfile (Default/NoFs)
    let handle = FileHandleBox::new();

    Ok(Box::new(handle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_filehandle_box_creation() {
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<FileHandleBox>().is_some());
    }
}
