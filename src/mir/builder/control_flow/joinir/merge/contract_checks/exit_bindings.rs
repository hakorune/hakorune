use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Phase 118 P2: Contract check (Fail-Fast) - exit_bindings carriers must have exit PHI dsts.
///
/// This prevents latent "Carrier '<name>' not found in carrier_phis" failures later in
/// ExprResultResolver and ExitLine reconnection.
pub(in crate::mir::builder::control_flow::joinir::merge) fn verify_exit_bindings_have_exit_phis(
    boundary: &JoinInlineBoundary,
    exit_carrier_phis: &BTreeMap<String, ValueId>,
) -> Result<(), String> {
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
    use crate::mir::join_ir::lowering::error_tags;

    for binding in &boundary.exit_bindings {
        if binding.role == CarrierRole::ConditionOnly {
            continue;
        }
        if exit_carrier_phis.contains_key(&binding.carrier_name) {
            continue;
        }

        return Err(error_tags::freeze_with_hint(
            "phase118/exit_phi/missing_carrier_phi",
            &format!(
                "exit_bindings carrier '{}' is missing from exit_carrier_phis",
                binding.carrier_name
            ),
            "exit_bindings carriers must be included in exit_phi_builder inputs; check carrier_inputs derivation from jump_args",
        ));
    }

    Ok(())
}
