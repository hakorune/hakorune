use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::r#macro::ast_json::{ast_to_json_roundtrip, json_to_ast};
use crate::tokenizer::{NyashTokenizer, TokenizeError};
use std::sync::{Mutex, MutexGuard, OnceLock};

#[path = "parser_opt_annotations_parts/compat.rs"]
mod compat;
#[path = "parser_opt_annotations_parts/metadata.rs"]
mod metadata;
#[path = "parser_opt_annotations_parts/placements.rs"]
mod placements;
#[path = "parser_opt_annotations_parts/rejects.rs"]
mod rejects;

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
