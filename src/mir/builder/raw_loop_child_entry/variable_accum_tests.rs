use super::{CallableLoopBindingProjectionDispositionV1, PreparedLocatedRawLoopChildEntryV1};
use crate::mir::builder::control_flow::plan::LoopFactsPolicyFrameV1;
use crate::mir::builder::normal_callable_loop_handoff::CallableLoopReadyBodyOnlyProductV1;
use crate::mir::builder::normal_callable_loop_source_facts::{
    issue_callable_variable_accum_recurrence, CallableGenericLoopSourceFactsRouteErrorV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawUnlocatedPortalV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionOwnerIssuerV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourceNodeSiteV1, SourcePathV1,
};
use crate::parser::NyashParser;

#[test]
fn candidate_without_callable_ledger_is_terminal_before_builder_effects() {
    use CallableLoopBindingProjectionDispositionV1 as Disposition;

    super::test_route_observation::reset();
    let program = NyashParser::parse_from_string(include_str!(
        "../../../../apps/tests/loop_simple_while_inline_explicit_step_min.hako"
    ))
    .expect("accepted callable source parses");
    let crate::ast::ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    let function = statements
        .into_iter()
        .find_map(|statement| match statement {
            crate::ast::ASTNode::BoxDeclaration { name, methods, .. } if name == "Main" => {
                methods.get_declaration("main").cloned()
            }
            _ => None,
        })
        .expect("accepted Main.main declaration");
    let crate::ast::ASTNode::FunctionDeclaration { body, .. } = &function else {
        panic!("Main.main must be a function declaration")
    };
    let loop_node = body.get(2).expect("accepted Loop site").clone();
    let syntax =
        CallableFunctionSyntaxViewV1::from_function_ast(&function).expect("callable syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("accepted callable resolves")
    else {
        panic!("accepted callable unexpectedly deferred")
    };
    let forest = forests
        .into_vec()
        .pop()
        .expect("one callable semantic forest");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("source projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .expect("exact resolved input");
    let state = CallableSemanticLoweringState::from_exact_source(input)
        .expect("callable semantic lowering state");
    let loop_site = SourcePathV1::root_body(2).node();
    let handoff = match state
        .loop_binding_source_projection()
        .project_disposition(loop_site.clone())
        .expect("accepted loop binding projection")
    {
        Disposition::Ready(schedule) => Disposition::ReadyWithBodyOnly(
            CallableLoopReadyBodyOnlyProductV1::without_body_only(schedule),
        ),
        ready @ Disposition::ReadyWithBodyOnly(_) => ready,
        other => panic!("accepted VAR loop must have a callable handoff: {other:?}"),
    };
    let parent_source = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: loop_site,
        body_kind: None,
    };
    let prepared =
        PreparedLocatedRawLoopChildEntryV1::prepare(&parent_source, loop_node, Some(handoff))
            .expect("located accepted loop entry");

    let mut builder = MirBuilder::new();
    let error = prepared
        .lower_v1_with_optional_root_scope(
            &mut builder,
            "Main.main",
            false,
            false,
            LoopFactsPolicyFrameV1::from_values(false, false, false, false, false, true),
            None,
            None,
            Some(input),
            crate::mir::builder::normal_callable_loop_source_route::
                CallableLoopSourceTargetProbeV1::empty(),
        )
        .expect_err("VAR cannot lower without its callable ledger");

    assert!(
        error.contains("callable-loop/variable-accum-recurrence/callable-ledger-missing"),
        "unexpected terminal: {error}"
    );
    assert!(builder.function_state.current_function.is_none());
    assert!(builder.function_state.current_block.is_none());
    assert_eq!(
        super::test_route_observation::counts(),
        (1, 0),
        "the selected VAR attempt reaches its named terminal without re-entering fallback"
    );
}

fn accepted_variable_accum_input() -> (
    ResolvedFunctionLoweringInputV1<'static>,
    SourceNodeSiteV1,
) {
    let program = NyashParser::parse_from_string(include_str!(
        "../../../../apps/tests/loop_simple_while_inline_explicit_step_min.hako"
    ))
    .expect("accepted callable source parses");
    let crate::ast::ASTNode::Program { statements, .. } = program else {
        panic!("parser must return Program")
    };
    let function = statements
        .into_iter()
        .find_map(|statement| match statement {
            crate::ast::ASTNode::BoxDeclaration { name, methods, .. } if name == "Main" => {
                methods.get_declaration("main").cloned()
            }
            _ => None,
        })
        .expect("accepted Main.main declaration");
    let syntax =
        CallableFunctionSyntaxViewV1::from_function_ast(&function).expect("callable syntax view");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver session");
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("accepted callable resolves")
    else {
        panic!("accepted callable unexpectedly deferred")
    };
    let forest = forests
        .into_vec()
        .pop()
        .expect("one callable semantic forest");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("source projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        Box::leak(Box::new(function)),
        Box::leak(Box::new(forest)),
        Box::leak(Box::new(projection)),
    )
    .expect("exact resolved input");
    (input, SourcePathV1::root_body(2).node())
}

#[test]
fn variable_accum_recurrence_rejects_unlocated_parent_source() {
    let (input, _loop_site) = accepted_variable_accum_input();
    let parent_source = RawInvocationSourceContextV1::UnlocatedCompatibility {
        reason: RawUnlocatedPortalV1::CallObject,
        expected_lineage: None,
    };
    let error = issue_callable_variable_accum_recurrence(input, input.owner(), &parent_source)
        .expect_err("VAR requires a located loop parent source");
    assert!(
        matches!(
            error,
            CallableGenericLoopSourceFactsRouteErrorV1::VariableAccumRecurrenceSourceUnavailable(..)
        ),
        "unexpected terminal: {error:?}"
    );
}

#[test]
fn variable_accum_recurrence_rejects_non_loop_parent_site() {
    let (input, _loop_site) = accepted_variable_accum_input();
    let parent_source = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: SourcePathV1::root_body(0).node(),
        body_kind: None,
    };
    let error = issue_callable_variable_accum_recurrence(input, input.owner(), &parent_source)
        .expect_err("a non-loop site cannot issue Loop membership");
    assert!(
        matches!(
            error,
            CallableGenericLoopSourceFactsRouteErrorV1::VariableAccumRecurrenceSourceUnavailable(..)
        ),
        "unexpected terminal: {error:?}"
    );
}

#[test]
fn variable_accum_recurrence_rejects_foreign_expected_owner() {
    let (input, loop_site) = accepted_variable_accum_input();
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("foreign owner issuer");
    let foreign = issuer.issue().expect("foreign owner id");
    assert_ne!(foreign, input.owner());
    let parent_source = RawInvocationSourceContextV1::Located {
        root: RawInvocationRootLineageV1::ScriptRoot,
        site: loop_site,
        body_kind: None,
    };
    let error = issue_callable_variable_accum_recurrence(input, foreign, &parent_source)
        .expect_err("a foreign owner must be terminal before membership lookup");
    assert!(
        matches!(
            error,
            CallableGenericLoopSourceFactsRouteErrorV1::VariableAccumRecurrenceSourceUnavailable(..)
        ),
        "unexpected terminal: {error:?}"
    );
}
