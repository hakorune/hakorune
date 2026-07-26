//! P0 failure and reuse matrix for the bounded pre-loop ingress.
//!
//! Every fixture owns and discards its Builder. These tests prove retained
//! source ownership and fail-fast routing only; they issue no typed physical
//! receipt, nested-result receipt, or Integer publication.

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::me_call_header_observation::{
    MeCallHeaderObservationPortV1, MeCallHeaderSourceV1, MeCallParameterObservationV1,
};
use crate::mir::builder::recursive_child_lowering::{
    RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::resolved_semantics::{ExprChildRoleV1, SourceExprSiteV1};
use crate::mir::source_call_target::{
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::mir::source_instance_result_contract::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, PreparedPreloopLocatedArgumentV1,
    VerifiedCurrentOwnerInstanceResultTargetV1,
};
use crate::mir::{
    BasicBlockId, Callee, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirInstruction,
    MirModule, MirType, TypeOpKind, ValueId,
};

use super::extern_calls::EnvMethodSpec;
use super::member_route::MemberCallRoutePlan;
use super::method_call_descent::RawLegacyMethodCallInputV1;
use super::method_call_terminal::{
    emit_env_value_terminal_raw_v1, emit_global_value_terminal_raw_v1,
    emit_standard_value_terminal_raw_v1, emit_typeop_value_terminal_raw_v1,
    MethodCallValueTerminalPortV1,
};
use super::preloop_located_argument_ingress::{
    lower_selected_preloop_located_argument_v1, PreloopLocatedArgumentIngressErrorV1,
    PreloopLocatedArgumentIngressStageV1, PreloopObservedMeRouteV1,
};
use super::preloop_located_argument_port::{
    PreloopLocatedArgumentPortV1, PreloopSelectedArgumentStateV1,
};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn configured_builder(bind_ret: bool) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder
        .comp_ctx
        .install_callable_declaration_catalog(
            actual_parser_add_fixture::declaration_catalog_for_lowering(),
        )
        .expect("fixture lowering catalog");
    builder.enter_function_for_test("ParserBox.static_const_parse_add/2".to_string());

    let text = builder.build_expression(integer(11)).expect("text value");
    let pos = builder.build_expression(integer(12)).expect("pos value");
    let ret = builder.build_expression(integer(13)).expect("ret value");
    let me = builder.build_expression(integer(14)).expect("me value");
    builder.bind_function_parameter_for_test("text", text);
    builder.bind_function_parameter_for_test("pos", pos);
    if bind_ret {
        builder.bind_variable_for_test("ret", ret);
    }
    builder.bind_variable_for_test("me", me);
    builder
}

fn with_prepared_preloop<R>(
    f: impl for<'site, 'view, 'catalog> FnOnce(
        PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        RawLegacyMethodCallInputV1,
        ASTNode,
        String,
        SourceExprSiteV1,
    ) -> R,
) -> R {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            let call = VerifiedSourceMethodCallSiteV1::verify(catalog, caller, sites[0].clone())
                .expect("selected pre-loop source MethodCall");
            let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call)
                .expect("selected pre-loop target");
            let proof = results
                .issue_unannotated_body_proof(target.target())
                .expect("selected pre-loop Integer proof");
            let contract = seal_nested_instance_result_contract(target, proof)
                .expect("selected pre-loop Integer contract");

            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let body = view.root_body();
            let statement = view.body_stmt(&body, 3).expect("Body(3)");
            let outer = view
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .expect("Body(3).Value");
            let inner = view
                .child_expr_from_expr(&outer, ExprChildRoleV1::CallArgument(1))
                .expect("Body(3).Value.Argument(1)");
            let association = prepare_preloop_nested_result_association_v1(
                contract,
                view.method_call_input(&inner)
                    .expect("located inner MethodCall"),
            )
            .expect("exact pre-loop association");
            let outer_call = view
                .method_call_input(&outer)
                .expect("located outer MethodCall");
            let outer_receiver = outer_call.receiver().clone();
            let outer_method = outer_call.method().to_string();
            let outer_input = RawLegacyMethodCallInputV1::new(
                outer_receiver.clone(),
                outer_method.clone(),
                outer_call.arguments().to_vec(),
            );
            let selected = view
                .method_call_argument(outer_call, 1)
                .expect("structural Argument(1)");
            let prepared = prepare_preloop_located_argument_v1(selected, association)
                .expect("exact outer/inner relation");

            f(
                prepared,
                outer_input,
                outer_receiver,
                outer_method,
                sites[0].clone(),
            )
        },
    )
}

fn call_count(builder: &MirBuilder) -> usize {
    builder
        .current_function_instructions()
        .iter()
        .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .count()
}

fn assert_configured_success() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_preloop(
            |prepared, outer_input, outer_receiver, outer_method, selected_site| {
                let mut builder = configured_builder(true);
                let route = builder
                    .plan_member_call_route(&outer_receiver, &outer_method)
                    .expect("existing outer route");
                assert!(matches!(route, MemberCallRoutePlan::StaticReceiver { .. }));
                let mut port =
                    PreloopLocatedArgumentPortV1::new(RawLegacyChildLoweringPortV1, prepared);
                builder
                    .execute_prepared_member_call_route_v1(&mut port, &outer_input, route)
                    .expect("fresh configured candidate");
                assert!(matches!(
                    port.selected_state(),
                    PreloopSelectedArgumentStateV1::Reached(reached)
                        if reached.selected_site() == &selected_site
                ));
                assert_eq!(call_count(&builder), 2);
                port.discard();
            },
        );
    });
}

#[test]
fn outer_route_drift_stops_before_candidate_argument_descent() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_preloop(
            |prepared, _outer_input, outer_receiver, outer_method, selected_site| {
                let mut builder = configured_builder(true);
                let shadow = builder
                    .build_expression(integer(99))
                    .expect("shadow receiver value");
                builder.bind_variable_for_test("ParserStringUtilsBox", shadow);

                let route = builder
                    .plan_member_call_route(&outer_receiver, &outer_method)
                    .expect("existing outer route");
                assert!(
                    !matches!(route, MemberCallRoutePlan::StaticReceiver { .. }),
                    "candidate proof must stop when the existing planner changes route"
                );
                let port =
                    PreloopLocatedArgumentPortV1::new(RawLegacyChildLoweringPortV1, prepared);
                assert_eq!(call_count(&builder), 0);
                assert!(matches!(
                    port.selected_state(),
                    PreloopSelectedArgumentStateV1::Armed(source)
                        if source.selected().child().site() == &selected_site
                ));
                port.discard();
            },
        );
    });
}

#[test]
fn candidate_rejects_unified_disabled_before_the_selected_inner_call() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "off", || {
        with_prepared_preloop(
            |prepared, outer_input, outer_receiver, outer_method, selected_site| {
                let mut builder = configured_builder(true);
                let route = builder
                    .plan_member_call_route(&outer_receiver, &outer_method)
                    .expect("existing outer route");
                let mut port =
                    PreloopLocatedArgumentPortV1::new(RawLegacyChildLoweringPortV1, prepared);
                let error = builder
                    .execute_prepared_member_call_route_v1(&mut port, &outer_input, route)
                    .expect_err("candidate must not use the legacy Call compatibility route");
                assert!(error.contains("unified-call-disabled"));
                assert_eq!(call_count(&builder), 0);
                assert!(matches!(
                    port.selected_state(),
                    PreloopSelectedArgumentStateV1::Rejected(rejected)
                        if rejected.selected_site() == &selected_site
                            && rejected.stage()
                                == PreloopLocatedArgumentIngressStageV1::UnifiedCapability
                            && rejected.cause()
                                == &PreloopLocatedArgumentIngressErrorV1::UnifiedCallDisabled
                ));
                port.discard();
            },
        );
    });

    assert_configured_success();
}

#[test]
fn configured_header_drift_rejects_lowered_global_before_call_emission() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_preloop(
            |prepared, _outer_input, _outer_receiver, _outer_method, selected_site| {
                let mut builder = configured_builder(true);
                let signature = FunctionSignature {
                    name: "ParserBox.static_const_eval_pos/1".to_string(),
                    params: vec![MirType::Integer],
                    return_type: MirType::Integer,
                    effects: EffectMask::PURE,
                };
                let mut module = MirModule::new("preloop-header-drift".to_string());
                module.add_function(MirFunction::new(signature, BasicBlockId::new(0)));
                builder.current_module = Some(module);

                let mut ordinary = RawLegacyChildLoweringPortV1;
                let rejected = lower_selected_preloop_located_argument_v1(
                    &mut builder,
                    &mut ordinary,
                    prepared,
                )
                .expect_err("header evidence must not select a different physical route");
                assert_eq!(call_count(&builder), 0);
                assert_eq!(rejected.selected_site(), &selected_site);
                assert_eq!(
                    rejected.stage(),
                    PreloopLocatedArgumentIngressStageV1::MeRoute
                );
                assert_eq!(
                    rejected.cause(),
                    &PreloopLocatedArgumentIngressErrorV1::AlternateMeRoute(
                        PreloopObservedMeRouteV1::LoweredGlobal
                    )
                );
                rejected.discard();
            },
        );
    });
}

#[test]
fn missing_inner_argument_retains_source_and_a_fresh_fixture_still_succeeds() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_preloop(
            |prepared, _outer_input, _outer_receiver, _outer_method, selected_site| {
                let mut builder = configured_builder(false);
                let mut ordinary = RawLegacyChildLoweringPortV1;
                let rejected = lower_selected_preloop_located_argument_v1(
                    &mut builder,
                    &mut ordinary,
                    prepared,
                )
                .expect_err("missing ret binding must fail during inner descent");
                assert_eq!(call_count(&builder), 0);
                assert_eq!(rejected.selected_site(), &selected_site);
                assert_eq!(
                    rejected.stage(),
                    PreloopLocatedArgumentIngressStageV1::ArgumentDescent
                );
                assert!(matches!(
                    rejected.cause(),
                    PreloopLocatedArgumentIngressErrorV1::ArgumentDescent { .. }
                ));
                rejected.discard();
            },
        );
    });

    assert_configured_success();
}

/// Test-only ordinary port that preserves every existing Raw child/terminal
/// path except the outer static terminal, which fails after the selected inner
/// Method Call has completed.
struct FailingOuterStaticTerminalPortV1;

impl RecursiveChildLoweringPortV1 for FailingOuterStaticTerminalPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        RawLegacyChildLoweringPortV1.lower_body(builder, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        RawLegacyChildLoweringPortV1.lower_statement(builder, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        RawLegacyChildLoweringPortV1.lower_expression(builder, input)
    }
}

impl MeCallHeaderObservationPortV1 for FailingOuterStaticTerminalPortV1 {
    fn observe_me_call_parameters(
        &mut self,
        _builder: &MirBuilder,
        symbol: &str,
    ) -> MeCallParameterObservationV1 {
        MeCallParameterObservationV1::missing(MeCallHeaderSourceV1::ModuleCompatibility, symbol)
    }
}

impl MethodCallValueTerminalPortV1 for FailingOuterStaticTerminalPortV1 {
    fn emit_typeop_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        value: ValueId,
        op: TypeOpKind,
        ty: MirType,
    ) -> Result<ValueId, String> {
        emit_typeop_value_terminal_raw_v1(builder, value, op, ty)
    }

    fn emit_static_global_value_terminal(
        &mut self,
        _builder: &mut MirBuilder,
        _owner: &str,
        _method: &str,
        _checked_source_arity: u32,
        _arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        Err("[preloop-fixture/outer-terminal-failure]".to_string())
    }

    fn emit_me_lowered_global_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        owner: &str,
        method: &str,
        checked_source_arity: u32,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_global_value_terminal_raw_v1(builder, owner, method, checked_source_arity, arguments)
            .map(|(value, _)| value)
    }

    fn emit_env_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        spec: &EnvMethodSpec,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_env_value_terminal_raw_v1(builder, spec, arguments)
    }

    fn emit_standard_value_terminal(
        &mut self,
        builder: &mut MirBuilder,
        receiver: ValueId,
        method: String,
        arguments: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        emit_standard_value_terminal_raw_v1(builder, receiver, method, arguments)
    }
}

#[test]
fn outer_terminal_failure_retains_the_completed_inner_source_owner() {
    crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
        with_prepared_preloop(
            |prepared, outer_input, outer_receiver, outer_method, selected_site| {
                let mut builder = configured_builder(true);
                let route = builder
                    .plan_member_call_route(&outer_receiver, &outer_method)
                    .expect("existing outer route");
                let mut port =
                    PreloopLocatedArgumentPortV1::new(FailingOuterStaticTerminalPortV1, prepared);
                let error = builder
                    .execute_prepared_member_call_route_v1(&mut port, &outer_input, route)
                    .expect_err("outer terminal is deliberately rejected");
                assert!(error.contains("outer-terminal-failure"));
                assert_eq!(call_count(&builder), 1, "inner Method Call completed once");
                assert!(builder.current_function_instructions().iter().any(
                    |instruction| matches!(
                        instruction,
                        MirInstruction::Call {
                            callee: Some(Callee::Method { method, .. }),
                            ..
                        } if method == "static_const_eval_pos"
                    )
                ));
                assert!(matches!(
                    port.selected_state(),
                    PreloopSelectedArgumentStateV1::Rejected(rejected)
                        if rejected.selected_site() == &selected_site
                            && rejected.stage()
                                == PreloopLocatedArgumentIngressStageV1::OuterTerminal
                            && matches!(
                                rejected.cause(),
                                PreloopLocatedArgumentIngressErrorV1::OuterTerminal { .. }
                            )
                ));
                port.discard();
            },
        );
    });
}
