use crate::ast::ASTNode;
use crate::parser::NyashParser;
use std::sync::{Mutex, OnceLock};

fn stage3_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|p| p.into_inner())
}

struct Stage3Env {
    previous: Option<String>,
}

impl Stage3Env {
    fn enter() -> Self {
        let previous = std::env::var("NYASH_FEATURES").ok();
        std::env::set_var("NYASH_FEATURES", "stage3");
        Self { previous }
    }
}

impl Drop for Stage3Env {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => std::env::set_var("NYASH_FEATURES", value),
            None => std::env::remove_var("NYASH_FEATURES"),
        }
    }
}

#[test]
fn parser_loopclean_while_stage3_normalizes_to_loop_ast() {
    let _lock = stage3_lock();
    let _env = Stage3Env::enter();
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
}
