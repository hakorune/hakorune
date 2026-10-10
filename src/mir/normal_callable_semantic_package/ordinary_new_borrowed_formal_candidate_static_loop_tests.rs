use super::*;

#[test]
fn real_mimalloc_static_loop_route_uses_one_source_bound_v2_product() {
    std::thread::Builder::new().name("mimalloc-static-loop-route".into())
        .stack_size(32 * 1024 * 1024).spawn(|| {
        let env_updates: Vec<_> = crate::test_support::JOINIR_DEFAULT_MODE.into_iter().chain([
            ("NYASH_ALLOW_USING_FILE", Some("1")), ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")), ("NYASH_MACRO_DISABLE", Some("1")),
        ]).collect();
        crate::test_support::with_env_vars(&env_updates, || {
            use crate::mir::builder::{NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1};
            use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
            use crate::runner::modes::common_util::normal_callable::{
                materialize_normal_callable_program_with_identity_and_lineage_v1,
                NormalCallableMaterializationOutcomeV1,
            };
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
            let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else { panic!("source-backed") };
            let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
            let consumed = NormalRootExecutionConsumerV1::consume_once(source).unwrap().into_consumed_source();
            let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1(
                &mut FunctionSemanticResolverSessionV1::new(94).unwrap(), consumed, Some(&catalog),
                crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &imports,
            ).unwrap();
            let key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "size_to_bin", 1);
            let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
            package.batch().with_lowering_input(slot, |input| {
                let loop_site = input.function().loop_sites().next().unwrap();
                let claims = &package.ordinary_new_claim_ledger;
                assert!(claims.expects_loop_static_source_loan_v1(input.owner(), loop_site));
                let first = crate::mir::builder::take_at_function_entry_v2(input, claims).unwrap();
                let product = first.expect("selected cataloged function takes one V2 product before lowering");
                assert_eq!(&product.semantic().roles().loop_site, loop_site);
                let actual = &product.semantic().source_calls().0.original().argument_sites()[0];
                assert_eq!(input.function().variable_ref(actual), Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(product.tagged_formal())));
                let row = package.physical_signature().row(slot).unwrap();
                let signature = crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1::from_s6c_row(row);
                let joined = product.join_physical_signature_v2(&signature).unwrap();
                assert_eq!(joined.formal(), product.tagged_formal());
                assert_eq!(joined.lane_index(), 0);
                assert_eq!(joined.carrier(), crate::mir::compiler::common_v2_physical_function_entry_input::PhysicalCallableLaneCarrierV1::BorrowedTaggedValue);
                let result = claims.take_static_loop_i64_result_v1(loop_site).unwrap().unwrap();
                assert!(result.corroborates(product.semantic()));
                let foreign = package.physical_signature().rows().find(|row| row.owner() != input.owner()).unwrap();
                let foreign = crate::mir::normal_callable_semantic_package::ResolvedCallablePhysicalSignatureLoanV1::from_s6c_row(foreign);
                assert!(product.join_physical_signature_v2(&foreign).unwrap_err().contains("static-tagged-signature-mismatch"));
                let second = crate::mir::builder::take_at_function_entry_v2(input, claims).unwrap_err();
                assert!(second.contains("static-i64-v2/source-unavailable"), "{second}");
            }).unwrap();
            let other = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "bin_size", 1);
            let other_slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(other)).unwrap();
            package.batch().with_lowering_input(other_slot, |input| {
                let loop_site = input.function().loop_sites().next().unwrap();
                assert!(!package.ordinary_new_claim_ledger.expects_loop_static_source_loan_v1(input.owner(), loop_site));
                assert!(crate::mir::builder::take_at_function_entry_v2(
                    input, &package.ordinary_new_claim_ledger,
                ).unwrap().is_none(), "unselected Loop keeps its existing route");
            }).unwrap();
        });
    }).unwrap().join().expect("original selected Loop route");
}

#[test]
fn real_mimalloc_static_loop_v2_rejects_source_and_home_mutations() {
    std::thread::Builder::new().name("mimalloc-static-loop-negatives".into())
        .stack_size(32 * 1024 * 1024).spawn(|| {
        let env_updates: Vec<_> = crate::test_support::JOINIR_DEFAULT_MODE.into_iter().chain([
            ("NYASH_ALLOW_USING_FILE", Some("1")), ("NYASH_ENABLE_USING", Some("1")),
            ("NYASH_OPERATOR_BOX_ALL", Some("0")), ("NYASH_MACRO_DISABLE", Some("1")),
        ]).collect();
        crate::test_support::with_env_vars(&env_updates, || {
            use crate::mir::builder::{NormalRootExecutionConsumerV1, SelectedNormalCallableKeyV1};
            use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
            use crate::runner::modes::common_util::normal_callable::{
                materialize_normal_callable_program_with_identity_and_lineage_v1,
                NormalCallableMaterializationOutcomeV1,
            };
            let filename = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("apps/mimalloc-lite/main.hako");
            let code = std::fs::read_to_string(&filename).unwrap();
            for (case, (before, after)) in [
                ("return bin\n      }\n      bin = bin + 1", "local skipped = bin\n      }\n      bin = bin + 1"),
                ("bin = bin + 1\n    }\n    return me.huge_bin()", "bin = true\n    }\n    return me.huge_bin()"),
                ("return me.huge_bin()\n  }", "return 73\n  }"),
                ("return words * me.word_size()", "print(0)\n    return words * me.word_size()"),
                ("local n = me.normalize_size(size)", "local n = me.normalize_size(1)"),
                ("local bin = 1", "local bin = 2"),
            ].into_iter().enumerate() {
                let runner = crate::runner::NyashRunner::new(Default::default());
                let prepared = crate::runner::modes::common_util::source_hint::prepare_normal_source_with_imports(
                    &runner, filename.to_str().unwrap(), &code,
                ).unwrap();
                assert_eq!(prepared.code.matches(before).count(), 1, "one original Loop source");
                let imports: Vec<_> = prepared.imports.into_iter().collect();
                let changed = prepared.code.replacen(before, after, 1);
                let transformed = materialize_normal_callable_program_with_identity_and_lineage_v1(
                    changed, runner.parser_build_config(), filename.to_string_lossy().into_owned(), prepared.lineage,
                ).unwrap();
                let NormalCallableMaterializationOutcomeV1::SourceBacked(source) = transformed else { panic!("source-backed") };
                let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
                let consumed = NormalRootExecutionConsumerV1::consume_once(source).unwrap().into_consumed_source();
                let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_with_brand_catalog_and_loop_policy_v1(
                    &mut FunctionSemanticResolverSessionV1::new(95).unwrap(), consumed, Some(&catalog),
                    crate::mir::builder::LoopFactsPolicyFrameV1::from_environment(), &imports,
                );
                if case == 1 {
                    let error = match package {
                        Ok(_) => panic!("Bool update must stop in package"),
                        Err(error) => error,
                    };
                    assert!(format!("{error:?}").contains("source-actual-unavailable"), "{error:?}");
                    continue;
                }
                let package = package.expect("missing inner return reaches V2 source producer");
                let key = CanonicalSameModuleCallableKeyV1::static_box_method("SizeClassBox", "size_to_bin", 1);
                let slot = package.selected.batch_slot(&SelectedNormalCallableKeyV1::Cataloged(key)).unwrap();
                package.batch().with_lowering_input(slot, |input| {
                    let loop_site = input.function().loop_sites().next().unwrap();
                    if case == 3 {
                        let home = package.ordinary_new_claim_ledger
                            .take_loop_static_home_neutral_v1(loop_site)
                            .expect("selected Loop has Home proof attempt")
                            .unwrap_err();
                        assert!(home.contains("static-home-effect-unavailable"), "{home}");
                        return;
                    }
                    if case == 4 {
                        let declaration = input.function().expression_source().initializers().next().unwrap().declaration_site();
                        let tagged = package.ordinary_new_claim_ledger
                            .take_loop_static_tagged_entry_v1(loop_site, declaration)
                            .expect("selected source has tagged entry attempt").unwrap_err();
                        assert!(tagged.contains("tagged-entry-source-unavailable"), "{tagged}");
                        return;
                    }
                    let error = crate::mir::builder::take_at_function_entry_v2(
                        input, &package.ordinary_new_claim_ledger,
                    ).unwrap_err();
                    let expected = match case {
                        2 => "static-i64-v2/source-unavailable",
                        _ => "static-i64-v2/source]",
                    };
                    assert!(error.contains(expected), "{before}: {error}");
                }).unwrap();
            }
        });
    }).unwrap().join().expect("changed original Loop rejects");
}
