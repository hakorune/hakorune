use super::lane::{classify_extern_provider_lane, ExternProviderLane};

#[test]
fn classify_runtime_direct_lane_for_console_and_env() {
    for extern_name in [
        "print",
        "env.console.log",
        "env.console.warn",
        "env.console.error",
        "env.get",
        "env.now_ms",
        "env.set",
    ] {
        assert_eq!(
            classify_extern_provider_lane(extern_name),
            Some(ExternProviderLane::RuntimeDirect),
            "expected runtime-direct lane for {}",
            extern_name
        );
    }
}

#[test]
fn classify_loader_cold_lane_for_provider_and_hostbridge() {
    for extern_name in [
        "env.mirbuilder.emit",
        "env.mirbuilder_emit",
        "env.codegen.emit_object",
        "env.codegen.compile_ll_text",
        "env.codegen.link_object",
        "env.box_introspect.kind",
        "hostbridge.extern_invoke",
    ] {
        assert_eq!(
            classify_extern_provider_lane(extern_name),
            Some(ExternProviderLane::LoaderCold),
            "expected loader-cold lane for {}",
            extern_name
        );
    }
}

#[test]
fn classify_non_provider_names_as_none() {
    for extern_name in ["nyash.string.concat_hh", "exit", "panic"] {
        assert_eq!(classify_extern_provider_lane(extern_name), None);
    }
}
