// Compiled-stage1 route table and local probes for using_resolver,
// BuildBox, MIR-builder, and llvm backend string-module dispatch helpers.
// Keep it internal-only and do not let it grow into a general semantic owner.

use crate::plugin::{materialize_owned_string, owned_string_from_handle};
#[cfg(test)]
use crate::test_support::with_env_vars;
use std::sync::OnceLock;

#[path = "module_string_dispatch/build_surrogate.rs"]
mod build_surrogate;
#[path = "module_string_dispatch/compat/llvm_backend_surrogate.rs"]
mod llvm_backend_surrogate;

const USING_RESOLVER_BOX_MODULE: &str = "lang.compiler.entry.using_resolver_box";
const USING_RESOLVER_MODULE: &str = "lang.compiler.entry.using_resolver";
const MIR_BUILDER_MODULE: &str = "lang.mir.builder.MirBuilderBox";
const TRACE_ENV: &str = "HAKO_STAGE1_MODULE_DISPATCH_TRACE";
const STAGE1_MODULE_DISPATCH_TAG: &str = "[stage1/module_dispatch]";

#[inline(always)]
fn trace_log_args(args: std::fmt::Arguments<'_>) {
    static TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
    if crate::env_flags::flag_on_cached(&TRACE_ENABLED, TRACE_ENV) {
        eprintln!("{}", args);
    }
}

#[derive(Clone, Copy)]
pub(super) struct DispatchRoute {
    pub(super) module: &'static str,
    pub(super) method: &'static str,
    pub(super) action: DispatchAction,
}

#[derive(Clone, Copy)]
pub(super) enum DispatchAction {
    ReturnEmptyString,
    MirBuilderProgramJson,
    MirBuilderSource,
}

const DISPATCH_ROUTES: [DispatchRoute; 4] = [
    DispatchRoute {
        module: USING_RESOLVER_BOX_MODULE,
        method: "resolve_for_source",
        action: DispatchAction::ReturnEmptyString,
    },
    DispatchRoute {
        module: USING_RESOLVER_MODULE,
        method: "resolve_for_source",
        action: DispatchAction::ReturnEmptyString,
    },
    DispatchRoute {
        module: MIR_BUILDER_MODULE,
        method: "emit_from_program_json_v0",
        action: DispatchAction::MirBuilderProgramJson,
    },
    DispatchRoute {
        module: MIR_BUILDER_MODULE,
        method: "emit_from_source_v0",
        action: DispatchAction::MirBuilderSource,
    },
];

#[inline(always)]
fn dispatch_mir_builder_route(
    route_label: &'static str,
    arg_name: &'static str,
    decode_error_text: &'static str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
    on_input: impl Fn(&str),
    build_mir_json: impl Fn(&str) -> Result<String, String>,
) -> Option<i64> {
    // This path remains a MIR-builder runtime boundary; BuildBox/Program
    // authority stays outside module_string_dispatch.
    let input_text = if arg_count < 1 {
        return Some(materialize_owned_string(format!(
            "[freeze:contract][stage1_mir_builder] missing arg0({})",
            arg_name
        )));
    } else {
        match owned_string_from_handle(arg1).or_else(|| owned_string_from_handle(arg2)) {
            Some(text) => text,
            None => {
                trace_log_args(format_args!(
                    "{} {} decode failed: arg1={} arg2={}",
                    STAGE1_MODULE_DISPATCH_TAG, route_label, arg1, arg2
                ));
                return Some(materialize_owned_string(format!(
                    "[freeze:contract][stage1_mir_builder] {}",
                    decode_error_text
                )));
            }
        }
    };
    on_input(&input_text);
    static MIR_BUILDER_INTERNAL_ON: OnceLock<bool> = OnceLock::new();
    static MIR_BUILDER_DELEGATE_ON: OnceLock<bool> = OnceLock::new();
    static MIR_BUILDER_NO_DELEGATE: OnceLock<bool> = OnceLock::new();
    let internal_on = crate::env_flags::flag_default_on_cached(
        &MIR_BUILDER_INTERNAL_ON,
        "HAKO_MIR_BUILDER_INTERNAL",
    );
    let delegate_on =
        crate::env_flags::flag_on_cached(&MIR_BUILDER_DELEGATE_ON, "HAKO_MIR_BUILDER_DELEGATE");
    let no_delegate =
        crate::env_flags::flag_on_cached(&MIR_BUILDER_NO_DELEGATE, "HAKO_SELFHOST_NO_DELEGATE");
    trace_log_args(format_args!(
        "{} {} gate internal_on={} delegate_on={} no_delegate={}",
        STAGE1_MODULE_DISPATCH_TAG, route_label, internal_on, delegate_on, no_delegate
    ));
    if !internal_on && (no_delegate || !delegate_on) {
        let reason = if no_delegate {
            "delegate disabled by HAKO_SELFHOST_NO_DELEGATE=1"
        } else {
            "internal off and delegate off"
        };
        return Some(materialize_owned_string(format!(
            "[freeze:contract][stage1_mir_builder] {}",
            reason
        )));
    }
    let mir_json = match build_mir_json(&input_text) {
        Ok(json_text) => json_text,
        Err(error_text) => {
            trace_log_args(format_args!(
                "{} {} error: {}",
                STAGE1_MODULE_DISPATCH_TAG, route_label, error_text
            ));
            return Some(materialize_owned_string(format!(
                "[freeze:contract][stage1_mir_builder] {}",
                error_text
            )));
        }
    };
    let out = materialize_owned_string(mir_json);
    trace_log_args(format_args!(
        "{} {} output_handle={}",
        STAGE1_MODULE_DISPATCH_TAG, route_label, out
    ));
    Some(out)
}

pub(crate) fn try_dispatch(
    recv_handle: i64,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    let module_name = owned_string_from_handle(recv_handle)?;
    trace_log_args(format_args!(
        "{} probe module={} method={} argc={}",
        STAGE1_MODULE_DISPATCH_TAG, module_name, method_name, arg_count
    ));

    if let Some(result) =
        build_surrogate::try_dispatch(&module_name, method_name, arg_count, arg1, arg2)
    {
        trace_log_args(format_args!(
            "{} hit build_surrogate module={} method={}",
            STAGE1_MODULE_DISPATCH_TAG, module_name, method_name
        ));
        return Some(result);
    }

    if let Some(result) =
        llvm_backend_surrogate::try_dispatch(&module_name, method_name, arg_count, arg1, arg2)
    {
        trace_log_args(format_args!(
            "{} hit llvm_backend_surrogate module={} method={}",
            STAGE1_MODULE_DISPATCH_TAG, module_name, method_name
        ));
        return Some(result);
    }

    static MIR_BUILDER_TRACE_ENABLED: OnceLock<bool> = OnceLock::new();

    for route in DISPATCH_ROUTES {
        if module_name == route.module && method_name == route.method {
            trace_log_args(format_args!(
                "{} hit module module={} method={}",
                STAGE1_MODULE_DISPATCH_TAG, route.module, route.method
            ));
            return match route.action {
                DispatchAction::ReturnEmptyString => Some(materialize_owned_string(String::new())),
                DispatchAction::MirBuilderProgramJson => dispatch_mir_builder_route(
                    "mir_builder",
                    "program_json",
                    "arg0 decode failed",
                    arg_count,
                    arg1,
                    arg2,
                    |program_json| {
                        trace_log_args(format_args!(
                            "{} mir_builder input_bytes={}",
                            STAGE1_MODULE_DISPATCH_TAG,
                            program_json.len()
                        ));
                        if crate::env_flags::flag_on_cached(
                            &MIR_BUILDER_TRACE_ENABLED,
                            TRACE_ENV,
                        ) {
                            let preview: String = program_json.chars().take(120).collect();
                            trace_log_args(format_args!(
                                "{} mir_builder input_preview={:?}",
                                STAGE1_MODULE_DISPATCH_TAG, preview
                            ));
                        }
                    },
                    nyash_rust::host_providers::mir_builder::program_json_to_mir_json_with_user_box_decls,
                ),
                DispatchAction::MirBuilderSource => dispatch_mir_builder_route(
                    "mir_builder source",
                    "source_text",
                    "source decode failed",
                    arg_count,
                    arg1,
                    arg2,
                    |_| {},
                    nyash_rust::host_providers::mir_builder::source_to_mir_json,
                ),
            };
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_route_returns_none() {
        let recv = materialize_owned_string("lang.compiler.unknown".to_owned());
        assert_eq!(try_dispatch(recv, "resolve_for_source", 0, 0, 0), None);
    }

    #[test]
    fn using_resolver_route_returns_empty_string_handle() {
        let recv = materialize_owned_string(USING_RESOLVER_MODULE.to_owned());
        let out = try_dispatch(recv, "resolve_for_source", 0, 0, 0).expect("dispatch result");
        assert_eq!(
            owned_string_from_handle(out).expect("result string handle"),
            ""
        );
    }

    #[test]
    fn mir_builder_missing_arg_returns_freeze_contract_handle() {
        let recv = materialize_owned_string(MIR_BUILDER_MODULE.to_owned());
        let out =
            try_dispatch(recv, "emit_from_program_json_v0", 0, 0, 0).expect("dispatch result");
        let message = owned_string_from_handle(out).expect("result string handle");
        assert!(message.starts_with("[freeze:contract][stage1_mir_builder]"));
        assert!(message.contains("missing arg0"));
    }

    #[test]
    fn mir_builder_decode_failure_returns_freeze_contract_handle() {
        let recv = materialize_owned_string(MIR_BUILDER_MODULE.to_owned());
        let out =
            try_dispatch(recv, "emit_from_program_json_v0", 1, -1, -1).expect("dispatch result");
        let message = owned_string_from_handle(out).expect("result string handle");
        assert!(message.starts_with("[freeze:contract][stage1_mir_builder]"));
        assert!(message.contains("decode failed"));
    }

    #[test]
    fn mir_builder_stageb_program_json_returns_mir_json_handle() {
        with_env_vars(
            &[
                ("HAKO_MIR_BUILDER_INTERNAL", "1"),
                ("HAKO_SELFHOST_NO_DELEGATE", "0"),
            ],
            || {
                let recv = materialize_owned_string(MIR_BUILDER_MODULE.to_owned());
                let program_json = materialize_owned_string(
                    r#"{"body":[{"expr":{"args":[{"name":"args","type":"Var"}],"name":"StageBDriverBox.main","type":"Call"},"type":"Return"}],"kind":"Program","version":0}"#.to_string(),
                );
                let out = try_dispatch(recv, "emit_from_program_json_v0", 1, program_json, 0)
                    .expect("dispatch result");
                assert!(out > 0, "dispatch must return a string handle");
                let message = owned_string_from_handle(out).expect("result string handle");
                assert!(
                    message.starts_with('{'),
                    "expected MIR json payload, got: {}",
                    message
                );
                assert!(message.contains("functions"));
            },
        );
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
                let recv = materialize_owned_string(MIR_BUILDER_MODULE.to_owned());
                let program_json = materialize_owned_string(
                    r#"{"body":[{"expr":{"type":"Int","value":1},"type":"Return"}],"kind":"Program","version":0}"#.to_string(),
                );

                let out = try_dispatch(recv, "emit_from_program_json_v0", 1, program_json, 0)
                    .expect("dispatch result");
                let message = owned_string_from_handle(out).expect("result string handle");

                assert!(message.starts_with("[freeze:contract][stage1_mir_builder]"));
                assert!(message.contains("delegate disabled") || message.contains("internal off"));
            },
        );
    }
}
