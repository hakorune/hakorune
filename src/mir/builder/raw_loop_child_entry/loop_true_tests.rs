use super::PreparedLocatedRawLoopChildEntryV1;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::GenericLoopFactsPolicyFrameV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::source_call_target::issue_source_bound_core_method_calls_v1;

struct ArmedLoopTrueEdge {
    ledger: std::rc::Rc<std::cell::RefCell<CallableSemanticLoweringState>>,
    loop_site: crate::mir::resolved_semantics::SourceNodeSiteV1,
    call_site: crate::mir::resolved_semantics::SourceExprSiteV1,
    loop_node: ASTNode,
    loop_index: usize,
}

fn armed_loop_true_edge() -> ArmedLoopTrueEdge {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let program = crate::parser::NyashParser::parse_from_string(
        r#"
static function caller(i: i64, text: String): i64 {
    loop(true) {
        local piece = text.substring(i, 1)
        if text.starts_with("a", 0) == 1 {
            i = i + 1
        } else {
            break
        }
    }
    return i
}
"#,
    )
    .expect("loop-true fixture parses");
    let ASTNode::Program { statements, .. } = program else {
        panic!("fixture is a program")
    };
    let function = statements
        .iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("loop-true function");
    let ASTNode::FunctionDeclaration { body, .. } = function else {
        panic!("fixture is a function")
    };
    let (loop_index, loop_node) = body
        .iter()
        .enumerate()
        .find(|(_, node)| matches!(node, ASTNode::Loop { .. }))
        .map(|(index, node)| (index, node.clone()))
        .expect("loop statement");
    let unit =
        crate::mir::compiler::VerifiedResolvedSourceUnitV1::resolve_function(function.clone())
            .expect("loop-true fixture resolves");
    let input = unit.root_function_input().expect("root input");
    let loop_site = input
        .function()
        .loop_sites()
        .find(|site| site.node().segments().len() == 1)
        .expect("root loop site")
        .node()
        .clone();
    let source_ledger = input
        .forest()
        .callable_source_ledger(input.owner())
        .expect("loop-true source ledger");
    let core_method_rows = issue_source_bound_core_method_calls_v1(&source_ledger)
        .expect("loop-true core method rows");
    assert_eq!(
        core_method_rows.len(),
        1,
        "fixture must issue the supported substring core-method row"
    );
    let state =
        CallableSemanticLoweringState::from_exact_source_with_dynamic_source_and_core_methods(
            input,
            None,
            crate::mir::normal_callable_semantic_package::unconditional_test_rows(core_method_rows),
        )
        .expect("loop-true callable state");
    let ledger = std::rc::Rc::new(std::cell::RefCell::new(state));
    let items = ledger
        .borrow()
        .source_loop_items(&loop_site)
        .expect("armed loop must catalog resolver method rows");
    let call_site = items
        .iter()
        .find(|item| item.selector() == "starts_with")
        .expect("loop-true fixture must contain starts_with publication call")
        .call_site()
        .clone();
    ArmedLoopTrueEdge {
        ledger,
        loop_site,
        call_site,
        loop_node,
        loop_index,
    }
}

fn armed_source_target(
    call_site: crate::mir::resolved_semantics::SourceExprSiteV1,
) -> crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetRelationV1 {
    let caller_key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserProgramBox",
        "parse",
        2,
    );
    let target_key = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
        "ParserStringUtilsBox",
        "starts_with",
        3,
    );
    let handoff = crate::mir::callable_result_representation::
        VerifiedStaticCallResultPublicationHandoffV1::from_test_parts(
        7,
        crate::mir::callable_result_representation::
            VerifiedStaticCallResultPublicationDemandV1::from_test_parts(
                caller_key,
                call_site.clone(),
                target_key.clone(),
            ),
        &[1],
    );
    let requirement = crate::mir::builder::normal_callable_loop_source_route::
        CallableLoopSourceTargetRequirementV1::from_handoff(&handoff);
    crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceTargetRelationV1::new(
        call_site,
        target_key,
        Some(requirement),
    )
}

fn builder_for_true_edge(ledger: &std::cell::RefCell<CallableSemanticLoweringState>) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("caller/2".to_owned());
    for (name, ty) in [
        ("i", crate::mir::MirType::Integer),
        ("text", crate::mir::MirType::String),
    ] {
        let parameter = builder.alloc_typed(ty);
        builder
            .function_state
            .current_function
            .as_mut()
            .expect("test function")
            .params
            .push(parameter);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_owned(), parameter);
    }
    let entry = crate::mir::builder::normal_callable_binding_materialization_port::
        PreparedCallableEntryValuesV1::static_function(&builder, 2)
        .expect("static entry values");
    ledger
        .borrow_mut()
        .install_entry_values(&entry)
        .expect("entry install");
    builder
}

#[test]
fn armed_loop_true_edge_lowers_through_the_source_port() {
    let edge = armed_loop_true_edge();
    let disposition = edge
        .ledger
        .borrow()
        .loop_binding_source_projection()
        .project_loop_true_disposition(edge.loop_site.clone())
        .expect("loop binding disposition");
    let (_, root) = RawInvocationSourceContextV1::from_transport(
        crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceTransportV1::root(
            (),
            RawInvocationRootLineageV1::ScriptRoot,
        ),
    );
    let (_, parent_source) = RawInvocationSourceContextV1::from_transport(
        root.body_statement(edge.loop_node.clone(), edge.loop_index),
    );
    let prepared = PreparedLocatedRawLoopChildEntryV1::prepare(
        &parent_source,
        edge.loop_node,
        Some(disposition),
    )
    .expect("located loop-true entry");
    let mut builder = builder_for_true_edge(&edge.ledger);
    let mut scope =
        crate::mir::builder::module_invocation_session::UnpublishedCallableLoopRootScopeV1::for_test(
        );
    crate::test_support::with_env_vars(&crate::test_support::JOINIR_STRICT_PLANNER_MODE, || {
        prepared
            .lower_v1_with_root_scope_and_callable_ledger(
                &mut builder,
                "caller",
                false,
                false,
                GenericLoopFactsPolicyFrameV1::from_values(true, true, false, true, true, true),
                &mut scope,
                &edge.ledger,
                crate::mir::builder::normal_callable_loop_source_route::
                    CallableLoopSourceTargetProbeV1::from_parts(
                        vec![armed_source_target(edge.call_site)].into_boxed_slice(),
                        Box::new([]),
                        false,
                    ),
            )
            .expect("armed loop-true edge must lower through the source port");
    });
}
