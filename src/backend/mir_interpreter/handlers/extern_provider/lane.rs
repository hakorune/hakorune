// Runtime bridge lane classification for extern dispatch.
// Keep this separate from smoke-retirement ownership docs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ExternProviderLane {
    RuntimeDirect,
    LoaderCold,
}

pub(super) fn classify_extern_provider_lane(extern_name: &str) -> Option<ExternProviderLane> {
    match extern_name {
        "nyash.console.log"
        | "env.console.log"
        | "print"
        | "nyash.builtin.print"
        | "env.console.warn"
        | "nyash.console.warn"
        | "env.error"
        | "env.error/1"
        | "env.console.error"
        | "env.console.error/1"
        | "nyash.console.error"
        | "env.get"
        | "env.file.read"
        | "env.now_ms"
        | "env.set"
        | "nyash.runtime_data.get_hh"
        | "nyash.runtime_data.set_hhh"
        | "nyash.runtime_data.has_hh"
        | "nyash.runtime_data.push_hh"
        | "hako.analysis.decoded_utf8_byte_len_v0"
        | "hako.analysis.strict_json_tree_v0.kind"
        | "hako.analysis.strict_json_tree_v0.object_len"
        | "hako.analysis.strict_json_tree_v0.object_key_at"
        | "hako.analysis.strict_json_tree_v0.object_value_at"
        | "hako.analysis.strict_json_tree_v0.array_len"
        | "hako.analysis.strict_json_tree_v0.array_at"
        | "hako.analysis.strict_json_tree_v0.string_value"
        | "hako.analysis.strict_json_tree_v0.bool_value"
        | "hako.analysis.strict_json_tree_v0.i64_value"
        | "hako.analysis.strict_json_tree_v0.u64_fits_i64"
        | "hako.analysis.strict_json_tree_v0.u64_as_i64" => Some(ExternProviderLane::RuntimeDirect),
        "env.mirbuilder.emit"
        | "env.mirbuilder_emit"
        | "env.codegen.emit_object"
        | "env.codegen.emit_object_compat_harness"
        | "env.codegen.compile_ll_text"
        | "env.codegen.link_object"
        | "env.box_introspect.kind"
        | "hostbridge.extern_invoke" => Some(ExternProviderLane::LoaderCold),
        _ => None,
    }
}
