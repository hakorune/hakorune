//! Shared runtime-side `env.mirbuilder.emit` bridge helpers.
//!
//! This module keeps the runtime/plugin-side `Program(JSON v0) -> MIR(JSON)` bridge
//! on one owner so interpreter/provider paths and plugin-loader paths do not each
//! reimplement env-import parsing or direct lowering/emit glue.

use std::collections::BTreeMap;

pub fn imports_from_env() -> BTreeMap<String, String> {
    if let Ok(imports_json) = std::env::var("HAKO_MIRBUILDER_IMPORTS") {
        match serde_json::from_str::<BTreeMap<String, String>>(&imports_json) {
            Ok(map) => map,
            Err(e) => {
                crate::runtime::get_global_ring0().log.error(&format!(
                    "[mirbuilder/imports] Failed to parse HAKO_MIRBUILDER_IMPORTS: {}",
                    e
                ));
                BTreeMap::new()
            }
        }
    } else {
        BTreeMap::new()
    }
}

pub fn emit_program_json_to_mir_json_with_env_imports(
    program_json: &str,
) -> Result<String, String> {
    let _env_guard = crate::host_providers::mir_builder::Phase0MirJsonEnvGuard::new();
    let module = crate::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        program_json,
        imports_from_env(),
    )
    .map_err(crate::host_providers::mir_builder::failfast_error)?;
    crate::host_providers::mir_builder::module_to_mir_json(&module)
}
