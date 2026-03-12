/*!
 * Stage-1 bridge binary-only direct route - emit-output helper.
 */

use crate::mir::MirModule;
use std::path::Path;

pub(super) fn emit_mir_json(module: &MirModule, cli_path: Option<String>) -> Result<(), String> {
    let Some(path) = super::super::emit_paths::resolve_mir_out_path(cli_path) else {
        return Err("output path is required".to_string());
    };

    let path_ref = Path::new(&path);
    crate::runner::mir_json_emit::emit_mir_json_for_harness_bin(module, path_ref)
        .map_err(|error| format!("MIR JSON emit error: {}", error))?;
    println!("MIR JSON written: {}", path_ref.display());
    Ok(())
}
