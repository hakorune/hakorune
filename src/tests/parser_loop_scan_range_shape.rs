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
        let while_stmt = body
            .iter()
            .find(|stmt| matches!(stmt, ASTNode::While { .. }))
            .expect("while statement must exist");

        let ASTNode::While { condition, .. } = while_stmt else {
            panic!("expected While");
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
