// Call result annotation helpers
// Call-result annotation helpers for the call-system owner modules.

use super::super::{MirBuilder, MirType, ValueId};
use crate::mir::definitions::call_unified::Callee;

/// Build function signature name from Callee for module signature lookup
/// SSOT: "BoxName.method/arity" format for method calls, "func_name" for globals
pub(in super::super) fn callee_sig_name(callee: &Callee, arity: usize) -> Option<String> {
    match callee {
        Callee::Global(name) => {
            // Global: if already has /arity, keep as-is; otherwise append it
            if name.contains('/') {
                Some(name.clone())
            } else {
                Some(format!("{}/{}", name, arity))
            }
        }
        Callee::Method {
            box_name, method, ..
        } => {
            // Method: "BoxName.method/arity" format (SSOT for annotation lookup)
            Some(format!("{}.{}/{}", box_name, method, arity))
        }
        _ => None, // Constructor/Closure/Value/Extern don't have module signatures
    }
}

/// Annotate a call result `dst` with the return type and origin if the callee
/// is a known user/static function in the current module.
pub(in super::super) fn annotate_call_result_from_func_name<S: AsRef<str>>(
    builder: &mut MirBuilder,
    dst: ValueId,
    func_name: S,
) {
    let name = func_name.as_ref();
    // 1) Prefer module signature when available
    if let Some(ref module) = builder.current_module {
        if let Some(func) = module.functions.get(name) {
            let mut ret = func.signature.return_type.clone();
            if crate::config::env::builder_debug_annotation() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[annotation] Found function {} with return type {:?}",
                    name, ret
                ));
            }
            // Targeted stabilization: JsonParser.parse/1 should produce JsonNode
            // If signature is Unknown/Void, normalize to Box("JsonNode")
            if name == "JsonParser.parse/1" {
                if matches!(ret, MirType::Unknown | MirType::Void) {
                    ret = MirType::Box("JsonNode".into());
                }
            }
            // Token path: JsonParser.current_token/0 should produce JsonToken
            if name == "JsonParser.current_token/0" {
                if matches!(ret, MirType::Unknown | MirType::Void) {
                    ret = MirType::Box("JsonToken".into());
                }
            }
            // Parser factory: JsonParserModule.create_parser/0 returns JsonParser
            if name == "JsonParserModule.create_parser/0" {
                // Normalize to Known Box(JsonParser)
                ret = MirType::Box("JsonParser".into());
            }
            builder.type_ctx.value_types.insert(dst, ret.clone());
            if let MirType::Box(bx) = ret {
                builder.type_ctx.value_origin_newbox.insert(dst, bx);
                if super::super::utils::builder_debug_enabled()
                    || crate::config::env::builder_debug_enabled()
                {
                    let bx = builder
                        .type_ctx
                        .value_origin_newbox
                        .get(&dst)
                        .cloned()
                        .unwrap_or_default();
                    super::super::utils::builder_debug_log(&format!(
                        "annotate call dst={} from {} -> Box({})",
                        dst.0, name, bx
                    ));
                }
            }
            return;
        }
    }
    // 2) No module signature—apply minimal heuristic for known functions
    if name == "JsonParser.parse/1" {
        let ret = MirType::Box("JsonNode".into());
        builder.type_ctx.value_types.insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder.type_ctx.value_origin_newbox.insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (fallback) dst={} from {} -> Box(JsonNode)",
                dst.0, name
            ));
        }
    } else if name == "JsonParser.current_token/0" {
        let ret = MirType::Box("JsonToken".into());
        builder.type_ctx.value_types.insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder.type_ctx.value_origin_newbox.insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (fallback) dst={} from {} -> Box(JsonToken)",
                dst.0, name
            ));
        }
    } else if name == "JsonTokenizer.tokenize/0" {
        // Tokenize returns an ArrayBox of tokens
        let ret = MirType::Box("ArrayBox".into());
        builder.type_ctx.value_types.insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder.type_ctx.value_origin_newbox.insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (fallback) dst={} from {} -> Box(ArrayBox)",
                dst.0, name
            ));
        }
    } else if name == "JsonParserModule.create_parser/0" {
        // Fallback path for parser factory
        let ret = MirType::Box("JsonParser".into());
        builder.type_ctx.value_types.insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder.type_ctx.value_origin_newbox.insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (fallback) dst={} from {} -> Box(JsonParser)",
                dst.0, name
            ));
        }
    } else {
        // Generic tiny whitelist for known primitive-like utilities (spec unchanged)
        crate::mir::builder::types::annotation::annotate_from_function(builder, dst, name);
    }
}
