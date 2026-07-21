#!/usr/bin/env python3
"""Reusable structural guard for MODULEDRAFT0-HEADERPORT0.

P0 pins the disconnected capability: the invocation is external, its header
port is read-only, and the old direct-reader inventory is explicit. I0/G0 will
extend this guard with the inverse reader assertions after the atomic cutover.
"""

from __future__ import annotations

import pathlib
import sys
from collections import Counter


ROOT = pathlib.Path(__file__).resolve().parents[3]
BUILDER = ROOT / "src/mir/builder.rs"
COMPILATION = ROOT / "src/mir/builder/compilation_context.rs"
INVOCATION = ROOT / "src/mir/builder/module_lowering_invocation.rs"
MODULE_DRAFT = ROOT / "src/mir/builder/module_draft_collector.rs"
SIGNATURE_LOOKUP = ROOT / "src/mir/builder/function_signature_lookup.rs"
PORT_AWARE_DRAFT = ROOT / "src/mir/builder/port_aware_function_draft.rs"
MODULE_SHELL = ROOT / "src/mir/builder/module_lowering_shell.rs"
INVOCATION_DRAIN = ROOT / "src/mir/builder/module_invocation_drain.rs"
ROUTE_MATRIX = ROOT / "src/mir/builder/module_invocation_route_matrix.rs"
INVOCATION_STATE = ROOT / "src/mir/builder/module_lowering_invocation_state.rs"
PENDING_TERMINAL = ROOT / "src/mir/builder/calls/function_session/terminal.rs"
LEGACYTERM_TESTS = ROOT / "src/mir/builder/module_lowering_invocation_legacyterm_tests.rs"
RAWPORT_TESTS = ROOT / "src/mir/builder/recursive_child_lowering_rawport_tests.rs"
REENTRANT_TESTS = ROOT / "src/mir/builder/module_lowering_invocation_reentrant_tests.rs"
RAW_DISPATCH = ROOT / "src/mir/builder/raw_expression_dispatch.rs"
RAW_PORT = ROOT / "src/mir/builder/recursive_child_lowering.rs"
RAW_LOOP_ENTRY = ROOT / "src/mir/builder/raw_loop_child_entry.rs"
LOOP_PLAN = ROOT / "src/mir/builder/control_flow/plan"
SOURCE_CENSUS_DOC = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-source-integration-consultation-2026-07-21.md"
)
STATE_CUTOVER_DOC = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md"
)

P0_DIRECT_HEADER_READER_FRAGMENTS = {
    "src/mir/builder/calls/annotation.rs": "module.functions.get(name)",
    "src/mir/builder/calls/lowering.rs": "self.current_module.take()",
    "src/mir/builder/method_call_handlers.rs": "module.functions.get(&fname)",
    "src/mir/builder/rewrite/known.rs": "module.functions.get(fname)",
    "src/mir/builder/builder_method_index.rs": "module.functions.keys()",
    "src/mir/builder/calls/static_resolution.rs": "module\n                    .functions\n                    .keys()",
    "src/mir/builder/calls/materializer.rs": "module.functions.contains_key(name)",
    "src/mir/builder/builder_build.rs": "module.functions.contains_key(&lowered)",
}

# I0-SHELL-P0 owns the complete production reader census.  The source anchor
# is checked against Rust and the future owner phrase is checked against the
# consultation card, so a hand-entered row cannot survive source drift.
P0_READER_CENSUS_ROWS = {
    "src/mir/builder/calls/annotation.rs": (
        "header",
        "module.functions.get(name)",
        "LoweringHeaderPortV1",
    ),
    "src/mir/builder/calls/lowering.rs": (
        "header",
        "self.current_module.take()",
        "LoweringHeaderPortV1",
    ),
    "src/mir/builder/method_call_handlers.rs": (
        "header",
        "module.functions.get(&fname)",
        "LoweringHeaderPortV1",
    ),
    "src/mir/builder/rewrite/known.rs": (
        "header",
        "module.functions.get(fname)",
        "collector header view",
    ),
    "src/mir/builder/builder_method_index.rs": (
        "header",
        "module.functions.keys()",
        "collector inventory projection",
    ),
    "src/mir/builder/calls/static_resolution.rs": (
        "header",
        "module\n                    .functions\n                    .keys()",
        "collector header inventory",
    ),
    "src/mir/builder/calls/materializer.rs": (
        "header",
        "module.functions.contains_key(name)",
        "collector header presence",
    ),
    "src/mir/builder/builder_build.rs": (
        "header",
        "module.functions.contains_key(&lowered)",
        "collector header presence",
    ),
    "src/mir/builder/builder_metadata.rs": (
        "shell_metadata",
        "intern_closure_body(body)",
        "ModuleLoweringShellPortV1",
    ),
    "src/mir/builder/indexing.rs": (
        "shell_metadata",
        "module.metadata.static_data_plans",
        "ModuleLoweringShellPortV1",
    ),
    "src/mir/builder/module_lifecycle.rs": (
        "lifecycle",
        "self.current_module = Some(module)",
        "shell + one collector drain",
    ),
    "src/mir/builder/calls/function_session.rs": (
        "lifecycle",
        "pub(super) fn publish_function_draft",
        "ModuleLoweringPortV1",
    ),
    "src/mir/builder/resolved_lowering/mod.rs": (
        "canonical",
        "self.current_module\n            .as_mut()",
        "prepared shell/collector admission",
    ),
    "src/mir/builder/resolved_lowering/callable_module_transaction.rs": (
        "canonical",
        "try_add_functions_atomic",
        "common collector adapter",
    ),
}

# STATE0-P0 assigns every source-census row to one future owner.  The boolean
# is deliberately explicit: no lowering-time reader may require a completed
# function body while this state seam is being introduced.
STATE0_READER_OWNER_ROWS = {
    relative: (
        "collector_header" if family == "header" else
        "shell_port" if family == "shell_metadata" else
        "invocation_lifecycle" if family == "lifecycle" else
        "canonical_catalog_adapter",
        False,
    )
    for relative, (family, _source_anchor, _future_owner) in P0_READER_CENSUS_ROWS.items()
}


def read(path: pathlib.Path) -> str:
    try:
        return path.read_text()
    except OSError as error:
        raise AssertionError(f"cannot read {path}: {error}") from error


def require(text: str, fragment: str, subject: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {subject}: {fragment!r}")


def forbid(text: str, fragment: str, subject: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {subject}: {fragment!r}")


def require_count(text: str, fragment: str, expected: int, subject: str) -> None:
    actual = text.count(fragment)
    if actual != expected:
        raise AssertionError(
            f"wrong {subject} count: fragment={fragment!r} expected={expected} actual={actual}"
        )


def main() -> int:
    invocation = read(INVOCATION)
    module_draft = read(MODULE_DRAFT)
    signature_lookup = read(SIGNATURE_LOOKUP)
    port_aware_draft = read(PORT_AWARE_DRAFT)
    module_shell = read(MODULE_SHELL)
    invocation_drain = read(INVOCATION_DRAIN)
    route_matrix = read(ROUTE_MATRIX)
    invocation_state = read(INVOCATION_STATE)
    builder = read(BUILDER)
    compilation = read(COMPILATION)
    pending_terminal = read(PENDING_TERMINAL)
    legacyterm_tests = read(LEGACYTERM_TESTS)
    rawport_tests = read(RAWPORT_TESTS)
    reentrant_tests = read(REENTRANT_TESTS)
    raw_dispatch = read(RAW_DISPATCH)
    raw_port = read(RAW_PORT)
    raw_loop_entry = read(RAW_LOOP_ENTRY)
    consultation = read(SOURCE_CENSUS_DOC)
    state_consultation = read(STATE_CUTOVER_DOC)

    for fragment in (
        "ModuleLoweringShellV1",
        "ModuleLoweringShellPortV1",
        "ModuleLoweringShellDrainInventoryV1",
        "PreparedModuleLoweringShellDrainV1",
        "from_empty_module",
        "with_port",
        "prepare_drain",
        "FunctionMapNotEmpty",
        "DuplicateFunction",
        "DuplicateInventorySymbol",
        "InventoryMismatch",
    ):
        require(module_shell, fragment, "HEADERPORT0 I0-SHELL-S0 vocabulary")
    for fragment in (
        "ConditionFnPolicyV1",
        "InvocationDrainExpectationV1",
        "ModuleLoweringInvocationDrainOwnerV1",
        "PreparedInvocationDrainV1",
        "InventoryMismatch",
        "commit_preflighted",
        "fn drain(self)",
    ):
        require(invocation_drain, fragment, "HEADERPORT0 I0-SHELL-I0-S0 vocabulary")
    for fragment in (
        "drain_preflights_complete_inventory_and_required_roots",
        "drain_rejects_missing_main_before_consuming_the_candidate",
        "drain_rejects_inventory_mismatch_before_any_shell_mutation",
    ):
        require(invocation_drain, fragment, "HEADERPORT0 I0-SHELL-I0-S0 fixture")
    for fragment in (
        "InvocationRootFamilyV1",
        "InvocationEntryV1",
        "InvocationIdentityV1",
        "InvocationFailureStageV1",
        "InvocationFailureLawV1",
        "InvocationRouteMatrixV1",
        "p0_route_matrix_covers_every_root_and_child_family",
        "p0_failure_matrix_forbids_retry_and_partial_publication",
        "raw_main_root",
        "canonical_a_plus_root",
        "binding_ssa_acyclic_module",
        "binding_ssa_recursive_module",
    ):
        require(route_matrix, fragment, "HEADERPORT0 I0-SHELL-I0-P0 route matrix")
    for fragment in (
        "RootCompletionStateV1",
        "ModuleLoweringInvocationStateV1",
        "shell: ModuleLoweringShellV1",
        "collector: ModuleDraftCollectorV1",
        "root: RootCompletionStateV1",
        "state_owns_empty_shell_and_collector_without_exposing_function_map",
        "state_parts_are_consumed_together_at_the_drain_boundary",
    ):
        require(invocation_state, fragment, "HEADERPORT0 I0-STATE0-S0 vocabulary")
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (
            INVOCATION_DRAIN,
            INVOCATION,
            MODULE_SHELL,
            ROUTE_MATRIX,
            INVOCATION_STATE,
            MODULE_DRAFT,
        ) or path.name.endswith("_tests.rs"):
            continue
        forbid(
            read(path),
            "PreparedInvocationDrainV1",
            f"HEADERPORT0 I0-SHELL-I0-S0 disconnected drain consumer {path.relative_to(ROOT)}",
        )
        forbid(
            read(path),
            "InvocationRouteMatrixV1",
            f"HEADERPORT0 I0-SHELL-I0-P0 disconnected matrix consumer {path.relative_to(ROOT)}",
        )
        forbid(
            read(path),
            "ModuleLoweringInvocationStateV1",
            f"HEADERPORT0 I0-STATE0-S0 disconnected state consumer {path.relative_to(ROOT)}",
        )
    state_consumers = []
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (INVOCATION_STATE,) or path.name.endswith("_tests.rs"):
            continue
        if "ModuleLoweringInvocationStateV1" in read(path):
            state_consumers.append(path)
    expected_state_consumers = {INVOCATION, INVOCATION_DRAIN}
    if set(state_consumers) != expected_state_consumers:
        actual = sorted(str(path.relative_to(ROOT)) for path in state_consumers)
        raise AssertionError(
            f"STATE0-I0 state consumers drifted: expected two owners, actual={actual}"
        )
    for fragment in (
        "shell_metadata_port_is_the_only_narrow_metadata_write_surface",
        "shell_drain_inventory_rejects_duplicate_symbols_before_commit",
        "shell_drain_rejects_inventory_function_mismatch_before_commit",
    ):
        require(module_shell, fragment, "HEADERPORT0 I0-SHELL-P0 fixture")
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (MODULE_SHELL, REENTRANT_TESTS) or path.name.endswith("_tests.rs"):
            continue
        forbid(
            read(path),
            "prepare_drain()",
            f"HEADERPORT0 I0-SHELL-S0 disconnected drain consumer {path.relative_to(ROOT)}",
        )

    for fragment in (
        "PreparedCollectorReplacementV1",
        "IndexDrift",
        "replacement: PreparedCollectorReplacementV1",
    ):
        require(module_draft, fragment, "HEADERPORT0 collector preflight vocabulary")
    forbid(module_draft, "remove_existing_symbol", "HEADERPORT0 post-collect lookup")
    forbid(module_draft, "remove_existing_key", "HEADERPORT0 post-collect lookup")
    collect_block = module_draft.split("fn collect_sealed", 1)[1].split(
        "impl CompletedDraftSignatureViewV1", 1
    )[0]
    forbid(collect_block, "debug_assert!", "HEADERPORT0 post-collect assertion")
    forbid(collect_block, "expect(", "HEADERPORT0 post-collect fallible lookup")

    for fragment in (
        "struct LoweringHeaderPortV1",
        "struct ModuleLoweringPortV1",
        "struct ModuleLoweringInvocationV1",
        "collector: ModuleDraftCollectorV1",
        "builder: &'builder mut MirBuilder",
        "fn with_header_port",
        "fn with_module_port",
        "fn prepare_draft_admission",
    ):
        require(invocation, fragment, "HEADERPORT0 invocation vocabulary")

    header_impl = invocation.split("impl LoweringHeaderPortV1", 1)[1].split(
        "/// Stack-owned capability", 1
    )[0]
    for fragment in (
        "signature",
        "contains_symbol",
        "symbol_count",
        "visit_symbols",
    ):
        require(header_impl, fragment, "HEADERPORT0 read capability")
    for fragment in ("prepare", "collect", "MirFunction", "current_module"):
        forbid(header_impl, fragment, "HEADERPORT0 read capability")
    require(invocation, "for<'header>", "HEADERPORT0 non-escaping read borrow")
    require(invocation, "for<'port>", "RAWPORT0 non-escaping stack port")

    module_port_decl = invocation.split("struct ModuleLoweringPortV1", 1)[1].split(
        "impl ModuleLoweringPortV1", 1
    )[0]
    module_port_impl = invocation.split("impl ModuleLoweringPortV1", 1)[1].split(
        "/// The external owner", 1
    )[0]
    require(
        module_port_decl,
        "collector: &'collector mut ModuleDraftCollectorV1",
        "RAWPORT0 stack-owned capability",
    )
    for fragment in (
        "with_headers",
        "prepare_draft_admission",
        "complete_resolved_child",
        "complete_legacy_child",
        "commit_resolved_pending",
        "commit_legacy_pending",
        "capture_resolved_pending",
        "capture_legacy_pending",
    ):
        require(module_port_impl, fragment, "RAWPORT0 stack-owned capability")
    for fragment in ("MirBuilder", "current_module", "thread_local", "OnceLock"):
        forbid(module_port_decl, fragment, "RAWPORT0 stack-owned capability field")

    pending_decl = pending_terminal.split("struct PendingFunctionSessionCloseV1", 1)[1].split(
        "impl<'builder> CanonicalFunctionLoweringSessionV1", 1
    )[0]
    for fragment in ("session: CanonicalFunctionLoweringSessionV1", "draft: Option<MirFunction>"):
        require(pending_decl, fragment, "RAWPORT0 pending terminal")
    for fragment in ("ModuleDraftCollectorV1", "MirBuilder", "current_module"):
        forbid(pending_decl, fragment, "RAWPORT0 pending terminal")
    forbid(pending_terminal, "derive(Clone)", "RAWPORT0 pending terminal")
    require(pending_terminal, "complete_before_restore", "RAWPORT0 port-owned terminal")
    require(
        invocation,
        "pending: PendingFunctionSessionCloseV1<'_>",
        "HEADERPORT0 resolved commit-only terminal",
    )
    require(
        invocation,
        "pending: LegacyFunctionPendingSessionV1<'_>",
        "HEADERPORT0 legacy commit-only terminal",
    )
    require(
        pending_terminal,
        "capture_resolved_function_pending_session_v1",
        "RAWPORT0 resolved pending seam",
    )
    require(
        pending_terminal,
        "struct LegacyFunctionPendingSessionV1",
        "RAWPORT0 legacy pending seam",
    )
    for fragment in (
        "pending_capture_ends_before_header_loan_and_commit",
        "rejected_commit_restores_parent_without_collector_delta",
        "capture_failure_never_reaches_commit_terminal",
        "port_aware_static_body_collects_nested_static_child_before_outer_commit",
        "port_aware_static_body_collects_nested_instance_child_before_outer_commit",
        "port_aware_nested_instance_constructor_uses_the_same_child_terminal",
        "invocation_main_box_is_rejected_before_root_effects",
        "port_aware_capture_failure_restores_parent_without_collection",
    ):
        require(reentrant_tests, fragment, "HEADERPORT0 reentrant P0 proof")
    require(
        pending_terminal,
        "capture_legacy_function_pending_session_v1",
        "RAWPORT0 legacy capture seam",
    )
    forbid(pending_terminal, "PreparedFunctionDraftAdmissionV1", "RAWPORT0 pending terminal")

    require(
        signature_lookup,
        "trait FunctionSignatureLookupV1",
        "HEADERPORT0 neutral signature lookup",
    )
    require(
        port_aware_draft,
        "PortAwareFunctionBodyRequestV1",
        "HEADERPORT0 port-aware body request",
    )
    require(
        port_aware_draft,
        "PortAwareFinalizerRequestV1",
        "HEADERPORT0 port-aware finalizer request",
    )
    require(
        port_aware_draft,
        "trait PortAwareFunctionDraftSurfaceV1",
        "HEADERPORT0 port-aware draft surface",
    )
    for fragment in (
        "build_static_method_draft_with_port_v1",
        "build_instance_method_draft_with_port_v1",
        "lower_function_body_with_port_v1",
        "lower_method_body_with_port_v1",
        "finalize_function_draft_with_headers",
    ):
        require(port_aware_draft, fragment, "HEADERPORT0 port-aware draft surface")
    forbid(
        port_aware_draft,
        "MirBuilder",
        "HEADERPORT0 disconnected port-aware draft vocabulary",
    )
    forbid(
        port_aware_draft,
        "ModuleDraftCollectorV1",
        "HEADERPORT0 disconnected port-aware draft vocabulary",
    )
    forbid(
        port_aware_draft,
        "ValueId",
        "HEADERPORT0 disconnected port-aware draft vocabulary",
    )
    require_count(
        invocation,
        "pub(in crate::mir::builder) fn commit_resolved_pending(",
        1,
        "HEADERPORT0 resolved commit-only owner",
    )
    require_count(
        invocation,
        "pub(in crate::mir::builder) fn commit_legacy_pending(",
        1,
        "HEADERPORT0 legacy commit-only owner",
    )
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path == INVOCATION or path.name == "module_lowering_invocation_reentrant_tests.rs":
            continue
        source = read(path)
        forbid(
            source,
            "commit_resolved_pending(",
            f"HEADERPORT0 S0 disconnected resolved commit consumer {path.relative_to(ROOT)}",
        )
        forbid(
            source,
            "commit_legacy_pending(",
            f"HEADERPORT0 S0 disconnected legacy commit consumer {path.relative_to(ROOT)}",
        )

    legacy_admission = invocation.split("struct LegacyChildDraftAdmissionV1", 1)[1].split(
        "/// Failure while", 1
    )[0]
    for fragment in ("symbol: String", "arity: usize", "legacy_symbol"):
        require(legacy_admission, fragment, "RAWPORT0 legacy admission")
    for fragment in ("Clone", "CanonicalResolvedOwner", "ModuleDraftCollectorV1", "MirBuilder"):
        forbid(legacy_admission, fragment, "RAWPORT0 legacy admission")
    require(
        module_port_impl,
        "DraftPublicationPolicyV1::LegacyReplaceWholePair",
        "RAWPORT0 legacy whole-pair policy",
    )
    for relative in (
        "src/mir/builder/raw_expression_dispatch.rs",
        "src/mir/builder/calls/function_session.rs",
    ):
        forbid(
            read(ROOT / relative),
            "complete_legacy_child(",
            f"RAWPORT0 S0 production caller {relative}",
        )

    for fragment in (
        "legacy_child_primary_and_during_cleanup_restore_without_collection",
        "legacy_child_success_cleanup_failure_restores_without_collection",
        "legacy_child_unwind_restores_without_collection",
        "legacy_child_port_receives_exact_static_and_instance_box_bodies",
        "NyashParser::parse_from_string",
        "complete_legacy_child",
    ):
        require(legacyterm_tests, fragment, "LEGACYTERM0 P0 proof")
    forbid(
        legacyterm_tests,
        "drive_legacy_expression_v1",
        "LEGACYTERM0 P0 duplicate raw dispatcher",
    )

    for fragment in (
        "raw_invocation_port_collects_static_and_instance_box_methods",
        "static box RawStatic",
        "box RawInstance",
    ):
        require(rawport_tests, fragment, "LEGACYTERM0 I0 raw Box proof")
    for fragment in (
        "port.lower_static_main_box",
        "port.lower_static_box_method",
        "port.lower_instance_box_method",
    ):
        require(raw_dispatch, fragment, "LEGACYTERM0 I0 dispatcher terminal")
    forbid(
        raw_dispatch,
        "self.build_static_main_box(",
        "HEADERPORT0 P0 direct Main root bypass",
    )
    require(
        raw_dispatch,
        "for (ctor_key, ctor_ast)",
        "HEADERPORT0 P0 constructor traversal",
    )
    forbid(
        raw_dispatch,
        "self.lower_method_as_function(",
        "HEADERPORT0 P0 constructor direct bypass",
    )
    require(
        raw_port,
        "impl RawBoxMethodChildPortV1 for RawInvocationChildPortV1",
        "LEGACYTERM0 I0 invocation terminal owner",
    )
    require(
        raw_port,
        "LegacyChildDraftAdmissionV1::legacy_symbol",
        "LEGACYTERM0 I0 legacy identity",
    )
    for fragment in (
        "capture_static_box_method_pending_v1",
        "capture_instance_box_method_pending_v1",
        "PortAwarePreparedDraftBodyV1",
        "finalize_function_draft_with_headers",
    ):
        require(raw_port, fragment, "HEADERPORT0 P0 port-aware child capture")
    require_count(
        invocation,
        "pub(in crate::mir::builder) fn complete_legacy_child(",
        1,
        "LEGACYTERM0 collector terminal owner",
    )
    require_count(
        raw_dispatch,
        "port.lower_static_main_box(",
        1,
        "HEADERPORT0 P0 Main root boundary",
    )
    require_count(
        raw_dispatch,
        "port.lower_static_box_method(",
        1,
        "LEGACYTERM0 static raw dispatch",
    )
    require_count(
        raw_dispatch,
        "port.lower_instance_box_method(",
        2,
        "LEGACYTERM0 instance raw dispatch",
    )
    require_count(
        raw_dispatch,
        "port.lower_loop(self, *condition, body)?",
        1,
        "LOOPBRIDGE0 I0 raw Loop boundary",
    )
    forbid(
        raw_dispatch,
        "self.cf_loop(*condition, body)?",
        "LOOPBRIDGE0 I0 raw Loop direct bypass",
    )
    for fragment in (
        "enum RawLoopChildEntryDispositionV1",
        "ASTNode::BoxDeclaration { .. } => true",
        "ASTNode::Lambda { .. } | ASTNode::FunctionDeclaration { .. } => false",
        "node.any_child(contains_reachable_box_declaration)",
    ):
        require(raw_loop_entry, fragment, "LOOPBRIDGE0 I0 syntax quarantine")
    for fragment in (
        "trait RawLoopChildEntryPortV1",
        "impl RawLoopChildEntryPortV1 for RawLegacyChildLoweringPortV1",
        "impl RawLoopChildEntryPortV1 for RawInvocationChildPortV1",
        "classify_raw_loop_child_entry_v1(&condition, &body)",
        "RawLoopChildEntryDispositionV1::ReachableBoxDeclaration",
        "raw_loop_child_entry: reachable BoxDeclaration requires a pure-plan/function-session bridge",
    ):
        require(raw_port, fragment, "LOOPBRIDGE0 I0 raw Loop boundary")
    require_count(
        raw_port,
        "trait RawLoopChildEntryPortV1",
        1,
        "LOOPBRIDGE0 G0 raw Loop boundary authority",
    )
    require_count(
        raw_port,
        "impl RawLoopChildEntryPortV1 for",
        2,
        "LOOPBRIDGE0 I0 raw Loop port implementations",
    )

    for path in (ROOT / "src/mir/builder/control_flow").rglob("*.rs"):
        forbid(read(path), "ModuleLoweringPortV1", "LEGACYTERM0 Loop bridge")
        forbid(read(path), "RawInvocationChildPortV1", "LOOPBRIDGE0 Loop bridge")
    for path in LOOP_PLAN.rglob("*.rs"):
        plan_source = read(path)
        forbid(plan_source, "ASTNode::BoxDeclaration", "LOOPBRIDGE0 plan child opener")
        forbid(plan_source, "lower_static_method_as_function", "LOOPBRIDGE0 plan child opener")
        forbid(plan_source, "lower_method_as_function", "LOOPBRIDGE0 plan child opener")

    for fragment in ("thread_local", "OnceLock", "static ", "derive(Clone)"):
        forbid(invocation, fragment, "HEADERPORT0 external invocation")
    for fragment in ("LoweringHeaderPortV1", "ModuleDraftCollectorV1"):
        forbid(builder.split("pub struct MirBuilder", 1)[1].split("impl Default", 1)[0], fragment, "MirBuilder field")
        forbid(compilation, fragment, "CompilationContext field")

    for relative, fragment in P0_DIRECT_HEADER_READER_FRAGMENTS.items():
        require(read(ROOT / relative), fragment, f"P0 direct header reader {relative}")

    census_counts = Counter()
    state_owner_counts = Counter()
    for relative, (family, source_anchor, future_owner) in P0_READER_CENSUS_ROWS.items():
        source = read(ROOT / relative)
        require(source, source_anchor, f"I0-SHELL-P0 source census anchor {relative}")
        doc_path = relative.removeprefix("src/mir/builder/")
        require(
            consultation,
            f"`{doc_path}`",
            f"I0-SHELL-P0 consultation row {relative}",
        )
        require(
            consultation,
            future_owner,
            f"I0-SHELL-P0 future owner {relative}",
        )
        census_counts[family] += 1
        owner, completed_body_required = STATE0_READER_OWNER_ROWS[relative]
        if completed_body_required:
            raise AssertionError(
                f"STATE0-P0 reader requires completed body: {relative}"
            )
        if owner not in {
            "collector_header",
            "shell_port",
            "invocation_lifecycle",
            "canonical_catalog_adapter",
        }:
            raise AssertionError(f"STATE0-P0 unknown reader owner: {owner}")
        require(
            state_consultation,
            f"`{owner}`",
            f"STATE0-P0 owner row {relative}",
        )
        state_owner_counts[owner] += 1

    if sum(census_counts.values()) != len(P0_READER_CENSUS_ROWS):
        raise AssertionError("I0-SHELL-P0 census family accounting drifted")
    if set(STATE0_READER_OWNER_ROWS) != set(P0_READER_CENSUS_ROWS):
        raise AssertionError("STATE0-P0 owner census does not cover all reader rows")

    print(
        "[module-draft-headerport-guard] ok "
        f"p0_reader_families={len(P0_DIRECT_HEADER_READER_FRAGMENTS)} "
        f"p0_census_rows={len(P0_READER_CENSUS_ROWS)} "
        f"census={dict(sorted(census_counts.items()))} "
        f"state0_owners={dict(sorted(state_owner_counts.items()))} "
        f"state0_consumers={len(state_consumers)} "
        "legacyterm0_g0_raw_box_consumers=2 loop_bridge_consumers=0 "
        "loopbridge0_i0_plan_child_openers=0"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"[module-draft-headerport-guard] FAIL: {error}", file=sys.stderr)
        raise SystemExit(1)
