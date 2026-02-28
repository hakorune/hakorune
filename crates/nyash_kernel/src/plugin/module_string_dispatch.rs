use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::runtime::host_handles;
use std::collections::BTreeMap;

const BUILD_BOX_MODULE: &str = "lang.compiler.build.build_box";
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

struct DispatchRoute {
    module: &'static str,
    method: &'static str,
    handler: fn(i64, i64, i64) -> Option<i64>,
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
        module: BUILD_BOX_MODULE,
        method: "emit_program_json_v0",
        handler: handle_build_box_emit_program_json_v0,
    },
    DispatchRoute {
        module: MIR_BUILDER_MODULE,
        method: "emit_from_program_json_v0",
        handler: handle_mir_builder_emit_from_program_json_v0,
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

fn handle_using_resolver_resolve_for_source(_arg_count: i64, _arg1: i64, _arg2: i64) -> Option<i64> {
    Some(encode_string_handle(""))
}

fn handle_build_box_emit_program_json_v0(arg_count: i64, arg1: i64, _arg2: i64) -> Option<i64> {
    if arg_count < 1 {
        return Some(0);
    }
    let source_text = match decode_string_handle(arg1) {
        Some(text) => text,
        None => return Some(0),
    };
    let program_json =
        match nyash_rust::stage1::program_json_v0::source_to_program_json_v0(&source_text) {
            Ok(json_text) => json_text,
            Err(error_text) => {
                return Some(encode_string_handle(&format!(
                    "[freeze:contract][stage1_program_json_v0] {}",
                    error_text
                )));
            }
        };
    Some(encode_string_handle(&program_json))
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
    let mir_json = match nyash_rust::host_providers::mir_builder::program_json_to_mir_json_with_imports(
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
    Some(encode_string_handle(&mir_json))
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
    fn build_box_missing_arg_returns_zero_handle() {
        let recv = module_handle(BUILD_BOX_MODULE);
        let out = try_dispatch(recv, "emit_program_json_v0", 0, 0, 0).expect("dispatch result");
        assert_eq!(out, 0);
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
}
