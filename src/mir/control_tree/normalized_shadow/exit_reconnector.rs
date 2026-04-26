//! Phase 131 P1.5: ExitReconnectorBox
//!
//! Option B implementation: Direct variable_map reconnection for Normalized shadow
//!
//! ## Purpose
//!
//! Normalized IR uses k_exit env params as SSOT for exit values.
//! This box reconnects those exit values directly to host's variable_map,
//! bypassing the traditional PHI-based merge pipeline.
//!
//! ## Why Option B?
//!
//! **Problem**: Normalized IR's exit values are passed via k_exit function params,
//! while the traditional merge pipeline expects ExitMeta → exit_bindings → PHI.
//! The two approaches are incompatible because:
//!
//! 1. Normalized k_exit params are ALREADY the final exit values (no PHI needed)
//! 2. Traditional pipeline generates PHI nodes to merge exit values
//! 3. Mixing the two creates duplicate/incorrect PHI generation
//!
//! **Solution**: Direct reconnection for Normalized shadow only:
//! - Skip traditional merge pipeline's exit PHI generation
//! - Use ExitReconnectorBox to update variable_map directly
//! - Maintain separation between Normalized and traditional paths
//!
//! ## Contract
//!
//! **Input**:
//! - `exit_values`: Vec<(String, ValueId)> from jump args to k_exit (after merge/remap)
//!   - Variable names are the carrier names (e.g., "i", "sum", "count")
//!   - ValueIds are the actual computed values passed to k_exit (host ValueIds)
//! - `variable_map`: &mut BTreeMap<String, ValueId> from MirBuilder
//!
//! **Effect**:
//! - Updates variable_map entries for each carrier with the jump arg ValueId
//! - This makes the exit values available to post-loop code
//!
//! **Output**:
//! - None (side effect only: variable_map mutation)
//!
//! ## Example
//!
//! ```text
//! Before reconnection:
//!   variable_map = { "i" => ValueId(10) }  // pre-loop value
//!
//! After loop merge:
//!   Jump to k_exit with args [ValueId(42)]  // computed exit value
//!
//! After ExitReconnectorBox::reconnect():
//!   variable_map = { "i" => ValueId(42) }  // jump arg is now SSOT
//! ```
//!
//! ## Design Notes
//!
//! - **Pure function**: No complex logic, just map update
//! - **No PHI generation**: k_exit params ARE the exit values
//! - **Normalized-specific**: Only used for Normalized shadow path
//! - **Fail-Fast**: Panics if carrier not in variable_map (contract violation)

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// ExitReconnectorBox: Direct variable_map reconnection for Normalized shadow
pub struct ExitReconnectorBox;

impl ExitReconnectorBox {
    /// Reconnect k_exit env params to host variable_map
    ///
    /// # Algorithm
    ///
    /// For each (carrier_name, k_exit_param_vid) in exit_values:
    /// 1. Look up carrier_name in variable_map
    /// 2. Update variable_map[carrier_name] = k_exit_param_vid
    ///
    /// # Panics
    ///
    /// Panics if carrier_name is not in variable_map, as this indicates
    /// a contract violation (Normalized lowering should only emit carriers
    /// that exist in host's variable scope).
    ///
    /// # Phase 131 P1.5: Normalized-specific design
    ///
    /// This is ONLY for Normalized shadow path. Traditional patterns use
    /// the standard merge pipeline with PHI generation.
    pub fn reconnect(
        exit_values: &[(String, ValueId)],
        variable_map: &mut BTreeMap<String, ValueId>,
    ) {
        let verbose = crate::config::env::joinir_dev_enabled();

        if verbose {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[normalized/exit-reconnect] Reconnecting {} exit values to variable_map",
                exit_values.len()
            ));
        }

        for (var_name, k_exit_param_vid) in exit_values {
            if verbose {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[normalized/exit-reconnect] Checking '{}' in variable_map",
                    var_name
                ));
            }

            // Phase 131 P1.5: variable_map MUST contain the carrier
            // (Normalized lowering guarantees this via AvailableInputsCollectorBox)
            if !variable_map.contains_key(var_name) {
                panic!(
                    "[ExitReconnectorBox] Carrier '{}' not in variable_map. \
                     This is a contract violation: Normalized lowering should only \
                     emit carriers that exist in host scope. \
                     Available carriers: {:?}",
                    var_name,
                    variable_map.keys().collect::<Vec<_>>()
                );
            }

            // Update variable_map: old host ValueId → k_exit param ValueId
            let old_vid = variable_map[var_name];
            variable_map.insert(var_name.clone(), *k_exit_param_vid);

            if verbose {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[normalized/exit-reconnect] Reconnected '{}': {:?} → {:?}",
                    var_name, old_vid, k_exit_param_vid
                ));
            }
        }

        if verbose {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[normalized/exit-reconnect] Reconnection complete. Updated {} carriers",
                exit_values.len()
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::ValueId;
    use std::collections::BTreeMap;

    #[test]
    fn test_reconnect_single_carrier() {
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(10));

        let exit_values = vec![("i".to_string(), ValueId(100))];

        ExitReconnectorBox::reconnect(&exit_values, &mut variable_map);

        assert_eq!(variable_map.get("i"), Some(&ValueId(100)));
    }

    #[test]
    fn test_reconnect_multiple_carriers() {
        let mut variable_map = BTreeMap::new();
        variable_map.insert("sum".to_string(), ValueId(20));
        variable_map.insert("count".to_string(), ValueId(30));

        let exit_values = vec![
            ("sum".to_string(), ValueId(200)),
            ("count".to_string(), ValueId(300)),
        ];

        ExitReconnectorBox::reconnect(&exit_values, &mut variable_map);

        assert_eq!(variable_map.get("sum"), Some(&ValueId(200)));
        assert_eq!(variable_map.get("count"), Some(&ValueId(300)));
    }

    #[test]
    #[should_panic(expected = "Carrier 'x' not in variable_map")]
    fn test_reconnect_missing_carrier_panics() {
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(10));

        let exit_values = vec![("x".to_string(), ValueId(999))];

        // This should panic because "x" is not in variable_map
        ExitReconnectorBox::reconnect(&exit_values, &mut variable_map);
    }

    #[test]
    fn test_reconnect_empty_exit_values() {
        let mut variable_map = BTreeMap::new();
        variable_map.insert("i".to_string(), ValueId(10));

        let exit_values = vec![];

        ExitReconnectorBox::reconnect(&exit_values, &mut variable_map);

        // variable_map should be unchanged
        assert_eq!(variable_map.get("i"), Some(&ValueId(10)));
    }
}
