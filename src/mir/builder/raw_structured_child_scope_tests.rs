use super::{PreparedRawChildSourceV1, RawStructuredChildScopePortV1};

#[test]
fn exact_demand_terminal_rejects_unconsumed_receipts() {
    let mut child = ();
    assert!(
        RawStructuredChildScopePortV1::new(&mut child, Vec::new(), Vec::new())
            .complete_exact_demands_v1()
            .is_ok()
    );

    let error = RawStructuredChildScopePortV1::new(
        &mut child,
        Vec::new(),
        vec![PreparedRawChildSourceV1::Preserve],
    )
    .complete_exact_demands_v1()
    .unwrap_err();
    assert_eq!(
        error,
        "[freeze:contract][raw-structured/unconsumed-demands] expressions=0 bodies=1"
    );
}

#[test]
fn failed_child_error_is_not_masked_by_later_expression_demand() {
    let mut child = ();
    let error = RawStructuredChildScopePortV1::new(
        &mut child,
        vec![
            PreparedRawChildSourceV1::Preserve,
            PreparedRawChildSourceV1::Preserve,
        ],
        Vec::new(),
    )
    .complete_after_result_v1::<()>(Err("[primary-child-error]".to_owned()))
    .unwrap_err();
    assert_eq!(error, "[primary-child-error]");
}
