use super::*;
use crate::ast::{DeclarationAttrs, Span};
use crate::parser::NyashParser;

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
        span: Span::unknown(),
    }
}

#[test]
fn transaction_co_seals_explicit_source_with_inventory_placement() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 4);
    let site = SourceBoxMethodSiteV1::Direct {
        member: transaction.current_member_site(),
    };
    let ordinal = transaction
        .commit_explicit_method(site, "length", function("length"), Span::unknown())
        .unwrap();
    transaction.finish_member().unwrap();

    let prepared = transaction
        .finish(&std::collections::HashMap::new())
        .unwrap();
    assert_eq!(prepared.box_site().statement_ordinal(), 4);
    assert_eq!(prepared.inventory().len(), 1);
    assert_eq!(prepared.method_relations().len(), 1);
    let MethodSourceRelationV1::Explicit(relation) = &prepared.method_relations()[0] else {
        panic!("direct method must produce an explicit source relation")
    };
    assert_eq!(relation.inventory_ordinal(), ordinal);
    assert_eq!(relation.name(), "length");
}

#[test]
fn foreign_invocation_site_is_rejected_before_inventory_mutation() {
    let brand = ParserInvocationBrandV1::issue();
    let foreign = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 1);
    let foreign_site = SourceBoxMethodSiteV1::Direct {
        member: SourceBoxMemberSiteV1 {
            box_site: SourceBoxDeclarationSiteV1 {
                path: SourceBoxDeclarationPathV1::root(foreign, 1),
            },
            member_ordinal: 0,
        },
    };

    assert_eq!(
        transaction
            .commit_explicit_method(foreign_site, "length", function("length"), Span::unknown())
            .unwrap_err(),
        SourceAuthorityErrorV1::ForeignBoxSite
    );
    assert!(transaction.inventory().is_empty());
}

#[test]
fn brand_is_identity_not_value_equality() {
    let left = ParserInvocationBrandV1::issue();
    let right = ParserInvocationBrandV1::issue();
    assert_ne!(left, right);
    assert_eq!(left, left.clone());
}

#[test]
fn duplicate_source_name_is_rejected_without_partial_relation() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 2);
    let site = SourceBoxMethodSiteV1::Direct {
        member: transaction.current_member_site(),
    };
    transaction
        .commit_explicit_method(site.clone(), "length", function("length"), Span::unknown())
        .unwrap();
    transaction.finish_member().unwrap();
    let duplicate_site = SourceBoxMethodSiteV1::Direct {
        member: transaction.current_member_site(),
    };

    let error = transaction
        .commit_explicit_method(
            duplicate_site,
            "length",
            function("length"),
            Span::unknown(),
        )
        .unwrap_err();
    assert!(matches!(
        error,
        SourceAuthorityErrorV1::Inventory(BoxMethodInventoryErrorV1::DuplicateMethod { .. })
    ));
    assert_eq!(transaction.inventory().len(), 1);
}

#[test]
fn stale_same_box_member_site_is_rejected_before_inventory_mutation() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 3);
    let stale_site = SourceBoxMethodSiteV1::Direct {
        member: transaction.current_member_site(),
    };
    transaction.finish_member().unwrap();

    assert_eq!(
        transaction
            .commit_explicit_method(stale_site, "length", function("length"), Span::unknown())
            .unwrap_err(),
        SourceAuthorityErrorV1::StaleMemberSite
    );
    assert!(transaction.inventory().is_empty());
}

fn constructor(name: &str, arity: usize) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: (0..arity).map(|index| format!("p{index}")).collect(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: Vec::new(),
        contracts: Vec::new(),
        uses: Vec::new(),
        is_static: false,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

#[test]
fn constructor_inventory_preserves_written_member_order() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 8);
    let mut constructors = std::collections::HashMap::new();
    for (key, node) in [
        ("init/2", constructor("init", 2)),
        ("pack/1", constructor("pack", 1)),
        ("birth/0", constructor("birth", 0)),
    ] {
        transaction
            .commit_constructor_at_current(key, &node)
            .unwrap();
        constructors.insert(key.to_owned(), node);
        transaction.finish_member().unwrap();
    }

    let prepared = transaction.finish(&constructors).unwrap();
    let rows = prepared.constructor_relations();
    assert_eq!(
        rows.iter().map(|row| row.key()).collect::<Vec<_>>(),
        vec!["init/2", "pack/1", "birth/0"]
    );
    for (ordinal, row) in rows.iter().enumerate() {
        assert!(matches!(
            row.origin(),
            ConstructorSourceOriginV1::Direct(site)
                if site.source_member_ordinal() == ordinal as u32
        ));
    }
}

#[test]
fn duplicate_constructor_key_rejects_before_map_overwrite() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 9);
    let node = constructor("init", 1);
    transaction
        .commit_constructor_at_current("init/1", &node)
        .unwrap();
    transaction.finish_member().unwrap();

    let error = transaction
        .commit_constructor_at_current("init/1", &node)
        .expect_err("same key must not be overwritten");
    assert!(error
        .to_string()
        .contains("duplicate constructor source key"));
}

#[test]
fn selected_gate_rebases_constructor_site_and_rejects_key_collision() {
    let brand = ParserInvocationBrandV1::issue();
    let mut destination = OpenBoxMethodSourceTransactionV1::open(brand, 11);
    let direct = constructor("init", 1);
    destination
        .commit_constructor_at_current("init/1", &direct)
        .unwrap();
    destination.finish_member().unwrap();

    let mut selected = destination.branch();
    let gated = constructor("pack", 1);
    selected
        .commit_constructor_at_current("pack/1", &gated)
        .unwrap();
    selected.finish_member().unwrap();
    destination
        .try_merge_selected_gate(
            selected,
            crate::ast::BoxMemberGateSiteV1::from_box_member_ordinal(7),
        )
        .unwrap();
    let mut constructors = std::collections::HashMap::new();
    constructors.insert("init/1".to_owned(), direct.clone());
    constructors.insert("pack/1".to_owned(), gated);
    let prepared = destination.finish(&constructors).unwrap();
    let gated_row = prepared
        .constructor_relations()
        .iter()
        .find(|row| row.key() == "pack/1")
        .expect("selected constructor row");
    assert!(matches!(
        gated_row.origin(),
        ConstructorSourceOriginV1::Direct(SourceBoxMethodSiteV1::SelectedBuildGate {
            path,
            ..
        }) if path.len() == 1
    ));

    let brand = ParserInvocationBrandV1::issue();
    let mut collision = OpenBoxMethodSourceTransactionV1::open(brand, 12);
    collision
        .commit_constructor_at_current("init/1", &direct)
        .unwrap();
    collision.finish_member().unwrap();
    let mut duplicate = collision.branch();
    duplicate
        .commit_constructor_at_current("init/1", &direct)
        .unwrap();
    duplicate.finish_member().unwrap();
    let error = collision
        .try_merge_selected_gate(
            duplicate,
            crate::ast::BoxMemberGateSiteV1::from_box_member_ordinal(8),
        )
        .expect_err("selected branch must not overwrite a constructor key");
    assert!(error
        .to_string()
        .contains("selected gate duplicates constructor source key `init/1`"));
}

#[test]
fn generated_birth_records_exact_initializer_trigger() {
    let brand = ParserInvocationBrandV1::issue();
    let mut transaction = OpenBoxMethodSourceTransactionV1::open(brand, 10);
    transaction.record_generated_birth_trigger_at_current(
        GeneratedBirthTriggerKindV1::StoredFieldInitializer,
    );
    transaction.finish_member().unwrap();
    let mut constructors = std::collections::HashMap::new();
    constructors.insert("birth/0".to_owned(), constructor("birth", 0));

    let prepared = transaction.finish(&constructors).unwrap();
    let [row] = prepared.constructor_relations() else {
        panic!("one generated birth row expected")
    };
    assert!(matches!(
        row.origin(),
        ConstructorSourceOriginV1::GeneratedBirthInitializer
    ));
    assert_eq!(row.initializer_triggers().len(), 1);
    assert_eq!(
        row.initializer_triggers()[0].kind(),
        GeneratedBirthTriggerKindV1::StoredFieldInitializer
    );
    assert_eq!(
        row.initializer_triggers()[0]
            .source_site()
            .source_member_ordinal(),
        0
    );
}

#[test]
fn constructor_seal_rejects_missing_and_malformed_ast_coverage() {
    let brand = ParserInvocationBrandV1::issue();
    let mut missing = OpenBoxMethodSourceTransactionV1::open(brand, 13);
    let node = constructor("init", 1);
    missing
        .commit_constructor_at_current("init/1", &node)
        .unwrap();
    missing.finish_member().unwrap();
    assert!(missing
        .finish(&std::collections::HashMap::new())
        .expect_err("missing AST constructor must reject")
        .to_string()
        .contains("coverage mismatch"));

    let brand = ParserInvocationBrandV1::issue();
    let malformed = OpenBoxMethodSourceTransactionV1::open(brand, 14);
    let mut constructors = std::collections::HashMap::new();
    constructors.insert(
        "init/1".to_owned(),
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: Span::unknown(),
        },
    );
    assert!(malformed
        .finish(&constructors)
        .expect_err("non-function constructor map row must reject")
        .to_string()
        .contains("coverage mismatch"));
}

#[test]
fn rich_parser_seal_retains_constructor_order_and_generated_origin() {
    let direct = NyashParser::parse_from_string_with_source_seal(
        "box Page { init(a, b) {} pack(a) {} birth() {} }",
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("direct constructors should seal");
    let [seal] = direct.source_seals() else {
        panic!("one Box seal expected")
    };
    assert_eq!(
        seal.constructor_relations()
            .iter()
            .map(|row| row.key())
            .collect::<Vec<_>>(),
        vec!["init/2", "pack/1", "birth/0"]
    );

    let generated = NyashParser::parse_from_string_with_source_seal(
        "box Page { value = 1 }",
        crate::parser::ParserBuildConfig::default(),
    )
    .expect("field initializer should issue generated birth source");
    let [seal] = generated.source_seals() else {
        panic!("one generated Box seal expected")
    };
    let [row] = seal.constructor_relations() else {
        panic!("one generated birth relation expected")
    };
    assert!(matches!(
        row.origin(),
        ConstructorSourceOriginV1::GeneratedBirthInitializer
    ));
}

#[test]
fn rich_parser_rejects_duplicate_constructor_before_last_write() {
    let error = NyashParser::parse_from_string_with_source_seal(
        "box Page { init(a) {} init(b) {} }",
        crate::parser::ParserBuildConfig::default(),
    )
    .expect_err("same constructor key must not overwrite its first source row");
    assert!(error
        .to_string()
        .contains("duplicate constructor source key `init/1`"));
}
