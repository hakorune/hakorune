use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::parser::NyashParser;
use std::sync::{Mutex, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

fn with_stage3_features<R>(f: impl FnOnce() -> R) -> R {
    let _lock = env_guard().lock().unwrap_or_else(|e| e.into_inner());
    let prev_features = std::env::var("NYASH_FEATURES").ok();
    std::env::set_var("NYASH_FEATURES", "stage3");
    let out = f();
    match prev_features {
        Some(v) => std::env::set_var("NYASH_FEATURES", v),
        None => std::env::remove_var("NYASH_FEATURES"),
    }
    out
}

fn find_method_body<'a>(ast: &'a ASTNode, box_name: &str, method_name: &str) -> &'a [ASTNode] {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    for stmt in statements {
        if let ASTNode::BoxDeclaration { name, methods, .. } = stmt {
            if name != box_name {
                continue;
            }
            if let Some(ASTNode::FunctionDeclaration { body, .. }) = methods.get(method_name) {
                return body;
            }
        }
    }
    panic!("method not found: {}.{}", box_name, method_name);
}

#[test]
fn parser_loop_scan_range_shape_preserves_lte_n_minus_one_ast() {
    with_stage3_features(|| {
        let src = r#"
box Scan {
  run(s, n) {
    local i = 0
    while i <= n - 1 {
      local ch = s.substring(i, i + 1)
      if ch == "," {
        i = i + 1
        continue
      }
      if ch == "]" {
        break
      }
      i = i + 1
    }
    return i
  }
}
"#;

        let ast = NyashParser::parse_from_string(src).expect("parse ok");
        let body = find_method_body(&ast, "Scan", "run");
        let loop_stmt = body
            .iter()
            .find(|stmt| matches!(stmt, ASTNode::Loop { .. }))
            .expect("while sugar should normalize to loop statement");

        let ASTNode::Loop { condition, .. } = loop_stmt else {
            panic!("expected Loop");
        };

        let ASTNode::BinaryOp {
            operator: BinaryOperator::LessEqual,
            left,
            right,
            ..
        } = condition.as_ref()
        else {
            panic!("expected while condition to be LessEqual");
        };
        assert!(matches!(
            left.as_ref(),
            ASTNode::Variable { name, .. } if name == "i"
        ));

        let ASTNode::BinaryOp {
            operator: BinaryOperator::Subtract,
            left: rhs_left,
            right: rhs_right,
            ..
        } = right.as_ref()
        else {
            panic!("expected right side to be `n - 1`");
        };
        assert!(matches!(
            rhs_left.as_ref(),
            ASTNode::Variable { name, .. } if name == "n"
        ));
        assert!(matches!(
            rhs_right.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        ));
    });
}

#[test]
fn parser_loop_range_surface_parses_parenless_loop_header() {
    with_stage3_features(|| {
        let src = r#"
box Scan {
  run(count) {
    loop i in 0..count {
      print(i)
    }
    return 0
  }
}
"#;

        let ast = NyashParser::parse_from_string(src).expect("parse ok");
        let body = find_method_body(&ast, "Scan", "run");
        let range_stmt = body
            .iter()
            .find(|stmt| matches!(stmt, ASTNode::ForRange { .. }))
            .expect("loop range statement must exist");

        let ASTNode::ForRange {
            var_name,
            start,
            end,
            body,
            ..
        } = range_stmt
        else {
            panic!("expected ForRange metadata node");
        };

        assert_eq!(var_name, "i");
        assert!(matches!(
            start.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(0),
                ..
            }
        ));
        assert!(matches!(
            end.as_ref(),
            ASTNode::Variable { name, .. } if name == "count"
        ));
        assert_eq!(body.len(), 1);
    });
}

#[test]
fn parser_loop_range_surface_parses_parenthesized_loop_header() {
    with_stage3_features(|| {
        let src = r#"
box Scan {
  run(count) {
    loop(i in 1..count) {
      print(i)
    }
    return 0
  }
}
"#;

        let ast = NyashParser::parse_from_string(src).expect("parse ok");
        let body = find_method_body(&ast, "Scan", "run");
        let range_stmt = body
            .iter()
            .find(|stmt| matches!(stmt, ASTNode::ForRange { .. }))
            .expect("loop range statement must exist");

        let ASTNode::ForRange { var_name, start, .. } = range_stmt else {
            panic!("expected ForRange metadata node");
        };
        assert_eq!(var_name, "i");
        assert!(matches!(
            start.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        ));
    });
}

#[test]
fn parser_legacy_for_range_surface_uses_shared_for_range_shape() {
    with_stage3_features(|| {
        let src = r#"
box Scan {
  run(count) {
    for i in 2..count {
      print(i)
    }
    return 0
  }
}
"#;

        let ast = NyashParser::parse_from_string(src).expect("parse ok");
        let body = find_method_body(&ast, "Scan", "run");
        let range_stmt = body
            .iter()
            .find(|stmt| matches!(stmt, ASTNode::ForRange { .. }))
            .expect("legacy for range statement must exist");

        let ASTNode::ForRange {
            var_name,
            start,
            end,
            ..
        } = range_stmt
        else {
            panic!("expected ForRange metadata node");
        };

        assert_eq!(var_name, "i");
        assert!(matches!(
            start.as_ref(),
            ASTNode::Literal {
                value: LiteralValue::Integer(2),
                ..
            }
        ));
        assert!(matches!(
            end.as_ref(),
            ASTNode::Variable { name, .. } if name == "count"
        ));
    });
}

#[test]
fn parser_loop_condition_surface_accepts_parenless_loop_condition() {
    with_stage3_features(|| {
        let src = r#"
box Scan {
  run(count) {
    local i = 0
    loop i < count {
      i = i + 1
    }
    return i
  }
}
"#;

        let ast = NyashParser::parse_from_string(src).expect("parse ok");
        let body = find_method_body(&ast, "Scan", "run");
        let loop_stmt = body
            .iter()
            .find(|stmt| matches!(stmt, ASTNode::Loop { .. }))
            .expect("loop condition statement must exist");

        let ASTNode::Loop { condition, .. } = loop_stmt else {
            panic!("expected Loop node");
        };
        assert!(matches!(
            condition.as_ref(),
            ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                ..
            }
        ));
    });
}
