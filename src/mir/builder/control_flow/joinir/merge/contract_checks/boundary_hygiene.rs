use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::builder::control_flow::joinir::merge::boundary_carrier_layout::BoundaryCarrierLayout;
use crate::mir::ValueId;
use std::collections::BTreeSet;

/// Phase 29af P1: Boundary hygiene contract checks (strict/dev only).
pub(in crate::mir::builder::control_flow::joinir::merge) fn verify_boundary_hygiene(
    boundary: &JoinInlineBoundary,
) -> Result<(), String> {
    use crate::mir::join_ir::lowering::carrier_info::{CarrierInit, CarrierRole};
    use crate::mir::join_ir::lowering::error_tags;

    let strict = crate::config::env::joinir_strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    if !strict {
        return Ok(());
    }

    let mut exit_names = BTreeSet::new();
    for binding in &boundary.exit_bindings {
        if binding.role != CarrierRole::LoopState {
            return Err(error_tags::freeze_with_hint(
                "phase29af/boundary_hygiene/exit_binding_role",
                &format!(
                    "exit_binding '{}' role {:?} is not LoopState",
                    binding.carrier_name, binding.role
                ),
                "exclude ConditionOnly carriers from exit_bindings",
            ));
        }
        if !exit_names.insert(binding.carrier_name.clone()) {
            return Err(error_tags::freeze_with_hint(
                "phase29af/boundary_hygiene/exit_binding_duplicate",
                &format!("duplicate exit_binding carrier '{}'", binding.carrier_name),
                "deduplicate exit_bindings by carrier_name",
            ));
        }
    }

    if let Some(carrier_info) = &boundary.carrier_info {
        let expected_names: Vec<&str> = std::iter::once(carrier_info.loop_var_name.as_str())
            .chain(carrier_info.carriers.iter().map(|c| c.name.as_str()))
            .collect();
        let layout = BoundaryCarrierLayout::from_boundary(boundary);
        let layout_names = layout.ordered_names();
        if expected_names != layout_names {
            return Err(error_tags::freeze_with_hint(
                "phase29af/boundary_hygiene/layout_order",
                &format!(
                    "boundary carrier order {:?} does not match carrier_info order {:?}",
                    layout_names, expected_names
                ),
                "ensure boundary carrier order matches carrier_info",
            ));
        }

        let mut carrier_names = BTreeSet::new();
        carrier_names.insert(carrier_info.loop_var_name.clone());

        for carrier in &carrier_info.carriers {
            carrier_names.insert(carrier.name.clone());
            if carrier.init == CarrierInit::FromHost && carrier.host_id == ValueId(0) {
                return Err(error_tags::freeze_with_hint(
                    "phase29af/boundary_hygiene/fromhost_host_id",
                    &format!(
                        "carrier '{}' has FromHost init with host_id=ValueId(0)",
                        carrier.name
                    ),
                    "assign a valid host ValueId for FromHost carriers",
                ));
            }
        }

        for binding in &boundary.exit_bindings {
            if !carrier_names.contains(&binding.carrier_name) {
                return Err(error_tags::freeze_with_hint(
                    "phase29af/boundary_hygiene/exit_binding_unknown",
                    &format!(
                        "exit_binding '{}' not found in carrier_info",
                        binding.carrier_name
                    ),
                    "ensure exit_bindings carriers are declared in carrier_info",
                ));
            }
        }
    }

    Ok(())
}
