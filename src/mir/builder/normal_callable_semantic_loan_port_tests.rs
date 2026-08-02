use super::CallableLoanConsumptionV1;
use crate::mir::builder::callable_declaration_catalog::{
    SelectedNormalCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::parser::NyashParser;
use std::collections::BTreeSet;

fn keys() -> Vec<SelectedNormalCallableKeyV1> {
    let program = NyashParser::parse_from_string(
        "function first() { return 1 } function second() { return 2 }",
    )
    .unwrap();
    VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program)
        .unwrap()
        .selected_source_inventory()
        .entries()
        .map(|(key, _)| key.clone())
        .collect()
}

fn tracker(
    expected: impl IntoIterator<Item = SelectedNormalCallableKeyV1>,
) -> CallableLoanConsumptionV1 {
    CallableLoanConsumptionV1 {
        expected: expected.into_iter().collect::<BTreeSet<_>>(),
        consumed: BTreeSet::new(),
    }
}

#[test]
fn callable_loan_consumption_rejects_missing_duplicate_and_unconsumed_rows() {
    let keys = keys();
    let mut missing = tracker([keys[0].clone()]);
    assert!(missing
        .consume(keys[1].clone())
        .unwrap_err()
        .contains("missing-loan"));
    let mut duplicate = tracker([keys[0].clone()]);
    duplicate.consume(keys[0].clone()).unwrap();
    assert!(duplicate
        .consume(keys[0].clone())
        .unwrap_err()
        .contains("duplicate-loan"));
    assert!(tracker([keys[0].clone()])
        .complete()
        .unwrap_err()
        .contains("unconsumed-loan"));
    let mut complete = tracker([keys[0].clone()]);
    complete.consume(keys[0].clone()).unwrap();
    complete.complete().unwrap();
}
