use crate::ast::{
    ASTNode, BoxMethodGeneratedProvenanceV1, BoxMethodProvenanceV1, BoxMethodSourceSelectionV1,
};
use crate::parser::{NyashParser, ParseError};
use crate::tests::helpers::parser::{
    find_box, find_constructor_body, parse_ok_with_unified_members,
};

fn has_method(methods: &crate::ast::BoxMethodInventoryV1, name: &str) -> bool {
    methods.contains_name(name)
}

fn method_body<'a>(methods: &'a crate::ast::BoxMethodInventoryV1, name: &str) -> &'a Vec<ASTNode> {
    let ASTNode::FunctionDeclaration { body, .. } =
        methods.get_declaration(name).expect("method should exist")
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

fn assert_once_getter_poison_write(body: &[ASTNode], name: &str) {
    let ASTNode::TryCatch {
        try_body,
        catch_clauses,
        finally_body,
        ..
    } = body
        .iter()
        .find(|node| matches!(node, ASTNode::TryCatch { .. }))
        .expect("once getter should wrap compute path in TryCatch")
    else {
        unreachable!();
    };

    assert!(finally_body.is_none());
    assert!(try_body.iter().any(|node| {
        matches!(node, ASTNode::Local { variables, .. } if variables == &vec![format!("__ny_val_{name}")])
    }));

    let catch = catch_clauses
        .first()
        .expect("once getter should have catch-all poison handler");
    assert!(catch.exception_type.is_none());
    assert!(catch.variable_name.is_none());

    let ASTNode::MethodCall {
        object,
        method,
        arguments,
        ..
    } = catch
        .body
        .first()
        .expect("poison handler should write slot")
    else {
        panic!("expected poison setField call");
    };
    assert!(matches!(object.as_ref(), ASTNode::Me { .. }));
    assert_eq!(method, "setField");
    assert_eq!(arguments.len(), 2);

    let ASTNode::Literal {
        value: crate::ast::LiteralValue::String(slot),
        ..
    } = &arguments[0]
    else {
        panic!("expected poison storage key literal");
    };
    assert_eq!(slot, &format!("__once_poison_{name}"));

    let expected_message = format!("once '{name}' previously failed");
    let ASTNode::Literal {
        value: crate::ast::LiteralValue::String(message),
        ..
    } = &arguments[1]
    else {
        panic!("expected poison message literal");
    };
    assert_eq!(message, &expected_message);

    let ASTNode::Throw { expression, .. } = catch.body.get(1).expect("poison handler should throw")
    else {
        panic!("expected poison throw");
    };
    let ASTNode::Literal {
        value: crate::ast::LiteralValue::String(thrown),
        ..
    } = expression.as_ref()
    else {
        panic!("expected poison throw message");
    };
    assert_eq!(thrown, &expected_message);
}

#[test]
fn block_first_computed_uses_computed_getter_not_birth_once() {
    let ast = parse_ok_with_unified_members(
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
    let ast = parse_ok_with_unified_members(
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
    assert_once_getter_poison_write(method_body(methods, "__get_once_a"), "a");
    assert_once_getter_poison_write(method_body(methods, "__get_once_b"), "b");
}

#[test]
fn birth_once_emit_is_shared_for_header_and_block_first() {
    let ast = parse_ok_with_unified_members(
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
    let ast = parse_ok_with_unified_members(
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

    let body = find_constructor_body(find_box(&ast, "Eager"), "birth/0");

    assert_birth_once_initializer_pair(body, 0, "a");
    assert_birth_once_initializer_pair(body, 2, "b");
    assert!(matches!(body[4], ASTNode::Return { .. }));
}

#[test]
fn birth_once_without_user_birth_synthesizes_birth_constructor() {
    let ast = parse_ok_with_unified_members(
        r#"
box EagerOnly {
  birth_once config: IntegerBox => 7
}
"#,
    );

    let body = find_constructor_body(find_box(&ast, "EagerOnly"), "birth/0");

    assert_eq!(body.len(), 2);
    assert_birth_once_initializer_pair(body, 0, "config");
}

#[test]
fn generated_property_rows_keep_direct_provenance_and_source_site() {
    let ast = parse_ok_with_unified_members(
        r#"
box Lazy {
  once value: IntegerBox => 1
}
"#,
    );
    let ASTNode::BoxDeclaration { methods, .. } = find_box(&ast, "Lazy") else {
        panic!("expected BoxDeclaration");
    };

    for entry in methods.iter_selected_declaration_order() {
        assert_eq!(
            (entry.diagnostic_span().line, entry.diagnostic_span().column),
            (3, 8)
        );
        assert!(matches!(
            entry.provenance(),
            BoxMethodProvenanceV1::Generated(
                BoxMethodGeneratedProvenanceV1::Property {
                    property_name,
                    selection: BoxMethodSourceSelectionV1::Direct,
                }
            ) if property_name.as_ref() == "value"
        ));
    }
}

#[test]
fn property_then_explicit_collision_reports_both_source_sites() {
    let error = crate::tests::helpers::env::with_env_var(
        "NYASH_ENABLE_UNIFIED_MEMBERS",
        "1",
        || {
            NyashParser::parse_from_string(
                "box Clash {\n  once value: IntegerBox => 1\n  __get_once_value() { return 2 }\n}\n",
            )
            .expect_err("generated/explicit collision must reject")
        },
    );

    assert!(matches!(
        error,
        ParseError::DuplicateBoxMethod {
            name,
            first_line: 2,
            first_column: 8,
            duplicate_line: 3,
            duplicate_column: 3,
        } if name == "__get_once_value"
    ));
}
