use crate::parser::callable_parameter_source::{
    ParserNormalAppProgramItemLoanV1, ParserNormalAppResultSyntaxV1,
    ParserNormalProgramBodySyntaxKindV1, ParserNormalRootConsumerLoanRejectV1,
    ParserNormalRootConsumerLoanV1, ParserNormalRootPreservationIssuerV1,
    ParserNormalRootPreservationRejectV1, ParserNormalRootPreservationV1, ParserNormalRootRoleV1,
};
use crate::parser::initial_callable_program_source::InitialCallableFinalSlotV1;
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{ParsedNormalCallableProgramV1, PreparedNormalCallableProgramSourceV1};

fn prepared(source: &str) -> PreparedNormalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let ParsedNormalCallableProgramV1::SourceBacked(prepared) = parsed else {
        panic!("fixture must remain source-backed")
    };
    prepared
}

#[test]
fn app_root_relation_accepts_exact_main_with_top_level_callable_sibling() {
    let final_source =
        prepared("function helper() { return 2 }\nstatic box Main { main() { return 1 } }")
            .begin_transform()
            .finish_exact()
            .expect("exact App root relation");

    assert!(matches!(
        final_source.normal_root_source(),
        ParserNormalRootPreservationV1::Ready(preserved)
            if preserved.role() == ParserNormalRootRoleV1::App
    ));
}

#[test]
fn app_root_consumer_loan_hides_main_and_lends_only_root_body() {
    let final_source = prepared("static box Main { main() { return 1 } }")
        .begin_transform()
        .finish_exact()
        .expect("exact App root relation");

    let item_count = final_source
        .with_normal_root_consumer_loan(|loan| {
            let ParserNormalRootConsumerLoanV1::App(mut app) = loan else {
                panic!("fixture must remain App")
            };
            assert_eq!(
                app.root().result_syntax(),
                ParserNormalAppResultSyntaxV1::Implicit
            );
            assert_eq!(app.root().body().len(), 1);
            assert!(app.root().uses().is_empty());
            let _attrs = app.root().attrs();
            app.program_items()
                .map(|item| {
                    assert!(matches!(item, ParserNormalAppProgramItemLoanV1::RootMain));
                })
                .count()
        })
        .expect("App root consumer loan");

    assert_eq!(item_count, 1);
}

#[test]
fn app_root_consumer_loan_preserves_sibling_then_root_main_order() {
    let final_source =
        prepared("function helper() { return 2 }\nstatic box Main { main() { return 1 } }")
            .begin_transform()
            .finish_exact()
            .expect("exact App root relation");

    let items = final_source
        .with_normal_root_consumer_loan(|loan| {
            let ParserNormalRootConsumerLoanV1::App(mut app) = loan else {
                panic!("fixture must remain App")
            };
            app.program_items()
                .map(|item| match item {
                    ParserNormalAppProgramItemLoanV1::RootMain => (true, None),
                    ParserNormalAppProgramItemLoanV1::Sibling { kind, statement } => {
                        assert!(matches!(
                            statement,
                            crate::ast::ASTNode::FunctionDeclaration { .. }
                        ));
                        (false, Some(kind))
                    }
                })
                .collect::<Vec<_>>()
        })
        .expect("App root consumer loan");

    assert_eq!(
        items,
        vec![
            (
                false,
                Some(ParserNormalProgramBodySyntaxKindV1::FunctionDeclaration)
            ),
            (true, None),
        ]
    );
}

#[test]
fn empty_script_root_consumer_loan_is_complete_zero() {
    let final_source = prepared("")
        .begin_transform()
        .finish_exact()
        .expect("empty Script source");

    let statement_count = final_source
        .with_normal_root_consumer_loan(|loan| {
            let ParserNormalRootConsumerLoanV1::Script(mut script) = loan else {
                panic!("empty Program must remain Script")
            };
            script.statements().count()
        })
        .expect("empty Script root consumer loan");

    assert_eq!(statement_count, 0);
}

#[test]
fn nonempty_script_root_consumer_loan_keeps_paired_statement_order() {
    let final_source = prepared("print(1)\n")
        .begin_transform()
        .finish_exact()
        .expect("nonempty Script source");

    let rows = final_source
        .with_normal_root_consumer_loan(|loan| {
            let ParserNormalRootConsumerLoanV1::Script(mut script) = loan else {
                panic!("fixture must remain Script")
            };
            script
                .statements()
                .map(|statement| {
                    assert!(matches!(
                        statement.statement(),
                        crate::ast::ASTNode::Print { .. }
                    ));
                    statement.kind()
                })
                .collect::<Vec<_>>()
        })
        .expect("Script root consumer loan");

    assert_eq!(
        rows,
        vec![ParserNormalProgramBodySyntaxKindV1::ExecutableItem]
    );
}

#[test]
fn nonzero_main_arity_rejects_before_root_consumer_callback() {
    let final_source = prepared("static box Main { main(argument) { return argument } }")
        .begin_transform()
        .finish_exact()
        .expect("typed non-ready Main source");
    let mut callback_ran = false;

    let error = final_source
        .with_normal_root_consumer_loan(|_| callback_ran = true)
        .expect_err("nonzero Main arity must remain terminal");

    assert!(matches!(
        error,
        ParserNormalRootConsumerLoanRejectV1::Outside(_)
    ));
    assert!(!callback_ran);
}

#[test]
fn app_root_relation_rejects_structurally_equal_foreign_callable_identity() {
    let first = prepared("static box Main { main() { return 1 } }");
    let foreign = prepared("static box Main { main() { return 1 } }");
    let (ast, _, _, _, authority, _, root) = first.into_transform_parts();
    let (_, foreign_rows, foreign_slots, _, _, _, _) = foreign.into_transform_parts();

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root,
        &authority,
        &ast,
        &ast,
        &foreign_rows,
        &foreign_slots,
    )
    .expect_err("foreign opaque identity must not pair by shape or ordinal");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::AppCallableIdentityMissing
    );
}

#[test]
fn app_root_relation_rejects_foreign_parser_witness_before_pairing() {
    let first = prepared("static box Main { main() { return 1 } }");
    let foreign = prepared("static box Main { main() { return 1 } }");
    let (ast, rows, slots, _, _, _, root) = first.into_transform_parts();
    let (_, _, _, _, foreign_authority, _, _) = foreign.into_transform_parts();

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root,
        &foreign_authority,
        &ast,
        &ast,
        &rows,
        &slots,
    )
    .expect_err("foreign parser witness must reject before App pairing");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::ParserWitnessMismatch
    );
}

#[test]
fn app_root_relation_rejects_unpaired_final_slot() {
    let source = prepared("static box Main { main() { return 1 } }");
    let (ast, rows, slots, _, authority, _, root) = source.into_transform_parts();
    let mut slots = slots.into_vec();
    slots[0] = InitialCallableFinalSlotV1::TopLevel { statement: 0 };

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root, &authority, &ast, &ast, &rows, &slots,
    )
    .expect_err("App identity must retain its paired BoxMethod slot");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::AppCallableFinalSlotMismatch
    );
}

#[test]
fn app_root_relation_rejects_callable_pairing_cardinality_drift() {
    let source = prepared("static box Main { main() { return 1 } }");
    let (ast, rows, _, _, authority, _, root) = source.into_transform_parts();

    let error = ParserNormalRootPreservationIssuerV1::seal_after_transform(
        root,
        &authority,
        &ast,
        &ast,
        &rows,
        &[],
    )
    .expect_err("parallel callable/slot drift must reject");

    assert_eq!(
        error,
        ParserNormalRootPreservationRejectV1::CallablePairingCardinalityMismatch {
            sources: 1,
            slots: 0,
        }
    );
}

#[test]
fn main_helper_stays_terminal_before_app_root_relation() {
    let final_source = prepared("static box Main { main() { return 1 } helper() { return 2 } }")
        .begin_transform()
        .finish_exact()
        .expect("typed non-ready Main source");

    assert!(matches!(
        final_source.normal_root_source(),
        ParserNormalRootPreservationV1::Terminal(_)
    ));

    let mut callback_ran = false;
    let error = final_source
        .with_normal_root_consumer_loan(|_| callback_ran = true)
        .expect_err("Main helper must stay terminal before the loan");
    assert!(matches!(
        error,
        ParserNormalRootConsumerLoanRejectV1::Outside(_)
    ));
    assert!(!callback_ran);
}
