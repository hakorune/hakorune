use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::runtime::host_handles;
use std::collections::{BTreeMap, BTreeSet};

#[path = "module_string_dispatch/build_surrogate.rs"]
mod build_surrogate;

const USING_RESOLVER_BOX_MODULE: &str = "lang.compiler.entry.using_resolver_box";
const USING_RESOLVER_MODULE: &str = "lang.compiler.entry.using_resolver";
const MIR_BUILDER_MODULE: &str = "lang.mir.builder.MirBuilderBox";
const TRACE_ENV: &str = "HAKO_STAGE1_MODULE_DISPATCH_TRACE";

#[inline(always)]
fn trace_enabled() -> bool {
    std::env::var(TRACE_ENV).ok().as_deref() == Some("1")
}

#[inline(always)]
fn trace_log(message: impl AsRef<str>) {
    if trace_enabled() {
        eprintln!("{}", message.as_ref());
    }
}

#[inline(always)]
fn env_is_on(key: &str) -> bool {
    matches!(
        std::env::var(key).ok().as_deref(),
        Some("1" | "true" | "on" | "TRUE" | "ON")
    )
}

#[inline(always)]
fn mir_builder_internal_on() -> bool {
    !matches!(
        std::env::var("HAKO_MIR_BUILDER_INTERNAL").ok().as_deref(),
        Some("0" | "false" | "off" | "FALSE" | "OFF")
    )
}

#[inline(always)]
fn mir_builder_delegate_on() -> bool {
    env_is_on("HAKO_MIR_BUILDER_DELEGATE")
}

#[inline(always)]
fn mir_builder_no_delegate() -> bool {
    env_is_on("HAKO_SELFHOST_NO_DELEGATE")
}

#[derive(Clone, Copy)]
pub(super) struct DispatchRoute {
    pub(super) module: &'static str,
    pub(super) method: &'static str,
    pub(super) handler: fn(i64, i64, i64) -> Option<i64>,
}

const DISPATCH_ROUTES: [DispatchRoute; 4] = [
    DispatchRoute {
        module: USING_RESOLVER_BOX_MODULE,
        method: "resolve_for_source",
        handler: handle_using_resolver_resolve_for_source,
    },
    DispatchRoute {
        module: USING_RESOLVER_MODULE,
        method: "resolve_for_source",
        handler: handle_using_resolver_resolve_for_source,
    },
    DispatchRoute {
        module: MIR_BUILDER_MODULE,
        method: "emit_from_program_json_v0",
        handler: handle_mir_builder_emit_from_program_json_v0,
    },
    DispatchRoute {
        module: MIR_BUILDER_MODULE,
        method: "emit_from_source_v0",
        handler: handle_mir_builder_emit_from_source_v0,
    },
];

pub(crate) fn try_dispatch(
    recv_handle: i64,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    let module_name = decode_string_handle(recv_handle)?;
    trace_log(format!(
        "[stage1/module_dispatch] probe module={} method={} argc={}",
        module_name, method_name, arg_count
    ));

    if let Some(result) =
        build_surrogate::try_dispatch(&module_name, method_name, arg_count, arg1, arg2)
    {
        trace_log(format!(
            "[stage1/module_dispatch] hit build_surrogate module={} method={}",
            module_name, method_name
        ));
        return Some(result);
    }

    for route in DISPATCH_ROUTES {
        if module_name == route.module && method_name == route.method {
            trace_log(format!(
                "[stage1/module_dispatch] hit module={} method={}",
                route.module, route.method
            ));
            return (route.handler)(arg_count, arg1, arg2);
        }
    }

    None
}

fn handle_using_resolver_resolve_for_source(
    _arg_count: i64,
    _arg1: i64,
    _arg2: i64,
) -> Option<i64> {
    Some(encode_string_handle(""))
}

fn handle_mir_builder_emit_from_program_json_v0(
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    if arg_count < 1 {
        return Some(encode_string_handle(
            "[freeze:contract][stage1_mir_builder] missing arg0(program_json)",
        ));
    }
    let program_json = match decode_string_handle(arg1).or_else(|| decode_string_handle(arg2)) {
        Some(text) => text,
        None => {
            trace_log(format!(
                "[stage1/module_dispatch] mir_builder decode failed: arg1={} arg2={}",
                arg1, arg2
            ));
            return Some(encode_string_handle(
                "[freeze:contract][stage1_mir_builder] arg0 decode failed",
            ));
        }
    };
    trace_log(format!(
        "[stage1/module_dispatch] mir_builder input_bytes={}",
        program_json.len()
    ));
    if trace_enabled() {
        let preview: String = program_json.chars().take(120).collect();
        trace_log(format!(
            "[stage1/module_dispatch] mir_builder input_preview={:?}",
            preview
        ));
    }
    let internal_on = mir_builder_internal_on();
    let delegate_on = mir_builder_delegate_on();
    let no_delegate = mir_builder_no_delegate();
    trace_log(format!(
        "[stage1/module_dispatch] mir_builder gate internal_on={} delegate_on={} no_delegate={}",
        internal_on, delegate_on, no_delegate
    ));
    if !internal_on && (no_delegate || !delegate_on) {
        let reason = if no_delegate {
            "delegate disabled by HAKO_SELFHOST_NO_DELEGATE=1"
        } else {
            "internal off and delegate off"
        };
        return Some(encode_string_handle(&format!(
            "[freeze:contract][stage1_mir_builder] {}",
            reason
        )));
    }
    let mir_json =
        match nyash_rust::host_providers::mir_builder::program_json_to_mir_json_with_imports(
            &program_json,
            BTreeMap::new(),
        ) {
            Ok(json_text) => json_text,
            Err(error_text) => {
                trace_log(format!(
                    "[stage1/module_dispatch] mir_builder error: {}",
                    error_text
                ));
                return Some(encode_string_handle(&format!(
                    "[freeze:contract][stage1_mir_builder] {}",
                    error_text
                )));
            }
        };
    let mir_json = match inject_stage1_user_box_decls_from_program_json(&program_json, &mir_json) {
        Ok(json_text) => json_text,
        Err(error_text) => {
            trace_log(format!(
                "[stage1/module_dispatch] mir_builder user_box_decls error: {}",
                error_text
            ));
            return Some(encode_string_handle(&format!(
                "[freeze:contract][stage1_mir_builder] {}",
                error_text
            )));
        }
    };
    trace_log(format!(
        "[stage1/module_dispatch] mir_builder post_provider bytes={}",
        mir_json.len()
    ));
    trace_log(format!(
        "[stage1/module_dispatch] mir_builder output_bytes={}",
        mir_json.len()
    ));
    let out = encode_string_handle(&mir_json);
    trace_log(format!(
        "[stage1/module_dispatch] mir_builder output_handle={}",
        out
    ));
    Some(out)
}

fn handle_mir_builder_emit_from_source_v0(arg_count: i64, arg1: i64, arg2: i64) -> Option<i64> {
    if arg_count < 1 {
        return Some(encode_string_handle(
            "[freeze:contract][stage1_mir_builder] missing arg0(source_text)",
        ));
    }
    let source_text = match decode_string_handle(arg1).or_else(|| decode_string_handle(arg2)) {
        Some(text) => text,
        None => {
            trace_log(format!(
                "[stage1/module_dispatch] mir_builder source decode failed: arg1={} arg2={}",
                arg1, arg2
            ));
            return Some(encode_string_handle(
                "[freeze:contract][stage1_mir_builder] source decode failed",
            ));
        }
    };
    let internal_on = mir_builder_internal_on();
    let delegate_on = mir_builder_delegate_on();
    let no_delegate = mir_builder_no_delegate();
    trace_log(format!(
        "[stage1/module_dispatch] mir_builder source gate internal_on={} delegate_on={} no_delegate={}",
        internal_on, delegate_on, no_delegate
    ));
    if !internal_on && (no_delegate || !delegate_on) {
        let reason = if no_delegate {
            "delegate disabled by HAKO_SELFHOST_NO_DELEGATE=1"
        } else {
            "internal off and delegate off"
        };
        return Some(encode_string_handle(&format!(
            "[freeze:contract][stage1_mir_builder] {}",
            reason
        )));
    }

    let mir_json = match nyash_rust::host_providers::mir_builder::source_to_mir_json(&source_text) {
        Ok(json_text) => json_text,
        Err(error_text) => {
            trace_log(format!(
                "[stage1/module_dispatch] mir_builder source error: {}",
                error_text
            ));
            return Some(encode_string_handle(&format!(
                "[freeze:contract][stage1_mir_builder] {}",
                error_text
            )));
        }
    };
    Some(encode_string_handle(&mir_json))
}

fn inject_stage1_user_box_decls_from_program_json(
    program_json: &str,
    mir_json: &str,
) -> Result<String, String> {
    let mut mir_value: serde_json::Value = serde_json::from_str(mir_json)
        .map_err(|error| format!("mir json parse error: {}", error))?;
    let mir_object = mir_value
        .as_object_mut()
        .ok_or_else(|| "mir json root must be object".to_string())?;
    mir_object.insert(
        "user_box_decls".to_string(),
        serde_json::Value::Array(stage1_user_box_decls_from_program_json(program_json)?),
    );
    serde_json::to_string(&mir_value)
        .map_err(|error| format!("mir json serialize error: {}", error))
}

fn stage1_user_box_decls_from_program_json(
    program_json: &str,
) -> Result<Vec<serde_json::Value>, String> {
    let program_value: serde_json::Value = serde_json::from_str(program_json)
        .map_err(|error| format!("program json parse error: {}", error))?;
    let mut seen = BTreeSet::new();
    seen.insert("Main".to_string());
    if let Some(defs) = program_value
        .get("defs")
        .and_then(serde_json::Value::as_array)
    {
        for def in defs {
            if let Some(box_name) = def.get("box").and_then(serde_json::Value::as_str) {
                if !box_name.is_empty() {
                    seen.insert(box_name.to_string());
                }
            }
        }
    }
    Ok(seen
        .into_iter()
        .map(|name| serde_json::json!({ "name": name, "fields": [] }))
        .collect())
}

fn decode_string_handle(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    let object = host_handles::get(handle as u64)?;
    if let Some(string_box) = object.as_any().downcast_ref::<StringBox>() {
        return Some(string_box.value.clone());
    }
    Some(object.to_string_box().value)
}

fn encode_string_handle(text: &str) -> i64 {
    let boxed_text: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(text));
    host_handles::to_handle_arc(boxed_text) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn module_handle(name: &str) -> i64 {
        encode_string_handle(name)
    }

    fn decode_result(handle: i64) -> String {
        decode_string_handle(handle).expect("result string handle")
    }

    #[test]
    fn unknown_route_returns_none() {
        let recv = module_handle("lang.compiler.unknown");
        assert_eq!(try_dispatch(recv, "resolve_for_source", 0, 0, 0), None);
    }

    #[test]
    fn using_resolver_route_returns_empty_string_handle() {
        let recv = module_handle(USING_RESOLVER_MODULE);
        let out = try_dispatch(recv, "resolve_for_source", 0, 0, 0).expect("dispatch result");
        assert_eq!(decode_result(out), "");
    }

    #[test]
    fn mir_builder_missing_arg_returns_freeze_contract_handle() {
        let recv = module_handle(MIR_BUILDER_MODULE);
        let out =
            try_dispatch(recv, "emit_from_program_json_v0", 0, 0, 0).expect("dispatch result");
        let message = decode_result(out);
        assert!(message.starts_with("[freeze:contract][stage1_mir_builder]"));
        assert!(message.contains("missing arg0"));
    }

    #[test]
    fn mir_builder_decode_failure_returns_freeze_contract_handle() {
        let recv = module_handle(MIR_BUILDER_MODULE);
        let out =
            try_dispatch(recv, "emit_from_program_json_v0", 1, -1, -1).expect("dispatch result");
        let message = decode_result(out);
        assert!(message.starts_with("[freeze:contract][stage1_mir_builder]"));
        assert!(message.contains("decode failed"));
    }

    #[test]
    fn mir_builder_stageb_program_json_returns_mir_json_handle() {
        let recv = module_handle(MIR_BUILDER_MODULE);
        let program_json = encode_string_handle(
            r#"{"body":[{"expr":{"args":[{"name":"args","type":"Var"}],"name":"StageBDriverBox.main","type":"Call"},"type":"Return"}],"kind":"Program","version":0}"#,
        );
        let out = try_dispatch(recv, "emit_from_program_json_v0", 1, program_json, 0)
            .expect("dispatch result");
        assert!(out > 0, "dispatch must return a string handle");
        let message = decode_result(out);
        assert!(
            message.starts_with('{'),
            "expected MIR json payload, got: {}",
            message
        );
        assert!(message.contains("functions"));
    }

    #[test]
    fn mir_builder_respects_impossible_gate_contract() {
        let recv = module_handle(MIR_BUILDER_MODULE);
        let program_json = encode_string_handle(
            r#"{"body":[{"expr":{"type":"Int","value":1},"type":"Return"}],"kind":"Program","version":0}"#,
        );

        std::env::set_var("HAKO_MIR_BUILDER_INTERNAL", "0");
        std::env::set_var("HAKO_MIR_BUILDER_DELEGATE", "0");
        std::env::set_var("HAKO_SELFHOST_NO_DELEGATE", "1");

        let out = try_dispatch(recv, "emit_from_program_json_v0", 1, program_json, 0)
            .expect("dispatch result");
        let message = decode_result(out);

        std::env::remove_var("HAKO_MIR_BUILDER_INTERNAL");
        std::env::remove_var("HAKO_MIR_BUILDER_DELEGATE");
        std::env::remove_var("HAKO_SELFHOST_NO_DELEGATE");

        assert!(message.starts_with("[freeze:contract][stage1_mir_builder]"));
        assert!(message.contains("delegate disabled") || message.contains("internal off"));
    }
}
