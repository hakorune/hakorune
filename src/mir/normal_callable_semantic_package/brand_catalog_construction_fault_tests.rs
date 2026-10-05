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
         box Holder { inner: Inner marker: i64
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
    assert_eq!(plan.stores().len(), 2);
    assert!(plan.stores()[0].fault_discharge().is_empty());
    assert!(matches!(
        plan.stores()[0].rhs(),
        super::super::ConstructionStoreRhsV1::Parameter { .. }
    ));
    let [owned] = plan.stores()[1].fault_discharge() else {
        panic!("prior Provided residence must be discharged")
    };
    assert_eq!(owned.field, plan.stores()[0].field());
    assert!(matches!(owned.kind, OwnedFieldChildKindV1::Object(_)));
}
