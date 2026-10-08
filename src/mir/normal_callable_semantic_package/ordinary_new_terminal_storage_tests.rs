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

fn owners(package: &Package, field: bool) -> (FunctionOwnerIdV1, FunctionOwnerIdV1) {
    let ledger = &package.ordinary_new_claim_ledger;
    let root = ledger.root_owner().expect("original Root");
    let child = package
        .batch()
        .declarations()
        .map(|row| row.owner())
        .find(|owner| {
            *owner != root
                && if field {
                    ledger.terminal_i64_field_return_for_owner(*owner).is_some()
                } else {
                    ledger
                        .terminal_integer_literal_return_for_owner(*owner)
                        .is_some()
                }
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
                    .prepare_terminal_integer_literal_return(owner, site.node())
                    .unwrap(),
                Some(source_value)
            );
            ledger
                .record_terminal_integer_literal_return(owner, site.node(), value)
                .unwrap();
            assert_eq!(
                ledger
                    .prepare_terminal_integer_literal_return(owner, site.node())
                    .unwrap_err(),
                "[freeze:contract][ordinary-new/literal-source-drift]"
            );
            assert_eq!(
                ledger
                    .record_terminal_integer_literal_return(owner, site.node(), value)
                    .unwrap_err(),
                "[freeze:contract][ordinary-new/literal-duplicate]"
            );
        }
        assert_eq!(
            ledger.terminal_integer_literal_value.borrow().get(&site),
            Some(&ValueId(900))
        );
        let children = ledger.terminal_integer_literal_values.borrow();
        assert_eq!(children.get(&(child, site.clone())), Some(&ValueId(901)));
        assert!(!children.contains_key(&(root, site)));
    }
}

#[test]
fn indexed_root_field_storage_requires_both_same_site_owners() {
    for root_first in [false, true] {
        let package = package(true);
        let ledger = &package.ordinary_new_claim_ledger;
        let (root, child) = owners(&package, true);
        let site = ledger
            .terminal_i64_field_return_for_owner(root)
            .unwrap()
            .return_site()
            .clone();
        assert_eq!(
            &site,
            ledger
                .terminal_i64_field_return_for_owner(child)
                .unwrap()
                .return_site()
        );
        assert!(ledger.terminal_relation_is_indexed(root, &site));
        assert!(!ledger.terminal_i64_field_return_complete());
        let order = if root_first {
            [root, child]
        } else {
            [child, root]
        };
        for (index, owner) in order.into_iter().enumerate() {
            let value = if owner == root {
                ValueId(900)
            } else {
                ValueId(901)
            };
            // This test isolates scalar storage, without installing a Home
            // or issuing a physical field read from the source-only fixture.
            ledger
                .record_terminal_i64_field_return(owner, &site, value)
                .unwrap();
            assert_eq!(ledger.terminal_i64_field_return_complete(), index == 1);
            assert_eq!(
                ledger
                    .prepare_terminal_i64_field_return(owner, site.node(), |_, _| panic!(
                        "duplicate cannot resolve"
                    ))
                    .err()
                    .unwrap(),
                "[freeze:contract][ordinary-terminal-field-return/source-drift]"
            );
            assert_eq!(
                ledger
                    .record_terminal_i64_field_return(owner, &site, value)
                    .unwrap_err(),
                "[freeze:contract][ordinary-terminal-field-return/duplicate-emission]"
            );
        }
        assert_eq!(
            ledger.terminal_i64_field_value.borrow().get(&site),
            Some(&ValueId(900))
        );
        assert_eq!(
            ledger
                .terminal_i64_field_values
                .borrow()
                .get(&(child, site.clone())),
            Some(&ValueId(901))
        );
        assert!(!ledger
            .terminal_i64_field_values
            .borrow()
            .contains_key(&(root, site.clone())));
        ledger.terminal_i64_field_value.borrow_mut().remove(&site);
        assert!(!ledger.terminal_i64_field_return_complete());
        ledger
            .terminal_i64_field_value
            .borrow_mut()
            .insert(site.clone(), ValueId(900));
        assert!(ledger.terminal_i64_field_return_complete());
        ledger
            .terminal_i64_field_values
            .borrow_mut()
            .remove(&(child, site.clone()));
        assert!(!ledger.terminal_i64_field_return_complete());
        assert_eq!(
            ledger.terminal_i64_field_value.borrow().get(&site),
            Some(&ValueId(900))
        );
    }
}
