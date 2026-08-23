use crate::ast::{ASTNode, LiteralValue, Span};
use crate::parser::{NyashParser, ParserBuildConfig, ParserNormalRootExecutionRoleV1};

use super::{
    FinalCallableProgramSourceRejectV1, ParsedNormalCallableProgramV1,
    ParserNormalRootExecutionPreservationRejectV1, PreparedNormalCallableProgramSourceV1,
    VerifiedFinalCallableProgramSourceV1,
};

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

fn transform_root(
    source: &str,
    mutate: impl FnOnce(&mut ASTNode),
) -> Result<VerifiedFinalCallableProgramSourceV1, FinalCallableProgramSourceRejectV1> {
    prepared(source)
        .begin_transform()
        .finish_test_transform(|ast| {
            let mut output = ast.clone();
            mutate(&mut output);
            output
        })
}

#[test]
fn exact_app_relation_preserves_main_and_static_children_as_one_aggregate() {
    let final_source = prepared(
        "function helper() { return 2 }\nstatic box Main { main() { return 1 } child() { return 3 } }",
    )
    .begin_transform()
    .finish_exact()
    .expect("exact App root relation");

    let source = final_source
        .normal_root_execution()
        .ready_source()
        .expect("ready source relation");
    assert_eq!(source.role(), ParserNormalRootExecutionRoleV1::App);
    let app = source.app_relation().expect("App relation");
    assert_eq!(app.static_children().len(), 1);
    final_source.discard_at_named_root_execution_terminal();
}

#[test]
fn total_root_preservation_rejects_foreign_parser_witness() {
    let first = prepared("static box Main { main() { return 1 } }");
    let foreign = prepared("static box Main { main() { return 1 } }");
    let error = super::transform::reject_foreign_root_authority_for_test(first, foreign);

    assert_eq!(
        error,
        ParserNormalRootExecutionPreservationRejectV1::ParserWitnessMismatch
    );
}

#[test]
fn main_helper_is_preserved_as_app_source_not_downgraded_to_terminal() {
    let final_source = prepared("static box Main { main() { return 1 } helper() { return 2 } }")
        .begin_transform()
        .finish_exact()
        .expect("total App source");

    assert!(matches!(
        final_source.normal_root_execution().ready_source(),
        Some(source) if source.role() == ParserNormalRootExecutionRoleV1::App
    ));
    final_source.discard_at_named_root_execution_terminal();
}

#[test]
fn preservation_rejects_root_statement_replacement() {
    let result = transform_root("print(1)", |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        let ASTNode::Print { span, .. } = &mut statements[0] else {
            unreachable!()
        };
        *span = Span::new(span.start, span.end, span.line + 1, span.column);
    });
    assert!(matches!(
        result,
        Err(FinalCallableProgramSourceRejectV1::RootPreservation(
            ParserNormalRootExecutionPreservationRejectV1::SourceStatementChanged { position: 0 }
        ))
    ));
}

#[test]
fn preservation_rejects_root_statement_addition() {
    let result = transform_root("print(1)", |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        statements.push(ASTNode::Literal {
            value: LiteralValue::Integer(2),
            span: Span::unknown(),
        });
    });
    assert!(matches!(
        result,
        Err(FinalCallableProgramSourceRejectV1::RootPreservation(
            ParserNormalRootExecutionPreservationRejectV1::SourceBodyCardinalityMismatch {
                source: 1,
                initial: 1,
                transformed: 2,
            }
        ))
    ));
}

#[test]
fn preservation_rejects_root_statement_removal() {
    let result = transform_root("print(1)\nprint(2)", |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        statements.pop();
    });
    assert!(matches!(
        result,
        Err(FinalCallableProgramSourceRejectV1::ProgramSource(
            crate::parser::callable_parameter_source::
                ParserNormalProgramSourceTransformRejectV1::BodyCountChanged
        ))
    ));
}

#[test]
fn preservation_rejects_root_statement_reorder() {
    let result = transform_root("print(1)\nprint(2)", |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        statements.swap(0, 1);
    });
    assert!(matches!(
        result,
        Err(FinalCallableProgramSourceRejectV1::RootPreservation(
            ParserNormalRootExecutionPreservationRejectV1::SourceStatementChanged { position: 0 }
        ))
    ));
}
