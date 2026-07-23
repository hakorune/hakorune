#!/usr/bin/env python3
"""ROOT0-DRAIN0-PHYSICAL0 P0/G0 census guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-drain0-execution-task-2026-07-23.md"
)
NEUTRAL = ROOT / "src/mir/canonical_physical_drain.rs"
COMPILER_MANIFEST = ROOT / "src/mir/compiler/canonical_drain_manifest.rs"
COMPILER_MOD = ROOT / "src/mir/compiler/mod.rs"
COMPLETE = ROOT / "src/mir/compiler/canonical_physical_completion.rs"
FIXTURE = ROOT / "src/mir/compiler/canonical_physical_completion_p0.rs"
BRIDGE_FIXTURE = ROOT / "src/mir/compiler/canonical_bridge_fixture0_p0.rs"
MANIFEST_P0 = ROOT / "src/mir/compiler/canonical_drain_manifest_p0.rs"
BUILDER_MOD = ROOT / "src/mir/builder.rs"
PHYSICAL = ROOT / "src/mir/builder/canonical_physical_drain.rs"
COLLECTOR = ROOT / "src/mir/builder/module_draft_collector.rs"
COLLECTOR_DRAIN = ROOT / "src/mir/builder/module_draft_collector/drain.rs"
BRAND0 = ROOT / "src/mir/builder/module_invocation_brand0.rs"
SESSION_P0 = ROOT / "src/mir/builder/module_invocation_session_p0.rs"

MANIFEST = (
    TASK,
    NEUTRAL,
    COMPILER_MANIFEST,
    COMPILER_MOD,
    COMPLETE,
    FIXTURE,
    BRIDGE_FIXTURE,
    MANIFEST_P0,
    BUILDER_MOD,
    PHYSICAL,
    COLLECTOR,
    COLLECTOR_DRAIN,
    BRAND0,
    SESSION_P0,
    pathlib.Path(__file__),
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def production_rust_files() -> list[pathlib.Path]:
    return [
        path
        for path in ROOT.glob("src/**/*.rs")
        if not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
    ]


def main() -> int:
    texts = {path: path.read_text() for path in MANIFEST}
    for path, text in texts.items():
        if len(text.splitlines()) >= 800:
            raise AssertionError(f"PHYSICAL0 file must remain below 800 lines: {path}")

    task = texts[TASK]
    require(task, "PHYSICAL0-P0", "P0 proof matrix")
    require(task, "ROOT0-DRAIN0-G0", "G0 guard boundary")
    require(task, "ROOT0-DRAIN0-PHYSICAL0-P0/G0 is next", "current task frontier")

    neutral = texts[NEUTRAL]
    for fragment in (
        "CanonicalPhysicalDrainManifestV1",
        "CanonicalPhysicalSingleRowV1",
        "CanonicalPhysicalCallableRowV1",
        "ModuleInvocationBrandV1",
        "ModuleInvocationFamilyV1",
        "CanonicalInsertedDispositionV1",
    ):
        require(neutral, fragment, f"neutral physical vocabulary: {fragment}")
    for forbidden in (
        "FunctionDraftKeyV1",
        "DraftPublicationPolicyV1",
        "ModuleInvocationPolicyV1",
        "crate::mir::compiler",
    ):
        if forbidden in neutral:
            raise AssertionError(f"neutral physical product leaks authority: {forbidden}")

    require(texts[COMPILER_MOD], "mod canonical_drain_manifest;", "compiler-private source manifest")
    if "pub(in crate::mir) mod canonical_drain_manifest" in texts[COMPILER_MOD]:
        raise AssertionError("source manifest remains Builder-visible")
    require(texts[COMPILER_MANIFEST], "pub(super) fn into_physical(self)", "consuming source handoff")

    for path in (PHYSICAL, COLLECTOR_DRAIN):
        if "crate::mir::compiler" in texts[path]:
            raise AssertionError(f"Builder physical file imports compiler: {path}")
    require(texts[COLLECTOR], "mod drain;", "keyed collector child module")
    require(texts[COLLECTOR_DRAIN], "prepare_canonical_drain", "keyed collector prepare")
    require(texts[COLLECTOR_DRAIN], "ordered_keys", "manifest-order keyed extraction")
    require(texts[BRAND0], "CollectedCanonicalSinglePhysicalV1", "single physical wrapper")
    if texts[BRAND0].count("pub(in crate::mir::builder) fn into_parts(") < 2:
        raise AssertionError("both collected physical wrappers need one narrow into_parts")

    require(texts[PHYSICAL], "prepare_drain", "physical preflight terminal")
    require(texts[PHYSICAL], "PreparedCanonicalSinglePhysicalDrainV1", "single prepared product")
    require(texts[PHYSICAL], "PreparedCanonicalCallablePhysicalDrainV1", "callable prepared product")
    require(texts[PHYSICAL], "has_published_functions", "shell-empty preflight")
    require(texts[PHYSICAL], "ReceiptCollectorBrandMismatch", "receipt provenance preflight")
    require(texts[PHYSICAL], "commit_preflighted", "infallible physical commit")

    require(texts[COMPLETE], "CanonicalPhysicalCompleteInvocationV1", "complete owner")
    require(texts[COMPLETE], "fn prepare_drain(", "completion-owned prepare")
    require(texts[COMPLETE], "PreparedCanonicalDrainV1", "prepared canonical drain")
    require(texts[COMPLETE], "fn drain(self)", "one-shot drain terminal")
    require(texts[COMPLETE], "CanonicalDrainedInvocationV1", "route-specific drained product")
    require(texts[COMPLETE], "CapabilityMismatch", "capability mismatch classification")
    if texts[COMPLETE].count("pub(in crate::mir) fn prepare_drain(") != 1:
        raise AssertionError("canonical completion must expose exactly one prepare_drain definition")
    if texts[COMPLETE].count("pub(in crate::mir) fn drain(self)") != 1:
        raise AssertionError("canonical completion must expose exactly one drain definition")
    physical_prepare_calls = texts[COMPLETE].count("physical.prepare_drain(manifest)")
    if physical_prepare_calls != 2:
        raise AssertionError(
            f"canonical completion must have exactly two route physical prepare consumers: {physical_prepare_calls}"
        )
    physical_drain_calls = texts[COMPLETE].count("physical.drain()")
    if physical_drain_calls != 2:
        raise AssertionError(
            f"canonical completion must have exactly two route physical drain consumers: {physical_drain_calls}"
        )
    for path in (COMPLETE, PHYSICAL, COLLECTOR_DRAIN):
        if "retry(" in texts[path] or "prepare_again(" in texts[path]:
            raise AssertionError(f"canonical physical path exposes retry authority: {path}")

    for fixture in (
        "compiler_bridge_drains_a_plus_single_route",
        "compiler_bridge_completion_retains_single_physical_receipt",
        "physical_prepare_failure_leaves_live_builder_unchanged",
        "compiler_bridge_completion_retains_acyclic_capability_and_receipt",
        "compiler_bridge_completion_retains_recursive_capability_and_receipt",
        "capability_brand_drift_is_not_misreported_as_foreign_physical_brand",
    ):
        require(texts[FIXTURE], fixture, f"four-route drain fixture: {fixture}")
    require(texts[BRIDGE_FIXTURE], "canonical_bridge_fixture0_condition_fn_spelling_is_canonical", "canonical condition_fn spelling fixture")
    require(texts[MANIFEST_P0], "callable_manifest_projects_catalog_in_canonical_key_order", "source reorder parity fixture")
    require(texts[MANIFEST_P0], "callable_source_manifest_keeps_canonical_key_order_across_handoff", "physical reorder handoff fixture")
    require(texts[COLLECTOR_DRAIN], "keyed_prepare_rejects_index_drift_and_returns_the_collector", "index-drift rejection fixture")
    require(texts[COLLECTOR_DRAIN], "keyed_prepare_rejects_foreign_receipt_before_consuming_collector", "foreign-receipt rejection fixture")
    require(texts[PHYSICAL], "published_shell_rejects_before_collector_prepare", "published-shell rejection fixture")
    require(texts[PHYSICAL], "manifest_symbol_drift_rejects_before_shell_mutation", "physical row mismatch fixture")
    require(texts[PHYSICAL], "collector_payload_receipt_brand_mismatch_rejects_before_shell_mutation", "collector payload receipt-brand fixture")
    require(texts[PHYSICAL], "foreign_manifest_brand_rejects_before_collector_prepare", "foreign manifest brand fixture")
    require(texts[PHYSICAL], "callable_row_cardinality_rejects_missing_and_surplus_manifest_rows", "callable cardinality fixture")
    require(texts[SESSION_P0], "dropping_failed_candidate_leaves_live_builder_unchanged", "live Builder failure fixture")

    forbidden_canonical = (
        "InvocationDrainExpectationV1",
        "ConditionFnPolicyV1::Optional",
        "DrainedModuleCandidateV1",
        "current_module.functions",
    )
    for path in (PHYSICAL, COLLECTOR_DRAIN, COMPLETE):
        for fragment in forbidden_canonical:
            if fragment in texts[path]:
                raise AssertionError(f"legacy/caller-authored drain authority leaked into {path}: {fragment}")

    old_drain = ROOT / "src/mir/builder/module_invocation_drain.rs"
    old_text = old_drain.read_text()
    production_callers = [
        path.relative_to(ROOT)
        for path in production_rust_files()
        if path != old_drain and "InvocationDrainExpectationV1" in path.read_text()
    ]
    if production_callers:
        raise AssertionError(f"canonical path has old drain callers: {production_callers}")
    if "CanonicalPhysicalCompleteInvocationV1" in old_text:
        raise AssertionError("old generic drain imports canonical completion")

    print(
        "[cut0-i0-root0-drain0-physical0-guard] ok "
        "neutral=1 keyed_collect=1 prep=1 completion_drain=1 four_routes=1 "
        f"physical_prepare_calls={physical_prepare_calls} physical_drain_calls={physical_drain_calls} "
        "legacy_callers=0 compiler_imports=0 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
