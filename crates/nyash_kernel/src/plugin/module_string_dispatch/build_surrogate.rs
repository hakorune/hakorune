use super::{decode_string_handle, encode_string_handle, trace_log};

const BUILD_BOX_MODULE: &str = "lang.compiler.build.build_box";
const BUILD_BOX_METHOD: &str = "emit_program_json_v0";

struct BuildSurrogateRoute;

struct BuildSurrogateCall {
    source_text: String,
}

enum BuildSurrogateHandleResult {
    EncodedText(String),
    MissingSourceArg,
}

pub(super) fn try_dispatch(
    module_name: &str,
    method_name: &str,
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> Option<i64> {
    if !BuildSurrogateRoute::matches(module_name, method_name) {
        return None;
    }

    Some(dispatch_build_box_emit_program_json_v0(arg_count, arg1, arg2).into_handle())
}

impl BuildSurrogateRoute {
    fn matches(module_name: &str, method_name: &str) -> bool {
        module_name == BUILD_BOX_MODULE && method_name == BUILD_BOX_METHOD
    }
}

impl BuildSurrogateCall {
    fn decode(arg_count: i64, arg1: i64, _arg2: i64) -> Option<Self> {
        if arg_count < 1 {
            return None;
        }

        let source_text = decode_string_handle(arg1)?;
        Some(Self { source_text })
    }

    fn execute(self) -> BuildSurrogateHandleResult {
        let program_json =
            nyash_rust::stage1::program_json_v0::emit_program_json_v0_for_current_stage1_build_box_mode(
                &self.source_text,
            );
        trace_log("[stage1/module_dispatch] build_surrogate emitted program_json");
        BuildSurrogateHandleResult::from_program_json_result(program_json)
    }
}

impl BuildSurrogateHandleResult {
    fn from_program_json_result(program_json: Result<String, String>) -> Self {
        match program_json {
            Ok(program_json) => Self::EncodedText(program_json),
            Err(error_text) => Self::EncodedText(error_text),
        }
    }

    fn into_handle(self) -> i64 {
        match self {
            Self::EncodedText(text) => encode_string_handle(&text),
            Self::MissingSourceArg => 0,
        }
    }
}

fn dispatch_build_box_emit_program_json_v0(
    arg_count: i64,
    arg1: i64,
    arg2: i64,
) -> BuildSurrogateHandleResult {
    match BuildSurrogateCall::decode(arg_count, arg1, arg2) {
        Some(call) => call.execute(),
        None => BuildSurrogateHandleResult::MissingSourceArg,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        dispatch_build_box_emit_program_json_v0, try_dispatch, BuildSurrogateHandleResult,
        BUILD_BOX_METHOD, BUILD_BOX_MODULE,
    };
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
        let result = dispatch_build_box_emit_program_json_v0(0, 0, 0);
        assert!(matches!(
            result,
            BuildSurrogateHandleResult::MissingSourceArg
        ));
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
