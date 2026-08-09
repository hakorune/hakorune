use nyash_rust::ast::ASTNode;
use serde_json::Value;

use super::{
    joinir_compat,
    roundtrip_decoder::{AstJsonDecoder, DecodeMode},
};

pub const SCHEMA: &str = "ast_json_roundtrip_v2";
pub const SCHEMA_VERSION: u32 = 2;
const LEGACY_SCHEMA: &str = "ast_json_roundtrip_v1";
const LEGACY_SCHEMA_VERSION: u32 = 1;

pub fn ast_to_json_roundtrip(ast: &ASTNode) -> Value {
    let mut v = joinir_compat::ast_to_json_roundtrip_v2(ast);
    if let Value::Object(ref mut m) = v {
        m.insert("schema".to_string(), Value::from(SCHEMA));
        m.insert("schema_version".to_string(), Value::from(SCHEMA_VERSION));
    }
    v
}

pub fn json_to_ast(v: &Value) -> Option<ASTNode> {
    let mode = match (
        v.get("schema").and_then(Value::as_str),
        v.get("schema_version").and_then(Value::as_u64),
    ) {
        (Some(SCHEMA), Some(version)) if version == SCHEMA_VERSION as u64 => {
            DecodeMode::RoundtripV2
        }
        (Some(LEGACY_SCHEMA), Some(version)) if version == LEGACY_SCHEMA_VERSION as u64 => {
            DecodeMode::Legacy
        }
        (None, None) => DecodeMode::Legacy,
        _ => return None,
    };
    let decoder = AstJsonDecoder::new(mode);
    let ast = decoder.decode(v)?;
    (!decoder.nested_failure.get()).then_some(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_box_is_not_erased_by_ast_json_roundtrip() {
        let scope = ASTNode::ScopeBox {
            body: Vec::new(),
            span: nyash_rust::ast::Span::unknown(),
        };
        let json = ast_to_json_roundtrip(&scope);
        assert_eq!(json["kind"], "ScopeBox");
        assert!(matches!(
            json_to_ast(&json),
            Some(ASTNode::ScopeBox { body, .. }) if body.is_empty()
        ));
    }

    #[test]
    fn release_is_preserved_only_by_roundtrip_v2() {
        let release = ASTNode::Release {
            root: "root".to_string(),
            span: nyash_rust::ast::Span::unknown(),
        };
        let json = ast_to_json_roundtrip(&release);
        assert_eq!(json["kind"], "Release");
        assert_eq!(json["root"], "root");
        assert!(matches!(
            json_to_ast(&json),
            Some(ASTNode::Release { root, .. }) if root == "root"
        ));

        let legacy = joinir_compat::ast_to_json(&release);
        assert_eq!(legacy["kind"], "Unsupported");
    }
}
