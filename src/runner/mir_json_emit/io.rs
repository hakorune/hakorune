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

/// Export an already sealed selected Dynamic candidate without cloning it.
/// The ordinary harness exporter deliberately clones/refreshes and therefore
/// scrubs linear candidate metadata; W6's site-id handoff must consume the
/// final candidate projection as-is.
pub fn emit_mir_json_for_selected_dynamic_candidate(
    module: &crate::mir::MirModule,
    path: &std::path::Path,
) -> Result<(), String> {
    let root = build_mir_json_root(module)?;
    write_mir_json_root(path, &root)
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

#[cfg(test)]
mod tests {
    use super::emit_mir_json_for_selected_dynamic_candidate;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
        MirType,
    };

    #[test]
    fn selected_candidate_export_uses_the_supplied_module_without_refresh_clone() {
        let mut module = MirModule::new("selected_candidate_export".to_owned());
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_owned(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry")
            .set_terminator(MirInstruction::Return { value: None });
        module.add_function(function);

        let path = std::env::temp_dir().join(format!(
            "hakorune_selected_candidate_export_{}_{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        emit_mir_json_for_selected_dynamic_candidate(&module, &path).expect("candidate export");
        let json = std::fs::read_to_string(&path).expect("candidate JSON");
        assert!(json.contains("\"name\": \"main\""));
        std::fs::remove_file(path).expect("cleanup candidate JSON");
    }
}
