//! Convert OwnershipPlan to P2/P3 lowering inputs.
//!
//! Phase 58: P2 conversion helper
//! Phase 59: P3 conversion helper

use super::super::OwnershipPlan;
use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole, CarrierVar};

/// Result of converting OwnershipPlan for P2 lowering
#[derive(Debug)]
pub struct P2LoweringInputs {
    /// Carriers derived from owned_vars (is_written=true)
    pub carriers: Vec<CarrierVar>,
    /// Captured variables (read-only)
    pub captures: Vec<String>,
    /// Condition captures
    pub condition_captures: Vec<String>,
}

/// Result of converting OwnershipPlan for P3 (if-sum) lowering
#[derive(Debug)]
pub struct P3LoweringInputs {
    /// Carriers derived from owned_vars (is_written=true)
    pub carriers: Vec<CarrierVar>,
    /// Captured variables (read-only)
    pub captures: Vec<String>,
    /// Condition captures (used in if conditions)
    pub condition_captures: Vec<String>,
}

/// Convert OwnershipPlan to P2 lowering inputs.
///
/// # Errors
/// Returns Err if relay_writes is non-empty (Phase 58 scope limitation).
pub fn plan_to_p2_inputs(plan: &OwnershipPlan, loop_var: &str) -> Result<P2LoweringInputs, String> {
    // Fail-Fast: relay_writes not supported in Phase 58
    if !plan.relay_writes.is_empty() {
        return Err(format!(
            "Phase 58 limitation: relay_writes not yet supported. Found: {:?}",
            plan.relay_writes
                .iter()
                .map(|r| &r.name)
                .collect::<Vec<_>>()
        ));
    }

    let mut carriers = Vec::new();

    for var in &plan.owned_vars {
        // Skip loop variable (pinned, handled separately)
        if var.name == loop_var {
            continue;
        }

        // Only written vars become carriers
        if !var.is_written {
            continue;
        }

        let role = if var.is_condition_only {
            CarrierRole::ConditionOnly
        } else {
            CarrierRole::LoopState
        };

        carriers.push(CarrierVar {
            name: var.name.clone(),
            role,
            init: CarrierInit::FromHost,     // Default (Phase 228)
            host_id: crate::mir::ValueId(0), // Placeholder - not used in dev analysis
            join_id: None,
        });
    }

    let captures: Vec<String> = plan.captures.iter().map(|c| c.name.clone()).collect();

    let condition_captures: Vec<String> = plan
        .condition_captures
        .iter()
        .map(|c| c.name.clone())
        .collect();

    Ok(P2LoweringInputs {
        carriers,
        captures,
        condition_captures,
    })
}

/// Convert OwnershipPlan to P2 lowering inputs, allowing relay_writes (dev-only Phase 60+).
///
/// Phase 66: Multihop relay support with structural Fail-Fast guards.
///
/// Rules:
/// - carriers = owned_vars where is_written && name != loop_var
///            + relay_writes where name != loop_var
/// - relay_path.len() >= 1 required (loop relay always has at least 1 hop)
/// - relay_path[0] == plan.scope_id (this scope is the first hop)
/// - relay.owner_scope != plan.scope_id (relay is exclusive with owned)
/// - owned_vars and relay_writes cannot share names (invariant)
pub fn plan_to_p2_inputs_with_relay(
    plan: &OwnershipPlan,
    loop_var: &str,
) -> Result<P2LoweringInputs, String> {
    // Phase 66: Pre-validation - check for owned/relay name conflicts
    let owned_names: std::collections::BTreeSet<_> =
        plan.owned_vars.iter().map(|v| &v.name).collect();

    let mut carriers = Vec::new();

    for var in &plan.owned_vars {
        if var.name == loop_var || !var.is_written {
            continue;
        }

        let role = if var.is_condition_only {
            CarrierRole::ConditionOnly
        } else {
            CarrierRole::LoopState
        };

        carriers.push(CarrierVar {
            name: var.name.clone(),
            role,
            init: CarrierInit::FromHost,
            host_id: crate::mir::ValueId(0),
            join_id: None,
        });
    }

    for relay in &plan.relay_writes {
        if relay.name == loop_var {
            continue;
        }

        // Phase 66 Fail-Fast: owned_vars と relay_writes で同名は不正（不変条件違反）
        if owned_names.contains(&relay.name) {
            return Err(format!(
                "Invariant violation: '{}' appears in both owned_vars and relay_writes",
                relay.name
            ));
        }

        // Phase 66 Fail-Fast: relay_path.is_empty() は不正（loop relay は最低 1 hop）
        if relay.relay_path.is_empty() {
            return Err(format!(
                "Invariant violation: relay '{}' has empty relay_path (loop relay requires at least 1 hop)",
                relay.name
            ));
        }

        // Phase 66 Fail-Fast: relay_path[0] != plan.scope_id は不正（この plan の scope が最初の hop であるべき）
        if relay.relay_path[0] != plan.scope_id {
            return Err(format!(
                "Invariant violation: relay '{}' relay_path[0]={:?} != plan.scope_id={:?} (this scope must be first hop)",
                relay.name, relay.relay_path[0], plan.scope_id
            ));
        }

        // Phase 66 Fail-Fast: relay.owner_scope == plan.scope_id は不正（relay は owned と排他）
        if relay.owner_scope == plan.scope_id {
            return Err(format!(
                "Invariant violation: relay '{}' owner_scope={:?} == plan.scope_id (relay cannot be owned by same scope)",
                relay.name, relay.owner_scope
            ));
        }

        // Phase 66: Multihop accepted (relay_path.len() > 1 is OK now)
        carriers.push(CarrierVar {
            name: relay.name.clone(),
            role: CarrierRole::LoopState,
            init: CarrierInit::FromHost,
            host_id: crate::mir::ValueId(0),
            join_id: None,
        });
    }

    let captures: Vec<String> = plan.captures.iter().map(|c| c.name.clone()).collect();
    let condition_captures: Vec<String> = plan
        .condition_captures
        .iter()
        .map(|c| c.name.clone())
        .collect();

    Ok(P2LoweringInputs {
        carriers,
        captures,
        condition_captures,
    })
}

/// Convert OwnershipPlan to P3 (if-sum) lowering inputs.
///
/// P3 patterns have multiple carriers (sum, count, etc.) updated conditionally.
/// Logic is same as P2 - relay_writes are rejected.
///
/// # Errors
/// Returns Err if relay_writes is non-empty (Phase 59 scope limitation).
pub fn plan_to_p3_inputs(plan: &OwnershipPlan, loop_var: &str) -> Result<P3LoweringInputs, String> {
    // Fail-Fast: relay_writes not supported in Phase 59
    if !plan.relay_writes.is_empty() {
        return Err(format!(
            "Phase 59 limitation: relay_writes not yet supported for P3. Found: {:?}",
            plan.relay_writes
                .iter()
                .map(|r| &r.name)
                .collect::<Vec<_>>()
        ));
    }

    let mut carriers = Vec::new();

    for var in &plan.owned_vars {
        // Skip loop variable (pinned, handled separately)
        if var.name == loop_var {
            continue;
        }

        // Only written vars become carriers
        if !var.is_written {
            continue;
        }

        let role = if var.is_condition_only {
            CarrierRole::ConditionOnly
        } else {
            CarrierRole::LoopState
        };

        carriers.push(CarrierVar {
            name: var.name.clone(),
            role,
            init: CarrierInit::FromHost,     // Default (Phase 228)
            host_id: crate::mir::ValueId(0), // Placeholder - not used in dev analysis
            join_id: None,
        });
    }

    let captures: Vec<String> = plan.captures.iter().map(|c| c.name.clone()).collect();

    let condition_captures: Vec<String> = plan
        .condition_captures
        .iter()
        .map(|c| c.name.clone())
        .collect();

    Ok(P3LoweringInputs {
        carriers,
        captures,
        condition_captures,
    })
}

/// Convert OwnershipPlan to P3 lowering inputs, allowing relay_writes (dev-only Phase 60).
///
/// Rules are identical to `plan_to_p2_inputs_with_relay`, but output type is P3LoweringInputs.
pub fn plan_to_p3_inputs_with_relay(
    plan: &OwnershipPlan,
    loop_var: &str,
) -> Result<P3LoweringInputs, String> {
    let p2 = plan_to_p2_inputs_with_relay(plan, loop_var)?;
    Ok(P3LoweringInputs {
        carriers: p2.carriers,
        captures: p2.captures,
        condition_captures: p2.condition_captures,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::ownership::{RelayVar, ScopeId, ScopeOwnedVar};

    #[test]
    fn test_simple_p2_conversion() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(), // loop var
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(), // carrier
            is_written: true,
            is_condition_only: false,
        });

        let inputs = plan_to_p2_inputs(&plan, "i").unwrap();

        // i is skipped (loop var), sum becomes carrier
        assert_eq!(inputs.carriers.len(), 1);
        assert_eq!(inputs.carriers[0].name, "sum");
        assert_eq!(inputs.carriers[0].role, CarrierRole::LoopState);
    }

    #[test]
    fn test_condition_only_carrier() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(), // loop var
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "is_digit_pos".to_string(), // condition-only carrier
            is_written: true,
            is_condition_only: true,
        });

        let inputs = plan_to_p2_inputs(&plan, "i").unwrap();

        // i is skipped, is_digit_pos becomes ConditionOnly carrier
        assert_eq!(inputs.carriers.len(), 1);
        assert_eq!(inputs.carriers[0].name, "is_digit_pos");
        assert_eq!(inputs.carriers[0].role, CarrierRole::ConditionOnly);
    }

    #[test]
    fn test_relay_rejected() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.relay_writes.push(RelayVar {
            name: "outer_var".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![],
        });

        let result = plan_to_p2_inputs(&plan, "i");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("relay_writes not yet supported"));
    }

    #[test]
    fn test_relay_single_hop_accepted_in_with_relay() {
        // Phase 66: relay_path[0] must be plan.scope_id
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(1)], // Phase 66: must start with plan.scope_id
        });

        let inputs = plan_to_p2_inputs_with_relay(&plan, "i").expect("with_relay should accept");
        assert_eq!(inputs.carriers.len(), 1);
        assert_eq!(inputs.carriers[0].name, "sum");
        assert_eq!(inputs.carriers[0].role, CarrierRole::LoopState);
    }

    #[test]
    fn test_relay_multi_hop_accepted_in_with_relay() {
        // Phase 66: Multihop is now accepted!
        // plan.scope_id = ScopeId(1), relay_path = [ScopeId(1), ScopeId(2)]
        // This represents: L3 (this scope) → L2 → L1 (owner)
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.relay_writes.push(RelayVar {
            name: "counter".to_string(),
            owner_scope: ScopeId(0),                  // L1 owns
            relay_path: vec![ScopeId(1), ScopeId(2)], // L3 → L2 → L1
        });

        let inputs = plan_to_p2_inputs_with_relay(&plan, "i")
            .expect("Phase 66: multihop should be accepted");
        assert_eq!(inputs.carriers.len(), 1);
        assert_eq!(inputs.carriers[0].name, "counter");
        assert_eq!(inputs.carriers[0].role, CarrierRole::LoopState);
    }

    #[test]
    fn test_relay_path_empty_rejected_in_with_relay() {
        // Phase 66: empty relay_path is invalid (loop relay requires at least 1 hop)
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.relay_writes.push(RelayVar {
            name: "outer_var".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![], // Invalid: empty
        });

        let result = plan_to_p2_inputs_with_relay(&plan, "i");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty relay_path"));
    }

    #[test]
    fn test_relay_path_not_starting_at_plan_scope_rejected() {
        // Phase 66: relay_path[0] must be plan.scope_id
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.relay_writes.push(RelayVar {
            name: "outer_var".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(42)], // Invalid: doesn't start with ScopeId(1)
        });

        let result = plan_to_p2_inputs_with_relay(&plan, "i");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("relay_path[0]"));
    }

    #[test]
    fn test_relay_owner_same_as_plan_scope_rejected() {
        // Phase 66: relay.owner_scope != plan.scope_id
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.relay_writes.push(RelayVar {
            name: "var".to_string(),
            owner_scope: ScopeId(1), // Invalid: same as plan.scope_id
            relay_path: vec![ScopeId(1)],
        });

        let result = plan_to_p2_inputs_with_relay(&plan, "i");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("relay cannot be owned by same scope"));
    }

    #[test]
    fn test_owned_and_relay_same_name_rejected() {
        // Phase 66: owned_vars and relay_writes cannot share names
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(), // Invalid: same name as owned
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(1)],
        });

        let result = plan_to_p2_inputs_with_relay(&plan, "i");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("appears in both owned_vars and relay_writes"));
    }

    #[test]
    fn test_read_only_vars_not_carriers() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(), // loop var
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "limit".to_string(), // read-only owned
            is_written: false,
            is_condition_only: false,
        });

        let inputs = plan_to_p2_inputs(&plan, "i").unwrap();

        // Only i is written, and it's skipped (loop var), so no carriers
        assert_eq!(inputs.carriers.len(), 0);
    }

    #[test]
    fn test_captures_and_condition_captures() {
        use crate::mir::join_ir::ownership::CapturedVar;

        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(), // loop var
            is_written: true,
            is_condition_only: false,
        });

        // Captured variable (read-only)
        plan.captures.push(CapturedVar {
            name: "limit".to_string(),
            owner_scope: ScopeId(0),
        });

        // Condition capture
        plan.condition_captures.push(CapturedVar {
            name: "limit".to_string(),
            owner_scope: ScopeId(0),
        });

        let inputs = plan_to_p2_inputs(&plan, "i").unwrap();

        assert_eq!(inputs.captures.len(), 1);
        assert_eq!(inputs.captures[0], "limit");
        assert_eq!(inputs.condition_captures.len(), 1);
        assert_eq!(inputs.condition_captures[0], "limit");
    }

    // Phase 59: P3 conversion tests

    #[test]
    fn test_p3_multi_carrier_conversion() {
        // P3 if-sum pattern: sum, count, i all loop-local
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "count".to_string(),
            is_written: true,
            is_condition_only: false,
        });

        let inputs = plan_to_p3_inputs(&plan, "i").unwrap();

        // i skipped, sum and count become carriers
        assert_eq!(inputs.carriers.len(), 2);
        assert!(inputs.carriers.iter().any(|c| c.name == "sum"));
        assert!(inputs.carriers.iter().any(|c| c.name == "count"));
    }

    #[test]
    fn test_p3_five_plus_carriers() {
        // Selfhost P3 pattern: 5+ carriers
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "total".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "valid".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "error".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "warn".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "info".to_string(),
            is_written: true,
            is_condition_only: false,
        });

        let inputs = plan_to_p3_inputs(&plan, "i").unwrap();

        // 5 carriers (excluding loop var)
        assert_eq!(inputs.carriers.len(), 5);
    }

    #[test]
    fn test_p3_condition_only_role() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "is_valid".to_string(),
            is_written: true,
            is_condition_only: true, // Used only in if condition
        });
        plan.owned_vars.push(ScopeOwnedVar {
            name: "sum".to_string(),
            is_written: true,
            is_condition_only: false,
        });

        let inputs = plan_to_p3_inputs(&plan, "i").unwrap();

        let is_valid = inputs
            .carriers
            .iter()
            .find(|c| c.name == "is_valid")
            .unwrap();
        assert_eq!(is_valid.role, CarrierRole::ConditionOnly);

        let sum = inputs.carriers.iter().find(|c| c.name == "sum").unwrap();
        assert_eq!(sum.role, CarrierRole::LoopState);
    }

    #[test]
    fn test_p3_relay_rejected() {
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        // sum is written but owned by outer scope -> relay
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![],
        });

        let result = plan_to_p3_inputs(&plan, "i");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("relay_writes not yet supported for P3"));
    }

    #[test]
    fn test_p3_with_relay_accepts_single_hop() {
        // Phase 66: relay_path must start with plan.scope_id and be non-empty
        let mut plan = OwnershipPlan::new(ScopeId(1));
        plan.owned_vars.push(ScopeOwnedVar {
            name: "i".to_string(),
            is_written: true,
            is_condition_only: false,
        });
        plan.relay_writes.push(RelayVar {
            name: "sum".to_string(),
            owner_scope: ScopeId(0),
            relay_path: vec![ScopeId(1)], // Phase 66: must be non-empty and start with plan.scope_id
        });

        let inputs = plan_to_p3_inputs_with_relay(&plan, "i").expect("P3 with_relay should accept");
        assert_eq!(inputs.carriers.len(), 1);
        assert_eq!(inputs.carriers[0].name, "sum");
    }
}
