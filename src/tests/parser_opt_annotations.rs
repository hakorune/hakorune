use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::tokenizer::{NyashTokenizer, TokenizeError};
use std::sync::{Mutex, MutexGuard, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

struct FeatureOverrideGuard {
    prev: Option<String>,
    _lock: MutexGuard<'static, ()>,
}

impl FeatureOverrideGuard {
    fn new(features: Option<&str>) -> Self {
        let lock = match env_guard().lock() {
            Ok(lock) => lock,
            Err(poisoned) => poisoned.into_inner(),
        };
        let prev = std::env::var("NYASH_FEATURES").ok();
        match features {
            Some(v) => std::env::set_var("NYASH_FEATURES", v),
            None => std::env::remove_var("NYASH_FEATURES"),
        }
        Self { prev, _lock: lock }
    }
}

impl Drop for FeatureOverrideGuard {
    fn drop(&mut self) {
        match &self.prev {
            Some(v) => std::env::set_var("NYASH_FEATURES", v),
            None => std::env::remove_var("NYASH_FEATURES"),
        }
    }
}

fn with_features<R>(features: Option<&str>, f: impl FnOnce() -> R) -> R {
    let _guard = FeatureOverrideGuard::new(features);
    f()
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
fn tokenizer_rejects_annotation_prefix_when_feature_off() {
    with_features(None, || {
        let mut t = NyashTokenizer::new("@hint(inline)");
        let err = t.tokenize().expect_err("tokenize should fail");
        match err {
            TokenizeError::UnexpectedCharacter { char, .. } => assert_eq!(char, '@'),
            _ => panic!("unexpected tokenize error: {err}"),
        }
    });
}

#[test]
fn parser_accepts_annotations_as_noop_when_feature_on() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @hint(inline)
  main() {
    @contract(no_alloc)
    local x = 1
    return x
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse with annotations");
        let body = find_method_body(&ast, "Main", "main");
        assert_eq!(
            body.len(),
            2,
            "annotation directives must not produce AST nodes"
        );
    });
}

#[test]
fn parser_rejects_unknown_hint_argument_fail_fast() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @hint(fastest)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/annotation] @hint(inline|noinline|hot|cold)"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_unknown_annotation_name_fail_fast() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @speed(hot)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/annotation] supported: hint|contract|intrinsic_candidate"
            ),
            "unexpected error: {msg}"
        );
    });
}
