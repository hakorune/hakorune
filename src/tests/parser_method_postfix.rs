use crate::parser::NyashParser;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

const ENV_KEYS: &[&str] = &[
    "NYASH_FEATURES",
    "NYASH_METHOD_CATCH",
    "NYASH_PARSER_STAGE3",
    "HAKO_PARSER_STAGE3",
];

struct EnvSnapshot {
    values: Vec<(&'static str, Option<String>)>,
}

impl EnvSnapshot {
    fn capture() -> Self {
        Self {
            values: ENV_KEYS
                .iter()
                .map(|key| (*key, std::env::var(key).ok()))
                .collect(),
        }
    }
}

impl Drop for EnvSnapshot {
    fn drop(&mut self) {
        for (key, value) in &self.values {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

fn enable_stage3() -> EnvSnapshot {
    let snapshot = EnvSnapshot::capture();
    std::env::set_var("NYASH_FEATURES", "stage3");
    std::env::set_var("NYASH_METHOD_CATCH", "1");
    std::env::remove_var("NYASH_PARSER_STAGE3");
    std::env::remove_var("HAKO_PARSER_STAGE3");
    snapshot
}

fn disable_member_postfix() -> EnvSnapshot {
    let snapshot = EnvSnapshot::capture();
    std::env::set_var("NYASH_FEATURES", "");
    std::env::remove_var("NYASH_METHOD_CATCH");
    std::env::set_var("NYASH_PARSER_STAGE3", "0");
    std::env::set_var("HAKO_PARSER_STAGE3", "0");
    snapshot
}

#[test]
fn method_postfix_cleanup_only_wraps_trycatch() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _env = enable_stage3();
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
    let ast = NyashParser::parse_from_string(src).expect("parse ok");
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
    let _lock = ENV_LOCK.lock().unwrap();
    let _env = enable_stage3();
    let src = r#"
box SafeInit {
  init() {
    return 0
  } cleanup {
    print("done")
  }
}
"#;
    let ast = NyashParser::parse_from_string(src).expect("parse ok");

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
    let _lock = ENV_LOCK.lock().unwrap();
    let _env = disable_member_postfix();
    let src = r#"
box SafeBox {
  update() {
    return 1
  } cleanup {
    print("done")
  }
}
"#;
    assert!(NyashParser::parse_from_string(src).is_err());
}

#[test]
fn init_constructor_postfix_cleanup_requires_member_gate() {
    let _lock = ENV_LOCK.lock().unwrap();
    let _env = disable_member_postfix();
    let src = r#"
box SafeInit {
  init() {
    return 0
  } cleanup {
    print("done")
  }
}
"#;
    assert!(NyashParser::parse_from_string(src).is_err());
}
