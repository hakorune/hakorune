//! Source-only Instance phase is distinct from executable forwarding.
use super::*;

#[test]
fn original_instance_forward_source_refuses_execution_and_snapshot_drift() {
    // The same forwarding source without a caller entry remains nonexecuting.
    // Original guarded Mul is kept, followed by the original borrowed call.
    let source = "box Transport { birth() { } probe(p): i64 {
        if p <= 8 { return p * 2 } local out = me.sink(p) return 0
    } sink(q): i64 { return 0 } } static box Main { main() { return 0 } }";
    let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(source).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let (site, rows) = ledger.borrowed_formal_actuals.iter()
        .find(|(_, rows)| matches!(rows, Ok(rows) if matches!(rows.phase, BorrowedCallActualEvidencePhaseV1::SourceInstance(_))))
        .expect("original source-only Instance forward retained");
    let rows = rows.as_ref().unwrap();
    assert!(rows.opaque_actuals.is_empty());
    assert!(rows
        .require_executable_v1()
        .unwrap_err()
        .contains("source-only-instance"));
    let ingress = ledger.borrowed_formal_source.as_ref().unwrap();
    let arguments = project_pending_instance_source_arguments_v1(
        ingress,
        &ledger.borrowed_formal_actuals,
        &package.parameter_contracts,
        site,
    )
    .unwrap()
    .unwrap();
    assert_eq!(arguments, rows.ordered_arguments);
    assert!(
        lend_pending_borrowed_arguments_v1(ingress, &ledger.borrowed_formal_actuals, site)
            .unwrap_err()
            .contains("source-only-instance")
    );
    let completion = ledger.completion_for_owner(site.owner()).unwrap();
    let flow = completion.cleanup().root_flow().unwrap();
    assert!(!flow.all_exits_ready());
    assert!(flow.local_calls().iter().all(|row| row.site() != site));
    for mutation in 0..5 {
        let mut pending = ledger.borrowed_formal_actuals.clone();
        let row = pending.get_mut(site).unwrap().as_mut().unwrap();
        let BorrowedCallActualEvidencePhaseV1::SourceInstance(identity) = &mut row.phase else {
            unreachable!()
        };
        match mutation {
            0 => identity.candidates[0].ordinal = 1,
            1 => identity.candidates[0].site = site.site().clone(),
            2 => identity.incoming_arguments = Box::new([]),
            3 => row.ordered_arguments = Box::new([]),
            _ => row.phase = BorrowedCallActualEvidencePhaseV1::Executable,
        }
        assert!(
            project_pending_instance_source_arguments_v1(
                ingress,
                &pending,
                &package.parameter_contracts,
                site,
            )
            .is_err(),
            "source snapshot mutation {mutation}"
        );
    }
    let site = site.clone();
    let mut pending = ledger.borrowed_formal_actuals.clone();
    pending.insert(site.clone(), Err("original-instance-source-error".into()));
    let ledger = std::rc::Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
    let ingress = ledger.borrowed_formal_source.as_mut().unwrap();
    ingress.as_mut().unwrap().incoming = Box::new([]);
    assert_eq!(
        project_pending_instance_source_arguments_v1(
            ingress,
            &pending,
            &package.parameter_contracts,
            &site,
        )
        .unwrap_err(),
        "original-instance-source-error"
    );
    pending.remove(&site);
    assert!(project_pending_instance_source_arguments_v1(
        ingress,
        &pending,
        &package.parameter_contracts,
        &site,
    )
    .unwrap_err()
    .contains("source-actuals-missing"));

    // A staged SourceInstance cannot survive loss of its original source row.
    let staged = ledger.borrowed_formal_actuals.clone();
    ingress
        .as_mut()
        .unwrap()
        .source_incoming
        .remove_exact_row_for_test(&site);
    assert!(project_pending_instance_source_arguments_v1(
        ingress,
        &staged,
        &package.parameter_contracts,
        &site,
    )
    .unwrap_err()
    .contains("source-phase-drift"));
}
