//! Source authority for partial Birth cleanup, independent of MIR order.
use super::super::OwnedFieldChildKindV1;
use super::issue_with_brand_catalog;

#[test]
fn construction_store_fault_inventory_is_prior_committed_source_order() {
    for class in ["Inner", "ArrayBox"] {
        let source = format!(
            "box Inner {{ value: i64 = 0 birth() {{ }} }}
             box Holder {{
                 left: {class} = new {class}()
                 right: {class} = new {class}()
                 marker: i64 = 0
                 birth() {{ }}
             }}
             static box Main {{ main() {{ local holder = new Holder() return 0 }} }}"
        );
        let package = issue_with_brand_catalog(&source).unwrap();
        let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
        let claims: Vec<_> = rows.values().collect();
        let [claim] = claims.as_slice() else {
            panic!("one Holder claim: {claims:?}")
        };
        let plan = claim.construction().as_ref().unwrap();
        let stores = plan.stores();
        assert_eq!(stores.len(), 3);
        for (store, expected) in stores.iter().zip([vec![], vec![0], vec![1, 0]]) {
            let actual: Vec<_> = store
                .fault_discharge()
                .iter()
                .map(|child| child.field.declaration_ordinal())
                .collect();
            assert_eq!(actual, expected);
            for child in store.fault_discharge() {
                assert_eq!(
                    matches!(child.kind, OwnedFieldChildKindV1::Array),
                    class == "ArrayBox"
                );
            }
        }
    }
}

#[test]
fn caller_provided_object_store_seals_prior_owned_cleanup() {
    let package = issue_with_brand_catalog(
        "box Inner { value: i64 = 0 birth() { } }
         box Holder { left: Inner = new Inner() right: ArrayBox = new ArrayBox()
             inner: Inner marker: i64
             birth(inner) { me.inner = inner me.marker = 1 } }
         static box Main { main() {
             local inner = new Inner() local holder = new Holder(inner) return 0
         } }",
    )
    .expect("valid source retains caller-provided object construction");
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claim = rows
        .values()
        .find(|claim| claim.class() == "Holder")
        .expect("Holder source claim");
    let plan = claim.construction().as_ref().unwrap();
    assert_eq!(plan.stores().len(), 4);
    for (store, expected) in plan.stores().iter().zip([vec![], vec![0], vec![1, 0], vec![1, 0]]) {
        assert_eq!(store.fault_discharge().iter().map(|child|
            child.field.declaration_ordinal()).collect::<Vec<_>>(), expected);
    }
    assert!(matches!(plan.stores()[2].rhs(),
        super::super::ConstructionStoreRhsV1::Parameter { provided: Some(_), .. }));
    let children = claim.children().expect("Normal parent End keeps all residences");
    assert_eq!(children.iter().map(|child| child.field.declaration_ordinal())
        .collect::<Vec<_>>(), vec![0, 1, 2]);
    let duplicated = issue_with_brand_catalog(
        "box Inner { value: i64 = 0 birth() { } }
         box Holder { left: Inner right: Inner
             birth(inner) { me.left = inner me.right = inner } }
         static box Main { main() { local inner = new Inner()
             local holder = new Holder(inner) return 0 } }",
    ).unwrap();
    assert!(duplicated.ordinary_new_claim_ledger.pending_claims_for_test().values()
        .find(|claim| claim.class() == "Holder").unwrap().construction().is_err(),
        "one provided Home cannot become two owning fields");

}
