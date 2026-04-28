use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn parse(src: &str) -> ASTNode {
    std::env::set_var("NYASH_ENABLE_UNIFIED_MEMBERS", "1");
    NyashParser::parse_from_string(src).expect("parse ok")
}

fn find_box<'a>(ast: &'a ASTNode, box_name: &str) -> &'a ASTNode {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    statements
        .iter()
        .find(|stmt| matches!(stmt, ASTNode::BoxDeclaration { name, .. } if name == box_name))
        .expect("box should exist")
}

fn has_method(methods: &std::collections::HashMap<String, ASTNode>, name: &str) -> bool {
    methods.contains_key(name)
}

#[test]
fn block_first_computed_uses_computed_getter_not_birth_once() {
    let ast = parse(
        r#"
box Shape {
  { 7 } as size: IntegerBox
}
"#,
    );

    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Shape") else {
        panic!("expected BoxDeclaration");
    };

    assert!(has_method(methods, "__get_size"));
    assert!(!has_method(methods, "__compute_birth_size"));
    assert!(!has_method(methods, "__get_birth_size"));
}

#[test]
fn once_emit_is_shared_for_header_and_block_first() {
    let ast = parse(
        r#"
box Lazy {
  once a: IntegerBox => 1
  { 2 } as once b: IntegerBox
}
"#,
    );

    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Lazy") else {
        panic!("expected BoxDeclaration");
    };

    assert!(has_method(methods, "__compute_once_a"));
    assert!(has_method(methods, "__get_once_a"));
    assert!(has_method(methods, "__compute_once_b"));
    assert!(has_method(methods, "__get_once_b"));
}

#[test]
fn birth_once_emit_is_shared_for_header_and_block_first() {
    let ast = parse(
        r#"
box Eager {
  birth_once a: IntegerBox => 1
  { 2 } as birth_once b: IntegerBox

  birth() {
    return 0
  }
}
"#,
    );

    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Eager") else {
        panic!("expected BoxDeclaration");
    };

    assert!(has_method(methods, "__compute_birth_a"));
    assert!(has_method(methods, "__get_birth_a"));
    assert!(has_method(methods, "__compute_birth_b"));
    assert!(has_method(methods, "__get_birth_b"));
}
