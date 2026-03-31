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
        | "env.now_ms"
        | "env.set" => Some(ExternProviderLane::RuntimeDirect),
        "env.mirbuilder.emit"
        | "env.mirbuilder_emit"
        | "env.codegen.emit_object"
        | "env.codegen.compile_ll_text"
        | "env.codegen.link_object"
        | "env.box_introspect.kind"
        | "hostbridge.extern_invoke" => Some(ExternProviderLane::LoaderCold),
        _ => None,
    }
}
