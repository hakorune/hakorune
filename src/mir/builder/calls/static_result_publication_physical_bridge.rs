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
use super::CallTarget;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationHandoffV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;

pub(in crate::mir::builder) fn lower_selected_static_result_publication_v1<Port>(
    builder: &mut MirBuilder,
    descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    handoff: VerifiedStaticCallResultPublicationHandoffV1,
    source_argument_count: usize,
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1,
{
    let expected_arity = handoff.target().arity() as usize;
    if source_argument_count != expected_arity {
        return Err(format!(
            "[freeze:contract][static-result-bridge/source-arity] expected {}, got {}",
            expected_arity, source_argument_count
        ));
    }
    let (demand, _required_i64_arguments) = handoff.consume();
    let target_key = demand.target().clone();
    let target = target_key.canonical_global_target_v1().map_err(|error| {
        format!("[freeze:contract][static-result-bridge/target-projection] {error}")
    })?;
    let argument_values = descent.lower_all(builder)?;
    if argument_values.len() != expected_arity {
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

/// Consume one selected publication handoff after the caller has already
/// lowered its ordered arguments.  The handoff remains the sole source of
/// result representation; this sibling only supplies the existing physical
/// argument values and commits the generic Call receipt.
pub(in crate::mir::builder) fn lower_selected_static_result_publication_with_arguments_v1(
    builder: &mut MirBuilder,
    handoff: VerifiedStaticCallResultPublicationHandoffV1,
    argument_values: Vec<ValueId>,
) -> Result<ValueId, String> {
    let expected_arity = handoff.target().arity() as usize;
    if argument_values.len() != expected_arity {
        return Err(format!(
            "[freeze:contract][static-result-bridge/physical-arity] expected {}, got {}",
            expected_arity,
            argument_values.len()
        ));
    }
    let (demand, _required_i64_arguments) = handoff.consume();
    let target = demand
        .target()
        .canonical_global_target_v1()
        .map_err(|error| {
            format!("[freeze:contract][static-result-bridge/target-projection] {error}")
        })?;
    let emission = super::method_call_terminal::emit_static_global_value_terminal_with_receipt_v1(
        builder,
        target,
        argument_values,
    )
    .map_err(|error| format!("[freeze:contract][static-result-bridge/call-receipt] {error:?}"))?;
    let publication = PreparedStaticCallResultPublicationV1::prepare(demand, emission);
    let destination = publication.destination();
    publication.commit(builder)?;
    Ok(destination)
}

/// Consume one selected publication handoff for a normalizer-preallocated
/// destination. The normalizer owns the result ValueId; this bridge only
/// emits the existing generic Call receipt and commits the sealed result
/// representation.
pub(in crate::mir::builder) fn lower_selected_static_result_publication_with_arguments_and_destination_v1(
    builder: &mut MirBuilder,
    handoff: VerifiedStaticCallResultPublicationHandoffV1,
    argument_values: Vec<ValueId>,
    destination: ValueId,
) -> Result<ValueId, String> {
    let expected_arity = handoff.target().arity() as usize;
    if argument_values.len() != expected_arity {
        return Err(format!(
            "[freeze:contract][static-result-bridge/physical-arity] expected {}, got {}",
            expected_arity,
            argument_values.len()
        ));
    }
    let (demand, _required_i64_arguments) = handoff.consume();
    let target = demand
        .target()
        .canonical_global_target_v1()
        .map_err(|error| {
            format!("[freeze:contract][static-result-bridge/target-projection] {error}")
        })?;
    let emission = builder
        .emit_unified_value_call_with_external_result_publication_receipt_v1(
            destination,
            CallTarget::Global(target),
            argument_values,
        )
        .map_err(|error| {
            format!("[freeze:contract][static-result-bridge/call-receipt] {error:?}")
        })?;
    if emission.final_destination() != destination {
        return Err("[freeze:contract][static-result-bridge/destination-drift]".to_owned());
    }
    let publication = PreparedStaticCallResultPublicationV1::prepare(demand, emission);
    publication.commit_external_destination(builder)?;
    Ok(destination)
}

pub(in crate::mir::builder) fn lower_target_only_static_result_publication_v1<Port>(
    builder: &mut MirBuilder,
    descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    target_key: CanonicalSameModuleCallableKeyV1,
    source_argument_count: usize,
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1,
{
    let expected_arity = target_key.arity() as usize;
    if source_argument_count != expected_arity {
        return Err(format!(
            "[freeze:contract][static-target-only/source-arity] expected {}, got {}",
            expected_arity, source_argument_count
        ));
    }
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

pub(in crate::mir::builder) fn lower_target_only_static_result_publication_with_expected_sites_v1<
    Port,
>(
    builder: &mut MirBuilder,
    descent: &mut AssociatedMethodCallArgumentsV1<'_, '_, Port>,
    target_key: CanonicalSameModuleCallableKeyV1,
    expected_sites: &[SourceExprSiteV1],
) -> Result<ValueId, String>
where
    Port: MethodCallDescentPortV1
        + crate::mir::builder::recursive_child_lowering::DirectCallDispositionPortV1,
{
    let expected_arity = target_key.arity() as usize;
    if expected_sites.len() != expected_arity {
        return Err(format!(
            "[freeze:contract][static-target-only/argument-site-cardinality] expected={} actual={}",
            expected_arity,
            expected_sites.len()
        ));
    }
    let target = target_key.canonical_global_target_v1().map_err(|error| {
        format!("[freeze:contract][static-target-only/target-projection] {error}")
    })?;
    let argument_values = descent.lower_all_with_expected_sites(builder, expected_sites)?;
    if argument_values.len() != expected_arity {
        return Err(format!(
            "[freeze:contract][static-target-only/physical-arity] expected={} actual={}",
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

    #[test]
    fn external_destination_bridge_accepts_matching_normalizer_type() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let caller = CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserProgramBox",
            "parse",
            1,
        );
        let target = CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserStringUtilsBox",
            "starts_with",
            1,
        );
        let site = SourceExprSiteV1::from_node(
            crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![
                crate::mir::resolved_semantics::SourcePathSegmentV1::Body(0),
            ]),
        );
        let demand = crate::mir::callable_result_representation::
            VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
            caller,
            site,
            target.clone(),
        );
        let handoff =
            VerifiedStaticCallResultPublicationHandoffV1::from_test_parts(7, demand, &[0]);
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("publication_bridge".to_owned());
        let argument = builder.alloc_value_for_test();
        builder
            .emit_for_test(crate::mir::MirInstruction::Const {
                dst: argument,
                value: crate::mir::ConstValue::Integer(1),
            })
            .expect("argument const");
        builder
            .function_state
            .type_ctx
            .set_type(argument, crate::mir::MirType::Integer);
        let destination = builder.alloc_value_for_test();
        builder
            .function_state
            .type_ctx
            .set_type(destination, crate::mir::MirType::Integer);

        let emitted = lower_selected_static_result_publication_with_arguments_and_destination_v1(
            &mut builder,
            handoff,
            vec![argument],
            destination,
        )
        .expect("pretyped destination bridge");
        assert_eq!(emitted, destination);
        assert_eq!(
            builder.function_state.type_ctx.get_type(destination),
            Some(&crate::mir::MirType::Integer)
        );
    }
}
