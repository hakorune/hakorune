use crate::mir::normal_callable_semantic_package::model::parameter_catalog_for_drift_test as mutable_catalog;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog as issue,
    NormalCallableSemanticPackageInstallIssueV1 as Issue,
};

const SOURCE: &str =
    "static box Main { main() { return helper(30) } helper(value: i64): i64 { return value } }";

#[test]
fn completed_context_rejects_missing_foreign_and_duplicate_formals() {
    for mode in 0..3 {
        let mut package = issue(SOURCE).unwrap();
        let mut contracts = mutable_catalog(&package.parameter_contracts);
        if mode == 0 {
            contracts.clear();
        } else if mode == 1 {
            let owner = contracts[0].owner;
            let row = contracts.iter_mut().find(|row| row.owner != owner).unwrap();
            row.owner = owner;
        } else {
            let slot = contracts[0].batch_slot;
            contracts[1].batch_slot = slot;
        }
        package.parameter_contracts = std::rc::Rc::new(contracts.into_iter().collect());
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

#[test]
fn original_parameter_catalog_loan_shares_package_and_completed_context() {
    use std::rc::Rc;
    let package = issue(SOURCE).unwrap();
    let original = Rc::clone(&package.parameter_contracts);
    let ledger = &package.ordinary_new_claim_ledger;
    assert!(Rc::ptr_eq(
        &original,
        ledger.original_parameter_contracts_v1()
    ));
    let context = package
        .result_contracts
        .retain_completed_context(package.selected, package.parameter_contracts)
        .unwrap();
    assert!(Rc::ptr_eq(
        &original,
        &context.completed_context.as_ref().unwrap().1
    ));
    assert!(Rc::ptr_eq(
        &original,
        ledger.original_parameter_contracts_v1()
    ));
}
