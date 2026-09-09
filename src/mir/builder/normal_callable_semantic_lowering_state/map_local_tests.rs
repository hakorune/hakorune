use super::*;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{CallableFunctionSyntaxViewV1,
    FunctionSemanticResolverSessionV1, ResolveSelectedCallableForestsOutcomeV1};
use crate::parser::NyashParser;

fn fixture() -> CallableSemanticLoweringState {
    let program = NyashParser::parse_from_string(
        "function caller() { local first = 1 local second = 2 return first }",
    )
    .expect("fixture parses");
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

#[test]
fn local_initializer_lookup_uses_exact_declaration_and_preserves_binding() {
    let state = fixture();
    assert_eq!(state.initializers.len(), 2);
    for (site, bindings) in &state.locals {
        for (ordinal, binding) in bindings.iter().enumerate() {
            assert_eq!(state.local_initializer(site, ordinal).unwrap().binding(), *binding);
        }
    }
    let site = state.locals.keys().next().unwrap();
    assert!(state.local_initializer(site, usize::MAX).unwrap_err().contains("placement-local-missing"));
}

#[test]
fn local_initializer_lookup_rejects_missing_locator_and_foreign_binding() {
    let mut state = fixture();
    let keys: Vec<_> = state.initializers.keys().cloned().collect();
    let SourceBindingSiteV1::Local { statement, ordinal } = &keys[0] else { panic!("local") };
    let first = state.initializers.remove(&keys[0]).unwrap();
    assert!(state.local_initializer(statement.node(), *ordinal as usize)
        .unwrap_err().contains("placement-initializer-missing"));
    // Even a row with a valid foreign declaration must not satisfy this slot.
    let other = state.initializers.get(&keys[1]).unwrap().clone();
    state.initializers.insert(keys[0].clone(), other);
    assert!(state.local_initializer(statement.node(), *ordinal as usize)
        .unwrap_err().contains("placement-initializer-binding-drift"));
    state.initializers.insert(keys[0].clone(), first);
    assert!(state.local_initializer(statement.node(), *ordinal as usize).is_ok());
}
