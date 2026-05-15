use crate::ast::ASTNode;
use crate::parser::NyashParser;
use crate::r#macro::ast_json::{ast_to_json_roundtrip, json_to_ast};
use crate::tests::helpers::env::with_env_vars;
use crate::tests::helpers::parser::find_method_body;
use crate::tokenizer::{NyashTokenizer, TokenizeError};

#[path = "parser_opt_annotations_parts/compat.rs"]
mod compat;
#[path = "parser_opt_annotations_parts/metadata.rs"]
mod metadata;
#[path = "parser_opt_annotations_parts/placements.rs"]
mod placements;
#[path = "parser_opt_annotations_parts/rejects.rs"]
mod rejects;

fn with_features<R>(features: Option<&str>, f: impl FnOnce() -> R) -> R {
    with_env_vars(&[("NYASH_FEATURES", features)], f)
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
