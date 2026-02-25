/*!
 * Builtin FileBox Implementation (Phase 15.5: Fallback Support)
 *
 * 🛡️ FALLBACK: Provides core-ro FileBox when plugin is unavailable or fails
 * 🎯 Auto mode: Plugin first, fallback to this if plugin fails
 * 🎯 Plugin-only mode: This won't be called (Fail-Fast)
 * 🎯 Core-ro mode: This is used directly
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;
use crate::boxes::file::core_ro::CoreRoFileIo;
use crate::boxes::file::FileBox;
use std::sync::Arc;

/// Create builtin FileBox instance (core-ro provider)
///
/// This provides a fallback FileBox implementation with core-ro provider
/// when plugins are unavailable or fail in auto mode.
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    use crate::runner::modes::common_util::provider_registry;

    // Check NYASH_FILEBOX_MODE - Fail-Fast in plugin-only mode
    let mode = provider_registry::read_filebox_mode_from_env();
    if matches!(mode, provider_registry::FileBoxMode::PluginOnly) {
        return Err(RuntimeError::InvalidOperation {
            message: "FileBox creation failed: plugin-only mode requires plugin, but plugins are disabled or unavailable".to_string(),
        });
    }

    if crate::config::env::cli_verbose_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug("[FileBox] Using builtin core-ro fallback implementation");
    }

    // Create FileBox with core-ro provider directly
    // Don't rely on global provider_lock which may not be initialized
    let provider = Arc::new(CoreRoFileIo::new());
    let filebox = FileBox::with_provider(provider);

    Ok(Box::new(filebox))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_file_box_creation() {
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<FileBox>().is_some());
    }
}
