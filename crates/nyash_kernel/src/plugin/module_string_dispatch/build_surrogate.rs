use super::{decode_string_handle, encode_string_handle, trace_log};

const BUILD_BOX_MODULE: &str = "lang.compiler.build.build_box";

pub(super) const BUILD_SURROGATE_ROUTE: super::DispatchRoute = super::DispatchRoute {
    module: BUILD_BOX_MODULE,
    method: "emit_program_json_v0",
    handler: handle_build_box_emit_program_json_v0,
};

fn handle_build_box_emit_program_json_v0(
    arg_count: i64,
    arg1: i64,
    _arg2: i64,
) -> Option<i64> {
    if arg_count < 1 {
        return Some(0);
    }
    let source_text = match decode_string_handle(arg1) {
        Some(text) => text,
        None => return Some(0),
    };
    let program_json = match nyash_rust::stage1::program_json_v0::
        emit_program_json_v0_for_current_stage1_build_box_mode(&source_text)
    {
        Ok(program_json) => program_json,
        Err(error_text) => {
            return Some(encode_string_handle(&error_text));
        }
    };
    trace_log("[stage1/module_dispatch] build_surrogate emitted program_json");
    Some(encode_string_handle(&program_json))
}

#[cfg(test)]
mod tests {
    use super::BUILD_SURROGATE_ROUTE;
    use crate::plugin::module_string_dispatch::{decode_string_handle, encode_string_handle};
    use crate::test_support::{with_env_var, with_env_vars};
    use nyash_rust::box_trait::{NyashBox, StringBox};
    use nyash_rust::runtime::host_handles as handles;
    use std::ffi::CString;
    use std::sync::Arc;

    fn dispatch_build_box_emit_program_json(source: &str) -> String {
        let source_handle = encode_string_handle(source);
        let out = (BUILD_SURROGATE_ROUTE.handler)(2, source_handle, 0).expect("dispatch");
        decode_string_handle(out).expect("program json string handle")
    }

    fn decode_string_like_handle(handle: i64) -> String {
        let object = handles::get(handle as u64).expect("result handle");
        object
            .as_any()
            .downcast_ref::<StringBox>()
            .map(|string_box| string_box.value.clone())
            .unwrap_or_else(|| object.to_string_box().value)
    }

    fn ensure_test_ring0() {
        let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
    }

    fn invoke_by_name_build_box_emit_program_json(source: &str) -> String {
        ensure_test_ring0();
        let receiver: Arc<dyn NyashBox> =
            Arc::new(StringBox::new(BUILD_SURROGATE_ROUTE.module.to_string()));
        let receiver_handle = handles::to_handle_arc(receiver) as i64;
        let source_handle =
            handles::to_handle_arc(Arc::new(StringBox::new(source.to_string()))) as i64;
        let method = CString::new(BUILD_SURROGATE_ROUTE.method).expect("CString");
        let result_handle = crate::nyash_plugin_invoke_by_name_i64(
            receiver_handle,
            method.as_ptr(),
            2,
            source_handle,
            0,
        );
        assert!(result_handle > 0, "expected StringBox handle");
        decode_string_handle(result_handle).expect("program json string handle")
    }

    fn invoke_stage1_mir_builder_for_program_json(program_json: String) -> String {
        ensure_test_ring0();
        let receiver: Arc<dyn NyashBox> =
            Arc::new(StringBox::new("lang.mir.builder.MirBuilderBox".to_string()));
        let receiver_handle = handles::to_handle_arc(receiver) as i64;
        let program_handle =
            handles::to_handle_arc(Arc::new(StringBox::new(program_json))) as i64;
        let method = CString::new("emit_from_program_json_v0").expect("CString");

        let result_handle = crate::nyash_plugin_invoke_by_name_i64(
            receiver_handle,
            method.as_ptr(),
            1,
            program_handle,
            0,
        );
        assert!(result_handle > 0, "expected MIR JSON StringBox handle");

        decode_string_like_handle(result_handle)
    }

    #[test]
    fn build_surrogate_route_registration_is_stable() {
        assert_eq!(BUILD_SURROGATE_ROUTE.module, "lang.compiler.build.build_box");
        assert_eq!(BUILD_SURROGATE_ROUTE.method, "emit_program_json_v0");
    }

    #[test]
    fn build_box_missing_arg_returns_zero_handle() {
        let out = (BUILD_SURROGATE_ROUTE.handler)(0, 0, 0).expect("dispatch");
        assert_eq!(out, 0);
    }

    #[test]
    fn invoke_by_name_accepts_stage1_build_box_module_receiver() {
        let program_json = invoke_by_name_build_box_emit_program_json(
            "static box Main { main() { print(42) return 0 } }",
        );
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(program_json.contains("\"version\":0"));
    }

    #[test]
    fn build_box_route_selection_prefers_strict_authority_mode() {
        with_env_var("NYASH_STAGE1_MODE", "emit-program", || {
            let program_json =
                dispatch_build_box_emit_program_json(include_str!(
                    "../../../../../lang/src/runner/stage1_cli_env.hako"
                ));
            assert_eq!(
                serde_json::from_str::<serde_json::Value>(&program_json)
                    .expect("valid json")["kind"],
                "Program"
            );
        });
    }

    #[test]
    fn build_box_route_selection_prefers_legacy_emit_program_env_alias() {
        with_env_var("STAGE1_EMIT_PROGRAM_JSON", "1", || {
            let program_json =
                dispatch_build_box_emit_program_json(include_str!(
                    "../../../../../lang/src/runner/stage1_cli_env.hako"
                ));
            assert!(program_json.contains("\"kind\":\"Program\""));
        });
    }

    #[test]
    fn build_box_route_selection_uses_strict_default_for_authority_safe_source() {
        with_env_vars(&[], || {
            let program_json =
                dispatch_build_box_emit_program_json(include_str!(
                    "../../../../../lang/src/runner/stage1_cli_env.hako"
                ));
            assert!(program_json.contains("\"kind\":\"Program\""));
        });
    }

    #[test]
    fn build_box_route_selection_reports_exact_relaxed_reason() {
        with_env_vars(&[], || {
            let program_json =
                dispatch_build_box_emit_program_json(include_str!(
                    "../../../../../lang/src/runner/launcher.hako"
                ));
            assert!(program_json.contains("\"kind\":\"Program\""));
        });
    }

    #[test]
    fn invoke_by_name_stage1_build_box_route_emits_stage1_cli_env_imports() {
        with_env_vars(&[], || {
            let program_json = invoke_by_name_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/stage1_cli_env.hako"
            ));
            assert!(program_json.contains("\"kind\":\"Program\""));
            assert!(program_json.contains("\"box\":\"Main\""));
            assert!(program_json.contains("\"box\":\"Stage1InputContractBox\""));
            assert!(program_json.contains("\"box\":\"Stage1ProgramAuthorityBox\""));
            assert!(!program_json.contains("\"box\":\"FuncScannerBox\""));
            assert!(program_json.contains("\"imports\":"));
            assert!(program_json.contains("\"BuildBox\":\"lang.compiler.build.build_box\""));
            assert!(program_json.contains("\"FuncScannerBox\":\"lang.compiler.entry.func_scanner\""));
        });
    }

    #[test]
    fn invoke_by_name_stage1_build_box_route_emits_launcher_multibox_defs() {
        with_env_vars(&[], || {
            let program_json = invoke_by_name_build_box_emit_program_json(include_str!(
                "../../../../../lang/src/runner/launcher.hako"
            ));
            assert!(program_json.contains("\"kind\":\"Program\""));
            assert!(program_json.contains("\"box\":\"HakoCli\""));
            assert!(program_json.contains("\"name\":\"run\""));
            assert!(program_json.contains("\"MirBuilderBox\":\"lang.mir.builder.MirBuilderBox\""));
        });
    }

    #[test]
    fn invoke_by_name_stage1_build_box_route_keeps_dev_local_alias_compat_only_when_needed() {
        with_env_vars(&[], || {
            let program_json = invoke_by_name_build_box_emit_program_json(
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
    fn invoke_by_name_stage1_build_box_route_is_strict_in_emit_program_mode() {
        with_env_var("NYASH_STAGE1_MODE", "emit-program", || {
            let result_text = invoke_by_name_build_box_emit_program_json(
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
                result_text.contains("source route rejects compat-only relaxed-compat source shape")
                    && result_text.contains("dev-local-alias-sugar"),
            );
        });
    }

    #[test]
    fn invoke_by_name_build_box_unsupported_source_returns_freeze_tag() {
        let result_text =
            invoke_by_name_build_box_emit_program_json("static box NotMain { main() { return 0 } }");
        assert!(result_text.contains("[freeze:contract][stage1_program_json_v0]"));
    }

    #[test]
    fn invoke_by_name_accepts_stage1_mir_builder_for_stage1_cli_env_program_json() {
        let source = include_str!("../../../../../lang/src/runner/stage1_cli_env.hako");
        let build_result_text = invoke_by_name_build_box_emit_program_json(source);
        let mir_json = invoke_stage1_mir_builder_for_program_json(build_result_text);
        assert!(
            mir_json.starts_with('{'),
            "expected MIR JSON payload, got: {}",
            mir_json
        );
        assert!(mir_json.contains("\"functions\""));
    }

    #[test]
    fn invoke_by_name_accepts_stage1_mir_builder_for_launcher_program_json() {
        let source = include_str!("../../../../../lang/src/runner/launcher.hako");
        let build_result_text = invoke_by_name_build_box_emit_program_json(source);
        let mir_json = invoke_stage1_mir_builder_for_program_json(build_result_text);
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
