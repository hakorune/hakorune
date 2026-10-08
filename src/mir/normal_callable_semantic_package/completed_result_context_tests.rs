use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog as issue,
    NormalCallableSemanticPackageInstallIssueV1 as Issue,
};

const SOURCE: &str =
    "static box Main { main() { return helper(30) } helper(value: i64): i64 { return value } }";

#[test]
fn completed_context_rejects_missing_foreign_and_duplicate_formals() {
    top_level_inputs_join_original_completed_context();
    for mode in 0..3 {
        let mut package = issue(SOURCE).unwrap();
        if mode == 0 {
            package.parameter_contracts = Box::new([]);
        } else if mode == 1 {
            let owner = package.parameter_contracts[0].owner;
            let row = package
                .parameter_contracts
                .iter_mut()
                .find(|row| row.owner != owner)
                .unwrap();
            row.owner = owner;
        } else {
            let slot = package.parameter_contracts[0].batch_slot;
            package.parameter_contracts[1].batch_slot = slot;
        }
        let result = package
            .result_contracts
            .retain_completed_context(package.selected, package.parameter_contracts);
        assert!(matches!(
            result,
            Err(Issue::MissingParameterContract
                | Issue::ParameterContractOwnerMismatch
                | Issue::DuplicateParameterContract)
        ));
    }
}

#[test]
fn completed_context_rejects_foreign_selection() {
    let package = issue(SOURCE).unwrap();
    let foreign = issue("static box Other { run(value: i64): i64 { return value } }").unwrap();
    assert!(matches!(
        package
            .result_contracts
            .retain_completed_context(foreign.selected, package.parameter_contracts),
        Err(Issue::ResultContractMismatch)
    ));
}

fn top_level_inputs_join_original_completed_context() {
    const MIXED: &str = "function helper(value: i64): i64 { return value } static box Api { run(value: i64): i64 { return value } } static box Main { main() { return helper(30) } }";
    for mode in 0..5 {
        let mut package = issue(MIXED).unwrap();
        let slot = package
            .result_contracts
            .rows
            .iter()
            .find(|row| row.top_level_input.is_some())
            .unwrap()
            .batch_slot;
        let owner = package.result_contracts.row(slot).unwrap().owner;
        assert!(std::ptr::eq(
            package
                .ordinary_new_claim_ledger
                .completion_for_owner(owner)
                .unwrap(),
            package
                .result_contracts
                .row(slot)
                .unwrap()
                .borrow()
                .completion(),
        ));
        if mode == 1 {
            package
                .result_contracts
                .rows
                .iter_mut()
                .find(|row| row.batch_slot == slot)
                .unwrap()
                .top_level_input = None;
        } else if mode == 2 {
            let mut foreign = issue("function other(value: i64): i64 { return value } static box Main { main() { return other(30) } }").unwrap();
            let input = foreign
                .result_contracts
                .rows
                .iter_mut()
                .find_map(|row| row.top_level_input.take())
                .unwrap();
            package
                .result_contracts
                .rows
                .iter_mut()
                .find(|row| row.batch_slot == slot)
                .unwrap()
                .top_level_input = Some(input);
        } else if mode == 3 {
            let owners: Vec<_> = package
                .result_contracts
                .rows
                .iter()
                .map(|row| row.owner)
                .collect();
            package
                .parameter_contracts
                .iter_mut()
                .find(|row| !owners.contains(&row.owner))
                .unwrap()
                .batch_slot = slot;
        } else if mode == 4 {
            let foreign_owner = package
                .result_contracts
                .rows
                .iter()
                .find(|row| row.batch_slot != slot)
                .unwrap()
                .owner;
            package
                .result_contracts
                .rows
                .iter_mut()
                .find(|row| row.batch_slot == slot)
                .unwrap()
                .owner = foreign_owner;
        }
        let context = package
            .result_contracts
            .retain_completed_context(package.selected, package.parameter_contracts);
        if mode == 0 {
            let context = context.unwrap();
            let key =
                crate::mir::builder::CanonicalSameModuleCallableKeyV1::free_function("helper", 1);
            assert_eq!(context.owner_for_canonical_key(&key), Some(owner));
            for key in [
                crate::mir::builder::CanonicalSameModuleCallableKeyV1::free_function("other", 1),
                crate::mir::builder::CanonicalSameModuleCallableKeyV1::free_function("helper", 0),
            ] {
                assert_eq!(context.owner_for_canonical_key(&key), None);
            }
        } else {
            assert!(
                matches!(
                    context,
                    Err(Issue::MissingParameterContract
                        | Issue::ResultContractMismatch
                        | Issue::DuplicateParameterContract)
                ),
                "mode {mode}: {context:?}"
            );
        }
    }
}
