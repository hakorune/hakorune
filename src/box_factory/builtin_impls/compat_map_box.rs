/*!
 * Deprecated builtin MapBox constructor shim.
 *
 * This is not the live MapBox implementation. It is compiled only with the
 * `builtin-mapbox-compat` feature and keeps the old BuiltinBoxFactory fallback
 * route working until NewBox(MapBox) is proven through plugin/provider-first
 * construction.
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;

/// Create builtin MapBox instance through the compatibility factory path.
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    if crate::config::env::cli_verbose_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.warn(
            "[DEPRECATED] Using builtin MapBox compat constructor - prefer plugin/provider route\n\
            Phase 15.5: Everything is Plugin\n\
            Check: plugins/nyash-map-plugin",
        );
    }

    Ok(Box::new(crate::boxes::map_box::MapBox::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boxes::map_box::MapBox;

    #[test]
    fn test_builtin_map_box_compat_creation() {
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<MapBox>().is_some());
    }
}
