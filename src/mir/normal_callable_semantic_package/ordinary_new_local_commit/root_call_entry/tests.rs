use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests;

fn package() -> crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1
{
    brand_catalog_tests::issue_with_brand_catalog(
        r#"static box Main {
            main() {
                local first = helper(10)
                local second = middle(20)
                return other(30)
            }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
            middle(value: i64): i64 { local m = %{"middle" => value} return 30 }
            other(value: i64): i64 { local m = %{"other" => value} return 30 }
        }"#,
    )
    .expect("two source local Call rows")
}

fn fake_binding() -> Vec<(BasicBlockId, MirInstruction)> {
    vec![(BasicBlockId(0), MirInstruction::Return { value: None })]
}

#[test]
fn local_binding_groups_reject_duplicate_foreign_and_swapped_sites() {
    let package = package();
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_owner().expect("root owner");
    let sites: Vec<_> = ledger
        .completion_for_owner(owner)
        .expect("root completion")
        .cleanup()
        .root_flow()
        .expect("root flow")
        .local_calls()
        .iter()
        .map(|call| call.site().clone())
        .collect();
    assert_eq!(sites.len(), 2);
    assert!(ledger
        .record_root_local_call_bindings(owner, sites[1].clone(), fake_binding())
        .is_err());
    ledger
        .record_root_local_call_bindings(owner, sites[0].clone(), fake_binding())
        .expect("first site");
    assert!(ledger
        .record_root_local_call_bindings(owner, sites[0].clone(), fake_binding())
        .is_err());
    let foreign_owner = package
        .batch
        .declarations()
        .map(|row| row.owner())
        .find(|candidate| *candidate != owner)
        .expect("foreign owner");
    assert!(ledger
        .record_root_local_call_bindings(
            owner,
            OwnedExprSiteV1::new(foreign_owner, sites[1].site().clone()),
            fake_binding(),
        )
        .is_err());
}

#[test]
fn local_binding_groups_reject_incomplete_sequence_before_transfer() {
    let package = package();
    let ledger = &package.ordinary_new_claim_ledger;
    let owner = ledger.root_owner().expect("root owner");
    let completion = ledger.completion_for_owner(owner).expect("root completion");
    let sites: Vec<_> = completion
        .cleanup()
        .root_flow()
        .expect("root flow")
        .local_calls()
        .iter()
        .map(|call| call.site().clone())
        .collect();
    let exit = completion
        .explicit_sites()
        .iter()
        .next()
        .expect("explicit exit")
        .clone();
    let groups =
        vec![RootLocalCallBindingGroupV1::new(sites[0].clone(), fake_binding(), None).unwrap()];
    assert!(ledger
        .validate_local_call_binding_groups(owner, &exit, &groups)
        .is_err());
}
