//! Shared runtime-side `env.mirbuilder.emit` bridge helpers.
//!
//! This module keeps the runtime/plugin-side `Program(JSON v0) -> MIR(JSON)` bridge
//! on one owner so interpreter/provider paths and plugin-loader paths do not each
//! reimplement env-import parsing or direct host-provider calls.

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
    crate::host_providers::mir_builder::program_json_to_mir_json_with_imports(
        program_json,
        imports_from_env(),
    )
}
