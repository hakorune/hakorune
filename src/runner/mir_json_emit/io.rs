use super::*;

/// Physical body projection of the same final view used for typed C rows.
pub(crate) fn emit_published_view_body(
    view: &crate::mir::function::PublishedMirBackendView<'_>,
) -> Result<String, String> {
    if view.route() != crate::mir::function::PublishedStaticMethodRouteV1::CanonicalTyped {
        return Err("[freeze:contract][published-body/not-canonical-typed]".into());
    }
    let module = view.module();
    crate::mir::semantic_refresh::validate_published_contracts(module)?;
    for function in module.functions.values() {
        for block in function.blocks.values() {
            let mut seen_non_phi = false;
            for instruction in &block.instructions {
                if matches!(instruction, crate::mir::MirInstruction::Phi { .. }) {
                    if seen_non_phi {
                        return Err("[freeze:contract][published-body/nonleading-phi]".into());
                    }
                } else {
                    seen_non_phi = true;
                }
            }
        }
    }
    let root = super::root::build_mir_json_root_with_profile(
        module,
        super::root::JsonEgressProfile::CanonicalV1,
    )?;
    serialize_mir_json_root(&root)
}

/// Same-borrow physical egress for the V2 lifecycle companion. Typed rows,
/// not this JSON, remain the authority for lifecycle identity and layout.
pub(crate) fn emit_published_lifecycle_body(
    view: &crate::mir::function::PublishedMirBackendView<'_>,
) -> Result<String, String> {
    if view.route() != crate::mir::function::PublishedStaticMethodRouteV1::CanonicalTyped
        || view.lifecycle_instructions().is_empty()
    { return Err("[freeze:contract][published-lifecycle-body/not-final-lifecycle-view]".into()); }
    let root = super::root::build_mir_json_root_with_profile(
        view.module(), super::root::JsonEgressProfile::PublishedLifecycleV2,
    )?;
    serialize_mir_json_root(&root)
}

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

/// Emit the reference lane's canonical v1 root as one owned Value.
///
/// This is deliberately separate from the compatibility/harness string
/// writers: the caller already selected the reference family, so no ambient
/// profile selector or intermediate JSON string is allowed here.
pub(crate) fn emit_canonical_v1_value_for_reference(
    module: &crate::mir::MirModule,
) -> Result<serde_json::Value, String> {
    let refreshed = refreshed_export_module(module)?;
    super::root::build_mir_json_root_with_profile(
        refreshed.module(),
        super::root::JsonEgressProfile::CanonicalV1,
    )
}

/// Caller-zero export for a final unpublished candidate. Unlike the harness
/// path, this does not refresh or clone the module, so affine carrier metadata
/// and its exact CFG image remain one lineage.
#[cfg(test)]
pub(crate) fn emit_mir_json_string_for_unpublished_candidate(
    module: &crate::mir::MirModule,
) -> Result<String, String> {
    let root = build_mir_json_root(module)?;
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

    fn published_print_module() -> MirModule {
        let mut module = MirModule::new("published-print".into());
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".into(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::IO,
            },
            BasicBlockId::new(0),
        );
        let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: crate::mir::ConstValue::Integer(42),
        });
        block.add_instruction(MirInstruction::call(
            None,
            crate::mir::Callee::Global(hakorune_mir_defs::CanonicalGlobalTargetV1::builtin_print()),
            vec![ValueId::new(1)],
            EffectMask::IO,
        ));
        block.set_terminator(MirInstruction::Return { value: None });
        module.add_function(function);
        module
    }

    #[test]
    fn published_body_preserves_the_borrowed_image_and_typed_site() {
        let module = published_print_module();
        let before = format!("{module:?}");
        let view = crate::mir::function::PublishedMirBackendView::try_new(&module).unwrap();
        crate::mir::backend_capability::enforce_published_backend_supported(&view, "ny-llvmc-obj")
            .unwrap();
        let frame = crate::mir::function::PublishedStaticMethodCFrameV1::from_view(&view).unwrap();
        let row = &frame.as_slice()[0];
        let body: serde_json::Value =
            serde_json::from_str(&super::emit_published_view_body(&view).unwrap()).unwrap();
        assert_eq!(row.block_id, 0);
        assert_eq!(row.instruction_index, 1);
        assert_eq!(
            body["functions"][0]["blocks"][0]["instructions"][1]["op"],
            "mir_call"
        );
        assert_eq!(before, format!("{module:?}"));
    }

    #[test]
    fn published_body_rejects_nonleading_phi_instead_of_reordering_sites() {
        let mut module = published_print_module();
        module
            .functions
            .get_mut("main")
            .unwrap()
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Phi {
                dst: ValueId::new(2),
                inputs: vec![],
                type_hint: None,
            });
        let view = crate::mir::function::PublishedMirBackendView::try_new(&module).unwrap();
        assert_eq!(
            super::emit_published_view_body(&view).unwrap_err(),
            "[freeze:contract][published-body/nonleading-phi]"
        );
    }

    #[test]
    fn published_body_does_not_rebuild_missing_array_write_witnesses() {
        let mut module = published_print_module();
        module
            .functions
            .get_mut("main")
            .unwrap()
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .add_instruction(
                crate::mir::array_element_write::instruction(
                    crate::mir::ArrayWriteSiteId::new(1),
                    None,
                    crate::mir::ArrayElementWriteKind::Push,
                    crate::mir::ArrayWriteProducerKind::MethodCall,
                    ValueId::new(2),
                    None,
                    ValueId::new(1),
                )
                .unwrap(),
            );
        let view = crate::mir::function::PublishedMirBackendView::try_new(&module).unwrap();
        let error = super::emit_published_view_body(&view).unwrap_err();
        assert!(error.contains("family=array_write"), "{error}");
        assert!(module.functions["main"]
            .metadata
            .array_element_write_witnesses
            .is_empty());
    }

    #[test]
    fn published_preflight_stops_mixed_compatibility_even_without_route_metadata() {
        for legacy in [false, true] {
            let mut module = published_print_module();
            let callee = crate::mir::Callee::Extern("nyash.env.get".into());
            let instruction = if legacy {
                MirInstruction::LegacyCallV0 {
                    dst: Some(ValueId::new(3)),
                    func: ValueId::INVALID,
                    callee: Some(callee),
                    args: vec![ValueId::new(1)],
                    effects: EffectMask::IO,
                }
            } else {
                MirInstruction::call(
                    Some(ValueId::new(3)),
                    callee,
                    vec![ValueId::new(1)],
                    EffectMask::IO,
                )
            };
            module
                .functions
                .get_mut("main")
                .unwrap()
                .blocks
                .get_mut(&BasicBlockId::new(0))
                .unwrap()
                .add_instruction(instruction);
            assert!(module.functions["main"]
                .metadata
                .extern_call_routes
                .is_empty());
            let view = crate::mir::function::PublishedMirBackendView::try_new(&module).unwrap();
            assert_eq!(
                crate::mir::backend_capability::enforce_published_backend_supported(
                    &view,
                    "ny-llvmc-obj",
                )
                .unwrap_err(),
                "[freeze:contract][published-backend/compatibility-ingress]"
            );
        }
    }

    #[test]
    fn published_preflight_rejects_residual_extern_route_without_a_call() {
        use crate::mir::extern_call_route_plan::{
            ExternCallRoute, ExternCallRouteKind, ExternCallRouteSite,
        };
        let mut module = published_print_module();
        module
            .functions
            .get_mut("main")
            .unwrap()
            .metadata
            .extern_call_routes
            .push(ExternCallRoute::new(
                ExternCallRouteSite::new(BasicBlockId::new(0), 1),
                ExternCallRouteKind::EnvGet,
                "nyash.env.get",
                ValueId::new(1),
                None,
                ValueId::new(3),
            ));
        let view = crate::mir::function::PublishedMirBackendView::try_new(&module).unwrap();
        assert_eq!(
            crate::mir::backend_capability::enforce_published_backend_supported(
                &view,
                "ny-llvmc-obj",
            )
            .unwrap_err(),
            "[freeze:contract][published-backend/compatibility-ingress]"
        );
    }

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
