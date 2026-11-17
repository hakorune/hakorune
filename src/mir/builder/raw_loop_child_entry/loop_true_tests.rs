use crate::ast::ASTNode;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
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
