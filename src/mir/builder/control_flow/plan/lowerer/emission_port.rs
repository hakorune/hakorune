//! One stack-scoped CorePlan effect-emission authority.
//!
//! Raw lowering keeps its historical effect payload exactly.  A located
//! execution bundle supplies the second mode: it consumes the exact prepared
//! source-site claim before emitting a selected canonical call.  The port is
//! deliberately borrowed through lowering recursion and is never Builder
//! state.

use super::PlanLowerer;
use crate::mir::builder::calls::CallTarget;
use crate::mir::builder::control_flow::plan::{CoreCallSourceV1, CoreEffectPlan};
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    CallableResultActivationDispositionV1, ClaimedCallableResultLoopBatchV1,
};
use crate::mir::{MirType, ValueId};

/// One lowering invocation owns exactly one effect-emission policy.
///
/// This type intentionally has no `Clone` implementation: a claim batch is
/// single-use, and recursive lowering only receives a mutable borrow.
#[derive(Debug)]
pub(in crate::mir::builder) enum CorePlanEffectEmissionPortV1<'plan> {
    Raw,
    Claimed(ClaimedCallableResultLoopBatchV1<'plan>),
}

impl CorePlanEffectEmissionPortV1<'_> {
    pub(super) const fn raw() -> Self {
        Self::Raw
    }

    pub(in crate::mir::builder) fn claimed<'plan>(
        claims: ClaimedCallableResultLoopBatchV1<'plan>,
    ) -> CorePlanEffectEmissionPortV1<'plan> {
        CorePlanEffectEmissionPortV1::Claimed(claims)
    }

    pub(super) fn emit_effect(
        &mut self,
        builder: &mut MirBuilder,
        effect: &CoreEffectPlan,
    ) -> Result<(), String> {
        match self {
            Self::Raw => PlanLowerer::emit_raw_effect(builder, effect),
            Self::Claimed(claims) => {
                let Some(source) = call_source(effect) else {
                    return PlanLowerer::emit_raw_effect(builder, effect);
                };
                let CoreCallSourceV1::LocatedMethodCall(site) = source else {
                    return PlanLowerer::emit_raw_effect(builder, effect);
                };
                let claim = claims.take_claim(site).map_err(|error| {
                    format!("[freeze:contract][callable_result/loop_claim] {error:?}")
                })?;
                if claim.site() != site {
                    return Err(format!(
                        "[freeze:contract][callable_result/loop_claim_site_mismatch] source={site:?} claim={:?}",
                        claim.site()
                    ));
                }
                match claim.disposition() {
                    CallableResultActivationDispositionV1::Unselected => {
                        PlanLowerer::emit_raw_effect(builder, effect)
                    }
                    CallableResultActivationDispositionV1::SelectedExactI64 {
                        target,
                        required_i64_arguments,
                    } => emit_selected_exact_i64(builder, effect, target, required_i64_arguments),
                }
            }
        }
    }

    pub(in crate::mir::builder) fn finish(self) -> Result<(), String> {
        match self {
            Self::Raw => Ok(()),
            Self::Claimed(claims) => claims.finish().map_err(|error| {
                format!("[freeze:contract][callable_result/loop_claim_finish] {error:?}")
            }),
        }
    }
}

fn call_source(effect: &CoreEffectPlan) -> Option<&CoreCallSourceV1> {
    match effect {
        CoreEffectPlan::MethodCall { source, .. }
        | CoreEffectPlan::GlobalCall { source, .. }
        | CoreEffectPlan::ValueCall { source, .. }
        | CoreEffectPlan::ExternCall { source, .. } => Some(source),
        _ => None,
    }
}

fn emit_selected_exact_i64(
    builder: &mut MirBuilder,
    effect: &CoreEffectPlan,
    target: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    required_i64_arguments: &[u32],
) -> Result<(), String> {
    let CoreEffectPlan::GlobalCall {
        dst: Some(dst),
        args,
        ..
    } = effect
    else {
        return Err(
            "[freeze:contract][callable_result/selected_call_shape] expected GlobalCall with destination"
                .to_string(),
        );
    };

    if args.len() != target.arity() as usize {
        return Err(format!(
            "[freeze:contract][callable_result/selected_call_arity] target={} expected={} actual={}",
            target.mir_symbol_projection(),
            target.arity(),
            args.len()
        ));
    }
    for ordinal in required_i64_arguments {
        let index = *ordinal as usize;
        let Some(argument) = args.get(index) else {
            return Err(format!(
                "[freeze:contract][callable_result/selected_call_required_argument] target={} ordinal={} arity={}",
                target.mir_symbol_projection(),
                ordinal,
                args.len()
            ));
        };
        if builder.function_state.type_ctx.value_types.get(argument) != Some(&MirType::Integer) {
            return Err(format!(
                "[freeze:contract][callable_result/selected_call_required_i64] target={} ordinal={} value=%{} actual={:?}",
                target.mir_symbol_projection(),
                ordinal,
                argument.0,
                builder.function_state.type_ctx.value_types.get(argument)
            ));
        }
    }

    let args: Vec<ValueId> = args
        .iter()
        .copied()
        .map(|value| builder.local_arg(value))
        .collect();
    builder.emit_unified_call(
        Some(*dst),
        CallTarget::Global(target.mir_symbol_projection()),
        args,
    )?;
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(*dst, MirType::Integer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::emit_selected_exact_i64;
    use crate::mir::builder::control_flow::plan::{CoreCallSourceV1, CoreEffectPlan};
    use crate::mir::callable_result_representation::{
        generic_selected_activation_fixture, CallableResultActivationDispositionV1,
    };
    use crate::mir::definitions::Callee;
    use crate::mir::{ConstValue, MirInstruction, MirType};

    fn generic_selected_target() -> (
        crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        Box<[u32]>,
    ) {
        let activation = generic_selected_activation_fixture::plan();
        let caller = generic_selected_activation_fixture::caller(&activation);
        activation
            .rows_for(&caller)
            .expect("generic caller rows")
            .iter()
            .find_map(|row| match row.disposition() {
                CallableResultActivationDispositionV1::SelectedExactI64 {
                    target,
                    required_i64_arguments,
                } => Some((target.clone(), required_i64_arguments.clone())),
                CallableResultActivationDispositionV1::Unselected => None,
            })
            .expect("generic fixture has one selected exact-i64 row")
    }

    #[test]
    fn selected_terminal_uses_claim_target_not_raw_global_spelling() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let (target, required_i64_arguments) = generic_selected_target();

        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.enter_function_for_test("selected_terminal".to_owned());
        let args = (0..target.arity())
            .map(|value| {
                let id = builder.alloc_value_for_test();
                builder
                    .emit_for_test(MirInstruction::Const {
                        dst: id,
                        value: ConstValue::Integer(value as i64),
                    })
                    .expect("argument const");
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(id, MirType::Integer);
                id
            })
            .collect::<Vec<_>>();
        let dst = builder.alloc_value_for_test();
        let effect = CoreEffectPlan::GlobalCall {
            dst: Some(dst),
            func: "forbidden.raw.target/999".to_owned(),
            args,
            source: CoreCallSourceV1::Unlocated,
        };

        emit_selected_exact_i64(&mut builder, &effect, &target, &required_i64_arguments)
            .expect("selected terminal");

        let function = builder
            .function_state
            .current_function
            .as_ref()
            .expect("function");
        let block = function
            .get_block(builder.function_state.current_block.expect("current block"))
            .expect("block");
        assert!(block.instructions.iter().any(|instruction| {
            matches!(instruction,
                MirInstruction::Call { callee: Some(Callee::Global(symbol)), .. }
                    if symbol == &target.mir_symbol_projection()
            )
        }));
        assert!(block.instructions.iter().all(|instruction| {
            !matches!(instruction,
                MirInstruction::Call { callee: Some(Callee::Global(symbol)), .. }
                    if symbol == "forbidden.raw.target/999"
            )
        }));
        assert_eq!(
            builder.function_state.type_ctx.value_types.get(&dst),
            Some(&MirType::Integer)
        );
    }

    #[test]
    fn selected_terminal_rejects_unknown_required_argument_before_call_or_result_publication() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let (target, required_i64_arguments) = generic_selected_target();
        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.enter_function_for_test("selected_terminal_unknown_argument".to_owned());
        let argument = builder.alloc_value_for_test();
        let dst = builder.alloc_value_for_test();
        let effect = CoreEffectPlan::GlobalCall {
            dst: Some(dst),
            func: "forbidden.raw.target/999".to_owned(),
            args: vec![argument],
            source: CoreCallSourceV1::Unlocated,
        };

        let error =
            emit_selected_exact_i64(&mut builder, &effect, &target, &required_i64_arguments)
                .expect_err("missing transient type must fail the selected terminal");
        assert!(error.contains("[freeze:contract][callable_result/selected_call_required_i64]"));

        let function = builder
            .function_state
            .current_function
            .as_ref()
            .expect("function");
        let block = function
            .get_block(builder.function_state.current_block.expect("current block"))
            .expect("block");
        assert!(block
            .instructions
            .iter()
            .all(|instruction| !matches!(instruction, MirInstruction::Call { .. })));
        assert_eq!(builder.function_state.type_ctx.value_types.get(&dst), None);
    }
}
