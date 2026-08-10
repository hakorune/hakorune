use crate::ast::{ASTNode, LiteralValue, Span};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::*;

fn parse(source: &str) -> ParsedNormalCallableProgramV1 {
    NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source")
}

fn transform(
    parsed: ParsedNormalCallableProgramV1,
    mutate: impl FnOnce(&mut ASTNode),
) -> Result<VerifiedFinalCallableProgramSourceV1, FinalCallableProgramSourceRejectV1> {
    let ParsedNormalCallableProgramV1::SourceBacked(initial) = parsed else {
        panic!("fixture must be source-backed")
    };
    let mut output = initial.ast().clone();
    mutate(&mut output);
    issue_final_callable_program_source_v1(initial, output)
}

#[test]
fn exact_static_callable_set_survives_one_transform() {
    let final_source = transform(parse("static box Scan { run(x) { return x } }"), |_| {})
        .expect("exact transform");
    assert_eq!(final_source.callable_count(), 1);
}

#[test]
fn non_callable_tail_may_change_without_reissuing_callable_identity() {
    let final_source = transform(parse("static box Scan { run(x) { return x } }"), |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        statements.push(ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        });
    })
    .expect("tail does not change callable set");
    assert_eq!(final_source.callable_count(), 1);
}

#[test]
fn added_or_changed_callable_rejects_without_compatibility_fallback() {
    let added = transform(parse("static box Scan { run(x) { return x } }"), |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        statements.push(ASTNode::FunctionDeclaration {
            name: "extra".to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            is_static: false,
            is_override: false,
            attrs: crate::ast::DeclarationAttrs::default(),
            uses: Vec::new(),
            contracts: Vec::new(),
            span: Span::unknown(),
        });
    });
    assert!(added.is_err(), "added callable must not retain old anchors");

    let changed = transform(parse("static box Scan { run(x) { return x } }"), |ast| {
        let ASTNode::Program { statements, .. } = ast else {
            unreachable!()
        };
        let ASTNode::BoxDeclaration { methods, .. } = &mut statements[0] else {
            unreachable!()
        };
        *methods = std::mem::take(methods)
            .map_declarations(|mut declaration| {
                let ASTNode::FunctionDeclaration { body, .. } = &mut declaration else {
                    unreachable!()
                };
                body.clear();
                declaration
            })
            .expect("valid transformed inventory");
    });
    assert!(matches!(
        changed,
        Err(FinalCallableProgramSourceRejectV1::CallableDeclarationChanged { row: 0 })
    ));
}
