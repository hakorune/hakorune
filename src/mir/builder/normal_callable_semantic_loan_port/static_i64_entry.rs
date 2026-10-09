//! The selected Static I64 Loop is taken at the cataloged function boundary.
//! Its checked draft is collected only after exact packet retention preflight.

use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
use crate::mir::builder::{MirBuilder, NormalCatalogedBoxMethodDraftAdmissionV1};
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;
use crate::mir::normal_callable_semantic_package::SelectedCallableLoweringInputRefV1;

pub(super) fn collect_if_selected(
    builder: &mut MirBuilder,
    module_port: &mut ModuleLoweringPortV1<'_>,
    selected: &SelectedCallableLoweringInputRefV1<'_>,
    admission: &NormalCatalogedBoxMethodDraftAdmissionV1,
    claims: &OrdinaryNewClaimLedgerV1,
    signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
) -> Result<bool, String> {
    let input = selected.source();
    let Some(product) =
        super::super::raw_loop_child_entry::take_at_function_entry_v2(input, claims)?
    else {
        return Ok(false);
    };
    let formal = product.join_physical_signature_v2(signature)?;
    let result = claims
        .take_static_loop_i64_result_v1(&product.semantic().roles().loop_site)
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-result-source-missing]".to_owned()
        })??;
    if !result.corroborates(product.semantic()) {
        return Err("[freeze:contract][callable-loop/static-result-source-mismatch]".to_owned());
    }
    let packet = claims
        .take_static_loop_packet_source_v1(
            &product.semantic().roles().loop_site,
            product.semantic().source_calls().0.declaration(),
        )
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-packet-source-missing]".to_owned()
        })??;
    claims.require_static_loop_local_route_v1(&packet)?;
    let publication =
        crate::mir::builder::resolved_lowering::SelectedStaticLoopPublicationBatchV1::preflight_source(
            product.semantic(), &packet, module_port,
        )?;
    let (caller, site) = packet.publication_source();
    let prepared = {
        let handoff = module_port
            .selected_static_result_handoff_for_source(caller, site)
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-publication-handoff-missing]".to_owned()
            })?;
        if !packet.corroborates_publication_handoff(handoff) {
            return Err(
                "[freeze:contract][callable-loop/static-publication-handoff-drift]".to_owned(),
            );
        }
        crate::mir::builder::resolved_lowering::prepare_selected_static_loop_entry_v1(
            builder,
            input,
            selected.block_expr_expectation(),
            admission,
            &product,
            &formal,
            &result,
            packet,
            handoff,
            publication,
            claims,
        )?
    };
    prepared.collect(module_port, admission)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::{
        DraftPublicationPolicyV1, FunctionDraftKeyV1,
    };
    use crate::mir::builder::{
        CanonicalSameModuleCallableKeyV1, CompilationContext, MirBuilder,
        NormalCatalogedBoxMethodDraftAdmissionV1,
    };
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule, MirType};
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_with_identity_and_lineage_v1,
        NormalCallableMaterializationOutcomeV1,
    };

    #[test]
    fn original_static_loop_collects_only_after_packet_preflight() {
        std::thread::Builder::new()
            .name("static-loop-function-entry".into())
            .stack_size(32 * 1024 * 1024)
            .spawn(|| {
                let env_updates: Vec<_> = crate::test_support::JOINIR_DEFAULT_MODE
                    .into_iter()
                    .chain([
                        ("NYASH_ALLOW_USING_FILE", Some("1")),
                        ("NYASH_ENABLE_USING", Some("1")),
                        ("NYASH_OPERATOR_BOX_ALL", Some("0")),
                        ("NYASH_MACRO_DISABLE", Some("1")),
                    ])
                    .collect();
                crate::test_support::with_env_vars(&env_updates, || {
                    for (missing_route, duplicate_collector) in
                        [(false, false), (true, false), (false, true)]
                    {
                        let filename = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                            .join("apps/mimalloc-lite/main.hako");
                        let code = std::fs::read_to_string(&filename).unwrap();
                        let runner = crate::runner::NyashRunner::new(Default::default());
                        let prepared = crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports(
                            &runner, filename.to_str().unwrap(), &code,
                        ).unwrap();
                        let imports: Vec<_> = prepared.imports.into_iter().collect();
                        let transformed = materialize_normal_callable_program_with_identity_and_lineage_v1(
                            prepared.code, runner.parser_build_config(), filename.to_string_lossy().into_owned(), prepared.lineage,
                        ).unwrap();
                        let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else {
                            panic!("source-backed")
                        };
                        let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
                        let consumed = crate::mir::builder::NormalRootExecutionConsumerV1::consume_once(source).unwrap().into_consumed_source();
                        let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1(
                            &mut FunctionSemanticResolverSessionV1::new(196).unwrap(), consumed,
                            Some(&catalog), crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(),
                            &imports,
                        ).unwrap();
                        let declarations = package.declaration_catalog();
                        let aliases = crate::mir::source_call_target::VerifiedStaticImportAliasViewV1::seal(
                            declarations, imports.clone(),
                        ).unwrap();
                        let targets = crate::mir::source_call_target::VerifiedWholeSourceStaticCallTargetInventoryV1::verify(
                            declarations, &aliases,
                        ).unwrap().into_targets();
                        let results = crate::mir::callable_result_representation::VerifiedSameModuleCallableResultCatalogV1::verify(
                            declarations, &targets,
                        ).unwrap();
                        let publication = crate::mir::callable_result_representation::VerifiedStaticCallResultPublicationOwnerV1::issue(
                            declarations, &targets, &results,
                        ).unwrap();
                        let key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "size_to_bin", 1);
                        let mut context = CompilationContext::new();
                        let installed = package.prepare_install(&mut context).unwrap().commit();
                        if missing_route {
                            installed.ordinary_new_claim_ledger().remove_static_loop_local_route_for_test(&key);
                        }
                        let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(key).unwrap();
                        let brand = crate::mir::module_invocation_identity::ModuleInvocationBrandV1::legacy_test();
                        let mut builder = MirBuilder::new();
                        let mut collector = crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1::with_brand(brand);
                        collector.install_static_result_publication_owner(publication).unwrap();
                        if duplicate_collector {
                            let symbol = admission.physical_symbol().to_owned();
                            let draft = MirFunction::new(
                                FunctionSignature {
                                    name: symbol.clone(),
                                    params: vec![MirType::Integer],
                                    return_type: MirType::Integer,
                                    effects: EffectMask::PURE,
                                },
                                BasicBlockId(0),
                            );
                            collector.prepare_admission(
                                FunctionDraftKeyV1::CatalogedBoxMethod(admission.source_key().clone()),
                                symbol,
                                admission.physical_arity(),
                                DraftPublicationPolicyV1::CanonicalRejectDuplicate,
                            ).unwrap().seal(draft).unwrap().collect();
                        }
                        let mut invocation = crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1::with_collector(
                            &mut builder, collector,
                        );
                        invocation.with_module_port(|builder, module_port| {
                            let mut raw_port = crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1::new(module_port);
                            let package_port = installed.begin_lowering(&context).unwrap();
                            let mut adapter = super::super::NormalCallableSemanticPackagePortAdapterV1::new(
                                &mut raw_port, package_port, None, None,
                            ).unwrap();
                            use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
                            let outcome = adapter.lower_cataloged_static_box_method(
                                builder, admission, Vec::new(), Vec::new(), None,
                                Vec::new(), Vec::new(), crate::ast::DeclarationAttrs::default(), None,
                            );
                            if missing_route {
                                let error = outcome.unwrap_err();
                                assert!(error.contains("static-packet-route-missing"), "{error}");
                            } else if duplicate_collector {
                                let error = outcome.unwrap_err();
                                assert!(error.contains("DuplicateKey"), "{error}");
                            } else {
                                assert!(outcome.is_ok(), "{outcome:?}");
                            }
                            drop(adapter);
                            let expected = usize::from(!missing_route && !duplicate_collector);
                            module_port.with_headers(|headers| assert_eq!(headers.symbol_count(), usize::from(!missing_route)));
                            assert_eq!(installed.ordinary_new_claim_ledger().selected_loop_body_packet_count_for_test(), expected);
                            assert_eq!(installed.ordinary_new_claim_ledger().selected_static_entry_group_count_for_test(), expected);
                        });
                        if !missing_route && !duplicate_collector {
                            let (_, collector, _) = invocation.into_state().into_parts();
                            let function = collector
                                .into_single_observation_draft("SizeClassBox.size_to_bin/1")
                                .unwrap();
                            let mut module = MirModule::new("selected-static-loop-finish".into());
                            module.functions.insert(function.signature.name.clone(), function);
                            let ledger = installed.ordinary_new_claim_ledger();
                            ledger.validate_finalized_child_functions(&module, false).unwrap();
                            ledger.with_selected_loop_body_packet_for_test(|site, packet| {
                                let (finished, coordinate) = ledger
                                    .finished_loop_body_call_producer_v1(site, packet, &module)
                                    .unwrap();
                                let invoke = finished.blocks[&coordinate.0]
                                    .all_instructions()
                                    .nth(coordinate.1)
                                    .unwrap()
                                    .clone();
                                assert!(matches!(invoke, MirInstruction::Invoke { .. }));
                                let mut missing = module.clone();
                                missing.functions.get_mut(&finished.signature.name).unwrap()
                                    .blocks.get_mut(&coordinate.0).unwrap().terminator = None;
                                assert!(ledger.finished_loop_body_call_producer_v1(site, packet, &missing)
                                    .unwrap_err().contains("producer-missing"));
                                let mut duplicate = module.clone();
                                duplicate.functions.get_mut(&finished.signature.name).unwrap()
                                    .blocks.get_mut(&coordinate.0).unwrap().instructions.push(invoke);
                                assert!(ledger.finished_loop_body_call_producer_v1(site, packet, &duplicate)
                                    .unwrap_err().contains("producer-duplicate"));
                            });
                        }
                        assert!(builder.function_state.current_function.is_none());
                        assert!(builder.function_state.current_block.is_none());
                    }
                });
            })
            .unwrap()
            .join()
            .expect("cataloged Static Loop entry");
    }
}
