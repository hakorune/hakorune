use super::*;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1, SourcePathSegmentV1, SourcePathV1,
};
use crate::mir::source_call_target::issue_source_bound_core_method_calls_v1;
use crate::mir::ValueId;
use crate::parser::NyashParser;

fn fixture() -> CallableSemanticLoweringState {
    fixture_from("function caller() { local first = 1 local second = 2 return first }")
}

fn fixture_from(source: &str) -> CallableSemanticLoweringState {
    let program = NyashParser::parse_from_string(source).expect("fixture parses");
    let crate::ast::ASTNode::Program { mut statements, .. } = program else {
        panic!("fixture must be a program")
    };
    let function = statements.remove(0);
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function)
        .expect("fixture callable syntax");
    let mut resolver = FunctionSemanticResolverSessionV1::new(9102).expect("resolver");
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("fixture forest")
    else {
        panic!("fixture unexpectedly deferred")
    };
    let forest = forests.into_vec().pop().expect("fixture root forest");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("fixture projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .expect("fixture input");
    let state =
        CallableSemanticLoweringState::from_exact_source(input).expect("fixture lowering state");
    state
}

fn core_method_fixture() -> (
    CallableSemanticLoweringState,
    crate::mir::resolved_semantics::SourceExprSiteV1,
) {
    let program = NyashParser::parse_from_string(
        "function caller(text) { loop(text.length() < 2) { local piece = text.substring(0, 1) } }",
    )
    .expect("core-method fixture parses");
    let crate::ast::ASTNode::Program { mut statements, .. } = program else {
        panic!("fixture must be a program")
    };
    let function = statements.remove(0);
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function)
        .expect("core-method callable syntax");
    let mut resolver = FunctionSemanticResolverSessionV1::new(9103).expect("resolver");
    let ResolveSelectedCallableForestsOutcomeV1::Complete(forests) = resolver
        .resolve_selected_callable_forests(&[syntax.function()])
        .expect("core-method forest")
    else {
        panic!("core-method fixture unexpectedly deferred")
    };
    let forest = forests.into_vec().pop().expect("core-method root forest");
    let owner = forest.roots()[0];
    let ledger = forest
        .callable_source_ledger(owner)
        .expect("core-method source ledger");
    let rows = issue_source_bound_core_method_calls_v1(&ledger).expect("core-method rows");
    let site = rows
        .iter()
        .find_map(|(site, row)| {
            (row.contract().target().row().row().op == CoreMethodOp::StringLen)
                .then(|| site.clone())
        })
        .expect("length row");
    let projection = VerifiedSourceProjectionV1::seal_with_root_profile(
        &function,
        &forest,
        syntax.function().root_profile(),
    )
    .expect("core-method projection");
    let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
        &function,
        &forest,
        &projection,
    )
    .expect("core-method input");
    let mut state =
        CallableSemanticLoweringState::from_exact_source_with_dynamic_source_and_core_methods(
            input,
            None,
            crate::mir::normal_callable_semantic_package::unconditional_test_rows(rows),
        )
        .expect("core-method lowering state");
    state.entry_installed = true;
    for (index, binding) in state.parameters.iter().copied().enumerate() {
        state.values.insert(binding, ValueId::new(index as u32 + 1));
    }
    (state, site)
}

#[test]
fn local_initializer_lookup_uses_exact_declaration_and_preserves_binding() {
    let state = fixture();
    assert_eq!(state.initializers.len(), 2);
    for (site, bindings) in &state.locals {
        for (ordinal, binding) in bindings.iter().enumerate() {
            assert_eq!(
                state.local_initializer(site, ordinal).unwrap().binding(),
                *binding
            );
        }
    }
    let site = state.locals.keys().next().unwrap();
    assert!(state
        .local_initializer(site, usize::MAX)
        .unwrap_err()
        .contains("placement-local-missing"));
}

#[test]
fn nested_program_local_registers_rootless_statement_site() {
    // A `local` inside a bare `{ }` block registers under the rootless
    // `[stmt, ProgramBody(i)]` spelling — the same key `body_item_site`
    // emits on the lowering lane after the nested-program collapse.
    let state = fixture_from("function caller() { { local inner = 1 } return 0 }");
    let nested = SourcePathV1::root_body(0)
        .child(SourcePathSegmentV1::ProgramBody(0))
        .node();
    let bindings = state
        .locals
        .get(&nested)
        .expect("nested-program local registers rootless");
    assert_eq!(bindings.len(), 1);
    state
        .local_initializer(&nested, 0)
        .expect("nested-program local resolves its initializer");
}

#[test]
fn local_initializer_lookup_rejects_missing_locator_and_foreign_binding() {
    let mut state = fixture();
    let keys: Vec<_> = state.initializers.keys().cloned().collect();
    let SourceBindingSiteV1::Local { statement, ordinal } = &keys[0] else {
        panic!("local")
    };
    let first = state.initializers.remove(&keys[0]).unwrap();
    assert!(state
        .local_initializer(statement.node(), *ordinal as usize)
        .unwrap_err()
        .contains("placement-initializer-missing"));
    // Even a row with a valid foreign declaration must not satisfy this slot.
    let other = state.initializers.get(&keys[1]).unwrap().clone();
    state.initializers.insert(keys[0].clone(), other);
    assert!(state
        .local_initializer(statement.node(), *ordinal as usize)
        .unwrap_err()
        .contains("placement-initializer-binding-drift"));
    state.initializers.insert(keys[0].clone(), first);
    assert!(state
        .local_initializer(statement.node(), *ordinal as usize)
        .is_ok());
}

#[test]
fn source_core_method_take_rejects_duplicate_exact_site() {
    let (mut state, site) = core_method_fixture();
    let missing = SourcePathV1::root_body(99).expr();
    assert!(state
        .take_source_core_method_call(&missing, "length", 0)
        .expect("missing exact site")
        .is_none());
    let first = state
        .take_source_core_method_call(&site, "length", 0)
        .expect("first exact take")
        .expect("length row");
    assert_eq!(first.result_type(), crate::mir::MirType::Integer);

    let duplicate = state
        .take_source_core_method_call(&site, "length", 0)
        .expect_err("duplicate take must freeze");
    assert!(duplicate.contains("duplicate-core-method-call-consumption"));
}
