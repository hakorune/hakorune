#!/usr/bin/env python3
"""Callable-source child checks for the single NORMAL-SOURCE-PLAN0 guard."""

from pathlib import Path
from typing import Callable


Require = Callable[[str, str, str], None]
RequireCount = Callable[[str, str, int, str], None]


def check_callable_source(
    root: Path,
    source_dir: Path,
    require: Require,
    require_count: RequireCount,
) -> tuple[Path, ...]:
    task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-callable-source0-s0-execution-task-2026-07-26.md"
    )
    direct_call_task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-main-direct-call0-s0-execution-task-2026-07-26.md"
    )
    acyclic_task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-callable-module0-a0-s0-execution-task-2026-07-26.md"
    )
    recursive_task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-callable-module0-r0-s0-execution-task-2026-07-26.md"
    )
    transaction_task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-callable-module0-tx0-s0-execution-task-2026-07-26.md"
    )
    handoff_task_path = root / (
        "docs/development/current/main/investigations/"
        "normal-callable-module0-tx0-handoff0-s0-execution-task-2026-07-26.md"
    )
    callable_source_path = source_dir / "callable_source.rs"
    callable_source_tests_path = source_dir / "callable_source_tests.rs"
    callable_catalog_source_path = source_dir / "callable_catalog_source.rs"
    direct_call_source_path = source_dir / "main_direct_call_source.rs"
    direct_call_source_tests_path = source_dir / "main_direct_call_source_tests.rs"
    direct_call_plan_path = source_dir / "main_direct_call_plan.rs"
    direct_call_plan_tests_path = source_dir / "main_direct_call_plan_tests.rs"
    normal_acyclic_plan_path = source_dir / "normal_acyclic_module_plan.rs"
    handoff_path = source_dir / "normal_callable_transaction_handoff.rs"
    handoff_tests_path = source_dir / "normal_callable_transaction_handoff_tests.rs"
    module_source_path = source_dir / "module_source.rs"
    instance_function_plan_path = source_dir / "instance_function_plan.rs"
    instance_integer_return_plan_path = (
        source_dir / "instance_integer_return_plan.rs"
    )
    instance_i64_parameter_return_plan_path = source_dir / "instance_i64_parameter_return_plan.rs"
    instance_integer_local_return_plan_path = (
        source_dir / "instance_integer_local_return_plan.rs"
    )
    main0_bridge_path = source_dir / "main0_bridge.rs"
    main_source_path = source_dir / "main_source.rs"
    main_resolved_source_path = source_dir / "main_resolved_source.rs"
    main_function_plan_path = source_dir / "main_function_plan.rs"
    main_function_plan_tests_path = source_dir / "main_function_plan_tests.rs"
    module_source_tests_path = source_dir / "tests.rs"
    canonical_dispatch_path = root / "src/mir/compiler/canonical_core_dispatch.rs"
    normal_frontdoor_path = (
        root / "src/runner/reference/normal_file_vm_frontdoor.rs"
    )
    capability_path = root / "src/mir/compiler/capability.rs"
    function_role_policy_path = (
        root / "src/mir/compiler/capability/function_role_policy.rs"
    )
    analyzer_policy_path = (
        root / "src/mir/resolved_value_profile/analyzer_policy.rs"
    )
    acyclic_graph_path = root / "src/mir/compiler/acyclic_callable_graph.rs"
    scc_partition_path = root / "src/mir/compiler/callable_scc_partition.rs"
    header_source_path = (
        root / "src/mir/resolved_semantics/callable_header_source_unit.rs"
    )
    header_source_tests_path = (
        root / "src/mir/resolved_semantics/callable_header_source_unit_tests.rs"
    )
    header_view_path = (
        root / "src/mir/resolved_semantics/callable_module_header_view.rs"
    )
    files = (
        task_path,
        direct_call_task_path,
        acyclic_task_path,
        recursive_task_path,
        transaction_task_path,
        handoff_task_path,
        callable_source_path,
        callable_source_tests_path,
        callable_catalog_source_path,
        direct_call_source_path,
        direct_call_source_tests_path,
        direct_call_plan_path,
        direct_call_plan_tests_path,
        normal_acyclic_plan_path,
        handoff_path,
        handoff_tests_path,
        module_source_path,
        instance_function_plan_path,
        instance_integer_return_plan_path,
        instance_i64_parameter_return_plan_path,
        instance_integer_local_return_plan_path,
        main0_bridge_path,
        main_source_path,
        main_resolved_source_path,
        main_function_plan_path,
        main_function_plan_tests_path,
        module_source_tests_path,
        canonical_dispatch_path,
        normal_frontdoor_path,
        capability_path,
        function_role_policy_path,
        analyzer_policy_path,
        acyclic_graph_path,
        scc_partition_path,
        header_source_path,
        header_source_tests_path,
        header_view_path,
        Path(__file__),
    )

    task = task_path.read_text()
    direct_call_task = direct_call_task_path.read_text()
    acyclic_task = acyclic_task_path.read_text()
    recursive_task = recursive_task_path.read_text()
    transaction_task = transaction_task_path.read_text()
    handoff_task = handoff_task_path.read_text()
    callable_source = callable_source_path.read_text()
    callable_source_tests = callable_source_tests_path.read_text()
    callable_catalog_source = callable_catalog_source_path.read_text()
    direct_call_source = direct_call_source_path.read_text()
    direct_call_source_tests = direct_call_source_tests_path.read_text()
    direct_call_plan = direct_call_plan_path.read_text()
    direct_call_plan_tests = direct_call_plan_tests_path.read_text()
    normal_acyclic_plan = normal_acyclic_plan_path.read_text()
    handoff = handoff_path.read_text()
    handoff_tests = handoff_tests_path.read_text()
    module_source = module_source_path.read_text()
    instance_function_plan = instance_function_plan_path.read_text()
    instance_integer_return_plan = instance_integer_return_plan_path.read_text()
    instance_i64_parameter_return_plan = instance_i64_parameter_return_plan_path.read_text()
    instance_integer_local_return_plan = instance_integer_local_return_plan_path.read_text()
    main0_bridge = main0_bridge_path.read_text()
    main_source = main_source_path.read_text()
    main_resolved_source = main_resolved_source_path.read_text()
    main_function_plan = main_function_plan_path.read_text()
    main_function_plan_tests = main_function_plan_tests_path.read_text()
    module_source_tests = module_source_tests_path.read_text()
    canonical_dispatch = canonical_dispatch_path.read_text()
    normal_frontdoor = normal_frontdoor_path.read_text()
    acyclic_graph = acyclic_graph_path.read_text()
    scc_partition = scc_partition_path.read_text()
    capability = capability_path.read_text()
    function_role_policy = function_role_policy_path.read_text()
    header_source = header_source_path.read_text()
    header_source_tests = header_source_tests_path.read_text()
    header_view = header_view_path.read_text()

    for fragment in (
        "NORMAL-CALLABLE-SOURCE0-S0",
        "one owned original Program",
        "Main-box additional methods",
        "AST clone/rewrite                                  = 0",
        "callable owner issuance                            = 0",
        "all modified/new source/check files                < 800 lines",
    ):
        require(task, fragment, f"callable source task {fragment}")

    for definition in (
        "struct VerifiedNormalCallableSourceUnitV1",
        "struct RejectedNormalCallableSourceV1",
        "enum NormalCallableSourceStageV1",
        "enum NormalCallableSourceErrorV1",
    ):
        require_count(
            callable_source,
            definition,
            1,
            f"sole normal callable source definition {definition}",
        )
    for fragment in (
        "VerifiedCallableHeaderSourceUnitV1::validate_exact_sites(",
        "VerifiedCallableHeaderSourceUnitV1::seal_exact_sites(",
        "NormalAdditionalCallableSiteV1::TopLevel",
        "NormalAdditionalCallableSiteV1::MainMethod",
        "MainMethodHelperUnsupported",
        "fn stage(&self)",
        "fn error(&self)",
        "fn discard(self)",
    ):
        require(callable_source, fragment, f"normal callable source law {fragment}")
    for fragment in (
        "fn seal_exact_sites(",
        "fn validate_exact_sites(",
        "declaration_sites.sort_unstable()",
        "DuplicateDeclarationSite",
        "MissingProgramStatement",
    ):
        require(header_source, fragment, f"exact-site header owner {fragment}")
    require(
        header_view,
        "fn from_statement_index(",
        "checked callable declaration-site constructor",
    )

    for test_name in (
        "one_program_owner_exposes_exact_top_level_helper_sites",
        "helper_declaration_reorder_preserves_exact_selected_meaning",
        "main_box_helpers_reject_before_catalog_sealing",
    ):
        require(
            callable_source_tests,
            f"fn {test_name}(",
            f"normal callable source fixture {test_name}",
        )
    for test_name in (
        "exact_sites_keep_one_mixed_program_owner_without_reclassifying_main",
        "exact_sites_reject_empty_duplicate_missing_and_non_function_rows",
    ):
        require(
            header_source_tests,
            f"fn {test_name}(",
            f"exact-site source fixture {test_name}",
        )

    for definition in (
        "struct NormalInstanceBoxSiteV1",
        "struct VerifiedNormalModuleSourceV1",
        "struct RejectedNormalModuleSourceV1",
        "enum NormalModuleSourceStageV1",
        "enum NormalModuleSourceErrorV1",
    ):
        require_count(
            module_source,
            definition,
            1,
            f"sole module-source definition {definition}",
        )
    for fragment in (
        "pub(super) fn seal(",
        "validate_main_surface(main_surface)",
        "verify_main_source_parts(",
        "VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(",
        ".keys()",
        "actual != expected_keys",
        "declaration_for(",
        "fn stage(&self)",
        "fn error(&self)",
        "fn discard(self)",
        "Main0WithPlainInstanceBoxes0SealV1",
    ):
        require(module_source, fragment, f"module-source law {fragment}")
    for test_name in (
        "main_with_plain_instance_box_seals_module_source",
        "multiple_instance_boxes_preserve_source_order",
        "instance_method_catalog_correspondence_is_exact",
        "explicit_constructor_is_rejected_before_builder",
        "static_method_inside_instance_box_is_rejected",
        "top_level_function_or_runtime_statement_is_rejected",
        "existing_exact_classifier_still_rejects_non_main_box",
        "rejection_retains_source_identity_and_has_no_retry",
    ):
        require(
            module_source_tests,
            f"fn {test_name}(",
            f"module-source fixture {test_name}",
        )
    for forbidden in (
        "MirBuilder",
        "MirInstruction",
        "MirModule",
        "ValueId",
        "MirType",
        "compile_legacy_candidate",
        "build_module(",
        "source_ast(",
        "into_ast(",
        "input.clone(",
        "source.clone(",
        "retry(",
        "fallback",
    ):
        if forbidden in module_source:
            raise AssertionError(
                f"module-source owner gained lowering/retry authority: {forbidden}"
            )
    for production_surface in (canonical_dispatch, normal_frontdoor):
        for symbol in (
            "VerifiedNormalModuleSourceV1",
            "RejectedNormalModuleSourceV1",
        ):
            if symbol in production_surface:
                raise AssertionError(
                    f"disconnected module-source product gained production consumer: {symbol}"
                )
    if "struct VerifiedSameModuleCallableDeclarationCatalogV1" in module_source:
        raise AssertionError("module-source owner duplicated callable catalog")

    definition_groups = (
        (instance_function_plan, "cumulative instance-plan", (
                "struct VerifiedNormalInstanceFunctionFactsV1",
                "enum VerifiedNormalInstanceFunctionPlanV1",
                "struct VerifiedNormalInstanceFunctionPlanSetV1",
                "struct RejectedGeneralFunctionPlanSetV1",
                "enum GeneralFunctionPlanStageV1",
                "enum GeneralFunctionPlanErrorV1",
        )),
        (instance_integer_return_plan, "integer-return variant", (
                "struct NormalInstanceIntegerReturnRecipeV1",
                "struct VerifiedNormalInstanceIntegerReturnPlanV1",
        )),
        (instance_i64_parameter_return_plan, "i64 parameter-return variant", (
                "struct VerifiedNormalInstanceI64ParameterV1",
                "struct NormalInstanceI64ParameterReturnRecipeV1",
                "struct VerifiedNormalInstanceI64ParameterReturnPlanV1",
        )),
        (instance_integer_local_return_plan, "integer Local-return variant", (
                "struct VerifiedNormalInstanceLocalV1",
                "struct NormalInstanceIntegerLocalReturnRecipeV1",
                "struct VerifiedNormalInstanceIntegerLocalReturnPlanV1",
        )),
    )
    for source, label, definitions in definition_groups:
        for definition in definitions:
            require_count(source, definition, 1, f"sole {label} definition {definition}")
    for fragment in (
        "borrow_instance_method_source(key)",
        "SameModuleCallableNamespaceV1::InstanceBoxMethod",
        "classify_instance_function(view)",
        "resolve_instance_function(&mut resolver, family.view())",
        "ExactTrivialParameterAbiV1::classify",
        "seal_integer_literal_return_one(view, value, forest, projection)",
        "seal_i64_parameter_return_one(",
        "Self::I64ParameterReturn(plan)",
        "seal_integer_local_return_one(",
        "Self::IntegerLocalReturn(plan)",
        "plans.keys().eq(keys.iter())",
        "FunctionSyntaxViewV1::from_borrowed_function_parts(",
        ".resolve_forest(syntax)",
        "VerifiedSourceProjectionV1::seal(view.function(), &forest)",
    ):
        require(instance_function_plan, fragment, f"cumulative instance-plan law {fragment}")
    variant_laws = (
        (instance_integer_return_plan, "integer-return", (
                "ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(",
                "ExprChildRoleV1::ReturnValue",
                "LiteralValue::Integer(integer)",
                "verify_function_completion_v1(input)",
                "SourceBindingSiteV1::Receiver",
                "BindingKindV1::Receiver",
        )),
        (instance_i64_parameter_return_plan, "i64 parameter-return", (
                "SourceBindingSiteV1::Parameter { index: 0 }",
                "BindingKindV1::Parameter { index: 0 }",
                "ResolvedLexicalRefV1::Local(parameter)",
                "ExprChildRoleV1::ReturnValue",
                "verify_function_completion_v1(input)",
        )),
        (instance_integer_local_return_plan, "integer Local-return", (
                "ExprChildRoleV1::LocalInitializer(0)",
                "SourceBindingSiteV1::Local {",
                "BindingKindV1::Local { ordinal: 0 }",
                "ResolvedLexicalRefV1::Local(local)",
                "ExprChildRoleV1::ReturnValue",
                "verify_function_completion_v1(input)",
        )),
    )
    for source, label, laws in variant_laws:
        for fragment in laws:
            require(source, fragment, f"instance {label} law {fragment}")
    for test_name in (
        "mixed_instance_function_variants_seal_once_and_bridge_main",
        "unsupported_method_rejects_whole_set_and_fresh_local_reuses",
        "empty_instance_boxes_do_not_issue_an_empty_plan_set",
        "instance_scalar_variants_reject_widening_without_retry",
    ):
        require(
            module_source_tests,
            f"fn {test_name}(",
            f"instance integer-return fixture {test_name}",
        )
    for source, label in (
        (instance_function_plan, "cumulative instance-plan owner"),
        (instance_integer_return_plan, "integer-return variant"),
        (instance_i64_parameter_return_plan, "i64 parameter-return variant"),
        (instance_integer_local_return_plan, "integer Local-return variant"),
    ):
        for forbidden in (
            "MirBuilder",
            "MirInstruction",
            "MirModule",
            "ValueId",
            "BasicBlockId",
            "compile_legacy",
            "build_module(",
            "build_expression(",
            "build_statement(",
            "CanonicalLoweringPreflightV1",
            "CanonicalTrivialBindingSsaPlanV1",
            "TrivialRepresentationV1",
            "retry(",
            "fallback",
        ):
            if forbidden in source:
                raise AssertionError(f"{label} gained lowering/retry authority: {forbidden}")
    for forbidden in ("or_else(", "filter_map("):
        if forbidden in instance_function_plan:
            raise AssertionError(f"cumulative owner gained family retry/skip: {forbidden}")
    for forbidden in (
        "ExactTrivialParameterAbiV1", "ExactTrivialScalarAbiV1",
        "LocalSlotContract", "MirType",
    ):
        if forbidden in instance_integer_local_return_plan:
            raise AssertionError(f"dynamic Local variant gained ABI authority: {forbidden}")
    normal_plan_surface = (
        instance_function_plan + instance_integer_return_plan
        + instance_i64_parameter_return_plan + instance_integer_local_return_plan
        + main0_bridge
    )
    for retired in (
        "VerifiedNormalInstanceIntegerReturnPlanSetV1",
        "seal_instance_integer_return_plans",
    ):
        if retired in normal_plan_surface:
            raise AssertionError(f"retired concrete instance-plan authority remains: {retired}")
    for production_surface in (canonical_dispatch, normal_frontdoor):
        for symbol in (
            "VerifiedNormalInstanceFunctionPlanV1",
            "VerifiedNormalInstanceFunctionPlanSetV1",
            "seal_instance_function_plans",
        ):
            if symbol in production_surface:
                raise AssertionError(
                    f"disconnected cumulative instance plan gained production consumer: {symbol}"
                )
    for definition in (
        "struct VerifiedNormalMain0BridgePlanV1",
        "struct VerifiedNormalModuleFunctionPlanSetV1",
        "struct RejectedNormalMain0BridgeV1",
        "enum NormalMain0BridgeStageV1",
        "enum NormalMain0BridgeErrorV1",
    ):
        require_count(main0_bridge, definition, 1, f"sole Main0 bridge {definition}")
    for fragment in (
        "pub(crate) fn seal_main0_bridge(",
        "prepare_main0_bridge(&self)",
        ".borrow_exact_main_function()",
        "resolve_normal_main_loan_v1(&source)",
        "verify_normal_main0_input_v1(input, role)",
        "lowering.into_parts()",
        "instance: self",
        "owner: self",
        "let [forest_root] = forest.roots()",
        "if_control.owner() != root_owner",
        "completion.owner() != root_owner",
        "profile.owner() != root_owner",
    ):
        require(main0_bridge, fragment, f"Main0 bridge law {fragment}")
    for source, fragment, label in (
        (main_source, "fn borrow_exact_main_function_v1", "shared exact Main locator"),
        (
            module_source,
            "borrow_exact_main_function_v1(&self.input, &self.main_box, &self.main_method)",
            "module-backed exact Main loan",
        ),
        (
            main_resolved_source,
            "fn resolve_normal_main_loan_v1",
            "shared Main resolver kernel",
        ),
        (
            main_function_plan,
            "fn verify_normal_main0_input_v1",
            "shared Main0 preflight kernel",
        ),
    ):
        require(source, fragment, label)
    for test_name in (
        "main0_bridge_preserves_module_source_and_instance_plans",
        "main0_bridge_matches_existing_main0_plan_contract",
        "main0_bridge_failure_retains_owner_without_retry_and_fresh_source_reuses",
    ):
        require(
            main_function_plan_tests,
            f"fn {test_name}(",
            f"Main0 bridge fixture {test_name}",
        )
    for forbidden in (
        "ASTNode",
        "FunctionSemanticResolverSessionV1",
        "CanonicalLoweringPreflightV1",
        "VerifiedNormalMainFunctionPlanV1",
        "MirBuilder",
        "MirInstruction",
        "MirModule",
        "ValueId",
        "compile_legacy",
        "build_module(",
        "retry(",
        "fallback",
        ".clone(",
    ):
        if forbidden in main0_bridge:
            raise AssertionError(f"Main0 bridge gained duplicate/lowering authority: {forbidden}")
    for production_surface in (canonical_dispatch, normal_frontdoor):
        for symbol in (
            "VerifiedNormalMain0BridgePlanV1",
            "VerifiedNormalModuleFunctionPlanSetV1",
            "seal_main0_bridge",
        ):
            if symbol in production_surface:
                raise AssertionError(
                    f"disconnected Main0 bridge gained production consumer: {symbol}"
                )

    for fragment in (
        "NORMAL-CALLABLE-MODULE0-R0-S0",
        "VerifiedCallableGraphInventoryV1 exactly once",
        "VerifiedCallableSccPartitionV1 exactly once",
        "select acyclic or recursive helper topology once",
    ):
        require(recursive_task, fragment, f"normal recursive task {fragment}")
    for fragment in (
        "NORMAL-CALLABLE-MODULE0-TX0-S0",
        "OpenNormalCallableModuleTransactionV1",
        "PreparedNormalCallableModuleTransactionV1",
        "CompletedNormalCallableModuleCandidateV1",
        "one infallible commit",
        "Acyclic-specific transaction terminal                 = 0",
        "Recursive-specific transaction terminal               = 0",
        "all modified/new source/check files                    < 800 lines",
    ):
        require(transaction_task, fragment, f"normal callable transaction task {fragment}")
    for fragment in (
        "NORMAL-CALLABLE-MODULE0-TX0-HANDOFF0-S0",
        "one durable source-authority owner",
        "one owned topology receipt",
        "stored owner-plus-borrowed-plan self-reference    = 0",
        "Builder/MIR/module/publication reference           = 0",
    ):
        require(handoff_task, fragment, f"TX0 handoff task {fragment}")
    for definition in (
        "struct RetainedNormalCallableSourceAuthorityV1",
        "struct ConsumableNormalMainLoweringProofV1",
        "struct OpenNormalCallableModuleTransactionV1",
        "enum PreparedNormalHelperTopologyReceiptV1",
        "struct OwnedNormalHelperLoweringScheduleV1",
        "struct RejectedNormalCallableHandoffV1",
        "enum NormalCallableHandoffStageV1",
    ):
        require_count(handoff, definition, 1, f"sole TX0 handoff definition {definition}")
    for fragment in (
        "pub(crate) fn into_tx0_handoff(self)",
        "pub(crate) fn with_helper_plans<R>(",
        "VerifiedCallableGraphInventoryV1::verify(&self.source.helpers)",
        "VerifiedCallableSccPartitionV1::verify(inventory)",
        "BTreeMap<CanonicalCallableKeyV1, CanonicalTrivialBindingSsaPlanV1<'source>>",
        "fn reject_schedule(",
    ):
        require(handoff, fragment, f"TX0 handoff ownership law {fragment}")
    for test_name in (
        "handoff_consumes_completed_resolution_and_seals_acyclic_schedule",
        "handoff_schedule_uses_canonical_key_order_not_declaration_order",
        "handoff_keeps_recursive_scc_receipt_with_independent_leaf",
        "schedule_rejection_retains_authority_without_running_callback",
        "success_rejection_then_success_preserves_one_shot_handoff_boundary",
    ):
        require(handoff_tests, f"fn {test_name}(", f"TX0 handoff fixture {test_name}")
    for forbidden in (
        "MirBuilder",
        "MirInstruction",
        "MirModule",
        "ValueId",
        "MirCompiler",
        "execute",
        "retry",
        "fallback",
    ):
        if forbidden in handoff:
            raise AssertionError(f"TX0 handoff gained lowering/retry authority: {forbidden}")
    for definition in (
        "enum VerifiedNormalHelperTopologyPlanV1",
        "struct VerifiedNormalRecursiveCallableModulePlanV1",
    ):
        require_count(
            normal_acyclic_plan,
            definition,
            1,
            f"sole normal topology owner {definition}",
        )
    for fragment in (
        "VerifiedCallableGraphInventoryV1::verify(&self.helpers)",
        "VerifiedCallableSccPartitionV1::verify(inventory)",
        "partition.recursive_component_count() == 0",
        "VerifiedAcyclicCallableGraphV1::from_nonrecursive_partition(partition)",
    ):
        require(
            normal_acyclic_plan,
            fragment,
            f"one-shot normal topology selection {fragment}",
        )
    require(
        acyclic_graph,
        "fn from_nonrecursive_partition(",
        "non-recursive partition consuming acyclic seam",
    )
    require(
        scc_partition,
        "fn into_inventory(self)",
        "single inventory consuming SCC seam",
    )
    for test_name in (
        "one_shot_topology_selector_keeps_zero_edge_helpers_acyclic",
        "one_shot_topology_selector_selects_recursive_without_acyclic_retry",
        "recursive_scc_and_independent_leaf_share_one_recursive_plan",
        "main_call_into_recursive_helper_keeps_main_outside_helper_partition",
        "recursive_declaration_reorder_preserves_normalized_partition",
        "topology_profile_rejection_borrows_owner_and_reuse_stays_green",
    ):
        require(
            direct_call_plan_tests,
            f"fn {test_name}(",
            f"normal topology fixture {test_name}",
        )

    for fragment in (
        "NORMAL-MAIN-DIRECT-CALL0-S0",
        "one retained helper-catalog resolver continuation",
        "second helper index",
        "source/AST clone or rewrite",
        "retry/fallback",
    ):
        require(direct_call_task, fragment, f"Main direct-call task {fragment}")

    for definition in (
        "struct VerifiedNormalCallableCatalogSourceUnitV1",
        "struct RejectedNormalCallableCatalogSourceV1",
        "struct VerifiedNormalMainDirectCallSourceUnitV1",
        "struct RejectedNormalMainDirectCallSourceV1",
        "struct VerifiedNormalMainDirectCallPlanV1",
        "struct RejectedNormalMainDirectCallPlanV1",
    ):
        combined = callable_catalog_source + direct_call_source + direct_call_plan
        require_count(combined, definition, 1, f"sole direct-call owner {definition}")
    for fragment in (
        "PreparedOwnerFreeCallableCatalogV1::prepare(",
        "PreparedCallableCatalogSealV1::prepare(",
        "catalog_plan.commit(owner_free)",
    ):
        require(
            callable_catalog_source,
            fragment,
            f"borrow-only helper catalog preparation {fragment}",
        )
    for fragment in (
        "resolve_forest_with_callable_index(",
        "CatalogSealedResolverContinuationV1::restore(",
        "from_exact_parts_with_callable_index(",
        "source.catalog().index()",
    ):
        require(
            direct_call_source,
            fragment,
            f"continuation-owned Main resolution {fragment}",
        )
    for fragment in (
        "verify_normal_main0_function_with_finite_direct_calls_v1(",
        "profile.direct_calls()",
        "fn discard(self)",
    ):
        require(direct_call_plan, fragment, f"Main direct-call plan law {fragment}")
    for fragment in (
        "NormalMainDirectCall0",
        "allows_zero_parameter_direct_call",
        "zero_parameter_direct_call_not_activated",
    ):
        require(
            function_role_policy + capability,
            fragment,
            f"Main-only zero-parameter caller policy {fragment}",
        )
    for test_name in (
        "call_free_main_and_helper_share_one_retained_catalog",
        "main_direct_call_uses_helper_owner_from_the_same_compilation_brand",
        "unresolved_main_call_rejects_without_call_free_retry",
    ):
        require(
            direct_call_source_tests,
            f"fn {test_name}(",
            f"Main direct-call source fixture {test_name}",
        )
    for test_name in (
        "finite_main_plan_seals_exact_helper_call",
        "multiple_nested_main_calls_preserve_child_before_parent_rows",
        "helper_declaration_order_does_not_change_main_call_meaning",
        "call_free_main_uses_the_same_combined_plan_without_dummy_calls",
    ):
        require(
            direct_call_plan_tests,
            f"fn {test_name}(",
            f"Main direct-call plan fixture {test_name}",
        )

    for forbidden in (
        "FunctionSemanticResolverSessionV1::new",
        "from_exact_parts_without_callable",
        "ASTNode::FunctionCall",
        "MirBuilder",
        "MirInstruction",
        "MirModule",
        "ValueId",
        "RawVm",
        "retry(",
        "fallback",
        ".clone()",
    ):
        if forbidden in direct_call_source + direct_call_plan:
            raise AssertionError(
                f"Main direct-call owner gained duplicate/lowering authority: {forbidden}"
            )

    for fragment in (
        "NORMAL-CALLABLE-MODULE0-A0-S0",
        "zero-edge-capable normal helper DAG plan",
        "second helper catalog/index",
        "retry/fallback",
    ):
        require(acyclic_task, fragment, f"normal acyclic task {fragment}")
    for definition in (
        "struct PreparedNormalMainHelperResolutionV1",
        "struct CompletedNormalMainHelperResolutionV1",
        "struct RejectedNormalMainHelperResolutionV1",
        "enum NormalMainHelperResolutionStageV1",
        "struct VerifiedNormalAcyclicCallableModulePlanV1",
        "enum NormalAcyclicCallableModuleErrorV1",
    ):
        require_count(
            normal_acyclic_plan,
            definition,
            1,
            f"sole normal acyclic owner {definition}",
        )
    for fragment in (
        "VerifiedResolvedCallableModuleV1::resolve_retaining(",
        "VerifiedAcyclicCallableGraphV1::verify(",
        "CanonicalLoweringPreflightV1::verify_function(input)",
        "verify_function_with_finite_direct_calls_v1(input)",
        "header_for_callable(target)",
    ):
        require(
            normal_acyclic_plan,
            fragment,
            f"normal acyclic composition law {fragment}",
        )
    for test_name in (
        "one_call_free_helper_forms_a_zero_edge_normal_dag",
        "independent_helpers_keep_one_zero_edge_graph",
        "helper_calls_form_the_existing_deterministic_dag",
        "helper_self_edge_and_cycle_reject_without_recursive_retry",
        "helper_resolution_rejection_retains_owner_and_later_sources_still_resolve",
    ):
        require(
            direct_call_plan_tests,
            f"fn {test_name}(",
            f"normal acyclic fixture {test_name}",
        )

    for forbidden in (
        "MirBuilder",
        "MirInstruction",
        "MirType",
        "ValueId",
        "RawRoot",
        "NYASH_ENTRY",
        "compile",
        "execute",
        "retry",
        "fallback",
        ".clone()",
    ):
        if forbidden in callable_source:
            raise AssertionError(
                f"normal callable source gained lowering/retry authority: {forbidden}"
            )

    for path in files:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(
                f"file must remain below 800 lines: {path.relative_to(root)}"
            )
    return files
