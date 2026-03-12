//! OwnershipPlan Validator Box
//!
//! Phase 71-Pre: Extracted from legacy IfPhiJoin lowering for reuse across route families.
//!
//! # Responsibility
//!
//! Validates OwnershipPlan against CarrierInfo and condition bindings.
//! This is **analysis-only** - no MIR generation or lowering.
//!
//! # Checks
//!
//! 1. **Relay support**: Multi-hop relay → Err with `[ownership/relay:runtime_unsupported]`
//! 2. **Carrier consistency**: Plan carriers vs existing carriers (warn-only)
//! 3. **Condition captures**: Plan captures vs condition bindings (warn-only)

use super::{OwnershipPlan, RelayVar};
use crate::mir::join_ir::lowering::carrier_info::CarrierInfo;
use crate::runtime::get_global_ring0;
use std::collections::BTreeSet;

/// Ownership Plan Validator
///
/// Provides reusable validation methods for OwnershipPlan consistency checks.
/// Used by `LoopBreak`, `IfPhiJoin`, and `LoopContinueOnly` lowering
/// to ensure plan integrity before execution.
pub struct OwnershipPlanValidator;

impl OwnershipPlanValidator {
    /// Validate relay support (Fail-Fast with structural detection)
    ///
    /// # Phase 70-B: Partial Multihop Support
    ///
    /// This method now distinguishes between:
    /// - **Supported multihop**: Simple passthrough patterns (relay_path is contiguous, no self-updates)
    /// - **Unsupported multihop**: Complex patterns requiring full implementation
    ///
    /// Returns Ok if:
    /// - Single-hop relay (relay_path.len() == 1)
    /// - Supported multihop pattern (relay_path.len() > 1, contiguous path, passthrough only)
    ///
    /// Returns Err with `[ownership/relay:runtime_unsupported]` if:
    /// - Unsupported multihop pattern detected
    ///
    /// # Supported Multihop Pattern (Phase 70-B)
    ///
    /// A multihop relay is supported if:
    /// 1. `relay_path` is non-empty and contiguous (each scope is immediate child of next)
    /// 2. Each intermediate scope is a pure passthrough (no self-updates to the relay variable)
    /// 3. Owner scope will perform final merge at exit PHI
    ///
    /// Example (3-layer loop, counter relay):
    /// ```text
    /// loop L1 {
    ///     local counter = 0    // owned by L1
    ///     loop L2 {
    ///         loop L3 {
    ///             counter++    // relay L3 → L2 → L1 (multihop)
    ///         }
    ///     }
    /// }
    /// ```
    /// - L3 plan: relay_path = [L3, L2], owner = L1 (2 hops)
    /// - L2 is pure passthrough (no self-update to counter)
    /// - L1 performs exit PHI merge
    ///
    /// Phase 70-A: Standardized runtime guard
    /// Phase 70-B: Relaxed to accept simple passthrough patterns
    pub fn validate_relay_support(plan: &OwnershipPlan) -> Result<(), String> {
        for relay in &plan.relay_writes {
            // Single-hop: always supported
            if relay.relay_path.len() <= 1 {
                continue;
            }

            // Multihop: check if it's a supported pattern
            if !Self::is_supported_multihop_pattern(plan, relay) {
                use crate::mir::join_ir::lowering::error_tags;
                return Err(error_tags::ownership_relay_unsupported(&format!(
                    "Multihop relay not executable yet: var='{}', owner={:?}, relay_path={:?}",
                    relay.name, relay.owner_scope, relay.relay_path
                )));
            }
        }
        Ok(())
    }

    /// Check if a multihop relay matches supported pattern (Phase 70-B)
    ///
    /// # Supported Pattern Criteria
    ///
    /// 1. **Contiguous path**: relay_path must be non-empty
    /// 2. **First hop is current scope**: relay_path[0] == plan.scope_id
    /// 3. **No self-conflict**: relay variable not in owned_vars (passthrough only)
    ///
    /// Note: Full contiguity check (parent-child relationship between scopes) would require
    /// scope tree metadata. For Phase 70-B, we rely on analyzer correctness (relay_path
    /// is already validated to be contiguous by analyzer).
    fn is_supported_multihop_pattern(plan: &OwnershipPlan, relay: &RelayVar) -> bool {
        // Check 1: relay_path must be non-empty (sanity check)
        if relay.relay_path.is_empty() {
            return false;
        }

        // Check 2: First hop must be current scope
        if relay.relay_path[0] != plan.scope_id {
            return false;
        }

        // Check 3: No self-conflict (passthrough only)
        // If relay var appears in owned_vars, this scope is trying to own AND relay
        // the same variable - not a pure passthrough
        let is_passthrough = !plan.owned_vars.iter().any(|v| v.name == relay.name);
        if !is_passthrough {
            return false;
        }

        // All checks passed - this is a supported multihop pattern
        true
    }

    /// Validate carrier set consistency (warn-only)
    ///
    /// Compares plan's owned_vars (is_written=true) against existing CarrierInfo.
    /// Warns on mismatch but does not fail (order SSOT not yet implemented).
    pub fn validate_carrier_consistency(
        plan: &OwnershipPlan,
        carrier_info: &CarrierInfo,
    ) -> Result<(), String> {
        let plan_carriers: BTreeSet<String> = plan
            .owned_vars
            .iter()
            .filter(|v| v.is_written)
            .map(|v| v.name.clone())
            .collect();

        let existing_carriers: BTreeSet<String> = carrier_info
            .carriers
            .iter()
            .map(|c| c.name.clone())
            .collect();

        if plan_carriers != existing_carriers {
            let ring0 = get_global_ring0();
            ring0
                .log
                .warn("[ownership/validator] Carrier set mismatch (warn-only):");
            ring0
                .log
                .warn(&format!("  OwnershipPlan carriers: {:?}", plan_carriers));
            ring0
                .log
                .warn(&format!("  Existing carriers: {:?}", existing_carriers));
        }

        Ok(())
    }

    /// Validate condition captures consistency (warn-only)
    ///
    /// Checks that plan's condition_captures are a subset of condition_bindings.
    /// Warns on extra captures but does not fail.
    pub fn validate_condition_captures(
        plan: &OwnershipPlan,
        condition_bindings: &BTreeSet<String>,
    ) -> Result<(), String> {
        let plan_cond_captures: BTreeSet<String> = plan
            .condition_captures
            .iter()
            .map(|c| c.name.clone())
            .collect();

        if !plan_cond_captures.is_subset(condition_bindings) {
            let extra: Vec<_> = plan_cond_captures.difference(condition_bindings).collect();
            get_global_ring0().log.warn(&format!(
                "[ownership/validator] Extra condition captures in plan (warn-only): {:?}",
                extra
            ));
        }

        Ok(())
    }

    /// Validate all checks (fail-fast on any Err)
    ///
    /// Runs all validation checks in order:
    /// 1. validate_relay_support (Fail-Fast)
    /// 2. validate_carrier_consistency (warn-only)
    /// 3. validate_condition_captures (warn-only)
    pub fn validate_all(
        plan: &OwnershipPlan,
        carrier_info: &CarrierInfo,
        condition_bindings: &BTreeSet<String>,
    ) -> Result<(), String> {
        Self::validate_relay_support(plan)?;
        Self::validate_carrier_consistency(plan, carrier_info)?;
        Self::validate_condition_captures(plan, condition_bindings)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};
    use crate::mir::join_ir::ownership::{CapturedVar, RelayVar, ScopeId, ScopeOwnedVar};
    use crate::mir::ValueId;

    #[test]
    fn test_validate_relay_support_single_hop_ok() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(1)], // Single hop
        });

        let result = OwnershipPlanValidator::validate_relay_support(&plan);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_relay_support_multihop_passthrough_accepted() {
        // Phase 70-B: Multihop relay with passthrough pattern should be accepted
        let mut plan = OwnershipPlan::new(ScopeId(2));
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(2), ScopeId(1)], // Multi-hop: L2 → L1 → L0
        });
        // No owned_vars for 'sum' - pure passthrough

        let result = OwnershipPlanValidator::validate_relay_support(&plan);
        assert!(
            result.is_ok(),
            "Phase 70-B: Passthrough multihop should be accepted, got: {:?}",
            result
        );
    }

    #[test]
    fn test_validate_relay_support_multihop_self_conflict_rejected() {
        // Phase 70-B: Multihop relay with self-conflict (owned + relay) should be rejected
        let mut plan = OwnershipPlan::new(ScopeId(2));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(2), ScopeId(1)], // Multi-hop
        });

        let result = OwnershipPlanValidator::validate_relay_support(&plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("[ownership/relay:runtime_unsupported]"),
            "Error should contain standard tag: {}",
            err
        );
    }

    #[test]
    fn test_validate_relay_support_multihop_wrong_first_hop_rejected() {
        // Phase 70-B: Multihop relay where first hop != plan.scope_id should be rejected
        let mut plan = OwnershipPlan::new(ScopeId(2));
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(3), ScopeId(1)], // Wrong first hop
        });

        let result = OwnershipPlanValidator::validate_relay_support(&plan);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("[ownership/relay:runtime_unsupported]"),
            "Error should contain standard tag: {}",
            err
        );
    }

    #[test]
    fn test_validate_all_with_consistent_data() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.condition_captures.push(CapturedVar {
            name: "limit".to_string(),
            owner_scope: ScopeId(0),
        });

        let carrier_info = CarrierInfo {
            loop_var_name: "i".to_string(),
            loop_var_id: ValueId(0),
            carriers: vec![CarrierVar::with_role_and_init(
                "sum".to_string(),
                ValueId(1),
                CarrierRole::LoopState,
                CarrierInit::FromHost,
            )],
            trim_helper: None,
            promoted_body_locals: vec![],
        };

        let condition_bindings: BTreeSet<String> =
            ["limit".to_string(), "i".to_string()].into_iter().collect();

        let result =
            OwnershipPlanValidator::validate_all(&plan, &carrier_info, &condition_bindings);
        assert!(result.is_ok());
    }
}
