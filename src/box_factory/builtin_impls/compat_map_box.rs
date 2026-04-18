/*!
 * Deprecated builtin MapBox constructor shim.
 *
 * This is not the live MapBox implementation and is no longer on the default
 * BuiltinBoxFactory route. It is compiled only with the `builtin-mapbox-compat`
 * feature for archive/compat probes while default NewBox(MapBox) construction
 * goes through the ring1 map provider seam.
 */

use crate::box_factory::RuntimeError;
use crate::box_trait::NyashBox;

/// Create a MapBox instance through the opt-in compatibility path.
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
