use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn find_enum_match(ast: &ASTNode) -> Option<(&str, &Vec<crate::ast::EnumMatchArm>, bool)> {
    match ast {
        ASTNode::EnumMatchExpr {
            enum_name,
            arms,
            else_expr,
            ..
        } => Some((enum_name.as_str(), arms, else_expr.is_some())),
        ASTNode::Program { statements, .. } => statements.iter().find_map(find_enum_match),
        ASTNode::BoxDeclaration {
            methods,
            constructors,
            static_init,
            ..
        } => methods
            .values()
            .find_map(find_enum_match)
            .or_else(|| constructors.values().find_map(find_enum_match))
            .or_else(|| {
                static_init
                    .as_ref()
                    .and_then(|statements| statements.iter().find_map(find_enum_match))
            }),
        ASTNode::FunctionDeclaration { body, .. } => body.iter().find_map(find_enum_match),
        ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
            initial_values
                .iter()
                .filter_map(|value| value.as_deref())
                .find_map(find_enum_match)
        }
        ASTNode::Assignment { target, value, .. } => {
            find_enum_match(target).or_else(|| find_enum_match(value))
        }
        ASTNode::Return { value, .. } => value.as_deref().and_then(find_enum_match),
        _ => None,
    }
}

fn find_from_call(ast: &ASTNode) -> Option<(&str, &str, usize)> {
    match ast {
        ASTNode::FromCall {
            parent,
            method,
            arguments,
            ..
        } => Some((parent.as_str(), method.as_str(), arguments.len())),
        ASTNode::Program { statements, .. } => statements.iter().find_map(find_from_call),
        ASTNode::BoxDeclaration {
            methods,
            constructors,
            static_init,
            ..
        } => methods
            .values()
            .find_map(find_from_call)
            .or_else(|| constructors.values().find_map(find_from_call))
            .or_else(|| {
                static_init
                    .as_ref()
                    .and_then(|statements| statements.iter().find_map(find_from_call))
            }),
        ASTNode::FunctionDeclaration { body, .. } => body.iter().find_map(find_from_call),
        ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
            initial_values
                .iter()
                .filter_map(|value| value.as_deref())
                .find_map(find_from_call)
        }
        ASTNode::Assignment { target, value, .. } => {
            find_from_call(target).or_else(|| find_from_call(value))
        }
        ASTNode::Return { value, .. } => value.as_deref().and_then(find_from_call),
        _ => None,
    }
}

fn find_scope_box(ast: &ASTNode) -> Option<&Vec<ASTNode>> {
    match ast {
        ASTNode::ScopeBox { body, .. } => Some(body),
        ASTNode::Program { statements, .. } => statements.iter().find_map(find_scope_box),
        ASTNode::BoxDeclaration {
            methods,
            constructors,
            static_init,
            ..
        } => methods
            .values()
            .find_map(find_scope_box)
            .or_else(|| constructors.values().find_map(find_scope_box))
            .or_else(|| {
                static_init
                    .as_ref()
                    .and_then(|statements| statements.iter().find_map(find_scope_box))
            }),
        ASTNode::FunctionDeclaration { body, .. } => body.iter().find_map(find_scope_box),
        ASTNode::Local { initial_values, .. } | ASTNode::Outbox { initial_values, .. } => {
            initial_values
                .iter()
                .filter_map(|value| value.as_deref())
                .find_map(find_scope_box)
        }
        ASTNode::Assignment { target, value, .. } => {
            find_scope_box(target).or_else(|| find_scope_box(value))
        }
        ASTNode::Return { value, .. } => value.as_deref().and_then(find_scope_box),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => find_scope_box(condition)
            .or_else(|| then_body.iter().find_map(find_scope_box))
            .or_else(|| {
                else_body
                    .as_ref()
                    .and_then(|body| body.iter().find_map(find_scope_box))
            }),
        _ => None,
    }
}

#[test]
fn parse_enum_match_shorthand_keeps_known_enum_shape() {
    let src = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local value = Option::Some(1)
    return match value {
      Some(v) => v
      None => 0
    }
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let Some((enum_name, arms, has_else)) = find_enum_match(&ast) else {
        panic!("expected EnumMatchExpr");
    };

    assert_eq!(enum_name, "Option");
    assert_eq!(arms.len(), 2);
    assert!(!has_else);
}

#[test]
fn parse_unit_enum_ctor_without_parentheses_keeps_enum_ctor_shape() {
    let src = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local value = Option::None
    return 0
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let Some((parent, method, arity)) = find_from_call(&ast) else {
        panic!("expected FromCall");
    };

    assert_eq!(parent, "Option");
    assert_eq!(method, "None");
    assert_eq!(arity, 0);
}

#[test]
fn parse_option_sugar_some_becomes_option_ctor() {
    let src = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local value = some 7
    return 0
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let Some((parent, method, arity)) = find_from_call(&ast) else {
        panic!("expected FromCall");
    };

    assert_eq!(parent, "Option");
    assert_eq!(method, "Some");
    assert_eq!(arity, 1);
}

#[test]
fn parse_if_some_sugar_rewrites_to_scopebox_over_enum_match_lane() {
    let src = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local value = some 7
    if some v = value {
      return v
    } else {
      return 0
    }
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let body = find_scope_box(&ast).expect("expected ScopeBox from `if some` sugar");
    assert_eq!(body.len(), 2);

    let temp_name = match &body[0] {
        ASTNode::Local { variables, .. } => {
            assert_eq!(variables.len(), 1);
            assert!(
                variables[0].starts_with("__ny_option_some_subject_"),
                "expected hidden temp local, got {:?}",
                variables
            );
            variables[0].clone()
        }
        other => panic!("expected leading Local temp, got {other:?}"),
    };

    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[1]
    else {
        panic!("expected rewritten If");
    };

    let ASTNode::EnumMatchExpr {
        enum_name, arms, ..
    } = condition.as_ref()
    else {
        panic!("expected EnumMatchExpr condition");
    };
    assert_eq!(enum_name, "Option");
    assert_eq!(arms.len(), 2);
    assert_eq!(arms[0].variant_name, "Some");
    assert_eq!(arms[1].variant_name, "None");

    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = &then_body[0]
    else {
        panic!("expected binding Local at start of then-body");
    };
    assert_eq!(variables, &vec!["v".to_string()]);
    let Some(binding_expr) = initial_values[0].as_deref() else {
        panic!("expected bound payload expression");
    };
    let ASTNode::EnumMatchExpr {
        enum_name,
        scrutinee,
        arms,
        ..
    } = binding_expr
    else {
        panic!("expected payload EnumMatchExpr");
    };
    assert_eq!(enum_name, "Option");
    let ASTNode::Variable { name, .. } = scrutinee.as_ref() else {
        panic!("expected temp scrutinee variable");
    };
    assert_eq!(name, &temp_name);
    assert_eq!(arms[0].variant_name, "Some");
    assert_eq!(arms[0].binding_name.as_deref(), Some("v"));
    assert!(
        else_body.is_some(),
        "expected else-body to survive desugaring"
    );
}

#[test]
fn parse_type_pattern_match_does_not_become_enum_match() {
    let src = r#"
static box Main {
  main(x) {
    return match x {
      IntegerBox(n) => n
      _ => 0
    }
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    assert!(
        find_enum_match(&ast).is_none(),
        "type-pattern match must not be reclassified as enum shorthand"
    );
}

#[test]
fn parse_enum_match_requires_exhaustive_variants() {
    let src = r#"
enum Option<T> {
  None
  Some(T)
}

static box Main {
  main() {
    local value = Option::Some(1)
    return match value {
      Some(v) => v
      _ => 0
    }
  }
}
"#;

    let error =
        NyashParser::parse_from_string(src).expect_err("non-exhaustive enum match should fail");
    let message = error.to_string();
    assert!(message.contains("non-exhaustive enum match"));
    assert!(message.contains("None"));
}

#[test]
fn parse_record_enum_match_rewrites_arm_with_hidden_payload_binding() {
    let src = r#"
enum Token {
  Ident { name: String }
  Eof
}

static box Main {
  main() {
    local token = Token::Ident { name: "hello" }
    return match token {
      Ident { name } => name
      Eof => "eof"
    }
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let Some((enum_name, arms, has_else)) = find_enum_match(&ast) else {
        panic!("expected EnumMatchExpr");
    };

    assert_eq!(enum_name, "Token");
    assert_eq!(arms.len(), 2);
    assert!(!has_else);
    assert_eq!(arms[0].variant_name, "Ident");
    assert!(arms[0]
        .binding_name
        .as_deref()
        .is_some_and(|name| name.starts_with("__ny_enum_record_payload_")));
    assert!(
        matches!(arms[0].body, ASTNode::BlockExpr { .. }),
        "record enum arm should be rewritten through BlockExpr prelude bindings"
    );
}

#[test]
fn parse_tuple_enum_match_rewrites_arm_with_hidden_payload_binding() {
    let src = r#"
enum Pair {
  Both(Integer, Integer)
  None
}

static box Main {
  main() {
    local pair = Pair::Both(1, 2)
    return match pair {
      Both(left, right) => left + right
      None => 0
    }
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let Some((enum_name, arms, has_else)) = find_enum_match(&ast) else {
        panic!("expected EnumMatchExpr");
    };

    assert_eq!(enum_name, "Pair");
    assert_eq!(arms.len(), 2);
    assert!(!has_else);
    assert_eq!(arms[0].variant_name, "Both");
    assert!(arms[0]
        .binding_name
        .as_deref()
        .is_some_and(|name| name.starts_with("__ny_enum_tuple_payload_")));
    assert!(
        matches!(arms[0].body, ASTNode::BlockExpr { .. }),
        "tuple enum arm should be rewritten through BlockExpr prelude bindings"
    );
}
