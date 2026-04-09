use crate::ast::{ASTNode, LiteralValue};
use crate::parser::NyashParser;

fn main_return_value(ast: &ASTNode) -> &ASTNode {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected program");
    };
    let Some(ASTNode::BoxDeclaration { methods, .. }) = statements.first() else {
        panic!("expected static box");
    };
    let main = methods.get("main").expect("main method");
    let ASTNode::FunctionDeclaration { body, .. } = main else {
        panic!("expected function declaration");
    };
    let Some(ASTNode::Return { value, .. }) = body.first() else {
        panic!("expected return");
    };
    value.as_deref().expect("return value")
}

#[test]
fn parse_void_literal_surface() {
    let src = r#"
static box Main {
  main() {
    return void
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    assert!(matches!(
        main_return_value(&ast),
        ASTNode::Literal {
            value: LiteralValue::Void,
            ..
        }
    ));
}

#[test]
fn parse_match_accepts_void_literal_arm() {
    let src = r#"
static box Main {
  main(x) {
    return match x {
      void => 1
      _ => 0
    }
  }
}
"#;

    let ast = NyashParser::parse_from_string(src).expect("parse ok");
    let ASTNode::MatchExpr { arms, .. } = main_return_value(&ast) else {
        panic!("expected match expr");
    };
    assert!(matches!(arms[0].0, LiteralValue::Void));
}
