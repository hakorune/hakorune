//! Physical consumer for one selected static-result publication handoff.
//!
//! This bridge is deliberately source-neutral after the handoff is claimed:
//! the canonical target comes from the handoff, ordered argument descent is
//! reused, and the existing unified Call receipt plus publication owner do
//! the only physical effects.

use super::super::{MirBuilder, ValueId};
use super::method_call_descent::{
    AssociatedMethodCallArgumentsV1, MethodCallArgumentDescentV1, MethodCallDescentPortV1,
};
use super::method_call_terminal::{
    emit_static_global_target_value_terminal_v1, emit_static_global_value_terminal_with_receipt_v1,
};
use super::static_result_publication::PreparedStaticCallResultPublicationV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;

pub(in crate::mir::builder) fn lower_selected_static_result_publication_v1<Port>(
    builder: &mut MirBuilder,
    descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    handoff: VerifiedStaticCallResultPublicationHandoffV1,
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1,
{
    let (demand, _required_i64_arguments) = handoff.consume();
    let target_key = demand.target().clone();
    let target = target_key.canonical_global_target_v1().map_err(|error| {
        format!("[freeze:contract][static-result-bridge/target-projection] {error}")
    })?;
    let argument_values = descent.lower_all(builder)?;
    if argument_values.len() != target_key.arity() as usize {
        return Err("[freeze:contract][static-result-bridge/physical-arity]".to_owned());
    }
    let emission =
        emit_static_global_value_terminal_with_receipt_v1(builder, target, argument_values)
            .map_err(|error| {
                format!("[freeze:contract][static-result-bridge/call-receipt] {error:?}")
            })?;
    let publication = PreparedStaticCallResultPublicationV1::prepare(demand, emission);
    let destination = publication.destination();
    publication.commit(builder)?;
    Ok(destination)
}

pub(in crate::mir::builder) fn lower_target_only_static_result_publication_v1<Port>(
    builder: &mut MirBuilder,
    descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    target_key: CanonicalSameModuleCallableKeyV1,
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1,
{
    let expected_arity = target_key.arity() as usize;
    let target = target_key.canonical_global_target_v1().map_err(|error| {
        format!("[freeze:contract][static-target-only/target-projection] {error}")
    })?;
    let argument_values = descent.lower_all(builder)?;
    if argument_values.len() != expected_arity {
        return Err(format!(
            "[freeze:contract][static-target-only/physical-arity] expected {}, got {}",
            expected_arity,
            argument_values.len()
        ));
    }
    emit_static_global_target_value_terminal_v1(builder, target, argument_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_module_accepts_only_the_sealed_handoff_type() {
        let _ = std::any::type_name::<VerifiedStaticCallResultPublicationHandoffV1>();
        assert!(std::any::type_name::<ValueId>().contains("ValueId"));
    }
}
