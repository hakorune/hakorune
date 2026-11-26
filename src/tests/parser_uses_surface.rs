use crate::ast::ASTNode;
use crate::tests::helpers::parser::program_statements;

#[test]
fn parser_uses_surface_parses_method_uses_metadata_only() {
    let statements = program_statements(
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
    let ASTNode::FunctionDeclaration { uses, body, .. } = methods.get_declaration("main").unwrap()
    else {
        panic!("expected main method");
    };

    assert_eq!(uses, &vec!["osvm".to_string(), "rawbuf".to_string()]);
    assert_eq!(body.len(), 1, "Stage0 must not inject capability checks");
}

#[test]
fn parser_uses_surface_refuses_file_function_before_metadata() {
    let error = crate::parser::NyashParser::parse_from_string(
        "function reserve(size: i64): i64 uses osvm { return size }",
    )
    .expect_err("retired syntax must be rejected");
    assert!(
        error.to_string().contains("decl/function-outside-box"),
        "{error}"
    );
}

#[test]
fn parser_uses_surface_keeps_uses_contextual() {
    let statements = program_statements(
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
    let ASTNode::FunctionDeclaration { uses, body, .. } = methods.get_declaration("main").unwrap()
    else {
        panic!("expected main method");
    };

    assert!(uses.is_empty());
    assert_eq!(body.len(), 2);
}
