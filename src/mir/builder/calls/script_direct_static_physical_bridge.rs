//! Selected-normal Script direct-static physical consumer.
//!
//! This box is intentionally narrow: it consumes one already-claimed Join
//! row, reuses the existing ordered argument driver, requires the existing
//! generic Call receipt, publishes only ExactI64, and completes the claim
//! last.  It never resolves a target, writes Return/signature, or retries an
//! alternate route.

use super::super::normal_script_direct_static_physical_publication::{
    PreparedScriptDirectStaticResultPublicationV1, ScriptDirectStaticPublicationErrorV1,
};
use super::super::normal_script_semantic_lowering_state::ScriptDirectStaticClaimedRowV1;
use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::super::{
    CanonicalSameModuleCallableKeyV1, MirBuilder, SameModuleCallableNamespaceV1, ValueId,
};
use super::method_call_descent::{
    AssociatedMethodCallArgumentsV1, MethodCallArgumentDescentV1, MethodCallDescentPortV1,
};
use super::method_call_terminal::emit_static_global_value_terminal_with_receipt_v1;
use super::unified_emitter::UnifiedValueCallReceiptErrorV1;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ScriptDirectStaticPhysicalBridgeErrorV1 {
    TargetMismatch,
    RequiredArgumentProof(String),
    ArgumentDescent(String),
    PhysicalArity { expected: u32, actual: usize },
    CallReceipt(UnifiedValueCallReceiptErrorV1),
    Publication(ScriptDirectStaticPublicationErrorV1),
    ClaimCompletion(String),
}

impl std::fmt::Display for ScriptDirectStaticPhysicalBridgeErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TargetMismatch => {
                write!(formatter, "[freeze:contract][script-direct-static/physical-target]")
            }
            Self::RequiredArgumentProof(detail) => write!(
                formatter,
                "[freeze:contract][script-direct-static/required-argument-proof] {detail}"
            ),
            Self::ArgumentDescent(detail) => write!(
                formatter,
                "[freeze:contract][script-direct-static/argument] {detail}"
            ),
            Self::PhysicalArity { expected, actual } => write!(
                formatter,
                "[freeze:contract][script-direct-static/physical-arity] expected={expected} actual={actual}"
            ),
            Self::CallReceipt(error) => write!(
                formatter,
                "[freeze:contract][script-direct-static/call-receipt] {error:?}"
            ),
            Self::Publication(error) => write!(
                formatter,
                "[freeze:contract][script-direct-static/publication] {error}"
            ),
            Self::ClaimCompletion(detail) => write!(
                formatter,
                "[freeze:contract][script-direct-static/claim-completion] {detail}"
            ),
        }
    }
}

pub(super) fn lower_claimed_script_direct_static_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::MethodCallInput,
    mut claimed: ScriptDirectStaticClaimedRowV1,
) -> Result<ValueId, ScriptDirectStaticPhysicalBridgeErrorV1>
where
    Port: MethodCallDescentPortV1 + RecursiveChildLoweringPortV1,
{
    let target = claimed.target().clone();
    validate_claimed_target_v1(&target, claimed.argument_sites().len())?;
    claimed.consume_required_argument_proof().map_err(|error| {
        ScriptDirectStaticPhysicalBridgeErrorV1::RequiredArgumentProof(error.to_owned())
    })?;

    let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
    let argument_values = descent
        .lower_all(builder)
        .map_err(ScriptDirectStaticPhysicalBridgeErrorV1::ArgumentDescent)?;
    if argument_values.len() != target.arity() as usize {
        return Err(ScriptDirectStaticPhysicalBridgeErrorV1::PhysicalArity {
            expected: target.arity(),
            actual: argument_values.len(),
        });
    }

    let emission = emit_static_global_value_terminal_with_receipt_v1(
        builder,
        target.owner(),
        target.name(),
        target.arity(),
        argument_values,
    )
    .map_err(ScriptDirectStaticPhysicalBridgeErrorV1::CallReceipt)?;

    let publication = PreparedScriptDirectStaticResultPublicationV1::prepare(
        claimed.representation(),
        emission,
    )
    .map_err(ScriptDirectStaticPhysicalBridgeErrorV1::Publication)?;
    let value = publication
        .commit(builder)
        .map_err(ScriptDirectStaticPhysicalBridgeErrorV1::Publication)?;
    port.complete_script_direct_static_claim_v1(claimed)
        .map_err(ScriptDirectStaticPhysicalBridgeErrorV1::ClaimCompletion)?;
    Ok(value)
}

fn validate_claimed_target_v1(
    target: &CanonicalSameModuleCallableKeyV1,
    argument_count: usize,
) -> Result<(), ScriptDirectStaticPhysicalBridgeErrorV1> {
    if target.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod
        || target.arity() as usize != argument_count
    {
        return Err(ScriptDirectStaticPhysicalBridgeErrorV1::TargetMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physical_bridge_accepts_only_static_target_with_exact_arity() {
        let target = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "run", 2);
        assert!(validate_claimed_target_v1(&target, 2).is_ok());
        assert!(validate_claimed_target_v1(&target, 1).is_err());
    }

    #[test]
    fn physical_bridge_rejects_instance_target_before_descent() {
        let target =
            CanonicalSameModuleCallableKeyV1::test_instance_box_method("Helpers", "run", 1);
        assert!(validate_claimed_target_v1(&target, 1).is_err());
    }

    #[test]
    fn bridge_keeps_receipt_variant_until_outer_display() {
        let error = ScriptDirectStaticPhysicalBridgeErrorV1::CallReceipt(
            UnifiedValueCallReceiptErrorV1::UnifiedDisabled,
        );
        assert!(error.to_string().contains("UnifiedDisabled"));
        assert!(matches!(
            error,
            ScriptDirectStaticPhysicalBridgeErrorV1::CallReceipt(
                UnifiedValueCallReceiptErrorV1::UnifiedDisabled
            )
        ));
    }
}
