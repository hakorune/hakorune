use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn parse_program(source: &str) -> Vec<ASTNode> {
    let ast = NyashParser::parse_from_string(source).expect("parse uses surface");
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
}

#[test]
fn parser_uses_surface_parses_method_uses_metadata_only() {
    let statements = parse_program(
        r#"
static box Main {
    main(size: i64): i64
        uses osvm, rawbuf
    {
        return size
    }
}
"#,
    );
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        panic!("expected box declaration");
    };
    let ASTNode::FunctionDeclaration { uses, body, .. } = &methods["main"] else {
        panic!("expected main method");
    };

    assert_eq!(uses, &vec!["osvm".to_string(), "rawbuf".to_string()]);
    assert_eq!(body.len(), 1, "Stage0 must not inject capability checks");
}

#[test]
fn parser_uses_surface_parses_free_function_uses_metadata() {
    let statements = parse_program(
        r#"
function reserve(size: i64): i64
    uses osvm
{
    return size
}
"#,
    );
    let ASTNode::FunctionDeclaration { uses, body, .. } = &statements[0] else {
        panic!("expected function declaration");
    };

    assert_eq!(uses, &vec!["osvm".to_string()]);
    assert_eq!(body.len(), 1);
}

#[test]
fn parser_uses_surface_keeps_uses_contextual() {
    let statements = parse_program(
        r#"
static box Main {
    main() {
        local uses = 1
        return uses
    }
}
"#,
    );
    let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
        panic!("expected box declaration");
    };
    let ASTNode::FunctionDeclaration { uses, body, .. } = &methods["main"] else {
        panic!("expected main method");
    };

    assert!(uses.is_empty());
    assert_eq!(body.len(), 2);
}
