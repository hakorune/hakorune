// Compiled-stage1 BuildBox route helper.
// Keep this as a module-local route. Program(JSON) authority remains in
// `nyash_rust::stage1::program_json_v0`.

use super::trace_log_args;

const BUILD_BOX_MODULE: &str = "lang.compiler.build.build_box";
const BUILD_BOX_METHOD: &str = "emit_program_json_v0";

pub(super) fn try_dispatch(
    module_name: &str,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    _arg2: i64,
) -> Option<i64> {
    if module_name != BUILD_BOX_MODULE || method_name != BUILD_BOX_METHOD {
        return None;
    }
    if arg_count < 1 {
        return Some(0);
    }

    let Some(source_text) = crate::plugin::owned_string_from_handle(arg1) else {
        return Some(0);
    };
    let program_json =
        nyash_rust::stage1::program_json_v0::emit_program_json_v0_for_current_stage1_build_box_mode(
            &source_text,
        );
    trace_log_args(format_args!(
        "[stage1/module_dispatch] build_surrogate emitted program_json"
    ));
    let text = match program_json {
        Ok(program_json) => program_json,
        Err(error_text) => error_text,
    };
    Some(crate::plugin::materialize_owned_string(text))
}

#[cfg(test)]
mod tests {
    use super::{try_dispatch, BUILD_BOX_METHOD, BUILD_BOX_MODULE};
    use crate::plugin::{materialize_owned_string, owned_string_from_handle};

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
    fn build_box_invalid_source_handle_returns_zero_handle() {
        let out = try_dispatch(BUILD_BOX_MODULE, BUILD_BOX_METHOD, 1, 0, 0).expect("dispatch");
        assert_eq!(out, 0);
    }

    #[test]
    fn build_box_unrelated_route_returns_none() {
        assert!(try_dispatch("lang.compiler.build.other_box", BUILD_BOX_METHOD, 0, 0, 0).is_none());
    }

    #[test]
    fn dispatch_missing_source_arg_returns_missing_source_result() {
        assert_eq!(
            try_dispatch(BUILD_BOX_MODULE, BUILD_BOX_METHOD, 0, 0, 0),
            Some(0)
        );
    }

    #[test]
    fn dispatch_accepts_stage1_build_box_module_receiver() {
        let source_handle = materialize_owned_string(
            "static box Main { main() { print(42) return 0 } }".to_string(),
        );
        let out = try_dispatch(BUILD_BOX_MODULE, BUILD_BOX_METHOD, 2, source_handle, 0)
            .expect("dispatch");
        let program_json = owned_string_from_handle(out).expect("program json string handle");
        assert!(program_json.contains("\"kind\":\"Program\""));
        assert!(program_json.contains("\"version\":0"));
    }

    #[test]
    fn dispatch_build_box_unsupported_source_returns_freeze_tag() {
        let source_handle =
            materialize_owned_string("static box NotMain { main() { return 0 } }".to_string());
        let out = try_dispatch(BUILD_BOX_MODULE, BUILD_BOX_METHOD, 2, source_handle, 0)
            .expect("dispatch");
        let result_text = owned_string_from_handle(out).expect("program json string handle");
        assert!(result_text.contains("[freeze:contract][stage1_program_json_v0]"));
    }
}
