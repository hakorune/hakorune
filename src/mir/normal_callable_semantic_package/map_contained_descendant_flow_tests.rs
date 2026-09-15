use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::{MapDestinationV1, MapHomeObservation};
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

fn expr(segments: &[SourcePathSegmentV1]) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments.to_vec()))
}

#[test]
fn contained_map_issues_contained_in_destination_row() {
    // `local xs = [[%{...}]]` — the map literal at
    // `Initializer(0) -> Element(0) -> Element(0)` is a descendant whose
    // parent is an array literal, not a map or a sealed call. It gets its
    // own flow row with generic ContainedIn evidence.
    let package = issue("static box Main { main() { local xs = [[%{\"a\" => 1}]] return 30 } }")
        .expect("contained Map source");
    let declaration = package
        .batch()
        .declarations()
        .next()
        .expect("main declaration");
    let inner_array_site = expr(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Element(0),
    ]);
    let map_site = expr(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Element(0),
        SourcePathSegmentV1::Element(0),
    ]);
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&OwnedExprSiteV1::new(declaration.owner(), map_site))
        .expect("contained Map row completes");
    assert_eq!(
        map.destination(),
        &MapDestinationV1::ContainedIn {
            parent: OwnedExprSiteV1::new(declaration.owner(), inner_array_site),
            role: SourcePathSegmentV1::Element(0),
        }
    );
    assert_eq!(map.local_binding(), None);
}

#[test]
fn contained_sweep_preserves_stronger_destinations() {
    // `local m = %{"a" => %{...}}` — the nested map is an exact EntryValue
    // child of a map parent, so the sweep keeps its EntrySlot evidence and
    // never downgrades or re-issues it as ContainedIn.
    let package =
        issue("static box Main { main() { local m = %{\"a\" => %{\"b\" => 1}} return 30 } }")
            .expect("nested-entry Map source");
    let declaration = package
        .batch()
        .declarations()
        .next()
        .expect("main declaration");
    let parent_site = expr(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let child_site = expr(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::EntryValue(0),
    ]);
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    let rows: Vec<_> = flow
        .maps()
        .iter()
        .filter(|observation| observation.site().site() == &child_site)
        .collect();
    assert_eq!(rows.len(), 1, "exactly one row per sealed map literal");
    let map = rows[0].complete().expect("child row completes");
    assert_eq!(
        map.destination(),
        &MapDestinationV1::EntrySlot {
            parent_map: OwnedExprSiteV1::new(declaration.owner(), parent_site),
            ordinal: 0,
        }
    );
}

#[test]
fn contained_map_inside_call_argument_array_completes() {
    // `local blocks = [Helpers.tag(0, [..., %{...}])]` — the merged-route
    // residual family: a map literal contained under Element -> Argument ->
    // Element. The sweep issues the row at the statement's program point.
    let package = issue(
        "static box Helpers { tag(a, b) { return 30 } run(flag) {
            local blocks = [Helpers.tag(0, [flag, %{\"a\" => 1}])] return 30 } }
         static box Main { main() { return 30 } }",
    )
    .expect("deeply contained Map source");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Helpers::run declaration");
    let map_site = expr(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Element(0),
        SourcePathSegmentV1::Argument(1),
        SourcePathSegmentV1::Element(1),
    ]);
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&OwnedExprSiteV1::new(declaration.owner(), map_site))
        .expect("contained Map row completes");
    assert!(matches!(
        map.destination(),
        MapDestinationV1::ContainedIn {
            role: SourcePathSegmentV1::Element(1),
            ..
        }
    ));
}

#[test]
fn contained_outward_rejects_foreign_and_wrong_membership() {
    let package =
        issue("static box Main { main() { local xs = [[%{\"a\" => 1}]] return 30 } }").unwrap();
    let declaration = package.batch().declarations().next().unwrap();
    let foreign_package = issue("static box Main { main() { return %{\"f\" => 1} } }").unwrap();
    let foreign_map = foreign_package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .maps()[0]
        .site()
        .clone();
    package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            let parent_site = expr(&[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
                SourcePathSegmentV1::Element(0),
            ]);
            let parent = OwnedExprSiteV1::new(declaration.owner(), parent_site);
            let map_site = expr(&[
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
                SourcePathSegmentV1::Element(0),
                SourcePathSegmentV1::Element(0),
            ]);
            let map = OwnedExprSiteV1::new(declaration.owner(), map_site);
            // A foreign-owned site never claims membership here.
            assert!(crate::mir::resolved_control_flow::map_contained_outward(
                input,
                &foreign_map,
                &parent,
                &SourcePathSegmentV1::Element(0),
            )
            .is_err());
            // A non-expression parent is not membership evidence.
            let statement_as_parent =
                OwnedExprSiteV1::new(declaration.owner(), expr(&[SourcePathSegmentV1::Body(0)]));
            assert!(crate::mir::resolved_control_flow::map_contained_outward(
                input,
                &map,
                &statement_as_parent,
                &SourcePathSegmentV1::Element(0),
            )
            .is_err());
            // A wrong role does not reproduce the exact path tail.
            assert!(crate::mir::resolved_control_flow::map_contained_outward(
                input,
                &map,
                &parent,
                &SourcePathSegmentV1::Element(9),
            )
            .is_err());
            // The real parent and role verify.
            assert!(crate::mir::resolved_control_flow::map_contained_outward(
                input,
                &map,
                &parent,
                &SourcePathSegmentV1::Element(0),
            )
            .is_ok());
        })
        .unwrap();
}

#[test]
fn contained_map_with_nested_array_entry_completes() {
    // The merged residual shape: `"incoming" => [[4, 1], [5, 2]]` — a map
    // entry whose array elements are themselves `[...]` literals of leaves.
    // The element classifier recurses one level; the map completes.
    let package = issue(
        "static box Helpers { tag(a, b) { return 30 } run(flag) {
            local blocks = [Helpers.tag(0, [%{\"op\" => \"phi\", \"incoming\" => [[4, 1], [5, 2]]}])]
            return 30 } }
         static box Main { main() { return 30 } }",
    )
    .expect("contained Map with nested-array entry");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Helpers::run declaration");
    let map_site = expr(&[
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Initializer(0),
        SourcePathSegmentV1::Element(0),
        SourcePathSegmentV1::Argument(1),
        SourcePathSegmentV1::Element(0),
    ]);
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&OwnedExprSiteV1::new(declaration.owner(), map_site))
        .expect("contained Map row completes");
    let [op, incoming] = map.entries() else {
        panic!("two entries");
    };
    assert_eq!(op.key(), "op");
    let elements = incoming.array_elements().expect("array elements");
    assert_eq!(elements.len(), 2);
    for (element, expected) in elements.iter().zip([[4, 1], [5, 2]]) {
        assert!(element.value_source().is_none());
        let nested = element.nested_elements().expect("nested array element");
        for (leaf, value) in nested.iter().zip(expected) {
            assert_eq!(
                leaf.value_source(),
                Some(
                    &crate::mir::resolved_semantics::home_new_prefix::MapValueSource::Integer(
                        value
                    )
                )
            );
        }
    }
}

#[test]
fn nested_body_map_literal_stays_unavailable_until_scoped_design() {
    // A `%{...}` inside a `loop` body is sealed under the loop statement's
    // subtree but lives in a nested scope — the current scope/target triple
    // contract does not cover it, so the sweep records Unavailable rather
    // than claiming completion. Per-owner nested-body coverage is a
    // separate card.
    let package = issue(
        "static box Main { main() { local n = 0 loop(n < 1) { local m = %{\"a\" => 1} n = n + 1 } return 30 } }",
    )
    .expect("nested-body source still issues rows");
    let map_site = expr(&[
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::LoopBody(0),
        SourcePathSegmentV1::Initializer(0),
    ]);
    let flow = package
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap();
    let rows: Vec<_> = flow
        .maps()
        .iter()
        .filter(|observation| observation.site().site() == &map_site)
        .collect();
    assert_eq!(rows.len(), 1, "one row for the nested-body map literal");
    assert!(matches!(rows[0], MapHomeObservation::Unavailable { .. }));
}
