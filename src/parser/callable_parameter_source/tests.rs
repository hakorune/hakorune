use crate::ast::{ASTNode, BoxMethodInventoryOrdinalV1, ParamDecl};
use crate::parser::source_authority::{
    ParserInvocationBrandV1, SourceBoxDeclarationSiteV1, SourceBoxMemberSiteV1,
    SourceBoxMethodSiteV1,
};
use crate::parser::callable_source_anchor::CallableDeclarationAnchorV1;
use crate::parser::source_path::SourceBoxDeclarationPathV1;
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

use super::model::ParserCallableDeclarationKindV1;
use super::parse_product::ParsedCallableParameterListV1;
use super::project_neutral_parameter_syntax_v1;
use super::session::{CallableParameterSourceIssueV1, ParserCallableParameterSourceSessionV1};
use super::syntax_loan::ParserCallableSyntaxLoanErrorV1;

fn first_method_inventory_ordinal() -> BoxMethodInventoryOrdinalV1 {
    let ASTNode::Program { statements, .. } =
        NyashParser::parse_from_string("box Placement { run(value) { return value } }").unwrap()
    else {
        unreachable!("fixture parses as Program")
    };
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        unreachable!("fixture contains one Box")
    };
    let ordinal = methods
        .iter_selected_declaration_order()
        .next()
        .expect("fixture method")
        .site();
    ordinal
}

fn test_callable_identity() -> crate::parser::CallableDeclarationIdentityV1 {
    CallableDeclarationAnchorV1::issue().identity()
}

#[test]
fn preserves_typed_and_untyped_parameter_syntax_in_source_order() {
    let declarations = vec![
        ParamDecl {
            name: "source".to_owned(),
            declared_type_name: None,
        },
        ParamDecl {
            name: "count".to_owned(),
            declared_type_name: Some("i64".to_owned()),
        },
    ];

    let rows = project_neutral_parameter_syntax_v1(&declarations, &[]);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name(), "source");
    assert_eq!(rows[0].declared_type_name(), None);
    assert_eq!(rows[1].name(), "count");
    assert_eq!(rows[1].declared_type_name(), Some("i64"));
}

#[test]
fn preserves_legacy_name_fallback_without_inventing_type_syntax() {
    let params = vec!["left".to_owned(), "right".to_owned()];
    let rows = project_neutral_parameter_syntax_v1(&[], &params);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name(), "left");
    assert_eq!(rows[1].name(), "right");
    assert!(rows.iter().all(|row| row.declared_type_name().is_none()));
}

#[test]
fn complete_sibling_catalog_covers_static_and_instance_direct_methods() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        r#"
static box StaticApi {
    field
    run(source, count: i64) { return count }
}
box InstanceApi {
    value
    read(offset) { return offset }
}
"#,
        ParserBuildConfig::default(),
    )
    .unwrap();
    parsed
        .with_callable_declaration_syntax(|catalog, _| {
            let declarations = catalog.declarations();

            assert_eq!(declarations.len(), 2);
            assert_eq!(
                declarations[0].kind(),
                ParserCallableDeclarationKindV1::StaticBoxMethod
            );
            assert_eq!(declarations[0].box_statement_ordinal(), 0);
            assert_eq!(declarations[0].source_member_ordinal(), 1);
            assert_eq!(declarations[0].inventory_ordinal().inventory_ordinal(), 0);
            assert_eq!(declarations[0].diagnostic_name(), "run");
            assert_eq!(declarations[0].parameters().len(), 2);
            assert_eq!(declarations[0].parameters()[0].name(), "source");
            assert_eq!(
                declarations[0].parameters()[0].declared_type().as_deref(),
                None
            );
            assert_eq!(
                declarations[0].parameters()[1].declared_type().as_deref(),
                Some("i64")
            );
            assert!(declarations[0]
                .parameters()
                .iter()
                .all(|row| row.transfer().is_ordinary()));

            assert_eq!(
                declarations[1].kind(),
                ParserCallableDeclarationKindV1::InstanceBoxMethod
            );
            assert_eq!(declarations[1].box_statement_ordinal(), 1);
            assert_eq!(declarations[1].source_member_ordinal(), 1);
            assert_eq!(declarations[1].inventory_ordinal().inventory_ordinal(), 0);
            assert_eq!(declarations[1].parameters()[0].ordinal(), 0);
        })
        .unwrap();
}

#[test]
fn consuming_syntax_loan_binds_exact_static_and_instance_declarations() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        r#"
static box StaticApi {
    field
    run(source, count: i64) { return count }
}
box InstanceApi {
    value
    read(offset) { return offset }
}
"#,
        ParserBuildConfig::default(),
    )
    .unwrap();

    let observed = parsed
        .with_callable_declaration_syntax(|catalog, loan| {
            assert_eq!(catalog.declarations().len(), loan.declarations().len());
            loan.declarations()
                .iter()
                .map(|row| {
                    let ASTNode::FunctionDeclaration {
                        name,
                        param_decls,
                        is_static,
                        ..
                    } = row.declaration()
                    else {
                        unreachable!("loan only retains exact function declarations")
                    };
                    (
                        row.source_row_index(),
                        name.clone(),
                        param_decls.len(),
                        *is_static,
                    )
                })
                .collect::<Vec<_>>()
        })
        .unwrap();

    assert_eq!(
        observed,
        [
            (0, "run".to_owned(), 2, true),
            (1, "read".to_owned(), 1, false),
        ]
    );
}

#[test]
fn unchanged_parser_scan_loop_box_has_four_methods_and_fifteen_rows() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        ParserBuildConfig::default(),
    )
    .unwrap();
    parsed
        .with_callable_declaration_syntax(|catalog, _| {
            let declarations = catalog.declarations();
            assert_eq!(
                declarations
                    .iter()
                    .map(|row| (row.diagnostic_name(), row.parameters().len()))
                    .collect::<Vec<_>>(),
                [
                    ("skip_while", 4),
                    ("scan_until_newline", 3),
                    ("scan_escape", 4),
                    ("scan_escape_piece_and_skip", 4),
                ]
            );
            assert_eq!(
                declarations
                    .iter()
                    .map(|row| row.parameters().len())
                    .sum::<usize>(),
                15
            );
            assert_eq!(declarations[0].parameters()[1].name(), "pos");
            assert_eq!(
                declarations[0].parameters()[1].declared_type().as_deref(),
                None
            );
        })
        .unwrap();
}

#[test]
fn parameter_catalogs_keep_parser_invocation_identity() {
    let parse = || {
        NyashParser::parse_from_string_with_callable_parameter_source(
            "static box Source { run(value) { return value } }",
            ParserBuildConfig::default(),
        )
        .unwrap()
        .with_callable_declaration_syntax(|catalog, _| catalog)
        .unwrap()
    };
    let first = parse();
    let second = parse();
    assert!(first.same_parser_source(&first));
    assert!(!first.same_parser_source(&second));
}

#[test]
fn source_session_rejects_foreign_and_duplicate_method_sites() {
    let brand = ParserInvocationBrandV1::issue();
    let foreign = ParserInvocationBrandV1::issue();
    let mut session = ParserCallableParameterSourceSessionV1::open(brand.clone());
    let list = || {
        ParsedCallableParameterListV1::from_neutral(vec![ParamDecl {
            name: "value".to_owned(),
            declared_type_name: None,
        }])
        .unwrap()
    };
    let site = |site_brand| SourceBoxMethodSiteV1::Direct {
        member: SourceBoxMemberSiteV1::new(
            SourceBoxDeclarationSiteV1::from_path(SourceBoxDeclarationPathV1::root(site_brand, 0)),
            0,
        ),
    };

    let error = session
        .commit(
            site(foreign),
            first_method_inventory_ordinal(),
            test_callable_identity(),
            ParserCallableDeclarationKindV1::StaticBoxMethod,
            "run".to_owned(),
            list(),
        )
        .unwrap_err();
    assert_eq!(
        error,
        CallableParameterSourceIssueV1::ForeignOrNonDirectMethod
    );

    session
        .commit(
            site(brand.clone()),
            first_method_inventory_ordinal(),
            test_callable_identity(),
            ParserCallableDeclarationKindV1::StaticBoxMethod,
            "run".to_owned(),
            list(),
        )
        .unwrap();
    let error = session
        .commit(
            site(brand),
            first_method_inventory_ordinal(),
            test_callable_identity(),
            ParserCallableDeclarationKindV1::StaticBoxMethod,
            "again".to_owned(),
            list(),
        )
        .unwrap_err();
    assert_eq!(
        error,
        CallableParameterSourceIssueV1::DuplicateMethodSite {
            statement: 0,
            member: 0,
        }
    );
}

#[test]
fn selected_build_gate_stays_outside_the_parameter_catalog_cohort() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        "gate Build.test { box Enabled { run(value) { return value } } } else { box Disabled { run(value) { return value } } }",
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .expect("total parser product keeps the explicit unsupported disposition");
    let error = parsed
        .with_callable_declaration_syntax(|_, _| ())
        .expect_err("unsupported gate must not expose an empty catalog");
    assert_eq!(
        error,
        ParserCallableSyntaxLoanErrorV1::ParameterSourceUnavailable
    );
}
