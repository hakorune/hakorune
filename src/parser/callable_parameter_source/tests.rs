use crate::ast::ParamDecl;
use crate::parser::source_authority::{
    ParserInvocationBrandV1, SourceBoxDeclarationSiteV1, SourceBoxMemberSiteV1,
    SourceBoxMethodSiteV1,
};
use crate::parser::source_path::SourceBoxDeclarationPathV1;
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};

use super::model::ParserCallableDeclarationKindV1;
use super::parse_product::ParsedCallableParameterListV1;
use super::project_neutral_parameter_syntax_v1;
use super::session::{CallableParameterSourceIssueV1, ParserCallableParameterSourceSessionV1};

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
    let (_, catalog) = parsed.into_ast_and_catalog();
    let declarations = catalog.declarations();

    assert_eq!(declarations.len(), 2);
    assert_eq!(
        declarations[0].kind(),
        ParserCallableDeclarationKindV1::StaticBoxMethod
    );
    assert_eq!(declarations[0].box_statement_ordinal(), 0);
    assert_eq!(declarations[0].source_member_ordinal(), 1);
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
    assert_eq!(declarations[1].parameters()[0].ordinal(), 0);
}

#[test]
fn unchanged_parser_scan_loop_box_has_four_methods_and_fifteen_rows() {
    let parsed = NyashParser::parse_from_string_with_callable_parameter_source(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        ParserBuildConfig::default(),
    )
    .unwrap();
    let (_, catalog) = parsed.into_ast_and_catalog();
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
}

#[test]
fn parameter_catalogs_keep_parser_invocation_identity() {
    let parse = || {
        NyashParser::parse_from_string_with_callable_parameter_source(
            "static box Source { run(value) { return value } }",
            ParserBuildConfig::default(),
        )
        .unwrap()
        .into_ast_and_catalog()
        .1
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
            ParserCallableDeclarationKindV1::StaticBoxMethod,
            "run".to_owned(),
            list(),
        )
        .unwrap();
    let error = session
        .commit(
            site(brand),
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
    let error = NyashParser::parse_from_string_with_callable_parameter_source(
        "gate Build.test { box Enabled { run(value) { return value } } } else { box Disabled {} }",
        ParserBuildConfig {
            mode: BuildMode::Test,
            ..ParserBuildConfig::default()
        },
    )
    .unwrap_err();
    assert!(matches!(
        error,
        crate::parser::ParseError::GrammarContract {
            stable_reject_tag: "parser/callable-parameter-source",
            ..
        }
    ));
}
