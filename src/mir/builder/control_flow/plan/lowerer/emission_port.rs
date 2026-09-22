//! One stack-scoped CorePlan effect-emission authority.
//!
//! Raw lowering keeps its historical effect payload exactly.  A located
//! execution bundle supplies the second mode: it consumes the exact prepared
//! source-site claim before emitting a selected canonical call.  The port is
//! deliberately borrowed through lowering recursion and is never Builder
//! state.

use super::PlanLowerer;
use crate::mir::builder::calls::lower_selected_static_result_publication_with_arguments_and_destination_v1;
use crate::mir::builder::calls::CallTarget;
use crate::mir::builder::control_flow::plan::{CoreCallSourceV1, CoreEffectPlan};
use crate::mir::builder::MirBuilder;
use crate::mir::callable_result_representation::{
    CallableResultActivationDispositionV1, ClaimedCallableResultLoopBatchV1,
};
use crate::mir::{MirType, ValueId};
use std::cell::RefCell;
use std::rc::Rc;

/// One lowering invocation owns exactly one effect-emission policy.
///
/// This type intentionally has no `Clone` implementation: a claim batch is
/// single-use, and recursive lowering only receives a mutable borrow.
#[derive(Debug)]
pub(in crate::mir::builder) enum CorePlanEffectEmissionPortV1<'plan> {
    Raw,
    Claimed(ClaimedCallableResultLoopBatchV1<'plan>),
    SourcePublication(
        Rc<RefCell<crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState>>,
    ),
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

    pub(in crate::mir::builder) fn source_publication(
        ledger: Rc<RefCell<crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState>>,
    ) -> Self {
        Self::SourcePublication(ledger)
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
            Self::SourcePublication(ledger) => {
                let Some(source) = call_source(effect) else {
                    return PlanLowerer::emit_raw_effect(builder, effect);
                };
                let CoreCallSourceV1::LocatedMethodCall(site) = source else {
                    return PlanLowerer::emit_raw_effect(builder, effect);
                };
                let Some(handoff) = ledger
                    .borrow_mut()
                    .take_source_static_result_publication(site)?
                else {
                    return PlanLowerer::emit_raw_effect(builder, effect);
                };
                let CoreEffectPlan::GlobalCall {
                    dst: Some(dst),
                    func,
                    args,
                    ..
                } = effect
                else {
                    return Err(
                        "[freeze:contract][callable-loop/static-publication/effect-shape]"
                            .to_owned(),
                    );
                };
                if handoff.target().mir_symbol_projection() != *func {
                    return Err(
                        "[freeze:contract][callable-loop/static-publication/effect-target]"
                            .to_owned(),
                    );
                }
                let args = args
                    .iter()
                    .copied()
                    .map(|value| builder.local_arg(value))
                    .collect();
                let emitted =
                    lower_selected_static_result_publication_with_arguments_and_destination_v1(
                        builder, handoff, args, *dst,
                    )?;
                if emitted != *dst {
                    return Err(
                        "[freeze:contract][callable-loop/static-publication/destination]"
                            .to_owned(),
                    );
                }
                Ok(())
            }
        }
    }

    pub(in crate::mir::builder) fn finish(self) -> Result<(), String> {
        match self {
            Self::Raw => Ok(()),
            Self::Claimed(claims) => claims.finish().map_err(|error| {
                format!("[freeze:contract][callable_result/loop_claim_finish] {error:?}")
            }),
            Self::SourcePublication(ledger) => {
                if ledger
                    .borrow()
                    .has_pending_source_static_result_publications()
                {
                    Err("[freeze:contract][callable-loop/static-publication/residual]".to_owned())
                } else {
                    Ok(())
                }
            }
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
        CallTarget::Global(
            target
                .canonical_global_target_v1()
                .map_err(|error| format!("[freeze:contract][callable_result/global/{error}]"))?,
        ),
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
    use super::CorePlanEffectEmissionPortV1;
    use crate::mir::builder::control_flow::plan::{CoreCallSourceV1, CoreEffectPlan};
    use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
    use crate::mir::callable_result_representation::{
        generic_selected_activation_fixture, CallableResultActivationDispositionV1,
        VerifiedStaticCallResultPublicationDemandV1, VerifiedStaticCallResultPublicationHandoffV1,
    };
    use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
    use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
    use crate::mir::definitions::Callee;
    use crate::mir::resolved_semantics::{
        CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
        ResolveSelectedCallableForestsOutcomeV1, SourceExprSiteV1, SourceNodeSiteV1,
        SourcePathSegmentV1,
    };
    use crate::mir::{ConstValue, MirInstruction, MirType};
    use crate::parser::NyashParser;
    use std::cell::RefCell;
    use std::rc::Rc;

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
        assert!(block
            .instructions
            .iter()
            .any(|instruction| match instruction {
                MirInstruction::Call(call) => {
                    matches!(&call.callee, Callee::Global(symbol)
                    if symbol.display_name() == target.mir_symbol_projection())
                }
                MirInstruction::LegacyCallV0 {
                    callee: Some(Callee::Global(symbol)),
                    ..
                } => symbol.display_name() == target.mir_symbol_projection(),
                _ => false,
            }));
        assert!(block
            .instructions
            .iter()
            .all(|instruction| match instruction {
                MirInstruction::Call(call) => {
                    !matches!(&call.callee, Callee::Global(symbol)
                    if symbol.display_name() == "forbidden.raw.target/999")
                }
                MirInstruction::LegacyCallV0 {
                    callee: Some(Callee::Global(symbol)),
                    ..
                } => symbol.display_name() != "forbidden.raw.target/999",
                _ => true,
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
            .all(|instruction| !matches!(instruction, MirInstruction::LegacyCallV0 { .. })));
        assert_eq!(builder.function_state.type_ctx.value_types.get(&dst), None);
    }

    fn source_publication_fixture(
        consume_source_call: bool,
    ) -> (
        Rc<RefCell<CallableSemanticLoweringState>>,
        SourceExprSiteV1,
        crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    ) {
        let program = NyashParser::parse_from_string("function caller() { return 1 }")
            .expect("publication fixture parses");
        let crate::ast::ASTNode::Program { mut statements, .. } = program else {
            panic!("publication fixture must be a program")
        };
        let function = statements.remove(0);
        let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function)
            .expect("publication fixture callable syntax");
        let mut resolver = FunctionSemanticResolverSessionV1::new(9114).expect("resolver");
        let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
            .resolve_selected_callable_forests(&[syntax.function()])
            .expect("publication fixture forest")
        else {
            panic!("publication fixture unexpectedly deferred")
        };
        let forest = forests
            .into_vec()
            .pop()
            .expect("publication fixture root forest");
        let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
            &function,
            &forest,
            syntax.function().root_profile(),
        )
        .expect("publication fixture projection");
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            &function,
            &forest,
            &projection,
        )
        .expect("publication fixture input");
        let mut state = CallableSemanticLoweringState::from_exact_source(input)
            .expect("publication fixture lowering state");
        let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
        ]));
        let caller = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserProgramBox",
            "parse",
            2,
        );
        let target = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserStringUtilsBox",
            "starts_with",
            3,
        );
        let demand = VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
            caller,
            site.clone(),
            target.clone(),
        );
        let handoff = VerifiedStaticCallResultPublicationHandoffV1::from_test_parts(7, demand, &[]);
        state
            .install_source_static_result_publication(&site, handoff)
            .expect("install selected publication");
        if consume_source_call {
            state
                .take_source_core_method_call(&site, "starts_with", 3)
                .expect("source call take")
                .expect("selected publication source call");
        }
        (Rc::new(RefCell::new(state)), site, target)
    }

    fn source_publication_effect(
        builder: &mut crate::mir::builder::MirBuilder,
        site: SourceExprSiteV1,
        target: &crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    ) -> CoreEffectPlan {
        let args = (0..target.arity())
            .map(|value| {
                let id = builder.alloc_value_for_test();
                builder
                    .emit_for_test(MirInstruction::Const {
                        dst: id,
                        value: ConstValue::Integer(value as i64),
                    })
                    .expect("publication argument const");
                builder
                    .function_state
                    .type_ctx
                    .set_type(id, MirType::Integer);
                id
            })
            .collect();
        let destination = builder.alloc_value_for_test();
        builder
            .function_state
            .type_ctx
            .set_type(destination, MirType::Integer);
        CoreEffectPlan::GlobalCall {
            dst: Some(destination),
            func: target.mir_symbol_projection(),
            args,
            source: CoreCallSourceV1::LocatedMethodCall(site),
        }
    }

    #[test]
    fn source_publication_consumer_takes_once_and_finishes_without_residual() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let (ledger, site, target) = source_publication_fixture(true);
        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.enter_function_for_test("source_publication_consumer".to_owned());
        let effect = source_publication_effect(&mut builder, site.clone(), &target);
        let mut port = CorePlanEffectEmissionPortV1::source_publication(Rc::clone(&ledger));

        port.emit_effect(&mut builder, &effect)
            .expect("selected source publication");
        let duplicate = port
            .emit_effect(&mut builder, &effect)
            .expect_err("duplicate source publication take must freeze");
        assert!(duplicate.contains("missing-static-publication-handoff"));
        port.finish()
            .expect("selected publication is residual-free");
        assert!(!ledger
            .borrow()
            .has_pending_source_static_result_publications());
    }

    #[test]
    fn source_publication_consumer_rejects_unconsumed_residual() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let (ledger, _site, _target) = source_publication_fixture(false);
        let mut builder = crate::mir::builder::MirBuilder::new();
        builder.enter_function_for_test("source_publication_residual".to_owned());
        let port = CorePlanEffectEmissionPortV1::source_publication(Rc::clone(&ledger));

        let error = port
            .finish()
            .expect_err("unconsumed source publication must remain visible");
        assert!(error.contains("static-publication/residual"));
        assert!(ledger
            .borrow()
            .has_pending_source_static_result_publications());
        let _ = builder;
    }
}
