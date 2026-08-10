use super::*;
use crate::ast::{ASTNode, BoxMethodCompatibilityOriginV1, BoxMethodInventoryV1};
use crate::parser::postpass_envelope::{CompletedParserPostpassV1, PostpassDemandV1};
use crate::parser::{BuildMode, NyashParser, ParserBuildConfig};
use crate::tokenizer::NyashTokenizer;
use std::collections::HashMap;

fn finish(source: &str, mode: BuildMode) -> CompletedParserPostpassV1 {
    let tokens = NyashTokenizer::new(source).tokenize().unwrap();
    let mut parser = NyashParser::new(tokens).with_build_config(ParserBuildConfig {
        mode,
        ..ParserBuildConfig::default()
    });
    let ast = parser.parse_program().unwrap();
    parser
        .open_postpass_product(ast)
        .unwrap()
        .finish_total_s0(&parser, PostpassDemandV1::default())
        .unwrap()
}

fn open_pruned(
    source: &str,
    mode: BuildMode,
) -> (
    NyashParser,
    crate::parser::source_seal::OpenParserPostpassProductV1,
) {
    let tokens = NyashTokenizer::new(source).tokenize().unwrap();
    let mut parser = NyashParser::new(tokens).with_build_config(ParserBuildConfig {
        mode,
        ..ParserBuildConfig::default()
    });
    let ast = parser.parse_program().unwrap();
    let product = parser
        .open_postpass_product(ast)
        .unwrap()
        .prune_build_gates(&parser)
        .unwrap();
    (parser, product)
}

fn loaned_names(completed: &CompletedParserPostpassV1) -> Vec<String> {
    completed
        .initial_callable_source()
        .expect("fixture must enter the initial callable semantic lane")
        .with_callable_syntax(|loan| {
            loan.rows()
                .iter()
                .map(|row| {
                    assert!(matches!(
                        row.declaration(),
                        ASTNode::FunctionDeclaration { .. }
                    ));
                    assert!(row.anchor().same_as(row.source().anchor()));
                    row.source().diagnostic_name().to_owned()
                })
                .collect()
        })
}

#[test]
fn co_seal_covers_mixed_direct_program_without_name_repair() {
    let completed = finish(
        "function free() {}\n\
         static function free_static() {}\n\
         static box Main { main() {} }\n\
         static box Utility { ping() {} }\n\
         box Node { value() {} }\n",
        BuildMode::Release,
    );
    assert_eq!(
        loaned_names(&completed),
        ["free", "free_static", "main", "ping", "value"]
    );
}

#[test]
fn co_seal_uses_selected_top_level_and_member_gate_receipts() {
    let completed = finish(
        "gate Build.test { function chosen() {} } else { function hidden() {} }\n\
         box Choice { gate Build.test { run() {} } else { run() {} } }\n",
        BuildMode::Test,
    );
    assert_eq!(loaned_names(&completed), ["chosen", "run"]);
}

#[test]
fn co_seal_covers_generated_property_and_delegate_origins() {
    let property = finish(
        "box Generated { once value: i64 => 1 }\n",
        BuildMode::Release,
    );
    assert_eq!(
        loaned_names(&property),
        ["__compute_once_value", "__get_once_value"]
    );

    let delegate = finish(
        "box Target { run() { return 1 } }\n\
         box Host {\n\
           target: Target\n\
           delegate target exposes { run as runAlias }\n\
         }\n",
        BuildMode::Release,
    );
    assert_eq!(loaned_names(&delegate), ["run", "runAlias"]);
}

#[test]
fn syntax_loan_is_repeatable_but_never_splits_the_program() {
    let completed = finish("box Plain { run() {} }\n", BuildMode::Release);
    let first = loaned_names(&completed);
    let second = loaned_names(&completed);
    assert_eq!(first, second);
}

#[test]
fn issuer_rejects_missing_rows_and_arbitrary_ast() {
    let (_parser, product) = open_pruned("box Plain { run() {} }\n", BuildMode::Release);
    let mut rows = product.source_session.callable_rows;
    rows.clear();
    let error = issue_initial_callable_program_source_v1(
        product.ast,
        rows.into_boxed_slice(),
        product.projected_program_item_slots,
        &product.source_session.prepared_source_seals,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        InitialCallableProgramSourceRejectV1::CallableCoverageMismatch {
            expected: 1,
            actual: 0
        }
    ));

    let (_parser, product) = open_pruned("function run() {}\n", BuildMode::Release);
    let error = issue_initial_callable_program_source_v1(
        ASTNode::Literal {
            value: crate::ast::LiteralValue::Integer(1),
            span: crate::ast::Span::unknown(),
        },
        product.source_session.callable_rows.into_boxed_slice(),
        product.projected_program_item_slots,
        &product.source_session.prepared_source_seals,
    )
    .unwrap_err();
    assert_eq!(error, InitialCallableProgramSourceRejectV1::NotProgram);
}

#[test]
fn issuer_rejects_foreign_slots_and_compatibility_only_methods() {
    let (_left_parser, left) = open_pruned("function left() {}\n", BuildMode::Release);
    let (_right_parser, right) = open_pruned("function right() {}\n", BuildMode::Release);
    let error = issue_initial_callable_program_source_v1(
        left.ast,
        left.source_session.callable_rows.into_boxed_slice(),
        right.projected_program_item_slots,
        &left.source_session.prepared_source_seals,
    )
    .unwrap_err();
    assert_eq!(error, InitialCallableProgramSourceRejectV1::ForeignParser);

    let (_parser, mut product) = open_pruned("box Plain { run() {} }\n", BuildMode::Release);
    let ASTNode::Program { statements, .. } = &mut product.ast else {
        unreachable!()
    };
    let ASTNode::BoxDeclaration { methods, .. } = &mut statements[0] else {
        unreachable!()
    };
    let declaration = methods
        .iter_selected_declaration_order()
        .next()
        .unwrap()
        .declaration()
        .clone();
    *methods = BoxMethodInventoryV1::try_from_compatibility_map(
        HashMap::from([("run".to_owned(), declaration)]),
        BoxMethodCompatibilityOriginV1::LegacyAstConstruction,
    )
    .unwrap();
    let error = issue_initial_callable_program_source_v1(
        product.ast,
        product.source_session.callable_rows.into_boxed_slice(),
        product.projected_program_item_slots,
        &product.source_session.prepared_source_seals,
    )
    .unwrap_err();
    assert_eq!(
        error,
        InitialCallableProgramSourceRejectV1::UnsupportedMethodProvenance
    );
}
