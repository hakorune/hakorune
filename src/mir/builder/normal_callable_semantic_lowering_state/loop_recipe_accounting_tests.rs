use super::*;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    CallableFunctionSyntaxViewV1, FunctionSemanticResolverSessionV1,
    ResolveSelectedCallableForestsOutcomeV1,
};
use crate::mir::ValueId;
use crate::parser::NyashParser;

fn accounting_fixture() -> CallableSemanticLoweringState {
    let program = NyashParser::parse_from_string(
        "function caller() { local first = 1 local second = 2 first = first + second return first }",
    )
    .expect("fixture parses");
    let crate::ast::ASTNode::Program { mut statements, .. } = program else {
        panic!("fixture must be a program")
    };
    let function = statements.remove(0);
    let syntax = CallableFunctionSyntaxViewV1::from_function_ast(&function)
        .expect("fixture callable syntax");
    let mut resolver = FunctionSemanticResolverSessionV1::new(9127).expect("resolver");
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
    let input = crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1::
        from_exact_parts_without_callable(&function, &forest, &projection)
            .expect("fixture input");
    let mut state =
        CallableSemanticLoweringState::from_exact_source(input).expect("fixture lowering state");
    state.entry_installed = true;
    for (index, binding) in state
        .locals
        .values()
        .flat_map(|bindings| bindings.iter().copied())
        .enumerate()
    {
        state.values.insert(binding, ValueId::new(index as u32 + 1));
    }
    state
}

#[test]
fn variable_accum_accounting_names_missing_entry_value() {
    let mut state = accounting_fixture();
    let (site, binding) = state
        .variables
        .iter()
        .next()
        .map(|(site, binding)| (site.clone(), *binding))
        .expect("one variable source site");
    state.values.remove(&binding);

    let error = state
        .validate_variable_accum_source_sites(&[(site, binding)], &[])
        .expect_err("missing entry value must reject before consumption");
    assert!(
        error.contains("variable-accum/entry-value-missing"),
        "{error}"
    );
}

#[test]
fn variable_accum_accounting_rejects_duplicate_read_and_write_consumption() {
    let mut state = accounting_fixture();
    let (read_site, read_binding) = state
        .variables
        .iter()
        .next()
        .map(|(site, binding)| (site.clone(), *binding))
        .expect("one variable source site");
    let read_rows = [(read_site, read_binding)];
    state
        .consume_variable_accum_source_sites(&read_rows, &[], &[])
        .expect("first read consumption");
    let duplicate_read = state
        .consume_variable_accum_source_sites(&read_rows, &[], &[])
        .expect_err("same source read cannot be consumed twice");
    assert!(duplicate_read.contains("variable-accum/read-site-invalid"));

    let (write_site, write_binding) = state
        .assignments
        .iter()
        .next()
        .map(|(site, binding)| (site.clone(), *binding))
        .expect("one assignment source site");
    let write_rows = [(write_site, write_binding)];
    let writeback = [(write_binding, ValueId::new(90))];
    state
        .consume_variable_accum_source_sites(&[], &write_rows, &writeback)
        .expect("first write consumption");
    let duplicate_write = state
        .consume_variable_accum_source_sites(&[], &write_rows, &writeback)
        .expect_err("same source write cannot be consumed twice");
    assert!(duplicate_write.contains("variable-accum/write-site-invalid"));
}
