use crate::parser::NyashParser;

#[test]
fn parse_top_level_semicolons_optional() {
    let src = r#"
        local a = 1; local b = 2
        return a + b;
    "#;
    let ast =
        NyashParser::parse_from_string(src).expect("parser should accept semicolons by default");
    // Smoke: just ensure it parses into a Program
    match ast {
        crate::ast::ASTNode::Program { .. } => {}
        _ => panic!("expected Program"),
    }
}

#[test]
fn parse_block_with_semicolons() {
    let src = r#"
        static box Main {
          static method main() {
            local out = ""; local digits = "0123456789"; return 0
          }
        }
    "#;
    let ast =
        NyashParser::parse_from_string(src).expect("parser should accept semicolons inside blocks");
    match ast {
        crate::ast::ASTNode::Program { .. } => {}
        _ => panic!("expected Program"),
    }
}
