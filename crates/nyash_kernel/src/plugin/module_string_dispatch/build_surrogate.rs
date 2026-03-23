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

    fn dispatch_build_box_emit_program_json(source: &str) -> String {
        let source_handle = encode_string_handle(source);
        let out = try_dispatch(BUILD_BOX_MODULE, BUILD_BOX_METHOD, 2, source_handle, 0)
            .expect("dispatch");
        decode_string_handle(out).expect("program json string handle")
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
    fn dispatch_build_box_unsupported_source_returns_freeze_tag() {
        let result_text =
            dispatch_build_box_emit_program_json("static box NotMain { main() { return 0 } }");
        assert!(result_text.contains("[freeze:contract][stage1_program_json_v0]"));
    }
}
