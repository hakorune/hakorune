use super::super::CallableSemanticLoweringState;
use super::ExactBindingValueErrorV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, CallableFunctionSyntaxViewV1, FunctionOwnerIssuerV1,
    FunctionSemanticResolverSessionV1, ResolveSelectedCallableForestsOutcomeV1,
};
use crate::mir::ValueId;
use crate::parser::NyashParser;

fn materialized_parameter_fixture() -> (
    CallableSemanticLoweringState,
    crate::mir::resolved_semantics::FunctionOwnerIdV1,
    BindingRefV1,
) {
    let program = NyashParser::parse_from_string(
        "function caller(first, second, third, fourth) { return first }",
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
    let mut state =
        CallableSemanticLoweringState::from_exact_source(input).expect("fixture lowering state");
    let owner = state.owner();
    let binding = state
        .parameters
        .first()
        .copied()
        .expect("fixture parameter");
    state.entry_installed = true;
    state.values.insert(binding, ValueId::new(77));
    (state, owner, binding)
}

#[test]
fn exact_binding_accessor_returns_the_materialized_entry_value() {
    let (state, owner, binding) = materialized_parameter_fixture();
    assert_eq!(
        state.value_for_exact_binding(owner, binding),
        Ok(ValueId::new(77))
    );
}

#[test]
fn exact_binding_accessor_rejects_owner_binding_and_entry_mismatches() {
    let (mut state, owner, binding) = materialized_parameter_fixture();
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("foreign issuer");
    let foreign_owner = issuer.issue().expect("foreign owner");
    let foreign_binding = BindingRefV1::new(foreign_owner, binding.binding());

    assert_eq!(
        state.value_for_exact_binding(foreign_owner, binding),
        Err(ExactBindingValueErrorV1::OwnerMismatch)
    );
    assert_eq!(
        state.value_for_exact_binding(owner, foreign_binding),
        Err(ExactBindingValueErrorV1::ForeignBinding)
    );

    state.entry_installed = false;
    assert_eq!(
        state.value_for_exact_binding(owner, binding),
        Err(ExactBindingValueErrorV1::EntryNotInstalled)
    );
    state.entry_installed = true;
    state.values.clear();
    assert_eq!(
        state.value_for_exact_binding(owner, binding),
        Err(ExactBindingValueErrorV1::ValueUnavailable)
    );
}

#[test]
fn exact_binding_accessor_is_observational_and_reusable() {
    let (state, owner, binding) = materialized_parameter_fixture();
    assert_eq!(
        state.value_for_exact_binding(owner, binding),
        Ok(ValueId::new(77))
    );
    assert_eq!(
        state.value_for_exact_binding(owner, binding),
        Ok(ValueId::new(77))
    );
}
