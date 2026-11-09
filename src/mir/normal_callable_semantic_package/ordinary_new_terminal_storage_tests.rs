//! Indexed Root source keeps its physical scalar storage independent of children.
use super::super::*;
use crate::mir::normal_callable_semantic_package::{
    brand_catalog_tests::issue_with_brand_catalog as issue,
    VerifiedNormalCallableSemanticPackageV1 as Package,
};
use crate::mir::ValueId;

fn package(field: bool) -> Package {
    let root = if field { "page.slot" } else { "7" };
    let child = if field { "page.slot" } else { "30" };
    issue(&format!(
        "box Page {{ slot: i64 birth() {{ me.slot = 7 }} }}
         static box Main {{
           main() {{ local page = new Page() local m = %{{\"v\" => 0}} return {root} }}
           helper(value: i64): i64 {{
             local page = new Page() local m = %{{\"v\" => value}} return {child}
           }}
         }}"
    ))
    .expect("same-site Root/child source")
}

fn owners(package: &Package, _field: bool) -> (FunctionOwnerIdV1, FunctionOwnerIdV1) {
    let ledger = &package.ordinary_new_claim_ledger;
    let root = ledger.root_owner().expect("original Root");
    let child = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .find(|owner| {
            *owner != root
                && ledger.terminal_integer_literal_return_for_owner(*owner).is_some()
        })
        .expect("original scalar child");
    assert!(Rc::ptr_eq(
        &ledger.terminal_relation_index[&root],
        &ledger.terminal_relation
    ));
    (root, child)
}

#[test]
fn indexed_root_literal_storage_is_independent_of_same_site_child() {
    for root_first in [false, true] {
        let package = package(false);
        let ledger = &package.ordinary_new_claim_ledger;
        let (root, child) = owners(&package, false);
        let site = ledger
            .terminal_integer_literal_return_for_owner(root)
            .unwrap()
            .return_site()
            .clone();
        assert_eq!(
            &site,
            ledger
                .terminal_integer_literal_return_for_owner(child)
                .unwrap()
                .return_site()
        );
        assert!(ledger.terminal_relation_is_indexed(root, &site));
        let order = if root_first {
            [root, child]
        } else {
            [child, root]
        };
        for owner in order {
            let source_value = if owner == root { 7 } else { 30 };
            let value = if owner == root {
                ValueId(900)
            } else {
                ValueId(901)
            };
            assert_eq!(
                ledger
                    .prepare_terminal_scalar_literal_return(owner, site.node())
                    .unwrap(),
                Some(crate::mir::ConstValue::Integer(source_value))
            );
            ledger
                .record_terminal_scalar_literal_return(owner, site.node(), value)
                .unwrap();
            assert_eq!(
                ledger
                    .prepare_terminal_scalar_literal_return(owner, site.node())
                    .unwrap_err(),
                "[freeze:contract][ordinary-new/literal-source-drift]"
            );
            assert_eq!(
                ledger
                    .record_terminal_scalar_literal_return(owner, site.node(), ValueId(999))
                    .unwrap_err(),
                "[freeze:contract][ordinary-new/literal-duplicate]"
            );
        }
        assert_eq!(
            ledger
                .terminal_scalar_literal_value
                .borrow()
                .get(&site)
                .map(|row| row.value),
            Some(ValueId(900))
        );
        let children = ledger.terminal_scalar_literal_values.borrow();
        assert_eq!(
            children.get(&(child, site.clone())).map(|row| row.value),
            Some(ValueId(901))
        );
        assert!(!children.contains_key(&(root, site)));
    }
}
