//! Received-call source identity and invalidation; no acquired Home is issued.
use super::*;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::parser::NyashParser;

fn unit() -> VerifiedResolvedSourceUnitV1 {
    let parsed = NyashParser::parse_from_string(
        "function probe(token) { local first = token.make() local second = token.make() local number = 7 return first }",
    ).unwrap();
    let ASTNode::Program { statements, .. } = parsed else {
        panic!("Program")
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .unwrap();
    VerifiedResolvedSourceUnitV1::resolve_function(function).unwrap()
}

fn initializers(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Vec<(BindingRefV1, OwnedExprSiteV1)> {
    input
        .function()
        .expression_source()
        .initializers()
        .map(|row| {
            (
                row.binding(),
                OwnedExprSiteV1::new(input.owner(), row.initializer_site().unwrap().clone()),
            )
        })
        .collect()
}

#[test]
fn received_call_identity_preserves_both_classes_without_direct_new_home() {
    let source = unit();
    let input = source.root_function_input().unwrap();
    let rows = initializers(input);
    let mut flow = PrefixLocalFlow::new(input);
    flow.install_received_handle(rows[0].0, &rows[0].1);
    flow.install_received_nullable(rows[1].0, &rows[1].1);
    assert_eq!(
        flow.received_call_acquisition(rows[0].0),
        Some((&rows[0].1, false))
    );
    assert_eq!(
        flow.received_call_acquisition(rows[1].0),
        Some((&rows[1].1, true))
    );
    assert!(flow.home_acquisition(rows[0].0).is_none());
    assert!(flow.home_acquisition(rows[1].0).is_none());
    assert!(flow.direct_available_home(rows[0].1.site()).is_none());
}

#[test]
fn received_call_identity_rejects_foreign_wrong_initializer_and_noncall_sources() {
    let source = unit();
    let foreign = unit();
    let input = source.root_function_input().unwrap();
    let foreign_input = foreign.root_function_input().unwrap();
    let rows = initializers(input);
    let other = initializers(foreign_input);
    let mut flow = PrefixLocalFlow::new(input);
    flow.install_received_handle(rows[0].0, &other[0].1);
    assert!(flow.received_call_acquisition(rows[0].0).is_none());
    flow.install_received_nullable(rows[0].0, &rows[1].1);
    assert!(flow.received_call_acquisition(rows[0].0).is_none());
    flow.install_received_handle(rows[2].0, &rows[2].1);
    assert!(flow.received_call_acquisition(rows[2].0).is_none());
}

#[test]
fn received_call_identity_is_invalidated_by_consumption_and_rebinding() {
    let source = unit();
    let input = source.root_function_input().unwrap();
    let rows = initializers(input);
    let mut flow = PrefixLocalFlow::new(input);
    flow.install_received_nullable(rows[0].0, &rows[0].1);
    flow.mark_nonnull(rows[0].0);
    assert!(flow.is_received_nullable(rows[0].0));
    flow.consume_home(rows[0].0);
    assert!(flow.received_call_acquisition(rows[0].0).is_none());
    assert!(!flow.is_received_nullable(rows[0].0));
    flow.install_received_nullable(rows[0].0, &rows[0].1);
    flow.install_uninitialized(rows[0].0);
    assert!(flow.received_call_acquisition(rows[0].0).is_none());
}

#[test]
fn received_call_branch_join_requires_same_exact_acquisition() {
    let source = unit();
    let input = source.root_function_input().unwrap();
    let rows = initializers(input);
    let mut first = PrefixLocalFlow::new(input);
    first.install_received_nullable(rows[0].0, &rows[0].1);
    let same = first.clone();
    assert!(first.join_branch(&same));
    assert_eq!(
        first.received_call_acquisition(rows[0].0),
        Some((&rows[0].1, true))
    );
    let mut drifted = same.clone();
    drifted.install_received_nullable(rows[0].0, &rows[1].1);
    assert!(!first.join_branch(&drifted));
    assert_eq!(
        first.received_call_acquisition(rows[0].0),
        Some((&rows[0].1, true))
    );
}

#[test]
fn received_return_borrows_original_call_row_and_fault_snapshot() {
    let source = unit();
    let input = source.root_function_input().unwrap();
    let rows = initializers(input);
    let mut flow = PrefixLocalFlow::new(input);
    flow.install_received_nullable(rows[0].0, &rows[0].1);
    let initializer = input
        .function()
        .expression_source()
        .initializers()
        .find(|row| row.binding() == rows[0].0)
        .unwrap();
    let super::super::SourceBindingSiteV1::Local { statement, .. } = initializer.declaration_site()
    else {
        panic!("local initializer")
    };
    let make_call = |result| {
        super::super::LocalCallObservationV1::issue(
            input.owner(),
            statement.clone(),
            rows[0].1.clone(),
            initializer.declaration_site().clone(),
            rows[0].0,
            vec![rows[1].0].into_boxed_slice(),
            vec![super::super::LocalCallArgumentV1::Integer(29)].into_boxed_slice(),
            result,
        )
    };
    let calls = vec![make_call(super::super::LocalCallResultClassV1::Nullable)];
    let mut covered = std::collections::BTreeSet::from([rows[0].1.clone()]);
    let original = flow
        .received_call_observation(rows[0].0, &calls, &covered)
        .unwrap();
    assert!(std::ptr::eq(original, &calls[0]));
    assert_eq!(original.prior_homes(), &[rows[1].0]);
    assert_eq!(
        original.arguments(),
        &[super::super::LocalCallArgumentV1::Integer(29)]
    );
    assert!(flow
        .received_call_observation(rows[1].0, &calls, &covered)
        .is_none());
    let wrong = vec![make_call(super::super::LocalCallResultClassV1::Handle)];
    assert!(flow
        .received_call_observation(rows[0].0, &wrong, &covered)
        .is_none());
    let other_declaration = input
        .function()
        .expression_source()
        .initializers()
        .find(|row| row.binding() == rows[1].0)
        .unwrap()
        .declaration_site();
    let super::super::SourceBindingSiteV1::Local {
        statement: other_statement,
        ..
    } = other_declaration
    else {
        panic!("other local initializer")
    };
    for (bad_statement, bad_declaration) in [
        (other_statement, initializer.declaration_site()),
        (statement, other_declaration),
    ] {
        let forged = vec![super::super::LocalCallObservationV1::issue(
            input.owner(),
            bad_statement.clone(),
            rows[0].1.clone(),
            bad_declaration.clone(),
            rows[0].0,
            calls[0].prior_homes().into(),
            calls[0].arguments().into(),
            super::super::LocalCallResultClassV1::Nullable,
        )];
        assert!(flow
            .received_call_observation(rows[0].0, &forged, &covered)
            .is_none());
    }
    let duplicate = vec![calls[0].clone(), calls[0].clone()];
    assert!(flow
        .received_call_observation(rows[0].0, &duplicate, &covered)
        .is_none());
    covered.clear();
    assert!(flow
        .received_call_observation(rows[0].0, &calls, &covered)
        .is_none());
    covered.insert(rows[0].1.clone());
    flow.consume_home(rows[0].0);
    assert!(flow
        .received_call_observation(rows[0].0, &calls, &covered)
        .is_none());
}
