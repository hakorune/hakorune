use super::*;

pub fn emit_mir_json_for_harness(
    module: &nyash_rust::mir::MirModule,
    path: &std::path::Path,
) -> Result<(), String> {
    emit_mir_json(module, path)
}

pub fn emit_mir_json_for_harness_bin(
    module: &crate::mir::MirModule,
    path: &std::path::Path,
) -> Result<(), String> {
    emit_mir_json(module, path)
}

pub fn emit_mir_json_string_for_harness_bin(
    module: &crate::mir::MirModule,
) -> Result<String, String> {
    let refreshed = refreshed_export_module(module)?;
    let root = build_mir_json_root(refreshed.module())?;
    serialize_mir_json_root(&root)
}

pub(super) fn emit_mir_json(
    module: &crate::mir::MirModule,
    path: &std::path::Path,
) -> Result<(), String> {
    let refreshed = refreshed_export_module(module)?;
    let root = build_mir_json_root(refreshed.module())?;
    write_mir_json_root(path, &root)
}

fn refreshed_export_module(
    module: &crate::mir::MirModule,
) -> Result<crate::mir::semantic_refresh::OwnedRefreshedContractBundle, String> {
    crate::mir::semantic_refresh::refresh_owned_for_boundary(
        module,
        crate::mir::ContractRefreshBoundary::MirJsonExport,
    )
}

pub(super) fn serialize_mir_json_root(root: &serde_json::Value) -> Result<String, String> {
    serde_json::to_string(root).map_err(|e| format!("write mir json: {}", e))
}

pub(super) fn write_mir_json_root(
    path: &std::path::Path,
    root: &serde_json::Value,
) -> Result<(), String> {
    let file = std::fs::File::create(path).map_err(|e| format!("write mir json: {}", e))?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &root)
        .map_err(|e| format!("write mir json: {}", e))?;
    writer
        .write_all(b"\n")
        .map_err(|e| format!("write mir json: {}", e))?;
    writer.flush().map_err(|e| format!("write mir json: {}", e))
}
