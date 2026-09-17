use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::{MapDestinationV1, MapValueSource};
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

fn source(body: &str) -> String {
    format!("box Page {{}} static box Main {{ main() {{ {body} }} }}")
}

#[test]
fn nested_map_entry_issues_entry_slot_child_row() {
    let package = issue(
        "static box Work { make(args) { return %{\"a\" => %{\"x\" => args}, \"b\" => \"s\"} } }
         static box Main { main() { return 30 } }",
    )
    .expect("nested map entry completes");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Work::make declaration");
    let parent_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]));
    let child_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::EntryValue(0),
    ]));
    let parent_owned = OwnedExprSiteV1::new(declaration.owner(), parent_site);
    let child_owned = OwnedExprSiteV1::new(declaration.owner(), child_site.clone());
    let parent = package
        .ordinary_new_claim_ledger
        .map_flow(&parent_owned)
        .expect("parent row completes");
    let [a, b] = parent.entries() else {
        panic!("two entries");
    };
    assert_eq!(a.key(), "a");
    assert_eq!(a.site(), &child_site);
    // NestedMap carries no value source, binding, or home claim — every
    // downstream scalar/transfer gate stays fail-closed.
    assert!(a.value_source().is_none() && a.transfer_home().is_none() && a.binding().is_none());
    assert_eq!(b.value_source(), Some(&MapValueSource::String("s".into())));
    let child = package
        .ordinary_new_claim_ledger
        .map_flow(&child_owned)
        .expect("nested child row completes");
    assert_eq!(
        child.destination(),
        &MapDestinationV1::EntrySlot {
            parent_map: parent_owned,
            ordinal: 0
        }
    );
    assert_eq!(child.local_binding(), None);
    let [x] = child.entries() else {
        panic!("one child entry");
    };
    assert!(matches!(
        x.value_source(),
        Some(MapValueSource::BorrowedHandle(_))
    ));
}

#[test]
fn nested_map_entry_in_local_position_and_deeper_recursion() {
    let package = issue(&source(
        "local m = %{\"a\" => %{\"b\" => %{\"c\" => 1}}} return 30",
    ))
    .unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    // Parent + two nested descendants, all Complete.
    assert_eq!(flow.maps().len(), 3);
    assert!(flow.maps().iter().all(|row| row.complete().is_some()));
}

#[test]
fn nested_map_child_failure_marks_both_rows_unavailable() {
    // The child's entry value `[p]` is an uncovered class (a live Home
    // element is a transfer question): child row is Unavailable with its
    // exact site and the parent row is Unavailable too.
    let package = issue(&source(
        "local p = new Page() return %{\"a\" => %{\"x\" => [p]}}",
    ))
    .unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    assert_eq!(flow.maps().len(), 2);
    assert!(flow.maps().iter().all(|row| row.complete().is_none()));
    let child_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::EntryValue(0),
    ]));
    assert!(flow
        .maps()
        .iter()
        .any(|row| row.site().site() == &child_site));
}

#[test]
fn nested_map_transfer_marks_parent_outer_at_parent_entry_index() {
    // `p` is transferred inside the nested child; the parent's outer must
    // still record the consumption at the parent's entry index so
    // `outer_after_installs` stays exact.
    let package = issue(&source(
        "local p = new Page() return %{\"a\" => %{\"h\" => p}, \"b\" => 1}",
    ))
    .unwrap();
    let declaration = package
        .batch()
        .declarations()
        .next()
        .expect("root declaration");
    let parent_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Value,
    ]));
    let parent_owned = OwnedExprSiteV1::new(declaration.owner(), parent_site);
    let parent = package
        .ordinary_new_claim_ledger
        .map_flow(&parent_owned)
        .expect("parent row completes");
    let p_binding = parent.allocation_fault().next().expect("one outer home");
    // Consumed at parent entry 0: live before entry 0, gone after.
    assert_eq!(
        parent.outer_after_installs(0).unwrap().collect::<Vec<_>>(),
        vec![p_binding]
    );
    assert_eq!(parent.outer_after_installs(1).unwrap().count(), 0);
    let child_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::EntryValue(0),
    ]));
    let child = package
        .ordinary_new_claim_ledger
        .map_flow(&OwnedExprSiteV1::new(declaration.owner(), child_site))
        .expect("child row completes");
    let [h] = child.entries() else {
        panic!("one child entry");
    };
    assert_eq!(
        h.transfer_home().map(|(_, binding)| binding),
        Some(p_binding)
    );
}

#[test]
fn array_entry_records_exact_leaf_elements() {
    let package = issue(
        "static box Work { make(args) { return %{\"a\" => [1, \"s\", args]} } }
         static box Main { main() { return 30 } }",
    )
    .expect("array entry completes");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Work::make declaration");
    let site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]));
    let owned = OwnedExprSiteV1::new(declaration.owner(), site);
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&owned)
        .expect("array entry row completes");
    let [a] = map.entries() else {
        panic!("one entry");
    };
    // NestedArray carries no value source, binding, or home claim — every
    // downstream scalar/transfer gate stays fail-closed.
    assert!(a.value_source().is_none() && a.transfer_home().is_none() && a.binding().is_none());
    let [first, second, third] = a.array_elements().expect("array elements") else {
        panic!("three leaf elements");
    };
    assert_eq!(first.value_source(), Some(&MapValueSource::Integer(1)));
    assert_eq!(
        second.value_source(),
        Some(&MapValueSource::String("s".into()))
    );
    assert!(matches!(
        third.value_source(),
        Some(MapValueSource::BorrowedHandle(_))
    ));
    // Each element site is the exact `Element(ordinal)` child path.
    assert_eq!(
        first.site(),
        &SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
            SourcePathSegmentV1::EntryValue(0),
            SourcePathSegmentV1::Element(0),
        ]))
    );
}

#[test]
fn array_entry_empty_literal_and_local_position_complete() {
    let package = issue(&source("local m = %{\"a\" => [], \"b\" => 2} return 30")).unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    let [observation] = flow.maps() else {
        panic!("one map row");
    };
    let map = observation.complete().expect("empty array completes");
    let [a, b] = map.entries() else {
        panic!("two entries");
    };
    assert_eq!(a.array_elements().map(<[_]>::len), Some(0));
    assert_eq!(b.value_source(), Some(&MapValueSource::Integer(2)));
}

#[test]
fn array_entry_rejects_home_and_container_elements() {
    for (body, statement) in [
        // a live Home element is a transfer question, never a leaf borrow
        ("local p = new Page() return %{\"a\" => [p]}", 1u32),
        // a `%{...}` element stays uncovered — the element map still gets
        // its own ContainedIn row from the sweep
        ("return %{\"a\" => [%{}]}", 0u32),
    ] {
        let package = issue(&source(body)).unwrap();
        let flow = package
            .ordinary_new_claim_ledger
            .root_completion_for_test()
            .cleanup()
            .root_flow()
            .unwrap();
        // The return-boundary outer map stays Unavailable — its array entry
        // still holds a non-leaf element. (A contained descendant map inside
        // that element now gets its own row; pin the outer row by site.)
        let return_map_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(statement),
            SourcePathSegmentV1::Value,
        ]));
        let observation = flow
            .maps()
            .iter()
            .find(|observation| observation.site().site() == &return_map_site)
            .expect("outer return map row");
        assert!(
            observation.complete().is_none(),
            "{body} must stay Unavailable"
        );
    }
}

#[test]
fn map_local_entry_is_a_non_consuming_borrow() {
    // `m` stays the owner of its constructed map; the parent entry borrows
    // it by reference — no transfer, no consume, and `m` still gets its
    // own End through the parent's outer.
    let package = issue(&source(
        "local m = %{\"x\" => 1} local n = %{\"inner\" => m, \"k\" => \"s\"} return 30",
    ))
    .unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    assert_eq!(flow.maps().len(), 2);
    assert!(flow.maps().iter().all(|row| row.complete().is_some()));
    let parent = flow.maps()[1].complete().unwrap();
    let [inner, k] = parent.entries() else {
        panic!("two entries");
    };
    let Some(MapValueSource::MapLocal(m_binding)) = inner.value_source() else {
        panic!("map local is a borrowed reference, not a transfer");
    };
    assert_eq!(inner.binding(), Some(*m_binding));
    assert_eq!(inner.transfer_home(), None);
    assert_eq!(k.value_source(), Some(&MapValueSource::String("s".into())));
    // `m` is not consumed: it stays in the parent's outer set after all
    // installs — the borrow marks no transfer.
    assert_eq!(
        parent
            .outer_after_installs(parent.entries().len())
            .unwrap()
            .collect::<Vec<_>>(),
        vec![*m_binding]
    );
}

#[test]
fn map_local_alias_and_array_element_borrow_the_same_root() {
    let package = issue(&source(
        "local m = %{} local a = m return %{\"x\" => a, \"list\" => [m, a]}",
    ))
    .unwrap();
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    let observation = flow.maps().last().expect("one map row");
    let map = observation.complete().expect("map-local borrows complete");
    let [x, list] = map.entries() else {
        panic!("two entries");
    };
    let Some(MapValueSource::MapLocal(x_root)) = x.value_source() else {
        panic!("alias of a map local borrows the root");
    };
    let elements = list.array_elements().expect("array elements");
    for element in elements {
        let Some(MapValueSource::MapLocal(root)) = element.value_source() else {
            panic!("map-local element borrows the root");
        };
        assert_eq!(*root, *x_root);
    }
}
