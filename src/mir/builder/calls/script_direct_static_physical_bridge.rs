//! Selected-normal Script direct-static physical consumer.
//!
//! This box is intentionally narrow: it consumes one already-claimed Join
//! row, reuses the existing ordered argument driver, requires the existing
//! generic Call receipt, publishes only ExactI64, and completes the claim
//! last.  It never resolves a target, writes Return/signature, or retries an
//! alternate route.

use super::super::normal_script_direct_static_physical_publication::PreparedScriptDirectStaticResultPublicationV1;
use super::super::normal_script_semantic_lowering_state::ScriptDirectStaticClaimedRowV1;
use super::super::recursive_child_lowering::RecursiveChildLoweringPortV1;
use super::super::{
    CanonicalSameModuleCallableKeyV1, MirBuilder, SameModuleCallableNamespaceV1, ValueId,
};
use super::method_call_descent::{
    AssociatedMethodCallArgumentsV1, MethodCallArgumentDescentV1, MethodCallDescentPortV1,
};
use super::method_call_terminal::emit_static_global_value_terminal_with_receipt_v1;

pub(super) fn lower_claimed_script_direct_static_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    input: &Port::MethodCallInput,
    claimed: ScriptDirectStaticClaimedRowV1,
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1 + RecursiveChildLoweringPortV1,
{
    let target = claimed.target().clone();
    validate_claimed_target_v1(&target, claimed.argument_sites().len())?;

    let mut descent = AssociatedMethodCallArgumentsV1::new(port, input);
    let argument_values = descent.lower_all(builder)?;
    if argument_values.len() != target.arity() as usize {
        return Err("[freeze:contract][script-direct-static/physical-arity]".to_owned());
    }

    let emission = emit_static_global_value_terminal_with_receipt_v1(
        builder,
        target.owner(),
        target.name(),
        target.arity(),
        argument_values,
    )
    .map_err(|error| format!("[freeze:contract][script-direct-static/call-receipt] {error:?}"))?;

    let publication =
        PreparedScriptDirectStaticResultPublicationV1::prepare(claimed.representation(), emission)?;
    let value = publication.commit(builder)?;
    port.complete_script_direct_static_claim_v1(claimed)?;
    Ok(value)
}

fn validate_claimed_target_v1(
    target: &CanonicalSameModuleCallableKeyV1,
    argument_count: usize,
) -> Result<(), String> {
    if target.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod
        || target.arity() as usize != argument_count
    {
        return Err("[freeze:contract][script-direct-static/physical-target]".to_owned());
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
}
