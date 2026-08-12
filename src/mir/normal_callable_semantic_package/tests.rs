use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CompilationContext, NormalCatalogedBoxMethodDraftAdmissionV1,
    SelectedNormalCallableKeyV1,
};
use crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{
    BuildMode, NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1,
};

use super::{
    issue_normal_callable_semantic_package_v1, NormalCallableDynamicProjectionRefV1,
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackageIssueV1,
    SelectedCallableSemanticRefV1,
};

use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    final_source_with_config(source, ParserBuildConfig::default())
}

fn final_source_with_config(
    source: &str,
    config: ParserBuildConfig,
) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(source, config)
        .expect("normal callable source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn issue_with_config(
    source: &str,
    config: ParserBuildConfig,
) -> Result<super::VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(92).unwrap();
    issue_normal_callable_semantic_package_v1(
        &mut resolver,
        final_source_with_config(source, config),
    )
}

fn issue(
    source: &str,
) -> Result<super::VerifiedNormalCallableSemanticPackageV1, NormalCallableSemanticPackageIssueV1> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(91).unwrap();
    issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
}

#[test]
fn parser_scan_source_seals_one_dynamic_candidate_and_all_parameter_contracts() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact parser scan semantic package");

    assert_eq!(package.batch().declarations().len(), 4);
    assert_eq!(package.parameter_declaration_count(), 4);
    assert_eq!(package.parameter_count(), 15);
    let NormalCallableDynamicProjectionRefV1::Selected { program } = package.dynamic_projection()
    else {
        panic!("exact Dynamic row must remain selected")
    };
    assert_eq!(
        program.current(),
        crate::mir::compiler::dynamic_full_body_recipe::DynamicInvocationCleanupCurrentDispositionV1::ExactI64TrivialNoEnd
    );
}

#[test]
fn selected_dynamic_loan_carries_the_package_source_seed() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact Dynamic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        ),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    port.with_selected_lowering_input(&key, |input| match input.semantic() {
        SelectedCallableSemanticRefV1::Dynamic { source, .. } => {
            assert_eq!(source.owner(), input.source().owner());
        }
        SelectedCallableSemanticRefV1::Ordinary => panic!("Dynamic row lost package seed"),
    })
    .expect("selected Dynamic loan");
}

#[test]
fn foreign_catalog_admission_rejects_before_selected_loan_callback() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact Dynamic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let foreign = NormalCatalogedBoxMethodDraftAdmissionV1::seal(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("ForeignBox", "skip_while", 4),
    )
    .expect("foreign admission shape");
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let mut called = false;
    let result = port.with_selected_cataloged_lowering_input(foreign, |_| {
        called = true;
    });
    assert_eq!(
        result,
        Err(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)
    );
    assert!(!called);
}

#[test]
fn selected_dynamic_loan_issues_one_builder_free_a_prime_demand() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact Dynamic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        ),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(match &key {
        SelectedNormalCallableKeyV1::Cataloged(source_key) => source_key.clone(),
        SelectedNormalCallableKeyV1::TopLevel(_) => unreachable!(),
    })
    .expect("catalog admission");
    port.with_selected_cataloged_lowering_input(admission, |input| {
        let owner = input.selected().source().owner();
        let demand = crate::mir::compiler::a_prime_i64_physical_capability::
            issue_selected_a_prime_i64_physical_demand(input)
            .expect("selected Dynamic A-prime demand");
        assert_eq!(
            demand.requirement(),
            crate::mir::compiler::a_prime_i64_physical_capability::
                APrimeI64PhysicalRequirementV1::DirectExactI64
        );
        assert_eq!(demand.identity().owner(), owner);
        assert_eq!(
            demand.physical_header().physical_symbol(),
            "ParserScanLoopBox.skip_while/4"
        );
        assert_eq!(demand.physical_header().physical_arity(), 4);
        assert_eq!(demand.source_relation().completion_sites().len(), 2);
        demand.with_operation_program(|program| {
            assert_eq!(program.placement_rows().len(), 17);
            assert_eq!(program.operation_rows().len(), 15);
            assert_eq!(program.coverage().fault_count(), 2);
        });
    })
    .expect("selected A-prime demand loan");
}

#[test]
fn selected_dynamic_loan_issues_one_v2_native_preflight_plan() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact Dynamic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        ),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(match &key {
        SelectedNormalCallableKeyV1::Cataloged(source_key) => source_key.clone(),
        SelectedNormalCallableKeyV1::TopLevel(_) => unreachable!(),
    })
    .expect("catalog admission");
    port.with_selected_cataloged_lowering_input(admission, |input| {
        let demand = crate::mir::compiler::a_prime_i64_physical_capability::
            issue_selected_a_prime_i64_physical_demand(input)
            .expect("selected Dynamic A-prime demand");
        let plan = crate::mir::builder::issue_selected_dynamic_v2_emission_plan(demand)
            .expect("selected V2 preflight plan");
        assert_eq!(plan.schedule_rows().len(), 15);
        assert_eq!(
            plan.schedule_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.segment(),
                        crate::mir::builder::resolved_lowering::
                            DynamicV2PhysicalScheduleSegmentV1::Prelude
                    )
                })
                .count(),
            10
        );
        assert_eq!(
            plan.schedule_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.segment(),
                        crate::mir::builder::resolved_lowering::
                            DynamicV2PhysicalScheduleSegmentV1::ThenTerminal
                    )
                })
                .count(),
            1
        );
        assert_eq!(
            plan.schedule_rows()
                .iter()
                .filter(|row| {
                    matches!(
                        row.segment(),
                        crate::mir::builder::resolved_lowering::
                            DynamicV2PhysicalScheduleSegmentV1::Continuation
                    )
                })
                .count(),
            4
        );
        let schedule_items = |segment| {
            plan.schedule_rows()
                .iter()
                .filter(|row| row.segment() == segment)
                .map(|row| row.item().raw())
                .collect::<Vec<_>>()
        };
        assert_eq!(
            schedule_items(
                crate::mir::builder::resolved_lowering::DynamicV2PhysicalScheduleSegmentV1::Prelude
            ),
            (0..10).collect::<Vec<_>>()
        );
        assert_eq!(
            schedule_items(crate::mir::builder::resolved_lowering::
                DynamicV2PhysicalScheduleSegmentV1::ThenTerminal),
            vec![11]
        );
        assert_eq!(
            schedule_items(crate::mir::builder::resolved_lowering::
                DynamicV2PhysicalScheduleSegmentV1::Continuation),
            vec![13, 14, 15, 16]
        );
        plan.with_ledger(|ledger| {
            assert_eq!(ledger.coverage_counts(), (17, 15, 2, 2));
        });
    })
    .expect("selected V2 preflight plan loan");
}

#[test]
fn selected_v2_capability_admission_is_all_or_nothing_before_effect() {
    let package = issue(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("exact Dynamic package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "ParserScanLoopBox",
            "skip_while",
            4,
        ),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(match &key {
        SelectedNormalCallableKeyV1::Cataloged(source_key) => source_key.clone(),
        SelectedNormalCallableKeyV1::TopLevel(_) => unreachable!(),
    })
    .expect("catalog admission");
    port.with_selected_cataloged_lowering_input(admission, |input| {
        let demand = crate::mir::compiler::a_prime_i64_physical_capability::
            issue_selected_a_prime_i64_physical_demand(input)
            .expect("selected Dynamic A-prime demand");
        let plan = crate::mir::builder::issue_selected_dynamic_v2_emission_plan(demand)
            .expect("selected V2 preflight plan");
        let admission =
            crate::mir::builder::issue_selected_dynamic_v2_physical_capability_admission(
                plan,
                std::num::NonZeroU64::new(1).expect("test registry generation"),
                crate::mir::module_invocation_identity::ModuleInvocationBrandV1::legacy_test(),
            )
                .expect("exact V2 capability requirements");
        assert_eq!(
            admission.disposition(),
            crate::mir::builder::resolved_lowering::
                DynamicV2PhysicalCapabilityDispositionV1::RejectBeforeEffect
        );
        assert_eq!(admission.aot_admission().contract_id(), "hako.text.scan@1");
        assert_eq!(admission.aot_admission().canonical_receiver(), "Text");
        assert_eq!(admission.aot_admission().aliases(), ["String", "StringBox"]);
        assert_eq!(admission.aot_admission().registry_branch_count(), 1);
        assert_eq!(admission.aot_admission().registry_generation(), 1);
        let compare_i64 = admission.compare_i64();
        assert_eq!(
            compare_i64.item(),
            crate::mir::loop_recipe_contract::LoopItemKeyV1::new(9)
        );
        assert_eq!(
            compare_i64.left(),
            crate::mir::loop_recipe_contract::LoopValueKeyV1::new(11)
        );
        assert_eq!(
            compare_i64.right(),
            crate::mir::loop_recipe_contract::LoopValueKeyV1::new(12)
        );
        assert_eq!(
            compare_i64.result(),
            crate::mir::loop_recipe_contract::LoopValueKeyV1::new(13)
        );
        assert_eq!(
            compare_i64.v11().family(),
            crate::mir::builder::resolved_lowering::DynamicV2ProducerFamilyV1::DynamicCallSlot
        );
        assert_eq!(
            compare_i64.v11().representation(),
            crate::mir::builder::resolved_lowering::DynamicV2PhysicalRepresentationV1::ImmediateI64
        );
        assert_eq!(
            compare_i64.v12().family(),
            crate::mir::builder::resolved_lowering::DynamicV2ProducerFamilyV1::ConstI64
        );
        assert_eq!(
            compare_i64.v12().representation(),
            crate::mir::builder::resolved_lowering::DynamicV2PhysicalRepresentationV1::ImmediateI64
        );
        let cleanup = admission.cleanup();
        assert_eq!(cleanup.len(), 4);
        assert_eq!(
            cleanup[0].item(),
            Some(crate::mir::loop_recipe_contract::LoopItemKeyV1::new(6))
        );
        assert_eq!(
            cleanup[1]
                .first()
                .map(|action| (action.producer(), action.result())),
            Some((
                crate::mir::loop_recipe_contract::LoopItemKeyV1::new(6),
                crate::mir::loop_recipe_contract::LoopValueKeyV1::new(10),
            ))
        );
        assert!(cleanup[2].inner_return_site().is_some());
        assert_eq!(
            cleanup[3].backedge_loop(),
            Some(crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0))
        );
        assert!(matches!(
            admission.into_rejected_plan(),
            Err(crate::mir::builder::resolved_lowering::
                SelectedDynamicV2PhysicalCapabilityRejectV1::ProducerReceiptUnavailable)
        ));
    })
    .expect("selected V2 capability admission loan");
}

#[test]
fn package_scoped_loan_retains_exact_parameter_contract() {
    let package = issue("static box Api { run(source, pos: i64, end: i64, tail) { return pos } }")
        .expect("typed parameter package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 4),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    port.with_selected_lowering_input(&key, |input| {
        let kinds = input
            .parameter_contracts()
            .map(|(_, _, kind)| kind)
            .collect::<Vec<_>>();
        assert_eq!(
            kinds,
            [
                CallableParameterContractKindV1::OpaqueHandle,
                CallableParameterContractKindV1::ExactTrivial(
                    crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64,
                ),
                CallableParameterContractKindV1::ExactTrivial(
                    crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64,
                ),
                CallableParameterContractKindV1::OpaqueHandle,
            ]
        );
    })
    .expect("exact contract loan");
    port.complete().expect("selected contract consumed");
}

#[test]
fn ordinary_selected_loan_cannot_enter_a_prime_dynamic_demand() {
    let package =
        issue("static box Api { run(value) { return value } }").expect("ordinary package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
    );
    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    let admission = NormalCatalogedBoxMethodDraftAdmissionV1::seal(match &key {
        SelectedNormalCallableKeyV1::Cataloged(source_key) => source_key.clone(),
        SelectedNormalCallableKeyV1::TopLevel(_) => unreachable!(),
    })
    .expect("catalog admission");
    port.with_selected_cataloged_lowering_input(admission, |input| {
        assert!(matches!(
            crate::mir::compiler::a_prime_i64_physical_capability::
                issue_selected_a_prime_i64_physical_demand(input),
            Err(
                crate::mir::compiler::a_prime_i64_physical_capability::
                    APrimeI64PhysicalDemandRejectV1::NotSelectedDynamic
            )
        ));
    })
    .expect("ordinary selected loan");
}

#[test]
fn top_level_and_dynamic_candidate_share_one_complete_package_batch() {
    let dynamic = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let source = format!("function helper(value) {{ return value }}\n{dynamic}");
    let package = issue(&source).expect("mixed top-level and Dynamic package");

    assert_eq!(package.batch().declarations().len(), 5);
    assert_eq!(package.parameter_declaration_count(), 4);
    assert_eq!(package.parameter_count(), 15);
    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::Selected { .. }
    ));
    assert_eq!(
        package
            .batch()
            .declarations()
            .next()
            .expect("top-level row remains first")
            .mode(),
        crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::TopLevel
    );
}

#[test]
fn zero_dynamic_candidates_are_valid_unselected_without_default_or_name_selection() {
    let package = issue("static box Api { run(value) { return value } }")
        .expect("fully observed non-Dynamic package");
    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::ValidUnselected
    ));
    assert_eq!(package.batch().declarations().len(), 1);
    assert_eq!(package.parameter_count(), 1);
}

#[test]
fn two_exact_dynamic_candidates_reject_without_ordinal_tiebreak() {
    let first = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let second = first.replace("ParserScanLoopBox", "ParserScanLoopBoxTwin");
    let source = format!("{first}\n{second}");
    assert!(matches!(
        issue(&source),
        Err(NormalCallableSemanticPackageIssueV1::DuplicateDynamicCandidate)
    ));
}

#[test]
fn unselected_main_dynamic_candidate_cannot_capture_production_selection() {
    let source = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako")
        .replace("ParserScanLoopBox", "Main");
    let package = issue(&source).expect("Main remains a valid unselected batch row");

    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::ValidUnselected
    ));
    assert_eq!(package.batch().declarations().len(), 4);
}

#[test]
fn unselected_main_candidate_does_not_duplicate_one_selected_dynamic_candidate() {
    let selected = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let unselected = selected.replace("ParserScanLoopBox", "Main");
    let package = issue(&format!("{unselected}\n{selected}"))
        .expect("only selected-map Dynamic rows participate in production selection");

    assert!(matches!(
        package.dynamic_projection(),
        NormalCallableDynamicProjectionRefV1::Selected { .. }
    ));
}

#[test]
fn selected_gate_dynamic_candidate_rejects_without_parameter_authority() {
    let source = r#"
gate Build.test {
  static box ParserScanLoopBox {
    skip_while(src, pos, end, pred_chars) {
      local i = pos
      loop(i < end) {
        local ch = src.substring(i, i + 1)
        if pred_chars.indexOf(ch) < 0 { return i }
        i = i + 1
      }
      return i
    }
  }
}
"#;
    assert!(matches!(
        issue_with_config(
            source,
            ParserBuildConfig {
                mode: BuildMode::Test,
                ..ParserBuildConfig::default()
            },
        ),
        Err(NormalCallableSemanticPackageIssueV1::MissingDynamicParameterContract)
    ));
}

#[test]
fn package_has_no_clone_or_split_surface() {
    let model = include_str!("model.rs");
    assert!(!model.contains("Clone)]\npub(crate) struct VerifiedNormalCallableSemanticPackageV1"));
    assert!(!model.contains("fn into_parts"));
}

#[test]
fn consuming_install_and_port_enforce_exact_selected_coverage() {
    let package =
        issue("static box Api { run(value) { return value } }").expect("ordinary package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    let key = SelectedNormalCallableKeyV1::Cataloged(
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Api", "run", 1),
    );

    let mut port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    port.with_selected_lowering_input(&key, |input| {
        assert_eq!(input.parameter_contracts().len(), 1);
        assert!(input.source().callable_header().is_none());
        let identity = input.source_identity();
        assert_eq!(identity.owner(), input.source().owner());
        assert_eq!(
            identity.mode(),
            ResolvedCallableDeclarationModeV1::StaticBoxMethod
        );
        assert!(identity.method_source_observation().is_some());
        assert!(matches!(
            input.semantic(),
            SelectedCallableSemanticRefV1::Ordinary
        ));
    })
    .expect("exact selected loan");
    assert!(matches!(
        port.with_selected_lowering_input(&key, |_| ()),
        Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey)
    ));
    port.complete().expect("all selected rows consumed once");

    let foreign = CompilationContext::new();
    assert!(matches!(
        installed.begin_lowering(&foreign),
        Err(NormalCallableSemanticPackageInstallIssueV1::ForeignCatalog)
    ));
}

#[test]
fn install_rejects_occupied_catalog_and_port_rejects_incomplete_coverage() {
    let source = "static box Api { run(value) { return value } stop(value) { return value } }";
    let mut context = CompilationContext::new();
    let installed = issue(source)
        .expect("first package")
        .prepare_install(&mut context)
        .expect("vacant catalog slot")
        .commit();
    assert!(issue(source)
        .expect("second package")
        .prepare_install(&mut context)
        .is_err());

    let port = installed
        .begin_lowering(&context)
        .expect("same installed catalog");
    assert!(matches!(
        port.complete(),
        Err(NormalCallableSemanticPackageInstallIssueV1::IncompleteSelectedCoverage)
    ));
}
