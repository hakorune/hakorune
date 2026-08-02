#!/usr/bin/env python3
"""Reusable HEADERPORT0 route-inventory extension guard."""

from __future__ import annotations

import pathlib

from headerport_borrow_canonical_guard import verify_borrow_canonical_p0
from headerport_borrow_root_guard import verify_borrow_root_p0


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def forbid(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def verify_single_header_s0(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
    state: str,
) -> None:
    source_path = root / "src/mir/compiler/capability/resolved_owner_header.rs"
    capability_path = root / "src/mir/compiler/capability.rs"
    tests_path = root / "src/mir/compiler/capability_tests.rs"
    p0_path = root / "src/mir/builder/resolved_owner_header_p0.rs"
    symbol_path = root / "src/mir/resolved_semantics/callable_symbol.rs"
    source = source_path.read_text()
    capability = capability_path.read_text()
    tests = tests_path.read_text()
    p0 = p0_path.read_text()
    symbol = symbol_path.read_text()
    if any(
        len(text.splitlines()) >= 800
        for text in (source, capability, tests, p0, symbol)
    ):
        raise AssertionError("ROUTEINV-P0c single-header source/proofs must remain below 800 lines")

    for fragment in (
        "VerifiedResolvedOwnerHeaderV1",
        "CanonicalFirstFamilyPlanBrandV1",
        "ResolvedOwnerHeaderFamilyV1",
        "ResolvedOwnerHeaderSealErrorV1",
        "pub(super) fn seal(",
        "CanonicalCallableSymbolV1",
        "OwnerMismatch",
        "ForeignPlan",
        "SourceNameContainsPhysicalSeparator",
    ):
        require(source + capability, fragment, "ROUTEINV-P0c single-header vocabulary")
    require(
        source,
        "#[derive(Debug)]\npub(crate) struct VerifiedResolvedOwnerHeaderV1",
        "non-Clone resolved-owner header product",
    )
    product = source.split("pub(crate) struct VerifiedResolvedOwnerHeaderV1", 1)[1].split(
        "struct ResolvedOwnerHeaderSealV1", 1
    )[0]
    for fragment in (
        "MirBuilder",
        "MirModule",
        "MirFunction",
        "ModuleDraftCollector",
        "ASTNode",
        "ValueId",
        "TypeContext",
        "retry",
        "fallback",
    ):
        forbid(product, fragment, f"resolved-owner header stores {fragment}")
    forbid(source, "impl Clone for VerifiedResolvedOwnerHeaderV1", "header Clone implementation")
    forbid(source, "pub(crate) fn seal(", "crate-visible header constructor")
    forbid(source, "fn from_parts(", "caller-owned header constructor")
    forbid(source, "fn new(", "caller-owned header constructor")
    forbid(source, 'format!("{}/{}"', "duplicated physical symbol projection")
    require(
        symbol,
        "pub(crate) fn from_name_arity",
        "neutral physical symbol projection",
    )

    for fragment in (
        "resolved_owner_header_seals_zero_arity_binding_ssa_before_plan_consumption",
        "resolved_owner_header_seals_a_plus_family_without_exact_i64_profile",
        "resolved_owner_header_rejects_foreign_plan_pairing",
        "ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa",
        "ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus",
        "header.arity(), 0",
        "ResolvedOwnerHeaderSealErrorV1::ForeignPlan",
    ):
        require(tests, fragment, "ROUTEINV-P0c single-header fixture")
    for fragment in (
        "single_header_exact_families_nonzero_arity_and_source_reorder_are_stable",
        "single_header_rejects_separator_and_foreign_owner_family_pairing",
        "single_header_projects_canonical_duplicate_symbol_and_arity_failures",
        "binding_header/1",
        "a_plus_header/1",
        "bad/name",
        "SourceNameContainsPhysicalSeparator",
        "FunctionDraftKeyV1::CanonicalResolvedOwner",
        "DraftPublicationPolicyV1::CanonicalRejectDuplicate",
        "ModuleDraftAdmissionErrorV1::DuplicateKey",
        "ModuleDraftAdmissionErrorV1::DuplicateSymbol",
        "ModuleDraftAdmissionErrorV1::SymbolMismatch",
        "ModuleDraftAdmissionErrorV1::ArityMismatch",
    ):
        require(p0, fragment, "ROUTEINV-P0c single-header P0 matrix")
    for fragment in (
        "FunctionDraftKeyV1::Main",
        "FunctionDraftKeyV1::SyntheticConditionFn",
        "condition_fn",
        "RawExpansion",
        "RawCondition",
    ):
        forbid(source + capability + p0, fragment, "raw policy leaks into single-header proof")
    require(
        builder_mod,
        "mod resolved_owner_header_p0;",
        "single-header P0 registration",
    )

    consumers = []
    excluded = {source_path, capability_path, tests_path, p0_path}
    for path in (root / "src/mir").rglob("*.rs"):
        if path in excluded:
            continue
        if "VerifiedResolvedOwnerHeaderV1" in path.read_text():
            consumers.append(str(path.relative_to(root)))
    if consumers:
        raise AssertionError("ROUTEINV-P0c production consumers: " + ", ".join(consumers))
    call_count = 0
    for path in (root / "src/mir").rglob("*.rs"):
        if path in (tests_path, p0_path):
            continue
        call_count += path.read_text().count("seal_resolved_owner_header_v1(")
    if call_count != 1:
        raise AssertionError(
            "ROUTEINV-P0c seal issuer must have zero production callers: "
            f"occurrences={call_count}"
        )

    require(card, "WIRING-I0-ROUTEINV-P0c-SINGLEHDR-S0 closeout", "single-header S0 closeout")
    require(card, "WIRING-I0-ROUTEINV-P0c-SINGLEHDR-P0 closeout", "single-header P0 closeout")
    require(
        state,
        "BORROW-P0-CANONICAL is closed",
        "single-header downstream closed-state pointer",
    )


def verify_callable_batch_p0(
    root: pathlib.Path,
    card: str,
    state: str,
) -> None:
    acyclic_path = root / "src/mir/compiler/acyclic_callable_module_plan/tests.rs"
    recursive_path = root / "src/mir/compiler/recursive_callable_module_plan/tests.rs"
    support_path = root / (
        "src/mir/compiler/callable_batch_correspondence_test_support.rs"
    )
    compiler_mod_path = root / "src/mir/compiler/mod.rs"
    transaction_path = root / (
        "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
    )
    p0d_path = root / (
        "src/mir/builder/resolved_lowering/"
        "callable_module_transaction_p0d_tests.rs"
    )
    catalog_failure_path = root / (
        "src/mir/resolved_semantics/callable_catalog_candidate_tests.rs"
    )
    source_failure_path = root / "src/mir/compiler/resolved_callable_module_tests.rs"
    publication_path = root / "src/mir/function/tests.rs"
    acyclic = acyclic_path.read_text()
    recursive = recursive_path.read_text()
    support = support_path.read_text()
    compiler_mod = compiler_mod_path.read_text()
    transaction = transaction_path.read_text()
    p0d = p0d_path.read_text()
    catalog_failure = catalog_failure_path.read_text()
    source_failure = source_failure_path.read_text()
    publication = publication_path.read_text()

    for path, text in (
        (acyclic_path, acyclic),
        (recursive_path, recursive),
        (support_path, support),
        (transaction_path, transaction),
        (p0d_path, p0d),
    ):
        if len(text.splitlines()) >= 800:
            raise AssertionError(
                f"ROUTEINV-P0d source/proof must remain below 800 lines: {path}"
            )

    for proof, label in ((acyclic, "acyclic"), (recursive, "recursive")):
        for fragment in (
            "fn borrowed_batch_rows(",
            "let functions = module.functions_by_key();",
            "let plans = plan.plans_by_key();",
            "let rows = borrowed_catalog_header_rows(module);",
            "assert_eq!(rows.len(), functions.len());",
            "assert!(functions.keys().eq(plans.keys()));",
        ):
            require(proof, fragment, f"ROUTEINV-P0d {label} borrowed correspondence")
        forbid(proof, "ModuleDraftCollectorV1", f"{label} proof collector connection")
        forbid(proof, "struct VerifiedCallableBatch", f"{label} second batch catalog")

    for fragment in (
        "fn borrowed_catalog_header_rows(",
        "let catalog = module.source().catalog();",
        "let functions = module.functions_by_key();",
        "assert_eq!(catalog.len(), functions.len());",
        "CanonicalCallableSymbolV1::from_name_arity(",
        "assert_eq!(header.source_key(), key);",
        "assert_eq!(header.symbol(), &physical);",
        "assert_eq!(header.signature().arity(), key.arity() as usize);",
        "module.function(key).is_some()",
    ):
        require(support, fragment, "ROUTEINV-P0d shared borrowed header proof")
    for fragment in (
        "BTreeMap",
        "ModuleDraftCollectorV1",
        "pub(crate) struct",
        "pub(in crate::mir) struct",
    ):
        forbid(support, fragment, f"P0d support owns {fragment}")
    require(
        compiler_mod,
        "#[cfg(test)]\nmod callable_batch_correspondence_test_support;",
        "test-only callable-batch support registration",
    )

    for fragment in (
        "let nodes = plan.graph().nodes();",
        "assert!(functions.keys().eq(nodes.iter()));",
        "declaration_reorder_preserves_graph_and_typed_plan_keys",
    ):
        require(acyclic, fragment, "ROUTEINV-P0d acyclic exact set/reorder")
    for fragment in (
        "let inventory = plan.partition().inventory();",
        "component_members.sort();",
        "assert!(functions.keys().eq(inventory.nodes().iter()));",
        "assert!(functions.keys().eq(component_members.iter()));",
        "plan.partition().component_for(key).is_some()",
        "declaration_reorder_preserves_partition_and_typed_plan_keys",
    ):
        require(recursive, fragment, "ROUTEINV-P0d recursive exact set/reorder")

    require(
        transaction,
        '#[cfg(test)]\n#[path = "callable_module_transaction_p0d_tests.rs"]',
        "test-only P0d transaction registration",
    )
    for fragment in (
        "acyclic_late_draft_failure_keeps_candidate_publication_at_zero",
        "recursive_late_draft_failure_keeps_candidate_publication_at_zero",
        "atomic_publication_failure_preserves_the_preexisting_module_prefix",
        "VerifiedUnpublishedCallableDraftSetV1::collect_acyclic_with",
        "VerifiedUnpublishedCallableDraftSetV1::collect_recursive_with",
        ".current_module",
        ".functions\n        .is_empty()",
        "CallableModuleTransactionErrorV1::Publication(_)",
        'module.get_function("second/1").is_none()',
    ):
        require(p0d, fragment, "ROUTEINV-P0d late failure/publication proof")
    for fragment in (
        "ModuleDraftCollectorV1",
        "RouteOwnedInvocationInventoryV2",
        "RawExpansionReceiptLedgerV1",
        "pub(crate) struct",
        "pub(in crate::mir) struct",
    ):
        forbid(p0d, fragment, f"P0d test proof owns {fragment}")

    require(
        catalog_failure,
        "rejects_duplicate_exact_key_with_both_declaration_sites",
        "catalog failure owner",
    )
    require(
        source_failure,
        "unknown_target_rejects_before_a_resolved_module_is_published",
        "source/resolution failure owner",
    )
    require(
        acyclic,
        "rejects_one_function_zero_call_cycles_and_nontrivial_function_profiles",
        "acyclic plan failure owner",
    )
    require(
        recursive,
        "rejects_zero_call_acyclic_and_nontrivial_profiles",
        "recursive plan failure owner",
    )
    require(
        publication,
        "atomic_function_batch_preserves_existing_module_on_late_collision",
        "atomic publication failure owner",
    )

    require(card, "WIRING-I0-ROUTEINV-P0d-CALLABLE-P0 closeout", "P0d closeout")
    require(
        state,
        "BORROW-P0-CANONICAL is closed",
        "P0d downstream closed-state pointer",
    )


def verify_route_matrix_g0(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
) -> None:
    proof_path = root / "src/mir/builder/module_wiring_route_matrix_p0e.rs"
    policy_path = root / "src/mir/builder/route_owned_invocation_inventory.rs"
    matrix_path = root / "src/mir/builder/module_invocation_route_matrix.rs"
    raw_port_path = root / "src/mir/builder/recursive_child_lowering.rs"
    raw_tests_path = root / "src/mir/builder/recursive_child_lowering_rawport_tests.rs"
    raw_header_tests_path = root / "src/mir/builder/recursive_child_lowering_rawport_header_tests.rs"
    proof = proof_path.read_text()
    policy = policy_path.read_text()
    matrix = matrix_path.read_text()
    raw_port = raw_port_path.read_text()
    raw_tests = raw_tests_path.read_text()
    raw_header_tests = raw_header_tests_path.read_text()

    for path, text in (
        (proof_path, proof),
        (policy_path, policy),
        (matrix_path, matrix),
        (raw_port_path, raw_port),
        (raw_tests_path, raw_tests),
        (raw_header_tests_path, raw_header_tests),
    ):
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"ROUTEINV-P0e source/proof must remain below 800 lines: {path}")

    for fragment in (
        "InvocationRouteMatrixV1::rows()",
        "RouteOwnedInvocationInventoryV2::derive(family)",
        "RouteAuthorityLaneV1::RawLedger",
        "RouteAuthorityLaneV1::SingleOwnerHeader",
        "RouteAuthorityLaneV1::CallableBatch",
        "assert_eq!(policy_lane_counts, [4, 3, 1, 1]);",
        "assert_eq!(projected.len(), 9);",
        "assert_eq!(authority_counts, [4, 3, 2]);",
        "route_matrix_projects_all_nine_rows_to_exactly_one_existing_authority",
        "route_failure_and_publication_laws_remain_matrix_projections",
    ):
        require(proof, fragment, "ROUTEINV-P0e matrix projection")
    for fragment in (
        "struct RouteObservationV1",
        "entered: bool",
        "changed: bool",
        "entered: true,\n            changed: false",
        "entered: true,\n            changed: true",
        "entered_and_changed_observations_are_independent_dimensions",
    ):
        require(proof, fragment, "ROUTEINV-P0e independent observation")
    for fragment in (
        "InvocationRouteMatrixRowV1 {",
        "Vec<String>",
        "BTreeMap",
        "MirBuilder",
        "MirModule",
        "MirFunction",
        "ModuleDraftCollectorV1",
    ):
        forbid(proof, fragment, f"P0e proof owns {fragment}")
    require(
        builder_mod,
        "#[cfg(test)]\nmod module_wiring_route_matrix_p0e;",
        "test-only P0e registration",
    )

    require(
        matrix,
        "pub(in crate::mir::builder) const fn rows()",
        "route-matrix SSOT",
    )
    require(policy, "pub(in crate::mir::builder) fn derive(", "family policy projection")
    require(
        policy,
        "fallback: RouteFallbackPolicyV2::Forbidden",
        "route fallback prohibition",
    )

    invocation_observer = raw_port.split(
        "impl MeCallHeaderObservationPortV1 for RawInvocationChildPortV1", 1
    )[1].split("impl RawLoopChildEntryPortV1", 1)[0]
    for fragment in ("with_function_headers", "MeCallHeaderSourceV1::InvocationCollector"):
        require(invocation_observer, fragment, "collector-only invocation observer")
    for fragment in ("current_module", "MeCallHeaderSourceV1::ModuleCompatibility"):
        forbid(invocation_observer, fragment, "invocation observer compatibility fallback")

    for fragment in (
        "raw_invocation_header_miss_does_not_retry_stale_current_module",
        'let symbol = "Ghost.m/1";',
        "ModuleDraftCollectorV1::default()",
        "MeCallHeaderSourceV1::InvocationCollector",
        "MeCallParameterObservationV1::Missing",
        "prepare_me_lowered_call_v1(observation, None).is_none()",
        "assert_eq!(instructions(&builder), instructions_before);",
        "next_value_before",
    ):
        require(raw_tests, fragment, "explicit invocation header-miss fixture")

    for symbol in ("RouteAuthorityProjectionV1", "RouteObservationV1"):
        consumers = []
        for path in (root / "src/mir/builder").rglob("*.rs"):
            if path in (proof_path, root / "src/mir/builder.rs"):
                continue
            if symbol in path.read_text():
                consumers.append(str(path.relative_to(root)))
        if consumers:
            raise AssertionError(
                f"ROUTEINV-P0e test proof has production consumers: {consumers}"
            )

    require(card, "WIRING-I0-ROUTEINV-P0e-MATRIX-G0 worker decision lock", "P0e task lock")
    require(card, "WIRING-I0-ROUTEINV-P0e-MATRIX-G0 closeout", "P0e closeout")


def verify_borrow_schedule_s0(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
    state: str,
) -> None:
    source_path = root / "src/mir/builder/module_lowering_borrow_schedule.rs"
    source = source_path.read_text()
    if len(source.splitlines()) >= 800:
        raise AssertionError("BORROW-S0 schedule source must remain below 800 lines")

    for fragment in (
        "ModuleLoweringBorrowScheduleV1",
        "InvocationBorrowScheduleDomainV1",
        "InvocationBorrowRouteScopeV1",
        "InvocationBorrowSurfaceV1",
        "InvocationBorrowPhaseV1",
        "InvocationBorrowArtifactV1",
        "InvocationBorrowScheduleErrorV1",
        "child.len() != 5 || invocation.len() != 11",
        "SharedHeaderOverlapsCollectorMutation",
        "BuilderLoanAfterMainPending",
        "LiveLoanAfterDrain",
        "CollectedInvocationDrafts",
        "RootBatchCommit",
        "PostDrainFinalize",
        "ExternalCommit",
        "commit_mutations_are_infallible_after_preflight",
    ):
        require(source, fragment, "BORROW-S0 passive schedule")
    require(
        source,
        "#[derive(Debug)]\npub(in crate::mir::builder) struct ModuleLoweringBorrowScheduleV1",
        "non-Clone BORROW-S0 schedule",
    )
    for fragment in (
        "MirBuilder",
        "MirModule",
        "MirFunction",
        "ModuleDraftCollectorV1",
        "ValueId",
        "RefCell",
        "Mutex",
        "unsafe {",
        "Vec<String>",
    ):
        forbid(source, fragment, f"BORROW-S0 schedule stores or escapes {fragment}")
    forbid(
        source,
        "impl Clone for ModuleLoweringBorrowScheduleV1",
        "BORROW-S0 schedule Clone implementation",
    )

    require(
        builder_mod,
        "mod module_lowering_borrow_schedule;",
        "BORROW-S0 module registration",
    )
    consumers = []
    root_proof_path = root / "src/mir/builder/module_lowering_borrow_root_p0.rs"
    root_p0d_proof_path = root / "src/mir/builder/module_lowering_borrow_root_p0d.rs"
    for path in (root / "src/mir").rglob("*.rs"):
        if path in (
            source_path,
            root_proof_path,
            root_p0d_proof_path,
            root / "src/mir/builder.rs",
        ):
            continue
        if "ModuleLoweringBorrowScheduleV1" in path.read_text():
            consumers.append(str(path.relative_to(root)))
    if consumers:
        raise AssertionError("BORROW-S0 production consumers: " + ", ".join(consumers))

    invocation_path = root / "src/mir/builder/module_lowering_invocation.rs"
    if len(invocation_path.read_text().splitlines()) >= 800:
        raise AssertionError("module_lowering_invocation.rs reached the 800-line stop")
    require(card, "WIRING-I0-BORROW-S0 closeout", "BORROW-S0 closeout")
    require(
        state,
        "BORROW-S0 is closed",
        "BORROW-S0 downstream state pointer",
    )


def verify_borrow_raw_p0(
    root: pathlib.Path,
    card: str,
    state: str,
) -> None:
    port_path = root / "src/mir/builder/recursive_child_lowering.rs"
    proof_path = root / "src/mir/builder/module_lowering_invocation_reentrant_tests.rs"
    port = port_path.read_text()
    proof = proof_path.read_text()

    for path, source in (
        (port_path, port),
        (proof_path, proof),
    ):
        if len(source.splitlines()) >= 800:
            raise AssertionError(f"BORROW-P0-RAW source/proof reached 800 lines: {path}")

    raw_terminal = port.split(
        "impl RawBoxMethodChildPortV1 for RawInvocationChildPortV1", 1
    )[1].split("impl RawFunctionHeaderLookupPortV1", 1)[0]
    for fragment in (
        "capture_static_box_method_pending_v1(",
        "capture_normalized_instance_box_method_pending_v1(",
        ".commit_legacy_pending(pending, admission)",
        "LegacyChildDraftAdmissionV1::legacy_symbol",
    ):
        require(raw_terminal, fragment, "BORROW-P0-RAW capture/commit terminal")
    for fragment in (
        "self.complete_legacy_child(",
        ".build_static_method_draft_v1(",
        ".build_instance_method_draft_v1(",
    ):
        forbid(raw_terminal, fragment, "BORROW-P0-RAW old closure terminal")
    forbid(
        port,
        "pub(in crate::mir::builder) fn complete_legacy_child(",
        "raw invocation closure-owning facade",
    )

    for fragment in (
        "raw_capture_commit_reaches_static_instance_constructor_depth_three",
        '"Leaf.birth/0"',
        '"Leaf.run/0"',
        '"Middle.run/0"',
        '"Outer.run/0"',
        "raw_capture_commit_failure_matrix_preserves_prefix_and_reuse",
        'collect_seed(&mut invocation, "prefix/0")',
        "CanonicalFunctionSessionErrorV1::Primary(_)",
        "CanonicalFunctionSessionErrorV1::Cleanup(_)",
        "ModuleLoweringPortChildErrorV1::Admission(_)",
        "catch_unwind(AssertUnwindSafe",
        'collect_seed(&mut invocation, "after/0")',
    ):
        require(proof, fragment, "BORROW-P0-RAW recursive/failure proof")
    constructors = []
    excluded = {port_path, proof_path}
    for path in (root / "src/mir").rglob("*.rs"):
        if path in excluded or "tests" in path.name:
            continue
        if "RawInvocationChildPortV1::new(" in path.read_text():
            constructors.append(str(path.relative_to(root)))
    if constructors:
        raise AssertionError(
            "BORROW-P0-RAW production invocation-port constructors: "
            + ", ".join(constructors)
        )

    require(card, "WIRING-I0-BORROW-P0-RAW closeout", "BORROW-P0-RAW closeout")
    require(
        state,
        "BORROW-P0-RAW is closed",
        "BORROW-P0-RAW downstream state pointer",
    )


def verify_nonmain_static_method_batch(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
) -> None:
    batch = (root / "src/mir/builder/nonmain_static_box_method_batch.rs").read_text()
    constructors = (root / "src/mir/builder/instance_box_constructor_batch.rs").read_text(); instance_methods = (root / "src/mir/builder/instance_box_method_batch.rs").read_text()
    instance_lifecycle = (root / "src/mir/builder/instance_box_declaration_lifecycle.rs").read_text(); instance_metadata = (root / "src/mir/builder/instance_box_declaration_metadata.rs").read_text()
    order = (root / "src/mir/builder/declaration_order.rs").read_text()
    program = (root / "src/mir/builder/program_root_lowering.rs").read_text()
    work_plan = (root / "src/mir/builder/program_root_work_plan.rs").read_text()
    raw = (root / "src/mir/builder/raw_expression_dispatch/mod.rs").read_text(); raw_static_lifecycle = (root / "src/mir/builder/raw_expression_dispatch/nonmain_static_box_lifecycle.rs").read_text()
    if any(len(text.splitlines()) >= 800 for text in (batch, constructors, instance_methods, instance_lifecycle, instance_metadata, program, work_plan, raw, raw_static_lifecycle)):
        raise AssertionError("Box member-batch sources reached 800 lines")
    require(builder_mod, "mod nonmain_static_box_method_batch;", "method-batch module"); require(builder_mod, "mod program_root_work_plan;", "Program-root work-plan module")
    require(builder_mod, "mod instance_box_declaration_lifecycle;", "instance lifecycle module")
    require(builder_mod, "mod instance_box_declaration_metadata;", "instance metadata module")
    require(raw, "mod nonmain_static_box_lifecycle;", "raw static lifecycle module")
    for fragment in ("PreparedNonMainStaticBoxMethodBatchV1", "entries.sort_by(", "ASTNode::FunctionDeclaration", 'format!("{}.{}/{}"', "port.lower_static_box_method("):
        require(batch, fragment, "static method-batch authority")
    for fragment in ("sorted_method_entries", "compilation_context", "root_is_app_mode", "register_user_box", "emit_void", "fallback", "retry"):
        forbid(batch, fragment, "static method-batch outer authority")
    lifecycle = program.split("pub(super) struct ProgramDeferredStaticBoxLifecycleV1", 1)[1]
    lifecycle = lifecycle.split("impl MirBuilder", 1)[0]
    forbid(lifecycle, "sorted_method_entries", "Program caller-local method sorting")
    forbid(lifecycle, ".lower_static_box_method(", "Program caller-local method dispatch")
    forbid(raw, ".lower_static_box_method(", "raw caller-local static method dispatch")
    if (program + raw_static_lifecycle).count("PreparedNonMainStaticBoxMethodBatchV1::prepare(") != 2:
        raise AssertionError("static method batch must have exactly two production issuers")
    require(raw, "PreparedRawNonMainStaticBoxLifecycleV1::prepare(name, methods)", "raw static lifecycle handoff")
    for fragment in ("ActiveRawStaticBoxCompilationStateV1::begin(", ".complete_success(", ".reject("):
        require(raw_static_lifecycle, fragment, "raw non-Main static Box lifecycle")
    for fragment in ("PreparedInstanceBoxConstructorBatchV1", "entries.sort_by(", "port.lower_instance_box_method("):
        require(constructors, fragment, "instance constructor-batch authority")
    forbid(order, "sorted_constructor_entries", "retired constructor order helper")
    for fragment in ("PreparedInstanceBoxMethodBatchV1", "lower_root_with_port_v1", "lower_raw_with_port_v1"):
        require(instance_methods, fragment, "instance method batch")
    for fragment in ("register_user_box_declared_fields(", "PreparedInstanceBoxDeclarationMetadataV1", "PreparedInstanceBoxConstructorBatchV1::prepare(", "PreparedInstanceBoxMethodBatchV1::prepare("):
        forbid(program, fragment, "Program caller-local instance lifecycle")
        forbid(raw, fragment, "raw caller-local instance lifecycle")
    for fragment in ("PreparedInstanceBoxDeclarationLifecycleV1", "lower_common_prefix_v1", "register_user_box_declared_fields(", "PreparedInstanceBoxDeclarationMetadataV1::prepare(", "metadata.lower_with_builder_v1(builder)?", "PreparedInstanceBoxConstructorBatchV1::prepare(", "PreparedInstanceBoxMethodBatchV1::prepare(", "lower_root_with_port_v1", "lower_raw_with_port_v1"):
        require(instance_lifecycle, fragment, "instance declaration lifecycle")
    if (work_plan + raw).count("PreparedInstanceBoxDeclarationLifecycleV1::prepare(") != 2:
        raise AssertionError("instance declaration lifecycle must have exactly two issuers")
    if work_plan.count(".lower_root_with_port_v1(builder, callables)") != 1:
        raise AssertionError("Program must select the root lifecycle terminal once")
    if raw.count(".lower_raw_with_port_v1(self, port)?") != 1:
        raise AssertionError("raw must select the lookup-free lifecycle terminal once")
    if instance_lifecycle.count("lower_common_prefix_v1(builder, port)?") != 2:
        raise AssertionError("both lifecycle terminals must consume the common prefix")
    for fragment in ("register_user_box_declared_fields(", "PreparedInstanceBoxDeclarationMetadataV1::prepare(", "PreparedInstanceBoxConstructorBatchV1::prepare(", "PreparedInstanceBoxMethodBatchV1::prepare("):
        if instance_lifecycle.count(fragment) != 1:
            raise AssertionError(f"instance declaration common prefix drift: {fragment}")
    effect_order = ("register_user_box_declared_fields(", "self.metadata.lower_with_builder_v1(builder)?", "self.constructors.lower_with_port_v1(builder, port)?", "Ok(self.instance_methods)")
    if [instance_lifecycle.index(item) for item in effect_order] != sorted(instance_lifecycle.index(item) for item in effect_order):
        raise AssertionError("instance declaration lifecycle effect order drift")
    forbid(instance_lifecycle, "callable_declaration_catalog", "root-only catalog authority")
    for fragment in ("build_box_declaration(", "methods.clone()", "fields.to_vec()", "weak_fields.to_vec()"):
        forbid(instance_lifecycle, fragment, "retired lower-side instance metadata authority")
    for fragment in ("PreparedInstanceBoxDeclarationMetadataV1", "sorted_method_entries", "get_or_assign_type_id", "reserve_method_slot", "register_property_getter_method"):
        require(instance_metadata, fragment, "instance metadata projection")
    for fragment in ("fallback", "retry", "emit_void"):
        forbid(instance_lifecycle, fragment, "instance declaration lifecycle outer authority")
    for row in ("NONMAIN-STATIC-BOX-METHOD-BATCH-SSOT0-I0-R0", "INSTANCE-BOX-CONSTRUCTOR-BATCH-SSOT0-I0-R0", "INSTANCE-BOX-METHOD-BATCH-SSOT0-I0-R0", "INSTANCE-BOX-DECLARATION-LIFECYCLE-SSOT0-I0-R0"):
        require(card, row, "Box lifecycle row")


def verify_route_inventory_extension(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
    state: str,
) -> None:
    source_path = root / "src/mir/builder/raw_expansion_receipt_ledger.rs"
    tests_path = root / "src/mir/builder/raw_expansion_receipt_ledger_tests.rs"
    p0_path = root / "src/mir/builder/raw_expansion_receipt_ledger_p0.rs"
    source = source_path.read_text()
    tests = tests_path.read_text()
    p0 = p0_path.read_text()
    if any(len(text.splitlines()) >= 800 for text in (source, tests, p0)):
        raise AssertionError("ROUTEINV-P0b raw ledger source/proofs must remain below 800 lines")

    for fragment in (
        "RawExpansionReceiptLedgerV1",
        "RawExpansionReservationV1",
        "RawExpansionDraftRequestV1",
        "RawExpansionCompletedEventV1",
        "SealedRawExpansionReceiptLedgerV1",
        "RawConditionDispositionV1::RequiredCompatibility",
        "CollectedDraftAdmissionReceiptV1",
        "ReplacedWholePair",
        "ForeignReservation",
        "LedgerPoisoned",
        "OpenReservations",
        "MissingRootMain",
        "MissingConditionFn",
        "RawCallableMainCompatibilityDispositionV1",
        "AbortedRawExpansionReceiptLedgerV1",
        "RawExpansionCutoverStopV1",
        "MissingCallableMainCompatibility",
        "UnexpectedCallableMainCompatibility",
    ):
        require(source, fragment, "ROUTEINV-P0b raw ledger vocabulary")
    for fragment in (
        "exact_reservations_consume_receipts_and_seal_required_raw_inventory",
        "foreign_reservation_and_identity_mismatch_fail_without_retry",
        "open_or_incomplete_required_inventory_cannot_seal",
    ):
        require(tests, fragment, "ROUTEINV-P0b raw ledger fixtures")
    for fragment in (
        "every_raw_role_is_receipt_backed_and_nested_completion_precedes_outer",
        "callable_main_selected_and_not_selected_dispositions_are_exact",
        "child_abort_preserves_completed_prefix_and_consumes_seal_authority",
        "root_abort_after_completed_children_returns_only_non_sealable_evidence",
        "duplicate_legacy_symbol_replaces_final_pair_and_keeps_event_history",
        "missing_required_condition_rejects_after_root_receipt",
        "RawExpansionDraftRoleV1::TopLevelFunction",
        "RawExpansionDraftRoleV1::StaticMethod",
        "RawExpansionDraftRoleV1::InstanceMethod",
        "RawExpansionDraftRoleV1::Constructor",
        "RawExpansionDraftRoleV1::CallableMainCompatibility",
        "RawExpansionDraftRoleV1::NestedStaticMethod",
        "RawExpansionDraftRoleV1::NestedInstanceMethod",
        "RawExpansionDraftRoleV1::NestedConstructor",
    ):
        require(p0, fragment, "ROUTEINV-P0b matrix fixture")

    ledger_struct = source.split(
        "pub(in crate::mir::builder) struct RawExpansionReceiptLedgerV1", 1
    )[1].split("struct RawExpansionReceiptLedgerSealV1", 1)[0]
    sealed_struct = source.split(
        "pub(in crate::mir::builder) struct SealedRawExpansionReceiptLedgerV1", 1
    )[1].split("struct SealedRawExpansionReceiptLedgerSealV1", 1)[0]
    for product, label in ((ledger_struct, "open ledger"), (sealed_struct, "sealed ledger")):
        for fragment in (
            "MirBuilder",
            "MirModule",
            "MirFunction",
            "ModuleDraftCollector",
            "ASTNode",
            "ValueId",
            "header",
            "retry",
            "fallback",
        ):
            forbid(product, fragment, f"{label} stores {fragment}")
    aborted_impl = source.split("impl AbortedRawExpansionReceiptLedgerV1", 1)[1]
    forbid(aborted_impl, "fn seal(", "aborted ledger regains seal authority")

    for fragment, label in (
        ("mod raw_expansion_receipt_ledger;", "raw ledger registration"),
        ("mod raw_expansion_receipt_ledger_tests;", "raw ledger fixture registration"),
        ("mod raw_expansion_receipt_ledger_p0;", "raw ledger P0 registration"),
    ):
        require(builder_mod, fragment, label)
    consumers = []
    for path in (root / "src/mir/builder").rglob("*.rs"):
        if path in (
            source_path,
            tests_path,
            p0_path,
            root / "src/mir/builder.rs",
        ):
            continue
        if "RawExpansionReceiptLedgerV1" in path.read_text():
            consumers.append(str(path.relative_to(root)))
    if consumers:
        raise AssertionError("ROUTEINV-P0b production consumers: " + ", ".join(consumers))

    main_expansion = (root / "src/mir/builder/main_expansion.rs").read_text()
    legacy_main = (root / "src/mir/builder/decls.rs").read_text()
    require(
        main_expansion,
        "MainExpansionErrorV1::DuplicateMainBox",
        "duplicate Main compatibility stop source",
    )
    require(
        legacy_main,
        "lower_static_method_as_function_typed(",
        "typed callable Main failure compatibility stop source",
    )
    forbid(
        legacy_main,
        "let _ = self.lower_static_method_as_function(",
        "swallowed callable Main failure compatibility stop source",
    )

    require(card, "WIRING-I0-ROUTEINV-P0b-RAWLEDGER-P0 closeout", "raw ledger closeout")
    require(
        state,
        "P0b-RAWLEDGER-S0/P0",
        "raw ledger closed-state pointer",
    )
    verify_single_header_s0(root, builder_mod, card, state)
    verify_callable_batch_p0(root, card, state)
    verify_route_matrix_g0(root, builder_mod, card)
    verify_borrow_schedule_s0(root, builder_mod, card, state)
    verify_borrow_raw_p0(root, card, state)
    verify_nonmain_static_method_batch(root, builder_mod, card)
    verify_borrow_canonical_p0(root, card, state)
    verify_borrow_root_p0(root, builder_mod, card, state)
