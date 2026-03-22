use super::{decode_string_handle, encode_string_handle, trace_log};

const BUILD_BOX_MODULE: &str = "lang.compiler.build.build_box";
const BUILD_BOX_METHOD: &str = "emit_program_json_v0";

pub(super) fn try_dispatch(
    module_name: &str,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    if !is_build_box_emit_program_json_route(module_name, method_name) {
        return None;
    }
    Some(handle_build_box_emit_program_json_v0(arg_count, arg1, arg2).unwrap_or(0))
}

fn is_build_box_emit_program_json_route(module_name: &str, method_name: &str) -> bool {
    module_name == BUILD_BOX_MODULE && method_name == BUILD_BOX_METHOD
}

fn handle_build_box_emit_program_json_v0(arg_count: i64, arg1: i64, _arg2: i64) -> Option<i64> {
    let source_text = decode_build_box_source_text(arg_count, arg1)?;
    let program_json = emit_build_box_program_json(&source_text);
    trace_log("[stage1/module_dispatch] build_surrogate emitted program_json");
    Some(encode_build_box_program_json_result(program_json))
}

fn decode_build_box_source_text(arg_count: i64, arg1: i64) -> Option<String> {
    if arg_count < 1 {
        return None;
    }
    decode_string_handle(arg1)
}

fn emit_build_box_program_json(source_text: &str) -> Result<String, String> {
    nyash_rust::stage1::program_json_v0::emit_program_json_v0_for_current_stage1_build_box_mode(
        source_text,
    )
}

fn encode_build_box_program_json_result(program_json: Result<String, String>) -> i64 {
    match program_json {
        Ok(program_json) => encode_string_handle(&program_json),
        Err(error_text) => encode_string_handle(&error_text),
    }
}

#[cfg(test)]
mod tests {
    use super::{BUILD_BOX_METHOD, BUILD_BOX_MODULE, try_dispatch};
    use crate::plugin::module_string_dispatch::{decode_string_handle, encode_string_handle};
    use crate::test_support::{with_env_var, with_env_vars};

    fn dispatch_build_box_emit_program_json(source: &str) -> String {
        let source_handle = encode_string_handle(source);
        let out = try_dispatch(BUILD_BOX_MODULE, BUILD_BOX_METHOD, 2, source_handle, 0)
            .expect("dispatch");
        decode_string_handle(out).expect("program json string handle")
    }

    fn module_handle(name: &str) -> i64 {
        encode_string_handle(name)
    }

    fn dispatch_stage1_mir_builder_for_program_json(program_json: String) -> String {
        let receiver_handle = module_handle("lang.mir.builder.MirBuilderBox");
        let program_handle = encode_string_handle(&program_json);
        let out = crate::plugin::module_string_dispatch::try_dispatch(
            receiver_handle,
            "emit_from_program_json_v0",
            1,
            program_handle,
            0,
        )
        .expect("dispatch");
        assert!(out > 0, "expected MIR JSON StringBox handle");
        decode_string_handle(out).expect("MIR JSON string handle")
    }

    #[test]
    fn build_surrogate_route_contract_is_stable() {
        assert_eq!(BUILD_BOX_MODULE, "lang.compiler.build.build_box");
        assert_eq!(BUILD_BOX_METHOD, "emit_program_json_v0");
    }

    #[test]
    fn build_box_missing_arg_returns_zero_handle() {
        let out = try_dispatch(BUILD_BOX_MODULE, BUILD_BOX_METHOD, 0, 0, 0).expect("dispatch");
        assert_eq!(out, 0);
    }

    #[test]
    fn dispatch_accepts_stage1_build_box_module_receiver() {
        let program_json = dispatch_build_box_emit_program_json(
            "static box Main { main() { print(42) return 0 } }",
        );
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(program_json.contains("\"version\":0"));
    }

    #[test]
    fn build_box_route_selection_prefers_strict_authority_mode() {
        with_env_var("NYASH_STAGE1_MODE", "emit-program", || {
            let program_json = dispatch_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/stage1_cli_env.hako"
            ));
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&program_json).expect("valid json")["kind"],
                "Program"
            );
        });
    }

    #[test]
    fn build_box_route_selection_prefers_legacy_emit_program_env_alias() {
        with_env_var("STAGE1_EMIT_PROGRAM_JSON", "1", || {
            let program_json = dispatch_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/stage1_cli_env.hako"
            ));
            assert!(program_json.contains("\"kind\":\"Program\""));
        });
    }

    #[test]
    fn build_box_route_selection_uses_strict_default_for_authority_safe_source() {
        with_env_vars(&[], || {
            let program_json = dispatch_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/stage1_cli_env.hako"
            ));
            assert!(program_json.contains("\"kind\":\"Program\""));
        });
    }

    #[test]
    fn build_box_route_selection_reports_exact_relaxed_reason() {
        with_env_vars(&[], || {
            let program_json = dispatch_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/launcher.hako"
            ));
            assert!(program_json.contains("\"kind\":\"Program\""));
        });
    }

    #[test]
    fn dispatch_stage1_build_box_route_emits_stage1_cli_env_imports() {
        with_env_vars(&[], || {
            let program_json = dispatch_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/stage1_cli_env.hako"
            ));
            assert!(program_json.contains("\"kind\":\"Program\""));
            assert!(program_json.contains("\"box\":\"Main\""));
            assert!(program_json.contains("\"box\":\"Stage1InputContractBox\""));
            assert!(program_json.contains("\"box\":\"Stage1ProgramAuthorityBox\""));
            assert!(!program_json.contains("\"box\":\"FuncScannerBox\""));
            assert!(program_json.contains("\"imports\":"));
            assert!(program_json.contains("\"BuildBox\":\"lang.compiler.build.build_box\""));
            assert!(
                program_json.contains("\"FuncScannerBox\":\"lang.compiler.entry.func_scanner\"")
            );
        });
    }

    #[test]
    fn dispatch_stage1_build_box_route_emits_launcher_multibox_defs() {
        with_env_vars(&[], || {
            let program_json = dispatch_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/launcher.hako"
            ));
            assert!(program_json.contains("\"kind\":\"Program\""));
            assert!(program_json.contains("\"box\":\"HakoCli\""));
            assert!(program_json.contains("\"name\":\"run\""));
            assert!(program_json.contains("\"MirBuilderBox\":\"lang.mir.builder.MirBuilderBox\""));
        });
    }

    #[test]
    fn dispatch_stage1_build_box_route_keeps_dev_local_alias_compat_only_when_needed() {
        with_env_vars(&[], || {
            let program_json = dispatch_build_box_emit_program_json(
                r#"
static box Main {
  main() {
    @x = 41
    return x + 1
  }
}
"#,
            );
            assert!(program_json.contains("\"kind\":\"Program\""));
            assert!(!program_json.contains("[freeze:contract][stage1_program_json_v0]"));
        });
    }

    #[test]
    fn dispatch_stage1_build_box_route_is_strict_in_emit_program_mode() {
        with_env_var("NYASH_STAGE1_MODE", "emit-program", || {
            let result_text = dispatch_build_box_emit_program_json(
                r#"
static box Main {
  main() {
    @x = 41
    return x + 1
  }
}
"#,
            );
            assert!(result_text.contains("[freeze:contract][stage1_program_json_v0]"));
            assert!(
                result_text
                    .contains("source route rejects compat-only relaxed-compat source shape")
                    && result_text.contains("dev-local-alias-sugar"),
            );
        });
    }

    #[test]
    fn dispatch_build_box_unsupported_source_returns_freeze_tag() {
        let result_text =
            dispatch_build_box_emit_program_json("static box NotMain { main() { return 0 } }");
        assert!(result_text.contains("[freeze:contract][stage1_program_json_v0]"));
    }

    #[test]
    fn dispatch_accepts_stage1_mir_builder_for_stage1_cli_env_program_json() {
        let source = include_str!("../../../../../lang/src/runner/stage1_cli_env.hako");
        let build_result_text = dispatch_build_box_emit_program_json(source);
        let mir_json = dispatch_stage1_mir_builder_for_program_json(build_result_text);
        assert!(
            mir_json.starts_with('{'),
            "expected MIR JSON payload, got: {}",
            mir_json
        );
        assert!(mir_json.contains("\"functions\""));
    }

    #[test]
    fn dispatch_accepts_stage1_mir_builder_for_launcher_program_json() {
        let source = include_str!("../../../../../lang/src/runner/launcher.hako");
        let build_result_text = dispatch_build_box_emit_program_json(source);
        let mir_json = dispatch_stage1_mir_builder_for_program_json(build_result_text);
        assert!(mir_json.contains("\"functions\""));
        assert!(
            mir_json.contains("\"name\":\"Main\""),
            "stage1 mir-builder surrogate should always expose Main user_box_decl"
        );
        assert!(
            mir_json.contains("\"name\":\"HakoCli\""),
            "stage1 mir-builder surrogate should expose launcher helper box decls"
        );
    }
}
