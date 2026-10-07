//! The physical Instance-only iterator exposes Static source as an error.
use super::super::borrowed_formal_uses::{BorrowedIncomingCallDraftV1, BorrowedIncomingSourceV1};
use super::*;
#[test]
fn source_graph_physical_incoming_iterator_refuses_static_without_filtering() {
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog("static box Layout { class_id(p) { return 0 } } static box Main { main() { local k = Layout.class_id(7) return 0 } }").unwrap();
    let source = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger)
        .unwrap()
        .borrowed_formal_source
        .as_mut()
        .unwrap()
        .as_mut()
        .unwrap();
    let original = std::rc::Rc::clone(
        source
            .source_incoming
            .static_observations()
            .values()
            .next()
            .unwrap()
            .as_ref()
            .unwrap(),
    );
    let owner = original.callee_owner();
    source.incoming = vec![BorrowedIncomingCallDraftV1 {
        call: original.call_site().clone(),
        callee: owner,
        arguments: Box::new([]),
        source: BorrowedIncomingSourceV1::QualifiedStatic(original),
    }]
    .into_boxed_slice();
    let entry = BorrowedOrdinaryEntrySourceRefV1 {
        owner,
        formals: Box::new([]),
        source,
        incoming: Box::new([]),
    };
    let rows: Vec<_> = entry.incoming_targets().collect();
    assert_eq!(rows.len(), 1, "Static may not disappear through filter_map");
    assert!(rows[0]
        .as_ref()
        .unwrap_err()
        .contains("instance-source-required"));
}
