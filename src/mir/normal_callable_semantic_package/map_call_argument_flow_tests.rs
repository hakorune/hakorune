use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::resolved_semantics::home_new_prefix::{MapDestinationV1, MapValueSource};
use crate::mir::resolved_semantics::{
    OwnedExprSiteV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
};

fn source(body: &str) -> String {
    format!("box Page {{}} static box Main {{ main() {{ {body} }} }}")
}

#[test]
fn call_argument_map_issues_call_slot_destination_row() {
    // `return Helpers.consume(%{...}, 7)` — the map literal at
    // Argument(0) gets its own flow row with exact call-slot evidence.
    let package = issue(
        "static box Helpers { consume(a, b) { return 30 } run(flag) {
            return Helpers.consume(%{\"a\" => flag}, 7) } }
         static box Main { main() { return 30 } }",
    )
    .expect("call-argument Map source");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Helpers::run declaration");
    let call_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
    ]));
    let arg_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(0),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Argument(0),
    ]));
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&OwnedExprSiteV1::new(declaration.owner(), arg_site))
        .expect("call-argument Map row completes");
    assert_eq!(
        map.destination(),
        &MapDestinationV1::CallArgument {
            call: OwnedExprSiteV1::new(declaration.owner(), call_site),
            ordinal: 0
        }
    );
    assert_eq!(map.local_binding(), None);
    let [a] = map.entries() else {
        panic!("one entry");
    };
    assert_eq!(a.key(), "a");
    assert!(matches!(
        a.value_source(),
        Some(MapValueSource::BorrowedHandle(_))
    ));
}

#[test]
fn call_argument_map_transfers_a_live_home_entry() {
    // `p` is consumed inside the argument map exactly like an
    // entry-slot transfer; no separate argument-transfer meaning.
    let package = issue(
        "static box Helpers { consume(a, b) { return 30 } run(flag) {
            local p = new Page() return Helpers.consume(%{\"h\" => p}, flag) } }
         static box Main { main() { return 30 } }
         box Page {}",
    )
    .expect("call-argument Map with a Home entry");
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Helpers::run declaration");
    let arg_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(1),
        SourcePathSegmentV1::Value,
        SourcePathSegmentV1::Argument(0),
    ]));
    let map = package
        .ordinary_new_claim_ledger
        .map_flow(&OwnedExprSiteV1::new(declaration.owner(), arg_site))
        .expect("call-argument Map row completes");
    let [h] = map.entries() else {
        panic!("one entry");
    };
    assert!(h.transfer_home().is_some());
    assert_eq!(
        map.outer_after_installs(0).unwrap().count(),
        1,
        "the transferred Home is the sole outer"
    );
    assert!(map.outer_after_installs(1).unwrap().next().is_none());
}

#[test]
fn call_argument_outward_rejects_non_argument_membership() {
    let package = issue(
        "static box Helpers { consume(a, b) { return 30 } run(flag) {
            local m = %{\"a\" => 1} return Helpers.consume(%{\"b\" => 2}, flag) } }
         static box Main { main() { return 30 } }",
    )
    .unwrap();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.parameter_count() == 1)
        .expect("Helpers::run declaration");
    // A map row issued under a different package is a foreign owner.
    let foreign_package = issue(&source("return %{\"f\" => 1}")).unwrap();
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
            let call_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
            ]));
            let call = OwnedExprSiteV1::new(declaration.owner(), call_site.clone());
            let local_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(0),
                SourcePathSegmentV1::Initializer(0),
            ]));
            // A local-initializer Map is not this call's Argument(0) child.
            assert!(crate::mir::resolved_control_flow::map_argument_outward(
                input,
                &OwnedExprSiteV1::new(declaration.owner(), local_site),
                &call,
                0,
            )
            .is_err());
            // A foreign-owned site never claims membership here.
            assert!(crate::mir::resolved_control_flow::map_argument_outward(
                input,
                &foreign_map,
                &call,
                0,
            )
            .is_err());
            // The real argument child verifies.
            let arg_site = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
                SourcePathSegmentV1::Body(1),
                SourcePathSegmentV1::Value,
                SourcePathSegmentV1::Argument(0),
            ]));
            assert!(crate::mir::resolved_control_flow::map_argument_outward(
                input,
                &OwnedExprSiteV1::new(declaration.owner(), arg_site.clone()),
                &call,
                0,
            )
            .is_ok());
            // The wrong ordinal is not membership.
            assert!(crate::mir::resolved_control_flow::map_argument_outward(
                input,
                &OwnedExprSiteV1::new(declaration.owner(), arg_site),
                &call,
                1,
            )
            .is_err());
        })
        .unwrap();
}
