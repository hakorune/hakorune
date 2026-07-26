#!/usr/bin/env python3
"""NORMAL-SOURCE-PLAN0 reusable source-family authority guard."""

from pathlib import Path
from normal_source_plan0_callable_guard import check_callable_source
from normal_source_plan0_transaction_guard import check_transaction


ROOT = Path(__file__).resolve().parents[3]
SOURCE_DIR = ROOT / "src/mir/compiler/normal_source_plan"
COMPILER_MOD = ROOT / "src/mir/compiler/mod.rs"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-source-plan0-s0-execution-task-2026-07-26.md"
)
INPUT_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-source-plan0-input0-s0-execution-task-2026-07-26.md"
)
MAIN_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-main0-source0-s0-execution-task-2026-07-26.md"
)
MAIN_F1_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-main0-f1-plan0-s0-execution-task-2026-07-26.md"
)
NORMAL_MODULE_TX_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-module-tx0-l0-execution-task-2026-07-26.md"
)
MAIN_THUNK_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "normal-main0-thunk0-s0-execution-task-2026-07-26.md"
)
MIR_ROOT = ROOT / "src/mir/mod.rs"
FRONTDOOR = ROOT / "src/runner/reference/normal_file_vm_frontdoor.rs"
FRONTDOOR_INPUT = (
    ROOT
    / "src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs"
)
FRONTDOOR_INPUT_TESTS = (
    ROOT
    / "src/runner/reference/normal_file_vm_frontdoor/source_plan_input_tests.rs"
)
CANONICAL_CORE_DISPATCH = ROOT / "src/mir/compiler/canonical_core_dispatch.rs"
PRODUCTION_FILES = tuple(
    SOURCE_DIR / name
    for name in ("mod.rs", "product.rs", "inventory.rs", "classifier.rs", "rejection.rs")
)
MAIN_SOURCE = SOURCE_DIR / "main_source.rs"
MAIN_SOURCE_TESTS = SOURCE_DIR / "main_source_tests.rs"
MAIN_RESOLVED_SOURCE = SOURCE_DIR / "main_resolved_source.rs"
MAIN_RESOLVED_SOURCE_TESTS = SOURCE_DIR / "main_resolved_source_tests.rs"
MAIN_FUNCTION_PLAN = SOURCE_DIR / "main_function_plan.rs"
MAIN_FUNCTION_PLAN_TESTS = SOURCE_DIR / "main_function_plan_tests.rs"
MAIN_THUNK_PLAN = SOURCE_DIR / "main_thunk_plan.rs"
MAIN_THUNK_PLAN_TESTS = SOURCE_DIR / "main_thunk_plan_tests.rs"
CAPABILITY = ROOT / "src/mir/compiler/capability.rs"
VALUE_PROFILE_ANALYZER = ROOT / "src/mir/resolved_value_profile/analyzer.rs"
VALUE_PROFILE_MOD = ROOT / "src/mir/resolved_value_profile/mod.rs"
BUILDER_MOD = ROOT / "src/mir/builder.rs"
NORMAL_MODULE_TX_DIR = ROOT / "src/mir/builder/normal_module_transaction"
NORMAL_MODULE_TX_PASSIVE_FILES = tuple(
    NORMAL_MODULE_TX_DIR / name
    for name in (
        "mod.rs",
        "entry_target.rs",
        "schema.rs",
        "rejection.rs",
        "canonical_batch.rs",
        "canonical_batch_tests.rs",
        "tests.rs",
    )
)
NORMAL_MODULE_TX_ACTIVATION_FILES = tuple(
    NORMAL_MODULE_TX_DIR / name
    for name in (
        "main_transaction.rs",
        "main_transaction_tests.rs",
        "physical_thunk.rs",
        "result_type.rs",
        "source_draft.rs",
    )
)
# The transaction is a closed Builder boundary. Its callable files are valid
# consumers of its schema/evidence, while every consumer outside this directory
# still has to be named explicitly below.
NORMAL_MODULE_TX_FILES = tuple(NORMAL_MODULE_TX_DIR.glob("*.rs"))
ALL_FILES = (
    *PRODUCTION_FILES,
    MAIN_SOURCE,
    SOURCE_DIR / "tests.rs",
    MAIN_SOURCE_TESTS,
    MAIN_RESOLVED_SOURCE,
    MAIN_RESOLVED_SOURCE_TESTS,
    MAIN_FUNCTION_PLAN,
    MAIN_FUNCTION_PLAN_TESTS,
    MAIN_THUNK_PLAN,
    MAIN_THUNK_PLAN_TESTS,
    SOURCE_DIR / "test_support.rs",
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def require_count(text: str, fragment: str, expected: int, label: str) -> None:
    actual = text.count(fragment)
    if actual != expected:
        raise AssertionError(
            f"{label}: expected {expected} occurrences of {fragment!r}, got {actual}"
        )


def main() -> int:
    task = TASK.read_text()
    input_task = INPUT_TASK.read_text()
    main_task = MAIN_TASK.read_text()
    main_f1_task = MAIN_F1_TASK.read_text()
    normal_module_tx_task = NORMAL_MODULE_TX_TASK.read_text()
    main_thunk_task = MAIN_THUNK_TASK.read_text()
    production = "\n".join(path.read_text() for path in PRODUCTION_FILES)
    classifier = (SOURCE_DIR / "classifier.rs").read_text()
    tests = (SOURCE_DIR / "tests.rs").read_text()
    compiler_mod = COMPILER_MOD.read_text()
    mir_root = MIR_ROOT.read_text()
    frontdoor = FRONTDOOR.read_text()
    frontdoor_input = FRONTDOOR_INPUT.read_text()
    frontdoor_input_tests = FRONTDOOR_INPUT_TESTS.read_text()
    main_source = MAIN_SOURCE.read_text()
    main_source_tests = MAIN_SOURCE_TESTS.read_text()
    main_resolved_source = MAIN_RESOLVED_SOURCE.read_text()
    main_resolved_source_tests = MAIN_RESOLVED_SOURCE_TESTS.read_text()
    main_function_plan = MAIN_FUNCTION_PLAN.read_text()
    main_function_plan_tests = MAIN_FUNCTION_PLAN_TESTS.read_text()
    main_thunk_plan = MAIN_THUNK_PLAN.read_text()
    main_thunk_plan_tests = MAIN_THUNK_PLAN_TESTS.read_text()
    capability = CAPABILITY.read_text()
    value_profile_analyzer = VALUE_PROFILE_ANALYZER.read_text()
    value_profile_mod = VALUE_PROFILE_MOD.read_text()
    builder_mod = BUILDER_MOD.read_text()
    normal_module_tx = "\n".join(
        path.read_text() for path in NORMAL_MODULE_TX_PASSIVE_FILES
    )
    normal_module_tx_tests = (NORMAL_MODULE_TX_DIR / "tests.rs").read_text()
    canonical_batch = (NORMAL_MODULE_TX_DIR / "canonical_batch.rs").read_text()
    canonical_batch_tests = (
        NORMAL_MODULE_TX_DIR / "canonical_batch_tests.rs"
    ).read_text()
    check_transaction(
        ROOT,
        NORMAL_MODULE_TX_DIR,
        ROOT
        / (
            "docs/development/current/main/investigations/"
            "normal-main0-tx0-i0-execution-task-2026-07-26.md"
        ),
    )
    callable_source_files = check_callable_source(
        ROOT, SOURCE_DIR, require, require_count
    )

    for fragment in (
        "NORMAL-SOURCE-PLAN0-S0",
        "profile-admission-free",
        "Builder-free, MIR-free, backend-free, runner-free",
        "production consumer                        = 0",
        "all touched source/check files             < 800 lines",
    ):
        require(task, fragment, f"S0 task contract {fragment}")

    for fragment in (
        "NORMAL-SOURCE-PLAN0-INPUT0-S0",
        "one disconnected consuming NormalFile-to-source-plan request",
        "Profile admission remains the later",
        "existing prepare_raw_vm_handoff caller count = unchanged",
        "new source-plan production caller            = 0",
        "Direct `pub use` additions to the MIR root vocabulary",
    ):
        require(input_task, fragment, f"INPUT0 task contract {fragment}")

    for fragment in (
        "NORMAL-MAIN0-SOURCE0-S0",
        "Program-owned embedded Main.main/0 source unit",
        "source-family reclassification         = 0",
        "VerifiedMainExpansion re-entry         = 0",
        "AST clone/rewrite                      = 0",
        "production consumer                    = 0",
    ):
        require(main_task, fragment, f"MAIN SOURCE0 task contract {fragment}")

    for fragment in (
        "NORMAL-MAIN0-F1-PLAN0-S0",
        "Program-owned embedded resolved Main.main/0 function plan",
        "ordinary callable main admission               = 0",
        "Builder / MirInstruction / publication         = 0",
        "fallback/retry                                 = 0",
        "all modified/new source and check files        < 800 lines",
    ):
        require(main_f1_task, fragment, f"MAIN F1 task contract {fragment}")

    for fragment in (
        "NORMAL-MODULE-TX0-L0",
        "disconnected heterogeneous canonical normal-module transaction schema",
        "source Main role                           = 1",
        "helper role                                = 1",
        "physical entry role                        = 1",
        "production consumer                         = 0",
        "all modified/new source/check files         < 800 lines",
    ):
        require(normal_module_tx_task, fragment, f"normal-module TX task {fragment}")

    for fragment in (
        "NORMAL-MAIN0-THUNK0-S0",
        "disconnected exact source-Main to physical-main thunk plan",
        "existing VerifiedResolvedOwnerHeaderV1",
        "existing F1/result authority consumer    = 1",
        "MIR writer / VM consumer = 0",
        "all modified/new source/check files      < 800 lines",
    ):
        require(main_thunk_task, fragment, f"Main THUNK0 task contract {fragment}")


    definitions = (
        "struct PreparedNormalSourcePlanInputV1",
        "struct NormalSourceSurfaceInventoryV1",
        "struct NormalSourcePlanClassifierV1",
        "enum SealedNormalSourcePlanV1",
        "enum SealedNormalScalarRootV1",
        "struct SealedNormalCallableModuleSourceV1",
        "struct RejectedNormalSourcePlanV1",
    )
    for definition in definitions:
        require_count(production, definition, 1, f"sole definition {definition}")
    require_count(
        classifier,
        "pub(crate) fn seal(",
        1,
        "sole source-plan classifier terminal",
    )
    require_count(
        compiler_mod,
        "pub(crate) mod normal_source_plan;",
        1,
        "compiler module declaration",
    )
    require_count(
        mir_root,
        "pub(crate) use compiler::normal_source_plan;",
        1,
        "owner-module facade",
    )

    for fragment in (
        "ScalarRoot(SealedNormalScalarRootV1)",
        "CallableModule(SealedNormalCallableModuleSourceV1)",
        "Script(SealedNormalScriptSourceV1)",
        "Main0(SealedNormalMainSourceV1)",
        "NormalSourcePlanStageV1::RootSurface",
        "NormalSourcePlanStageV1::SourceEntry",
        "NormalSourcePlanStageV1::FamilyClosure",
        "method_entries.sort_by",
    ):
        require(production, fragment, f"source-family law {fragment}")

    for test_name in (
        "empty_and_scalar_programs_are_scripts",
        "main_zero_only_is_a_scalar_main_root",
        "top_level_or_main_box_helpers_make_callable_modules",
        "function_only_program_has_no_source_entry",
        "script_mixed_with_main_or_function_is_rejected_in_either_order",
        "duplicate_main_is_rejected_in_either_order",
        "main_must_be_static_and_define_static_main_zero",
        "unsupported_declaration_is_rejected_before_family_selection",
        "non_program_root_is_rejected_at_root_surface",
    ):
        require(tests, f"fn {test_name}(", f"fixture {test_name}")

    input_definitions = (
        "struct PreparedNormalFileSourcePlanRequestV1",
        "struct ClassifiedNormalFileSourcePlanV1",
        "struct RejectedNormalFileSourcePlanningV1",
    )
    for definition in input_definitions:
        require_count(frontdoor_input, definition, 1, f"sole INPUT0 definition {definition}")
    for fragment in (
        "fn prepare_source_plan_request(",
        "PreparedNormalSourcePlanInputV1::new(ast, display_identity)",
        "fn classify(",
        "NormalSourcePlanClassifierV1::seal(input)",
        "profile: SealedNormalEntryProfileV1",
        "receipt: NormalFileSourceReceiptV1",
        "fn stage(&self)",
        "fn error(&self)",
        "fn discard(self)",
    ):
        require(frontdoor_input, fragment, f"INPUT0 boundary {fragment}")
    require_count(
        frontdoor,
        "mod source_plan_input;",
        1,
        "front-door child-module declaration",
    )
    require_count(
        frontdoor,
        "fn prepare_raw_vm_handoff(",
        1,
        "unchanged narrow handoff terminal",
    )

    for test_name in (
        "parsed_empty_and_scalar_sources_become_script_plans_once",
        "parsed_main_zero_becomes_scalar_main_plan_once",
        "parsed_main_with_top_level_or_box_helper_becomes_callable_module",
        "parsed_function_only_retains_missing_entry_rejection",
        "parsed_script_plus_main_retains_mixed_family_rejection",
        "parse_and_using_rejections_never_issue_source_plan_requests",
    ):
        require(
            frontdoor_input_tests,
            f"fn {test_name}(",
            f"INPUT0 fixture {test_name}",
        )

    for definition in (
        "enum NormalModuleDraftRoleV1",
        "struct NormalModuleDraftExpectationV1",
        "struct NormalModuleEntryRelationV1",
        "struct NormalModuleTransactionDraftV1",
        "struct NormalModuleTransactionSchemaV1",
        "enum NormalModuleTransactionSchemaErrorV1",
        "struct RejectedNormalModuleTransactionSchemaV1",
    ):
        require_count(
            normal_module_tx,
            definition,
            1,
            f"sole normal-module transaction definition {definition}",
        )
    for definition in (
        "struct CanonicalNormalMainEntryTargetV1",
        "fn canonical_normal_main_entry_target()",
    ):
        require_count(
            normal_module_tx,
            definition,
            1,
            f"sole canonical physical-entry identity {definition}",
        )
    for fragment in (
        "FunctionDraftKeyV1::Main",
        "CanonicalInsertedDispositionV1::from_canonical_source()",
        'symbol: "main".into()',
        "arity: 0",
    ):
        require(normal_module_tx, fragment, f"canonical physical-entry law {fragment}")
    for fragment in (
        "SourceMain { owner: FunctionOwnerIdV1 }",
        "Helper { key: CanonicalCallableKeyV1 }",
        "PhysicalEntry",
        "FunctionDraftKeyV1::CanonicalResolvedOwner(owner)",
        "FunctionDraftKeyV1::CanonicalCallable(key)",
        "FunctionDraftKeyV1::Main",
        "CanonicalInsertedDispositionV1::from_canonical_source()",
        "rows.sort_by(compare_rows)",
        "pub(in crate::mir::builder) fn seal(",
        "pub(in crate::mir::builder) fn error(&self)",
        "pub(in crate::mir::builder) fn discard(self)",
    ):
        require(normal_module_tx, fragment, f"normal-module transaction law {fragment}")
    require_count(
        builder_mod,
        "mod normal_module_transaction;",
        1,
        "normal-module transaction module declaration",
    )
    for test_name in (
        "main_only_and_helper_batches_seal_in_deterministic_role_order",
        "missing_and_duplicate_required_roles_are_typed_and_retained",
        "duplicate_key_and_symbol_are_rejected_before_any_mutation",
        "role_key_arity_and_entry_relation_drift_are_typed",
    ):
        require(
            normal_module_tx_tests,
            f"fn {test_name}(",
            f"normal-module transaction fixture {test_name}",
        )
    for definition in (
        "struct NormalCanonicalModuleBatchV1",
        "struct PreparedNormalCanonicalModuleBatchV1",
        "struct RejectedNormalCanonicalModuleBatchV1",
        "enum NormalCanonicalModuleBatchErrorV1",
    ):
        require_count(
            canonical_batch,
            definition,
            1,
            f"sole canonical batch definition {definition}",
        )
    for fragment in (
        "VerifiedNormalMainThunkPlanV1",
        "NormalModuleDraftExpectationV1::source_main(",
        "NormalModuleDraftExpectationV1::physical_entry(",
        "NormalModuleEntryRelationV1::new(",
        "NormalModuleTransactionSchemaV1::seal(draft)",
        "fn error(&self)",
        "fn discard(self)",
    ):
        require(canonical_batch, fragment, f"canonical batch law {fragment}")
    for test_name in (
        "main_only_batch_seals_unit_and_scalar_manifests",
        "batch_projection_uses_thunk_identity_without_raw_policy",
        "malformed_batch_retains_complete_thunk_owner",
    ):
        require(
            canonical_batch_tests,
            f"fn {test_name}(",
            f"canonical batch fixture {test_name}",
        )
    for forbidden in (
        "ASTNode",
        "MirBuilder",
        "MirFunction",
        "MirInstruction",
        "ValueId",
        "MirType",
        "NYASH_ENTRY",
        "RawMainEntryTargetV1",
        "RawRootBatchSlotV1",
        "LegacyReplaceWholePair",
        "ModuleDraftCollectorV1",
        "retry",
        "fallback",
    ):
        if forbidden in canonical_batch:
            raise AssertionError(f"canonical batch gained forbidden authority: {forbidden}")
    for forbidden in (
        "LegacyReplaceWholePair",
        "MirBuilder",
        "MirInstruction",
        "MirModule",
        "ModuleDraftCollectorV1",
        "try_add_functions_atomic",
        "process::exit",
        "crate::runner",
        "crate::runtime",
        "retry",
        "fallback",
    ):
        if forbidden in normal_module_tx:
            raise AssertionError(
                f"passive normal-module schema gained forbidden authority: {forbidden}"
            )

    for definition in (
        "struct VerifiedNormalMainResolvedSourceUnitV1",
        "struct VerifiedNormalMainRoleV1",
        "struct VerifiedNormalMainFunctionPlanV1",
        "struct RejectedNormalMainFunctionPlanV1",
    ):
        require_count(
            main_resolved_source + main_function_plan,
            definition,
            1,
            f"sole Main F1 definition {definition}",
        )
    for fragment in (
        "FunctionSemanticResolverSessionV1::new(0)",
        "VerifiedSourceProjectionV1::seal(function.function(), &forest)",
        "ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(",
        "pub(crate) fn prepare_embedded_resolved_main(",
        "pub(crate) fn borrow_function_input(",
        "pub(crate) const fn role(&self)",
    ):
        require(main_resolved_source, fragment, f"Main resolved-source boundary {fragment}")
    for fragment in (
        "CanonicalLoweringPreflightV1::verify_normal_main0_function_v1(",
        "pub(crate) fn seal(",
        "pub(crate) fn completion(&self)",
        "pub(crate) fn into_lowering(self)",
        "pub(crate) fn error(&self)",
        "pub(crate) fn discard(self)",
    ):
        require(main_function_plan, fragment, f"Main F1 plan boundary {fragment}")
    for fragment in (
        "analyze_trivial_canonical_main_owner_v1",
        "CanonicalFunctionRolePolicyV1::OrdinaryFirstFamily",
        "CanonicalFunctionRolePolicyV1::NormalMain0",
        "name == \"main\"",
        "name != \"main\"",
        "function_role_capability_mismatch",
    ):
        require(capability, fragment, f"role-scoped canonical capability {fragment}")
    for fragment in (
        "RootProfilePolicyV1::OrdinaryFirstFamily",
        "RootProfilePolicyV1::NormalMain0",
        "name == \"main\"",
        "name != \"main\"",
        "TrivialRepresentationV1::ExplicitVoidValue",
        "TrivialRepresentationV1::NullSentinel",
    ):
        require(value_profile_analyzer, fragment, f"role-scoped value profile {fragment}")
    require(
        value_profile_mod,
        "pub(crate) fn analyze_trivial_canonical_main_owner_v1(",
        "sole Main value-profile facade",
    )

    for test_name in (
        "embedded_main_resolution_keeps_exact_program_owned_function_identity",
        "embedded_main_resolution_reuses_nested_owner_forest_and_source_projection",
    ):
        require(
            main_resolved_source_tests,
            f"fn {test_name}(",
            f"Main resolved-source fixture {test_name}",
        )
    for test_name in (
        "main_f1_seals_empty_fallthrough_and_expression_statement_as_unit",
        "main_f1_preserves_explicit_unit_origins",
        "main_f1_admits_exact_scalar_value_carriers",
        "main_f1_admits_void_and_exact_i64_declared_contracts",
        "main_f1_rejects_contract_mismatch_and_unsupported_carrier_before_lowering",
        "main_f1_rejects_multiple_nested_and_nonterminal_returns",
        "main_f1_rejects_direct_call_and_nested_owner_before_lowering",
        "ordinary_preflight_still_rejects_standalone_main",
        "main_plan_retains_exact_role_unit_and_consumable_trivial_plan",
    ):
        require(
            main_function_plan_tests,
            f"fn {test_name}(",
            f"Main F1 fixture {test_name}",
        )

    for definition in (
        "enum VerifiedNormalMainThunkResultV1",
        "enum NormalMainThunkPlanErrorV1",
        "struct VerifiedNormalMainEntryRelationV1",
        "struct VerifiedNormalMainThunkPlanV1",
        "struct RejectedNormalMainThunkPlanV1",
    ):
        require_count(main_thunk_plan, definition, 1, f"sole Main thunk definition {definition}")
    for fragment in (
        ".seal_source_header()",
        "source.completion(),",
        "let contract = completion.function_exit_contract();",
        "source.terminal_profile()",
        "source_header.owner() != contract.owner()",
        "canonical_normal_main_entry_target()",
        "fn error(&self)",
        "fn discard(self)",
    ):
        require(main_thunk_plan, fragment, f"Main thunk boundary {fragment}")
    for test_name in (
        "thunk_seals_unit_origins_and_scalar_results",
        "thunk_rejects_physical_entry_relation_drift_and_retains_source",
        "thunk_rejects_completion_profile_disagreement",
        "thunk_rejects_no_value_representation_on_explicit_value_route",
    ):
        require(
            main_thunk_plan_tests,
            f"fn {test_name}(",
            f"Main thunk fixture {test_name}",
        )
    for forbidden in (
        "ASTNode",
        "MirBuilder",
        "MirInstruction",
        "ValueId",
        "MirType",
        "NYASH_ENTRY",
        "module.functions",
        "RawMainEntryTargetV1",
        "LegacyReplaceWholePair",
        "retry",
        "fallback",
    ):
        if forbidden in main_thunk_plan:
            raise AssertionError(f"Main thunk gained forbidden authority: {forbidden}")

    for definition in (
        "struct VerifiedNormalMainFunctionSourceUnitV1",
        "struct NormalMainFunctionSourceViewV1",
        "struct RejectedNormalMainFunctionSourceV1",
        "enum NormalMainFunctionSourceErrorV1",
    ):
        require_count(main_source, definition, 1, f"sole Main source definition {definition}")
    for fragment in (
        "source: SealedNormalMainSourceV1",
        "fn borrow_exact_function(&self)",
        "CallableFunctionSyntaxViewV1::from_function_ast(function)",
        "fn verify_main_source_relation(",
        "fn error(&self)",
        "fn discard(self)",
        "[normal-main-source/invariant]",
    ):
        require(main_source, fragment, f"Main source boundary {fragment}")
    require_count(
        production,
        "fn prepare_function_source(",
        1,
        "one sealed Main consuming delegation",
    )
    for test_name in (
        "exact_private_site_does_not_reclassify_unrelated_program_statements",
        "main_zero_seals_one_borrowed_exact_function_without_clone",
        "main_body_annotation_and_program_owned_box_fields_survive_source_sealing",
        "missing_or_drifted_main_statement_is_typed_and_retained",
        "root_and_missing_method_are_typed_and_retained",
        "method_key_name_shape_static_and_arity_drift_are_typed",
    ):
        require(
            main_source_tests,
            f"fn {test_name}(",
            f"Main source fixture {test_name}",
        )


    forbidden_classifier_authority = (
        "SealedNormalEntryProfileV1",
        "NormalFileNoImportVmReferenceV1",
        "RawVmReference",
        "MirBuilder",
        "MirInstruction",
        "ValueId",
        "MirType",
        "crate::runner",
        "crate::runtime",
        "NYASH_ENTRY",
        "module.functions",
        "compile_with_source",
        "build_module",
    )
    for forbidden in forbidden_classifier_authority:
        if forbidden in production:
            raise AssertionError(
                f"source classifier gained non-source authority: {forbidden}"
            )

    for forbidden in (
        "retry",
        "fallback",
        "reclassify",
        "into_ast",
        ".clone()",
        "ASTNode::Program { statements:",
    ):
        if forbidden in production:
            raise AssertionError(f"source owner gained forbidden operation: {forbidden}")

    for forbidden in (
        "prepare_raw_vm_handoff",
        "RawVmReference",
        "into_downstream",
        "match profile",
        "std::fs::",
        "NyashParser",
        "parse_from_string",
        ".clone()",
        "retry",
        "fallback",
        "compile_with_source",
        "build_module",
    ):
        if forbidden in frontdoor_input:
            raise AssertionError(
                f"INPUT0 gained forbidden route/profile/I/O authority: {forbidden}"
            )

    for forbidden in (
        "VerifiedMainExpansionV1",
        "VerifiedRawRootExpansionV1",
        "::from_program",
        "OwnedRawSourceV1",
        "RawSourceLocatorV1",
        "MirBuilder",
        "MirInstruction",
        "ValueId",
        "MirType",
        "crate::runner",
        "crate::runtime",
        ".clone()",
        "into_ast",
        "rewrite",
        "retry",
        "fallback",
    ):
        if forbidden in main_source:
            raise AssertionError(
                f"Main source owner gained reclassification/lowering authority: {forbidden}"
            )

    for marker in ("#[derive(Debug, Clone", "#[derive(Clone", "#[derive(Debug, Copy"):
        if marker in production:
            raise AssertionError(f"move-only source product became duplicable: {marker}")

    watched_symbols = (
        "NormalSourcePlanClassifierV1",
        "SealedNormalSourcePlanV1",
        "PreparedNormalSourcePlanInputV1",
        "VerifiedNormalMainFunctionSourceUnitV1",
        "VerifiedNormalMainResolvedSourceUnitV1",
        "VerifiedNormalMainFunctionPlanV1",
        "VerifiedNormalMainThunkPlanV1",
        "VerifiedNormalCallableSourceUnitV1",
        "CanonicalNormalMainEntryTargetV1",
        "PreparedNormalCanonicalModuleBatchV1",
        "NormalModuleTransactionSchemaV1",
    )
    allowed = set(ALL_FILES) | {
        COMPILER_MOD,
        MIR_ROOT,
        FRONTDOOR,
        FRONTDOOR_INPUT,
        FRONTDOOR_INPUT_TESTS,
        CAPABILITY,
        VALUE_PROFILE_ANALYZER,
        VALUE_PROFILE_MOD,
        BUILDER_MOD,
        CANONICAL_CORE_DISPATCH,
        *NORMAL_MODULE_TX_FILES,
        *callable_source_files,
    }
    for path in (ROOT / "src").rglob("*.rs"):
        if path in allowed:
            continue
        text = path.read_text()
        if any(symbol in text for symbol in watched_symbols):
            raise AssertionError(
                f"disconnected source-plan authority escaped S0: {path.relative_to(ROOT)}"
            )

    for path in (
        *ALL_FILES,
        FRONTDOOR,
        FRONTDOOR_INPUT,
        FRONTDOOR_INPUT_TESTS,
        CAPABILITY,
        VALUE_PROFILE_ANALYZER,
        VALUE_PROFILE_MOD,
        BUILDER_MOD,
        CANONICAL_CORE_DISPATCH,
        *NORMAL_MODULE_TX_FILES,
        *callable_source_files,
        Path(__file__),
        ROOT / "tools/checks/lib/normal_source_plan0_transaction_guard.py",
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(
                f"file must remain below 800 lines: {path.relative_to(ROOT)}"
            )

    print(
        "[normal-source-plan0-guard] ok "
        "classifier=1 script=1 main0=1 callable=1 profile=0 "
        "frontdoor_input=1 disconnected_consumer=1 production_consumer=0 "
        "main_source=1 reclassification=0 second_read_parse=0 "
        "main_role=1 main_f1=1 ordinary_main_admission=0 "
        "normal_module_schema=1 mutation=0 "
        "main_thunk=1 result_authority=1 physical_entry_identity=1 "
        "canonical_batch=1 batch_mutation=0 "
        "raw_route_delta=0 rewrite=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
