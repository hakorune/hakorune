// Call result annotation helpers
// Call-result annotation helpers for the call-system owner modules.

use super::super::{FunctionSignature, MirBuilder, MirType, ValueId};
use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
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
    let signature = builder
        .current_module
        .as_ref()
        .and_then(|module| FunctionSignatureLookupV1::signature(module, func_name.as_ref()))
        .cloned();
    annotate_call_result_from_func_name_with_signature(builder, dst, func_name, signature);
}

/// Header-port sibling for call-result annotation.  The semantic fallback
/// heuristics remain owned by the legacy facade; only completed signature
/// lookup is injected through the narrow reader surface.
pub(in super::super) fn annotate_call_result_from_func_name_with_lookup<S: AsRef<str>>(
    builder: &mut MirBuilder,
    dst: ValueId,
    func_name: S,
    lookup: Option<&dyn FunctionSignatureLookupV1>,
) {
    let signature = lookup
        .and_then(|view| view.signature(func_name.as_ref()))
        .cloned();
    annotate_call_result_from_func_name_with_signature(builder, dst, func_name, signature);
}

fn annotate_call_result_from_func_name_with_signature<S: AsRef<str>>(
    builder: &mut MirBuilder,
    dst: ValueId,
    func_name: S,
    signature: Option<FunctionSignature>,
) {
    let name = func_name.as_ref();
    // 1) Prefer the explicitly supplied header when available.
    if let Some(func) = signature {
        let mut ret = func.return_type.clone();
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
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .insert(dst, bx);
            if super::super::utils::builder_debug_enabled()
                || crate::config::env::builder_debug_enabled()
            {
                let bx = builder
                    .function_state
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
    // 2) No module signature—apply minimal heuristic for known functions
    if name == "JsonParser.parse/1" {
        let ret = MirType::Box("JsonNode".into());
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (known-heuristic) dst={} from {} -> Box(JsonNode)",
                dst.0, name
            ));
        }
    } else if name == "JsonParser.current_token/0" {
        let ret = MirType::Box("JsonToken".into());
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (known-heuristic) dst={} from {} -> Box(JsonToken)",
                dst.0, name
            ));
        }
    } else if name == "JsonTokenizer.tokenize/0" {
        // Tokenize returns an ArrayBox of tokens
        let ret = MirType::Box("ArrayBox".into());
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (known-heuristic) dst={} from {} -> Box(ArrayBox)",
                dst.0, name
            ));
        }
    } else if name == "JsonParserModule.create_parser/0" {
        // Known parser factory heuristic when no module signature is available.
        let ret = MirType::Box("JsonParser".into());
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(dst, ret.clone());
        if let MirType::Box(bx) = ret {
            builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .insert(dst, bx);
        }
        if super::super::utils::builder_debug_enabled()
            || crate::config::env::builder_debug_enabled()
        {
            super::super::utils::builder_debug_log(&format!(
                "annotate call (known-heuristic) dst={} from {} -> Box(JsonParser)",
                dst.0, name
            ));
        }
    } else {
        // Generic tiny whitelist for known primitive-like utilities (spec unchanged)
        crate::mir::builder::types::annotation::annotate_from_function(builder, dst, name);
    }
}
