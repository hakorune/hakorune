use super::*;

#[test]
fn transaction_records_one_delegate_source_row_per_expose() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 5);
    let delegate = DelegateDecl::explicit_source(
        "inner".to_owned(),
        vec![
            crate::ast::DelegateExposeDecl {
                source_name: "length".to_owned(),
                exposed_name: "length".to_owned(),
            },
            crate::ast::DelegateExposeDecl {
                source_name: "size".to_owned(),
                exposed_name: "count".to_owned(),
            },
        ],
        transaction.current_member_ordinal(),
    );

    transaction
        .record_delegate_source_at_current(&delegate)
        .expect("parser delegate source rows should be recorded");
    transaction.finish_member().unwrap();

    let prepared = transaction.finish();
    let rows = prepared.delegate_source_declarations();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].expose_ordinal(), 0);
    assert_eq!(rows[0].delegate_field_name(), "inner");
    assert_eq!(rows[0].source_method_name(), "length");
    assert_eq!(rows[0].exposed_method_name(), "length");
    assert_eq!(rows[1].expose_ordinal(), 1);
    assert_eq!(rows[1].source_method_name(), "size");
    assert_eq!(rows[1].exposed_method_name(), "count");
    assert_eq!(rows[0].source_site().source_member_ordinal(), 0);
    assert_eq!(rows[1].source_site().source_member_ordinal(), 0);
}

#[test]
fn selected_gate_rebases_delegate_source_member_path() {
    let brand = ParserInvocationBrandV1::issue();
    let mut destination = OpenBoxMethodSourceTransactionV1::open(brand, 6);
    let mut selected = destination.branch();
    let delegate = DelegateDecl::explicit_source(
        "inner".to_owned(),
        vec![crate::ast::DelegateExposeDecl {
            source_name: "run".to_owned(),
            exposed_name: "run".to_owned(),
        }],
        selected.current_member_ordinal(),
    );
    selected
        .record_delegate_source_at_current(&delegate)
        .unwrap();
    selected.finish_member().unwrap();

    destination
        .try_merge_selected_gate(
            selected,
            crate::ast::BoxMemberGateSiteV1::from_box_member_ordinal(3),
        )
        .unwrap();
    let prepared = destination.finish();
    let row = &prepared.delegate_source_declarations()[0];
    assert_eq!(row.source_site().source_member_ordinal(), 0);
    assert!(matches!(
        row.source_site(),
        SourceBoxMethodSiteV1::SelectedBuildGate { path, .. } if path.len() == 1
    ));
}

#[test]
fn compatibility_delegate_cannot_enter_source_transport() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 7);
    let delegate = DelegateDecl::compatibility_only(
        "legacy".to_owned(),
        vec![crate::ast::DelegateExposeDecl {
            source_name: "run".to_owned(),
            exposed_name: "run".to_owned(),
        }],
        crate::ast::BoxMethodCompatibilityOriginV1::LegacyJsonV1,
    );

    assert!(matches!(
        transaction.record_delegate_source_at_current(&delegate),
        Err(ParseError::BuildCfg { .. })
    ));
    assert!(transaction.delegate_source_declarations().is_empty());
}
