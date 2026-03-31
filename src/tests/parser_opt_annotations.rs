use crate::ast::ASTNode;
use crate::r#macro::ast_json::{ast_to_json_roundtrip, json_to_ast};
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

fn find_runes(metadata: &crate::parser::ParserMetadata) -> Vec<(String, Vec<String>)> {
    metadata
        .runes
        .iter()
        .map(|rune| (rune.name.clone(), rune.args.clone()))
        .collect()
}

fn find_box_and_method_runes(
    ast: &ASTNode,
    box_name: &str,
    method_name: &str,
) -> (Vec<(String, Vec<String>)>, Vec<(String, Vec<String>)>) {
    let ASTNode::Program { statements, .. } = ast else {
        panic!("expected Program");
    };
    for stmt in statements {
        if let ASTNode::BoxDeclaration {
            name,
            attrs,
            methods,
            ..
        } = stmt
        {
            if name != box_name {
                continue;
            }
            let box_runes = attrs
                .runes
                .iter()
                .map(|rune| (rune.name.clone(), rune.args.clone()))
                .collect::<Vec<_>>();
            let Some(ASTNode::FunctionDeclaration { attrs, .. }) = methods.get(method_name) else {
                panic!("method not found: {box_name}.{method_name}");
            };
            let method_runes = attrs
                .runes
                .iter()
                .map(|rune| (rune.name.clone(), rune.args.clone()))
                .collect::<Vec<_>>();
            return (box_runes, method_runes);
        }
    }
    panic!("box not found: {box_name}");
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
fn parser_accepts_body_position_legacy_annotations_as_noop_under_rune_gate() {
    with_features(Some("stage3,rune"), || {
        let src = r#"
static box Main {
  main() {
    @contract(no_alloc)
    local x = 1
    return x
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse with legacy annotation under rune gate");
        let body = find_method_body(&ast, "Main", "main");
        assert_eq!(body.len(), 2, "body-position legacy annotations stay noop during compat window");
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

#[test]
fn parser_accepts_legacy_annotations_as_rune_metadata_on_callable_declarations() {
    with_features(Some("stage3,rune"), || {
        let src = r#"
static box Main {
  @hint(hot)
  @contract(no_alloc)
  @intrinsic_candidate("StringBox.length/0")
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) =
            NyashParser::parse_from_string_with_metadata(src).expect("parse with legacy declaration metadata");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Hint".to_string(), vec!["hot".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["StringBox.length/0".to_string()]
                ),
            ]
        );
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(method_runes, runes);
    });
}

#[test]
fn parser_accepts_canonical_optimization_runes_and_preserves_metadata() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Hint(inline)
  @rune Contract(no_alloc)
  @rune IntrinsicCandidate("StringBox.length/0")
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) =
            NyashParser::parse_from_string_with_metadata(src).expect("parse with canonical rune families");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Hint".to_string(), vec!["inline".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["StringBox.length/0".to_string()]
                ),
            ]
        );
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(method_runes, runes);
    });
}

#[test]
fn parser_accepts_mixed_legacy_aliases_and_canonical_runes_on_same_declaration() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Hint(hot)
  @contract(no_alloc)
  @intrinsic_candidate("StringBox.length/0")
  @rune Symbol("main_sym")
  @rune CallConv("c")
  main() {
    return 0
  }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse mixed metadata preamble");
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(
            method_runes,
            vec![
                ("Hint".to_string(), vec!["hot".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["StringBox.length/0".to_string()]
                ),
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()]),
            ]
        );
    });
}

#[test]
fn parser_accepts_canonical_rune_surface_under_opt_annotations_gate() {
    with_features(Some("stage3,opt-annotations"), || {
        let src = r#"
static box Main {
  @rune Hint(hot)
  main() { return 0 }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("canonical rune surface should parse under compat gate");
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(method_runes, vec![("Hint".to_string(), vec!["hot".to_string()])]);
    });
}

#[test]
fn parser_accepts_canonical_rune_control_plane_surface_and_roundtrips_ast_json() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Public
static box Main {
  @rune FfiSafe
  @rune ReturnsOwned
  @rune FreeWith("cleanup_main")
  @rune Symbol("main_sym")
  @rune CallConv("c")
  @rune Hint(inline)
  @rune Contract(no_alloc)
  @rune IntrinsicCandidate("Main.main/0")
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) =
            NyashParser::parse_from_string_with_metadata(src).expect("parse canonical rune surface");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Public".to_string(), vec![]),
                ("FfiSafe".to_string(), vec![]),
                ("ReturnsOwned".to_string(), vec![]),
                ("FreeWith".to_string(), vec!["cleanup_main".to_string()]),
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()]),
                ("Hint".to_string(), vec!["inline".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["Main.main/0".to_string()]
                ),
            ]
        );

        let (box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(box_runes, vec![("Public".to_string(), vec![])]);
        assert_eq!(
            method_runes,
            vec![
                ("FfiSafe".to_string(), vec![]),
                ("ReturnsOwned".to_string(), vec![]),
                ("FreeWith".to_string(), vec!["cleanup_main".to_string()]),
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()]),
                ("Hint".to_string(), vec!["inline".to_string()]),
                ("Contract".to_string(), vec!["no_alloc".to_string()]),
                (
                    "IntrinsicCandidate".to_string(),
                    vec!["Main.main/0".to_string()]
                ),
            ]
        );

        let roundtrip = json_to_ast(&ast_to_json_roundtrip(&ast)).expect("ast roundtrip");
        let (roundtrip_box_runes, roundtrip_method_runes) =
            find_box_and_method_runes(&roundtrip, "Main", "main");
        assert_eq!(roundtrip_box_runes, box_runes);
        assert_eq!(roundtrip_method_runes, method_runes);
    });
}

#[test]
fn parser_accepts_rune_annotations_and_preserves_metadata() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Public
static box Main {
  @rune Ownership(owned)
  main() {
    return 0
  }
}
"#;
        let (ast, metadata) =
            NyashParser::parse_from_string_with_metadata(src).expect("parse with rune");
        let runes = find_runes(&metadata);
        assert_eq!(
            runes,
            vec![
                ("Public".to_string(), vec![]),
                ("Ownership".to_string(), vec!["owned".to_string()])
            ]
        );
        let (box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(box_runes, vec![("Public".to_string(), vec![])]);
        assert_eq!(
            method_runes,
            vec![("Ownership".to_string(), vec!["owned".to_string()])]
        );
    });
}

#[test]
fn parser_rejects_unknown_rune_name_fail_fast() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Strange
static box Main {
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] supported: Public|Internal|FfiSafe|Symbol|CallConv|ReturnsOwned|FreeWith|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_rune_on_non_declaration_statement() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  main() {
    @rune Public
    local x = 1
    return x
  }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] declaration required after @rune")
                || msg.contains("[freeze:contract][parser/rune] invalid placement"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_canonical_optimization_rune_on_non_declaration_statement() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  main() {
    @rune Hint(inline)
    local x = 1
    return x
  }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] declaration required after @rune")
                || msg.contains("[freeze:contract][parser/rune] invalid placement"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_duplicate_runes_on_same_declaration() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Symbol("main_a")
  @rune Symbol("main_b")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] duplicate rune Symbol"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_conflicting_visibility_runes() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Public
@rune Internal
static box Main {
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] conflicting visibility runes"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_non_visibility_rune_on_box_target() {
    with_features(Some("rune"), || {
        let src = r#"
@rune Symbol("main_sym")
static box Main {
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] box target supports only Public|Internal"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_accepts_abi_runes_on_static_box_method() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Symbol("main_sym")
  @rune CallConv("c")
  main() { return 0 }
}
"#;
        let ast = NyashParser::parse_from_string(src).expect("parse should succeed");
        let (_box_runes, method_runes) = find_box_and_method_runes(&ast, "Main", "main");
        assert_eq!(
            method_runes,
            vec![
                ("Symbol".to_string(), vec!["main_sym".to_string()]),
                ("CallConv".to_string(), vec!["c".to_string()])
            ]
        );
    });
}

#[test]
fn parser_rejects_abi_rune_on_instance_method() {
    with_features(Some("rune"), || {
        let src = r#"
box Main {
  @rune Symbol("main_sym")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] instance method target supports only Public|Internal|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_abi_rune_on_constructor() {
    with_features(Some("rune"), || {
        let src = r#"
box Main {
  @rune Symbol("ctor_sym")
  birth() {}
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] constructor target supports only Public|Internal|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_abi_rune_on_interface_method() {
    with_features(Some("rune"), || {
        let src = r#"
interface box Main {
  @rune Symbol("iface_sym")
  run()
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] interface method target supports only Public|Internal|Ownership|Hint|Contract|IntrinsicCandidate"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_invalid_hint_rune_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Hint(fastest)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] Hint(inline|noinline|hot|cold)"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_invalid_contract_rune_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune Contract(mutable)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] Contract(pure|readonly|no_alloc|no_safepoint)"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_empty_intrinsic_candidate_rune_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune IntrinsicCandidate("")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains(
                "[freeze:contract][parser/rune] IntrinsicCandidate(\"symbol\") with non-empty symbol"
            ),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_invalid_callconv_value() {
    with_features(Some("rune"), || {
        let src = r#"
static box Main {
  @rune CallConv("sysv")
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] CallConv(\"c\")"),
            "unexpected error: {msg}"
        );
    });
}

#[test]
fn parser_rejects_invalid_ownership_value() {
    with_features(Some("rune"), || {
        let src = r#"
box Main {
  @rune Ownership(unique)
  main() { return 0 }
}
"#;
        let err = NyashParser::parse_from_string(src).expect_err("parse should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("[freeze:contract][parser/rune] Ownership(owned|borrowed|shared)"),
            "unexpected error: {msg}"
        );
    });
}
