use crate::parser::NyashParser;
use crate::tests::helpers::env::with_env_vars;

fn with_stage3_member_postfix<R>(f: impl FnOnce() -> R) -> R {
    with_env_vars(
        &[
            ("NYASH_FEATURES", Some("stage3")),
            ("NYASH_METHOD_CATCH", Some("1")),
            ("NYASH_PARSER_STAGE3", None),
            ("HAKO_PARSER_STAGE3", None),
        ],
        f,
    )
}

fn without_member_postfix<R>(f: impl FnOnce() -> R) -> R {
    with_env_vars(
        &[
            ("NYASH_FEATURES", Some("")),
            ("NYASH_METHOD_CATCH", None),
            ("NYASH_PARSER_STAGE3", Some("0")),
            ("HAKO_PARSER_STAGE3", Some("0")),
        ],
        f,
    )
}

#[test]
fn method_postfix_cleanup_only_wraps_trycatch() {
    let src = r#"
box SafeBox {
  value: IntegerBox

  update() {
    value = 41
    return value
  } cleanup {
    value = value + 1
  }
}
"#;
    let ast = with_stage3_member_postfix(|| NyashParser::parse_from_string(src).expect("parse ok"));
    // Find FunctionDeclaration 'update' and ensure its body contains a TryCatch
    fn has_method_trycatch(ast: &crate::ast::ASTNode) -> bool {
        match ast {
            crate::ast::ASTNode::BoxDeclaration { methods, .. } => {
                for (_name, m) in methods {
                    if let crate::ast::ASTNode::FunctionDeclaration { name, body, .. } = m {
                        if name == "update" {
                            return body
                                .iter()
                                .any(|n| matches!(n, crate::ast::ASTNode::TryCatch { .. }));
                        }
                    }
                }
                false
            }
            crate::ast::ASTNode::Program { statements, .. } => {
                statements.iter().any(has_method_trycatch)
            }
            _ => false,
        }
    }
    assert!(
        has_method_trycatch(&ast),
        "expected TryCatch inside method body"
    );
}

#[test]
fn init_constructor_postfix_cleanup_wraps_trycatch() {
    let src = r#"
box SafeInit {
  init() {
    return 0
  } cleanup {
    print("done")
  }
}
"#;
    let ast = with_stage3_member_postfix(|| NyashParser::parse_from_string(src).expect("parse ok"));

    fn init_has_trycatch(ast: &crate::ast::ASTNode) -> bool {
        match ast {
            crate::ast::ASTNode::BoxDeclaration { constructors, .. } => {
                let Some(crate::ast::ASTNode::FunctionDeclaration { body, .. }) =
                    constructors.get("init/0")
                else {
                    return false;
                };
                body.iter()
                    .any(|node| matches!(node, crate::ast::ASTNode::TryCatch { .. }))
            }
            crate::ast::ASTNode::Program { statements, .. } => {
                statements.iter().any(init_has_trycatch)
            }
            _ => false,
        }
    }

    assert!(
        init_has_trycatch(&ast),
        "expected TryCatch inside init constructor body"
    );
}

#[test]
fn method_postfix_cleanup_requires_member_gate() {
    let src = r#"
box SafeBox {
  update() {
    return 1
  } cleanup {
    print("done")
  }
}
"#;
    assert!(without_member_postfix(|| {
        NyashParser::parse_from_string(src).is_err()
    }));
}

#[test]
fn init_constructor_postfix_cleanup_requires_member_gate() {
    let src = r#"
box SafeInit {
  init() {
    return 0
  } cleanup {
    print("done")
  }
}
"#;
    assert!(without_member_postfix(|| {
        NyashParser::parse_from_string(src).is_err()
    }));
}
