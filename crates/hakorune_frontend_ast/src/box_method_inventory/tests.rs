use super::*;
use crate::DeclarationAttrs;

fn function(name: &str) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: Vec::new(),
        contracts: Vec::new(),
        uses: Vec::new(),
        is_static: false,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::new(0, 1, 1, 1),
    }
}

#[test]
fn direct_rows_keep_source_order_and_issue_ordinals() {
    let mut inventory = BoxMethodInventoryV1::empty();
    let zeta = inventory
        .try_push_explicit_source("zeta", function("zeta"), Span::new(0, 1, 1, 1))
        .unwrap();
    let alpha = inventory
        .try_push_explicit_source("alpha", function("alpha"), Span::new(2, 3, 2, 1))
        .unwrap();

    assert_eq!(zeta.selected_method_ordinal(), 0);
    assert_eq!(alpha.selected_method_ordinal(), 1);
    assert_eq!(
        inventory
            .iter_selected_declaration_order()
            .map(BoxMethodEntryV1::name)
            .collect::<Vec<_>>(),
        vec!["zeta", "alpha"]
    );
    assert_eq!(
        inventory
            .iter_compat_name_order()
            .map(BoxMethodEntryV1::name)
            .collect::<Vec<_>>(),
        vec!["alpha", "zeta"]
    );
}

#[test]
fn duplicate_rejects_without_mutation() {
    let mut inventory = BoxMethodInventoryV1::empty();
    let first_span = Span::new(0, 0, 3, 2);
    let duplicate_span = Span::new(0, 0, 8, 4);
    inventory
        .try_push_explicit_source("run", function("run"), first_span)
        .unwrap();

    let error = inventory
        .try_push_explicit_source("run", function("run"), duplicate_span)
        .unwrap_err();

    assert_eq!(
        error,
        BoxMethodInventoryErrorV1::DuplicateMethod {
            name: "run".into(),
            first_span,
            duplicate_span,
        }
    );
    assert_eq!(inventory.len(), 1);
}

#[test]
fn declaration_transform_preserves_inventory_metadata() {
    let mut inventory = BoxMethodInventoryV1::empty();
    let diagnostic_span = Span::new(0, 0, 4, 7);
    inventory
        .try_push_explicit_source("run", function("run"), diagnostic_span)
        .unwrap();
    let before = inventory.get("run").unwrap().clone();

    let transformed = inventory
        .try_map_declarations_preserving_metadata::<(), _>(|mut declaration| {
            let ASTNode::FunctionDeclaration { is_override, .. } = &mut declaration else {
                unreachable!()
            };
            *is_override = true;
            Ok(declaration)
        })
        .unwrap();
    let after = transformed.get("run").unwrap();

    assert_eq!(after.name(), before.name());
    assert_eq!(after.provenance(), before.provenance());
    assert_eq!(after.site(), before.site());
    assert_eq!(after.diagnostic_span(), diagnostic_span);
    assert!(matches!(
        after.declaration(),
        ASTNode::FunctionDeclaration {
            is_override: true,
            ..
        }
    ));
}

#[test]
fn declaration_transform_rejects_changed_method_name() {
    let mut inventory = BoxMethodInventoryV1::empty();
    inventory
        .try_push_explicit_source("run", function("run"), Span::unknown())
        .unwrap();

    let error = inventory
        .try_map_declarations_preserving_metadata::<(), _>(|mut declaration| {
            let ASTNode::FunctionDeclaration { name, .. } = &mut declaration else {
                unreachable!()
            };
            *name = "renamed".to_owned();
            Ok(declaration)
        })
        .unwrap_err();

    assert!(matches!(
        error,
        BoxMethodDeclarationTransformErrorV1::InvalidInventory(
            BoxMethodInventoryErrorV1::DeclarationNameMismatch { .. }
        )
    ));
}

#[test]
fn immutable_lookup_preserves_identity() {
    let mut inventory = BoxMethodInventoryV1::empty();
    let site = inventory
        .try_push_explicit_source("run", function("run"), Span::new(5, 8, 3, 2))
        .unwrap();
    let entry = inventory.get("run").expect("method must exist");
    assert_eq!(entry.site(), site);
    assert_eq!(entry.name(), "run");
    assert!(matches!(
        entry.declaration(),
        ASTNode::FunctionDeclaration { name, .. } if name == "run"
    ));
}

#[test]
fn selected_gate_merge_is_atomic_and_keeps_nested_path() {
    let mut destination = BoxMethodInventoryV1::empty();
    destination
        .try_push_explicit_source("outer", function("outer"), Span::unknown())
        .unwrap();

    let mut inner = BoxMethodInventoryV1::empty();
    inner
        .try_push_explicit_source("selected", function("selected"), Span::unknown())
        .unwrap();
    let mut selected = BoxMethodInventoryV1::empty();
    selected
        .try_merge_selected_gate(inner, &[4], BoxMemberGateSiteV1::from_box_member_ordinal(7))
        .unwrap();

    destination
        .try_merge_selected_gate(
            selected,
            &[9],
            BoxMemberGateSiteV1::from_box_member_ordinal(3),
        )
        .unwrap();

    let entry = destination.get("selected").unwrap();
    assert_eq!(entry.site().selected_method_ordinal(), 1);
    let Some(BoxMethodSourceSelectionV1::SelectedBuildGate { path }) =
        entry.provenance().explicit_source_selection()
    else {
        panic!("selected source must retain its gate path")
    };
    assert_eq!(path.len(), 2);
    assert_eq!(path[0].gate_site().box_member_ordinal(), 3);
    assert_eq!(path[0].branch_member_ordinal(), 9);
    assert_eq!(path[1].gate_site().box_member_ordinal(), 7);
    assert_eq!(path[1].branch_member_ordinal(), 4);
}

#[test]
fn selected_gate_rejects_missing_source_member_ordinals_without_mutation() {
    let mut destination = BoxMethodInventoryV1::empty();
    let before = destination.clone();
    let mut selected = BoxMethodInventoryV1::empty();
    selected
        .try_push_explicit_source("run", function("run"), Span::unknown())
        .unwrap();

    assert_eq!(
        destination
            .try_merge_selected_gate(
                selected,
                &[],
                BoxMemberGateSiteV1::from_box_member_ordinal(0),
            )
            .unwrap_err(),
        BoxMethodInventoryErrorV1::BranchMemberOrdinalCountMismatch {
            methods: 1,
            ordinals: 0,
        }
    );
    assert_eq!(destination, before);
}

#[test]
fn selected_gate_collision_leaves_destination_unchanged() {
    let mut destination = BoxMethodInventoryV1::empty();
    destination
        .try_push_explicit_source("run", function("run"), Span::unknown())
        .unwrap();
    let before = destination.clone();

    let mut selected = BoxMethodInventoryV1::empty();
    selected
        .try_push_explicit_source("run", function("run"), Span::unknown())
        .unwrap();

    let error = destination
        .try_merge_selected_gate(
            selected,
            &[0],
            BoxMemberGateSiteV1::from_box_member_ordinal(1),
        )
        .unwrap_err();
    assert!(matches!(
        error,
        BoxMethodInventoryErrorV1::DuplicateMethod { .. }
    ));
    assert_eq!(destination, before);
}

#[test]
fn generated_and_compatibility_rows_are_not_explicit_source() {
    let mut inventory = BoxMethodInventoryV1::empty();
    inventory
        .try_push_generated(
            "__get_name",
            function("__get_name"),
            BoxMethodGeneratedProvenanceV1::Property {
                property_name: "name".into(),
                selection: BoxMethodSourceSelectionV1::Direct,
            },
            Span::unknown(),
        )
        .unwrap();
    inventory
        .try_push_compatibility(
            "legacy",
            function("legacy"),
            BoxMethodCompatibilityOriginV1::LegacyJsonV1,
            Span::unknown(),
        )
        .unwrap();

    assert!(inventory
        .get("__get_name")
        .unwrap()
        .provenance()
        .explicit_source_selection()
        .is_none());
    assert!(inventory
        .get("legacy")
        .unwrap()
        .provenance()
        .explicit_source_selection()
        .is_none());
}

#[test]
fn compatibility_batch_is_atomic_and_never_source_authority() {
    let error = BoxMethodInventoryV1::try_from_compatibility_entries(
        vec![("run", function("run")), ("run", function("run"))],
        BoxMethodCompatibilityOriginV1::LegacyAstConstruction,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        BoxMethodInventoryErrorV1::DuplicateMethod { .. }
    ));

    let inventory = BoxMethodInventoryV1::try_from_compatibility_entries(
        vec![("zeta", function("zeta")), ("alpha", function("alpha"))],
        BoxMethodCompatibilityOriginV1::LegacyAstConstruction,
    )
    .unwrap();
    assert_eq!(
        inventory
            .iter_selected_declaration_order()
            .map(BoxMethodEntryV1::name)
            .collect::<Vec<_>>(),
        vec!["zeta", "alpha"]
    );
    assert!(inventory
        .iter_selected_declaration_order()
        .all(|entry| entry.provenance().explicit_source_selection().is_none()));
}

#[test]
fn generated_batch_rejects_internal_duplicates_before_publication() {
    let first_span = Span::new(0, 0, 2, 3);
    let duplicate_span = Span::new(0, 0, 4, 5);
    let rows = [
        PreparedGeneratedBoxMethodV1::new(
            "__get_value",
            function("__get_value"),
            BoxMethodGeneratedProvenanceV1::Property {
                property_name: "value".into(),
                selection: BoxMethodSourceSelectionV1::Direct,
            },
            first_span,
        )
        .unwrap(),
        PreparedGeneratedBoxMethodV1::new(
            "__get_value",
            function("__get_value"),
            BoxMethodGeneratedProvenanceV1::Property {
                property_name: "other".into(),
                selection: BoxMethodSourceSelectionV1::Direct,
            },
            duplicate_span,
        )
        .unwrap(),
    ];

    let error = PreparedGeneratedBoxMethodBatchV1::try_new(rows).unwrap_err();
    assert_eq!(
        error,
        BoxMethodInventoryErrorV1::DuplicateMethod {
            name: "__get_value".into(),
            first_span,
            duplicate_span,
        }
    );
}

#[test]
fn generated_batch_collision_leaves_destination_unchanged() {
    let mut destination = BoxMethodInventoryV1::empty();
    destination
        .try_push_explicit_source("run", function("run"), Span::new(0, 0, 1, 1))
        .unwrap();
    let before = destination.clone();
    let batch = PreparedGeneratedBoxMethodBatchV1::try_new([
        PreparedGeneratedBoxMethodV1::new(
            "helper",
            function("helper"),
            BoxMethodGeneratedProvenanceV1::Property {
                property_name: "helper".into(),
                selection: BoxMethodSourceSelectionV1::Direct,
            },
            Span::new(0, 0, 3, 1),
        )
        .unwrap(),
        PreparedGeneratedBoxMethodV1::new(
            "run",
            function("run"),
            BoxMethodGeneratedProvenanceV1::Property {
                property_name: "run".into(),
                selection: BoxMethodSourceSelectionV1::Direct,
            },
            Span::new(0, 0, 4, 1),
        )
        .unwrap(),
    ])
    .unwrap();

    assert!(matches!(
        destination.try_commit_generated_batch(batch),
        Err(BoxMethodInventoryErrorV1::DuplicateMethod { .. })
    ));
    assert_eq!(destination, before);
}
