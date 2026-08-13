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
        MirType, ValueId,
    };
    use crate::runner::modes::common_util::selected_dynamic_identity::validate_selected_dynamic_launch_helper_identity;

    fn production_shaped_dual_function_fixture() -> MirModule {
        let mut module = MirModule::new("selected_dual_identity_fixture".to_owned());

        let mut launch = MirFunction::new(
            FunctionSignature {
                name: "main".to_owned(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        launch
            .get_block_mut(BasicBlockId::new(0))
            .expect("launch entry")
            .set_terminator(MirInstruction::Return { value: None });

        let mut helper = MirFunction::new(
            FunctionSignature {
                name: "ParserScanLoopBox.skip_while/4".to_owned(),
                params: vec![MirType::Unknown; 4],
                return_type: MirType::Integer,
                effects: EffectMask::READ,
            },
            BasicBlockId::new(0),
        );
        helper
            .metadata
            .install_a_prime_i64_physical_receipt_for_test(
                crate::mir::test_support::a_prime_receipt(),
            )
            .expect("fixture receipt");
        helper
            .metadata
            .install_dynamic_v2_aot_metadata_for_test(
                crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(),
            )
            .expect("fixture admission");
        helper
            .get_block_mut(BasicBlockId::new(0))
            .expect("helper entry")
            .set_terminator(MirInstruction::Return {
                value: Some(ValueId::new(31)),
            });

        module.add_function(launch);
        module.add_function(helper);
        module
    }

    #[test]
    fn production_shaped_dual_function_fixture_keeps_launch_and_helper_distinct() {
        let module = production_shaped_dual_function_fixture();
        validate_selected_dynamic_launch_helper_identity(&module).expect("identity fence");

        let root =
            crate::runner::mir_json_emit::root::build_mir_json_root(&module).expect("fixture JSON");
        let functions = root["functions"].as_array().expect("functions");
        assert_eq!(functions.len(), 2);

        let launch = functions
            .iter()
            .find(|function| function["name"] == "main")
            .expect("zero-argument launch");
        assert_eq!(launch["params"], serde_json::json!([]));
        assert!(launch["metadata"]
            .as_object()
            .expect("launch metadata")
            .get("a_prime_i64_physical_receipt")
            .is_none());
        assert!(launch["metadata"]
            .as_object()
            .expect("launch metadata")
            .get("dynamic_v2_aot_call_admission_v2")
            .is_none());

        let helper = functions
            .iter()
            .find(|function| function["name"] == "ParserScanLoopBox.skip_while/4")
            .expect("selected helper");
        assert_eq!(helper["params"], serde_json::json!([0, 1, 2, 3]));
        let metadata = helper["metadata"].as_object().expect("helper metadata");
        assert!(metadata.contains_key("a_prime_i64_physical_receipt"));
        assert!(metadata.contains_key("dynamic_v2_aot_call_admission_v2"));
    }

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

    #[test]
    fn selected_candidate_export_rejects_scrubbed_metadata_clone() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "ParserScanLoopBox.skip_while/4".to_owned(),
                params: vec![MirType::Unknown; 4],
                return_type: MirType::Integer,
                effects: EffectMask::READ,
            },
            BasicBlockId::new(0),
        );
        function
            .metadata
            .install_a_prime_i64_physical_receipt_for_test(
                crate::mir::test_support::a_prime_receipt(),
            )
            .expect("receipt install");
        function
            .metadata
            .install_dynamic_v2_aot_metadata_for_test(
                crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1::for_test(),
            )
            .expect("admission install");

        let scrubbed = function.clone();
        let mut module = MirModule::new("scrubbed-candidate".to_owned());
        module.add_function(scrubbed);
        let path = std::env::temp_dir().join(format!(
            "hakorune_scrubbed_candidate_{}_{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos()
        ));
        let error = emit_mir_json_for_selected_dynamic_candidate(&module, &path)
            .expect_err("scrubbed selected metadata must reject");
        assert!(error.contains("invalid selected Dynamic metadata lifecycle"));
        assert!(!path.exists());
    }
}
