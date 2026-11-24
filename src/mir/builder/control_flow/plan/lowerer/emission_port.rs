//! One stack-scoped CorePlan effect-emission authority.
//!
//! Raw and source-publication lowering share one stack-scoped emission port.
//! The current publication ledger is borrowed through lowering recursion.

use super::PlanLowerer;
use crate::mir::builder::calls::lower_selected_static_result_publication_with_arguments_and_destination_v1;
use crate::mir::builder::control_flow::plan::{CoreCallSourceV1, CoreEffectPlan};
use crate::mir::builder::MirBuilder;
use std::cell::RefCell;
use std::rc::Rc;

/// One lowering invocation owns exactly one effect-emission policy.
///
/// Recursive lowering receives a mutable borrow of the same emission policy.
#[derive(Debug)]
pub(in crate::mir::builder) enum CorePlanEffectEmissionPortV1<'plan> {
    Raw,
    SourcePublication {
        ledger: Rc<RefCell<crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState>>,
        loop_completion: Option<&'plan super::loop_cond_after_role::LoopCondPhysicalCompletionV1<'plan>>,
    },
}

impl<'plan> CorePlanEffectEmissionPortV1<'plan> {
    pub(super) const fn raw() -> Self {
        Self::Raw
    }

    pub(in crate::mir::builder) fn source_publication(
        ledger: Rc<RefCell<crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState>>,
    ) -> Self {
        Self::SourcePublication { ledger, loop_completion: None }
    }

    pub(super) fn source_publication_with_loop_completion(
        ledger: Rc<RefCell<crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState>>,
        completion: &'plan super::loop_cond_after_role::LoopCondPhysicalCompletionV1<'plan>,
    ) -> Self { Self::SourcePublication { ledger, loop_completion: Some(completion) } }

    pub(super) fn prepare_source_loop_phis(&self, builder: &mut MirBuilder,
        plan: &crate::mir::builder::control_flow::plan::CoreLoopPlan,
    ) -> Result<Option<()>, String> {
        if let Self::SourcePublication { loop_completion: Some(completion), .. } = self {
            if let Some(session) = completion.matching_phi_session(plan)? {
                return session.verify_plan_adoption(builder, plan).map(Some);
            }
        }
        Ok(None)
    }
    pub(super) fn completed_loop(&self, plan: &crate::mir::builder::control_flow::plan::CoreLoopPlan,
        function: &crate::mir::MirFunction,
    ) -> Result<(), String> {
        if let Self::SourcePublication { loop_completion: Some(completion), .. } = self {
            completion.observe_completed_loop(plan, function)?;
        }
        Ok(())
    }

    pub(super) fn emit_effect(
        &mut self,
        builder: &mut MirBuilder,
        effect: &CoreEffectPlan,
    ) -> Result<(), String> {
        match self {
            Self::Raw => PlanLowerer::emit_raw_effect(builder, effect),
            Self::SourcePublication { ledger, .. } => {
                let source_ledger = ledger.borrow().ordinary_new_claim_ledger().cloned();
                if let (Some(source_ledger), CoreEffectPlan::FieldGet { source: Some(site), receiver_source, dst, base, .. }) =
                    (source_ledger.as_ref(), effect) {
                    if ledger.borrow().owner() != site.owner() {
                        return Err("[freeze:contract][ordinary-field-read/plan-owner]".into());
                    }
                    let checked = source_ledger.local_field_requires_exact_receiver_v1(site);
                    let mut original_read = None;
                    let receiver_site = crate::mir::resolved_semantics::SourcePathV1::from_node(site.site().node())
                        .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Receiver).expr();
                    if let Some((expected, source)) = source_ledger.take_local_field_read(site, |binding| {
                        if checked {
                            let receiver = receiver_source.as_ref().ok_or("[freeze:contract][ordinary-field-read/plan-receiver-source-missing]")?;
                            original_read = Some(receiver.exact_variable_read_v1(site.owner(), &receiver_site, binding)?);
                            Ok(receiver.root_value())
                        } else {
                            ledger.borrow().value_for_exact_binding(site.owner(), binding)
                                .map_err(|error| format!("[freeze:contract][ordinary-field-read/receiver-binding] {error:?}"))
                        }
                    })? {
                        if expected != *base {
                            return Err("[freeze:contract][ordinary-field-read/receiver-value-mismatch]".into());
                        }
                        let field = source.field();
                        let claim = if checked {
                            crate::mir::builder::fields::PreparedExactFieldReadClaimV1::CheckedLocal { source,
                                read: original_read.ok_or("[freeze:contract][ordinary-field-read/plan-exact-receiver-missing]")?,
                                state: Rc::clone(ledger) }
                        } else { crate::mir::builder::fields::PreparedExactFieldReadClaimV1::Local(source) };
                        builder.complete_exact_field_read_with_destination_v1(*base, field, site,
                            source_ledger, claim, Some(*dst))?;
                        return Ok(());
                    }
                }
                if let (Some(source_ledger), CoreEffectPlan::MethodCall {
                    source: CoreCallSourceV1::LocatedMethodCall(site), dst, object, method, args, ..
                }) = (source_ledger.as_ref(), effect) {
                    let owner = ledger.borrow().owner();
                    let call = crate::mir::resolved_semantics::OwnedExprSiteV1::new(owner, site.clone());
                    if let Some(source) = source_ledger.borrowed_array_set_source_at_v1(owner, &call)? {
                        if crate::boxes::array::ArrayMethodId::from_name_and_arity(method, args.len())
                            != Some(crate::boxes::array::ArrayMethodId::Set) {
                            return Err("[freeze:contract][borrowed-set/call-shape]".into());
                        }
                        let [index, value] = args.as_slice() else {
                            return Err("[freeze:contract][borrowed-set/call-shape]".into());
                        };
                        let receiver = crate::mir::builder::ssa::local::try_ensure(builder, *object,
                            crate::mir::builder::ssa::local::LocalKind::Recv)?;
                        let arguments = [*index, *value].into_iter()
                            .map(|value| crate::mir::builder::ssa::local::try_arg(builder, value))
                            .collect::<Result<Vec<_>, _>>()?;
                        let write = builder.try_emit_known_array_method_write_recorded_v1(*dst, receiver, method, &arguments)?
                            .ok_or("[freeze:contract][borrowed-set/write-missing]")?;
                        return builder.complete_borrowed_array_set_v1(source_ledger, source, [*object, *index, *value], write);
                    }
                }
                if let CoreEffectPlan::NamedArrayPush { emission, .. } = effect {
                    if emission.is_field_residence_v1()? {
                        return Err("[freeze:contract][field-array-append/loop-source-missing]".into());
                    }
                    return PlanLowerer::emit_raw_effect(builder, effect);
                }
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
                    .map(|value| crate::mir::builder::ssa::local::try_arg(builder, value))
                    .collect::<Result<Vec<_>, _>>()?;
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
            Self::SourcePublication { ledger, loop_completion } => {
                if let Some(completion) = loop_completion { completion.require_finished()?; }
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


#[cfg(test)]
mod tests {
    use super::CorePlanEffectEmissionPortV1;
    use crate::mir::builder::control_flow::plan::{CoreCallSourceV1, CoreEffectPlan};
    use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
    use crate::mir::callable_result_representation::{
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

#[cfg(test)]
#[path = "source_publication_tests.rs"]
mod source_publication_tests;
