/*!
 * Builtin MapBox Implementation (Phase 15.5: Scheduled for Removal)
 *
 * ⚠️ DEPRECATED: This will be replaced by nyash-map-plugin (exists?)
 * 🎯 Phase 2.5: Delete this file to remove builtin MapBox support
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;

/// Create builtin MapBox instance
///
/// ⚠️ DEPRECATED: Check if nyash-map-plugin exists
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    if crate::config::env::cli_verbose_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.warn(
            "⚠️ [DEPRECATED] Using builtin MapBox - check nyash-map-plugin!\n\
            📋 Phase 15.5: Everything is Plugin!\n\
            🔧 Check: plugins/nyash-map-plugin"
        );
    }

    Ok(Box::new(crate::boxes::map_box::MapBox::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boxes::map_box::MapBox;

    #[test]
    fn test_builtin_map_box_creation() {
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<MapBox>().is_some());
    }
}
