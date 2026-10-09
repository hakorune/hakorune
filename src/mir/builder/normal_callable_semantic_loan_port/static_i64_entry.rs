//! The selected Static I64 Loop is taken at the cataloged function boundary.
//! Its physical entry is disposable until the executable packet is proved.

use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
use crate::mir::builder::MirBuilder;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1;
use crate::mir::normal_callable_semantic_package::SelectedCallableLoweringInputRefV1;

pub(super) fn stop_if_selected(
    builder: &mut MirBuilder,
    module_port: &ModuleLoweringPortV1<'_>,
    selected: &SelectedCallableLoweringInputRefV1<'_>,
    physical_symbol: &str,
    claims: &OrdinaryNewClaimLedgerV1,
    signature: &ResolvedCallablePhysicalSignatureLoanV1<'_>,
) -> Result<(), String> {
    let input = selected.source();
    let Some(product) =
        super::super::raw_loop_child_entry::take_at_function_entry_v2(input, claims)?
    else {
        return Ok(());
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
    let (caller, site) = packet.publication_source();
    let handoff = module_port
        .selected_static_result_handoff_for_source(caller, site)
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-publication-handoff-missing]".to_owned()
        })?;
    if !packet.corroborates_publication_handoff(handoff) {
        return Err("[freeze:contract][callable-loop/static-publication-handoff-drift]".to_owned());
    }
    crate::mir::builder::resolved_lowering::stop_after_unpublished_static_loop_entry_v1(
        builder,
        input,
        selected.block_expr_expectation(),
        physical_symbol,
        &product,
        &formal,
        &result,
        packet,
        handoff,
        claims,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::{
        CanonicalSameModuleCallableKeyV1, CompilationContext, MirBuilder,
        NormalCatalogedBoxMethodDraftAdmissionV1,
    };
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::runner::modes::common_util::normal_callable::{
        materialize_normal_callable_program_with_identity_and_lineage_v1,
        NormalCallableMaterializationOutcomeV1,
    };

    #[test]
    fn original_static_loop_stops_at_cataloged_entry_before_builder_effects() {
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
                    for missing_route in [false, true] {
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
                            let error = adapter.lower_cataloged_static_box_method(
                                builder, admission, Vec::new(), Vec::new(), None,
                                Vec::new(), Vec::new(), crate::ast::DeclarationAttrs::default(), None,
                            ).unwrap_err();
                            let expected = if missing_route {
                                "static-packet-route-missing"
                            } else {
                                "static-i64-v2/physical-abi-coverage-missing"
                            };
                            assert!(error.contains(expected), "{error}");
                            drop(adapter);
                            module_port.with_headers(|headers| assert_eq!(headers.symbol_count(), 0));
                        });
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
