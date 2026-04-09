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
