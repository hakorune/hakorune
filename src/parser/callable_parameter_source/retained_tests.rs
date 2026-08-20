use crate::ast::ASTNode;
use crate::parser::{NyashParser, ParserBuildConfig};

fn retained(source: &str) -> super::RetainedParserCallableSemanticSourceV1 {
    NyashParser::parse_from_string_with_callable_parameter_source(
        source,
        ParserBuildConfig::default(),
    )
    .unwrap()
    .into_retained_source()
    .unwrap()
}

#[test]
fn repeated_scoped_loans_retain_the_exact_declaration_rows() {
    let source = retained("static box Api { run(value) { return value } }");
    let observe = || {
        source
            .with_callable_declaration_syntax(|catalog, loan| {
                assert_eq!(catalog.declarations().len(), 1);
                assert_eq!(loan.declarations().len(), 1);
                let row = &loan.declarations()[0];
                let ASTNode::FunctionDeclaration { name, .. } = row.declaration() else {
                    unreachable!("exact loan retains one function declaration")
                };
                (
                    row.source_row_index(),
                    name.clone(),
                    row.declaration() as *const ASTNode as usize,
                )
            })
            .unwrap()
    };

    let first = observe();
    let second = observe();
    assert_eq!(first, second);
    assert_eq!(first.0, 0);
    assert_eq!(first.1, "run");
}

#[test]
fn equal_source_text_does_not_merge_parser_authority() {
    let first = retained("static box Api { run(value) { return value } }");
    let second = retained("static box Api { run(value) { return value } }");

    first
        .with_callable_declaration_syntax(|first_catalog, _| {
            second
                .with_callable_declaration_syntax(|second_catalog, _| {
                    assert!(!first_catalog.same_parser_source(second_catalog));
                })
                .unwrap();
        })
        .unwrap();
}
