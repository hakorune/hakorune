#[cfg(test)]
use crate::test_support::with_env_vars;
use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::runtime::host_handles;
#[cfg(not(test))]
use std::sync::OnceLock;

#[path = "module_string_dispatch/build_surrogate.rs"]
mod build_surrogate;
#[path = "module_string_dispatch/llvm_backend_surrogate.rs"]
mod llvm_backend_surrogate;

const USING_RESOLVER_BOX_MODULE: &str = "lang.compiler.entry.using_resolver_box";
const USING_RESOLVER_MODULE: &str = "lang.compiler.entry.using_resolver";
const MIR_BUILDER_MODULE: &str = "lang.mir.builder.MirBuilderBox";
const TRACE_ENV: &str = "HAKO_STAGE1_MODULE_DISPATCH_TRACE";

#[inline(always)]
fn trace_enabled() -> bool {
    #[cfg(test)]
    {
        std::env::var(TRACE_ENV).ok().as_deref() == Some("1")
    }
    #[cfg(not(test))]
    {
        static TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
        *TRACE_ENABLED.get_or_init(|| std::env::var(TRACE_ENV).ok().as_deref() == Some("1"))
    }
}

#[inline(always)]
fn trace_log(message: impl AsRef<str>) {
    if trace_enabled() {
        eprintln!("{}", message.as_ref());
    }
}

#[inline(always)]
fn mir_builder_internal_on() -> bool {
    #[cfg(test)]
    {
        !matches!(
            std::env::var("HAKO_MIR_BUILDER_INTERNAL").ok().as_deref(),
            Some("0" | "false" | "off" | "FALSE" | "OFF")
        )
    }
    #[cfg(not(test))]
    {
        static MIR_BUILDER_INTERNAL_ON: OnceLock<bool> = OnceLock::new();
        *MIR_BUILDER_INTERNAL_ON.get_or_init(|| {
            !matches!(
                std::env::var("HAKO_MIR_BUILDER_INTERNAL").ok().as_deref(),
                Some("0" | "false" | "off" | "FALSE" | "OFF")
            )
        })
    }
}

#[inline(always)]
fn mir_builder_delegate_on() -> bool {
    #[cfg(test)]
    {
        matches!(
            std::env::var("HAKO_MIR_BUILDER_DELEGATE").ok().as_deref(),
            Some("1" | "true" | "on" | "TRUE" | "ON")
        )
    }
    #[cfg(not(test))]
    {
        static MIR_BUILDER_DELEGATE_ON: OnceLock<bool> = OnceLock::new();
        *MIR_BUILDER_DELEGATE_ON.get_or_init(|| {
            matches!(
                std::env::var("HAKO_MIR_BUILDER_DELEGATE").ok().as_deref(),
                Some("1" | "true" | "on" | "TRUE" | "ON")
            )
        })
    }
}

#[inline(always)]
fn mir_builder_no_delegate() -> bool {
    #[cfg(test)]
    {
        matches!(
            std::env::var("HAKO_SELFHOST_NO_DELEGATE").ok().as_deref(),
            Some("1" | "true" | "on" | "TRUE" | "ON")
        )
    }
    #[cfg(not(test))]
    {
        static MIR_BUILDER_NO_DELEGATE: OnceLock<bool> = OnceLock::new();
        *MIR_BUILDER_NO_DELEGATE.get_or_init(|| {
            matches!(
                std::env::var("HAKO_SELFHOST_NO_DELEGATE").ok().as_deref(),
                Some("1" | "true" | "on" | "TRUE" | "ON")
            )
        })
    }
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

    if let Some(result) =
        llvm_backend_surrogate::try_dispatch(&module_name, method_name, arg_count, arg1, arg2)
    {
        trace_log(format!(
            "[stage1/module_dispatch] hit llvm_backend_surrogate module={} method={}",
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
    let program_json = match decode_mir_builder_input_text(
        "mir_builder",
        "program_json",
        "arg0 decode failed",
        arg_count,
        arg1,
        arg2,
    ) {
        Ok(text) => text,
        Err(result) => return Some(result),
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
    if let Some(result) = mir_builder_gate_result("mir_builder") {
        return Some(result);
    }
    let mir_json =
        match nyash_rust::host_providers::mir_builder::program_json_to_mir_json_with_user_box_decls(
            &program_json,
        ) {
            Ok(json_text) => json_text,
            Err(error_text) => return Some(mir_builder_error_result("mir_builder", &error_text)),
        };
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
    let source_text = match decode_mir_builder_input_text(
        "mir_builder source",
        "source_text",
        "source decode failed",
        arg_count,
        arg1,
        arg2,
    ) {
        Ok(text) => text,
        Err(result) => return Some(result),
    };
    if let Some(result) = mir_builder_gate_result("mir_builder source") {
        return Some(result);
    }

    let mir_json = match nyash_rust::host_providers::mir_builder::source_to_mir_json(&source_text) {
        Ok(json_text) => json_text,
        Err(error_text) => {
            return Some(mir_builder_error_result("mir_builder source", &error_text));
        }
    };
    Some(encode_string_handle(&mir_json))
}

fn decode_mir_builder_input_text(
    route_label: &str,
    arg_name: &str,
    decode_error_text: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Result<String, i64> {
    if arg_count < 1 {
        return Err(encode_string_handle(&format!(
            "[freeze:contract][stage1_mir_builder] missing arg0({})",
            arg_name
        )));
    }
    match decode_string_handle(arg1).or_else(|| decode_string_handle(arg2)) {
        Some(text) => Ok(text),
        None => {
            trace_log(format!(
                "[stage1/module_dispatch] {} decode failed: arg1={} arg2={}",
                route_label, arg1, arg2
            ));
            Err(encode_string_handle(&format!(
                "[freeze:contract][stage1_mir_builder] {}",
                decode_error_text
            )))
        }
    }
}

fn mir_builder_gate_result(route_label: &str) -> Option<i64> {
    let internal_on = mir_builder_internal_on();
    let delegate_on = mir_builder_delegate_on();
    let no_delegate = mir_builder_no_delegate();
    trace_log(format!(
        "[stage1/module_dispatch] {} gate internal_on={} delegate_on={} no_delegate={}",
        route_label, internal_on, delegate_on, no_delegate
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
    None
}

fn mir_builder_error_result(route_label: &str, error_text: &str) -> i64 {
    trace_log(format!(
        "[stage1/module_dispatch] {} error: {}",
        route_label, error_text
    ));
    encode_string_handle(&format!(
        "[freeze:contract][stage1_mir_builder] {}",
        error_text
    ))
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
        with_env_vars(
            &[
                ("HAKO_MIR_BUILDER_INTERNAL", "0"),
                ("HAKO_MIR_BUILDER_DELEGATE", "0"),
                ("HAKO_SELFHOST_NO_DELEGATE", "1"),
            ],
            || {
                let recv = module_handle(MIR_BUILDER_MODULE);
                let program_json = encode_string_handle(
                    r#"{"body":[{"expr":{"type":"Int","value":1},"type":"Return"}],"kind":"Program","version":0}"#,
                );

                let out = try_dispatch(recv, "emit_from_program_json_v0", 1, program_json, 0)
                    .expect("dispatch result");
                let message = decode_result(out);

                assert!(message.starts_with("[freeze:contract][stage1_mir_builder]"));
                assert!(message.contains("delegate disabled") || message.contains("internal off"));
            },
        );
    }
}
