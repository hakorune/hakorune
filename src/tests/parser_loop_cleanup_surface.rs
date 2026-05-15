use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::tests::helpers::env::with_stage3_features;

#[test]
fn parser_loopclean_while_stage3_normalizes_to_loop_ast() {
    with_stage3_features(|| {
        let ast = NyashParser::parse_from_string(
            r#"
static box Main {
  main() {
    local i = 0
    while i < 3 {
      i = i + 1
    }
    return i
  }
}
"#,
        )
        .expect("parse while sugar");

        let ASTNode::Program { statements, .. } = ast else {
            panic!("expected Program");
        };
        let ASTNode::BoxDeclaration { methods, .. } = &statements[0] else {
            panic!("expected box declaration");
        };
        let ASTNode::FunctionDeclaration { body, .. } = &methods["main"] else {
            panic!("expected main method");
        };

        assert!(
            body.iter().any(|node| matches!(node, ASTNode::Loop { .. })),
            "while sugar should emit canonical Loop"
        );
    });
}
