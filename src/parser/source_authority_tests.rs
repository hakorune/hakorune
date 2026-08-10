use super::*;
use crate::ast::{DeclarationAttrs, Span};

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

    let prepared = transaction.finish().unwrap();
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
