use super::*;
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
