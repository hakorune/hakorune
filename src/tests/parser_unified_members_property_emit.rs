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

fn birth_body<'a>(box_node: &'a ASTNode, key: &str) -> &'a Vec<ASTNode> {
    let ASTNode::BoxDeclaration { constructors, .. } = box_node else {
        panic!("expected BoxDeclaration");
    };
    let ASTNode::FunctionDeclaration { body, .. } = constructors
        .get(key)
        .expect("birth constructor should exist")
    else {
        panic!("expected FunctionDeclaration");
    };
    body
}

fn assert_birth_once_initializer_pair(body: &[ASTNode], offset: usize, name: &str) {
    let tmp = format!("__ny_birth_{}", name);

    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = &body[offset]
    else {
        panic!("expected local birth_once tmp at offset {}", offset);
    };
    assert_eq!(variables, &vec![tmp.clone()]);

    let Some(Some(init)) = initial_values.first() else {
        panic!("expected birth_once tmp initializer");
    };
    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = init.as_ref()
    else {
        panic!("expected compute method call");
    };
    assert!(matches!(object.as_ref(), ASTNode::Me { .. }));
    assert_eq!(method, &format!("__compute_birth_{}", name));
    assert!(arguments.is_empty());

    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = &body[offset + 1]
    else {
        panic!("expected setField call at offset {}", offset + 1);
    };
    assert!(matches!(object.as_ref(), ASTNode::Me { .. }));
    assert_eq!(method, "setField");
    assert_eq!(arguments.len(), 2);

    let ASTNode::Literal {
        value: crate::ast::LiteralValue::String(slot),
        ..
    } = &arguments[0]
    else {
        panic!("expected birth_once storage key literal");
    };
    assert_eq!(slot, &format!("__birth_{}", name));

    let ASTNode::Variable { name: arg_name, .. } = &arguments[1] else {
        panic!("expected birth_once tmp variable");
    };
    assert_eq!(arg_name, &tmp);
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

#[test]
fn birth_once_prologue_is_inserted_into_canonical_birth_constructor() {
    let ast = parse(
        r#"
box Eager {
  birth_once a: IntegerBox => 1

  birth() {
    return 0
  }

  birth_once b: IntegerBox => 2
}
"#,
    );

    let body = birth_body(find_box(&ast, "Eager"), "birth/0");

    assert_birth_once_initializer_pair(body, 0, "a");
    assert_birth_once_initializer_pair(body, 2, "b");
    assert!(matches!(body[4], ASTNode::Return { .. }));
}

#[test]
fn birth_once_without_user_birth_synthesizes_birth_constructor() {
    let ast = parse(
        r#"
box EagerOnly {
  birth_once config: IntegerBox => 7
}
"#,
    );

    let body = birth_body(find_box(&ast, "EagerOnly"), "birth/0");

    assert_eq!(body.len(), 2);
    assert_birth_once_initializer_pair(body, 0, "config");
}
