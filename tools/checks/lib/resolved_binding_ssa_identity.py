#!/usr/bin/env python3
"""Validate behavior-neutral SSA-S2 identity/value separation."""

from __future__ import annotations

from pathlib import Path
import sys


def fail(message: str) -> None:
    raise SystemExit(f"SSA-S2 identity separation: {message}")


def require(text: str, anchor: str, owner: str) -> None:
    if anchor not in text:
        fail(f"{owner}: missing anchor {anchor!r}")


def ordered(text: str, anchors: tuple[str, ...], owner: str) -> None:
    positions = [text.find(anchor) for anchor in anchors]
    if any(position < 0 for position in positions) or positions != sorted(positions):
        fail(f"{owner}: order drifted: {anchors!r}")


def section(text: str, start: str, end: str) -> str:
    if start not in text or end not in text:
        fail(f"facade section boundary missing: {start!r} / {end!r}")
    return text.split(start, 1)[1].split(end, 1)[0]


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: resolved_binding_ssa_identity.py ROOT")
    root = Path(sys.argv[1]).resolve()
    lower = root / "src/mir/builder/resolved_lowering"
    paths = {
        "facade": lower / "identity.rs",
        "ledger": lower / "identity/ledger.rs",
        "values": lower / "identity/value_environment.rs",
        "tests": lower / "identity_separation_tests.rs",
    }
    for path in paths.values():
        if not path.is_file():
            fail(f"missing required file {path}")
    text = {name: path.read_text() for name, path in paths.items()}

    for anchor in (
        "struct ResolvedIdentityLedgerV2<'a>",
        "adoption: ResolvedIdentityAdoptionLedgerV2",
        "coverage: LoweringSourceCoverageV2",
        "retired: BTreeSet<BindingRefV1>",
        "fn adopt_declaration(",
        "fn claim_variable_use(",
        "fn claim_assignment_binding(",
        "fn verify_scope_active(",
        "fn finish(",
    ):
        require(text["ledger"], anchor, "identity ledger")
    for forbidden in ("ValueId", "BasicBlockId", "MirBuilder", "BindingSsaBuilderV1"):
        if forbidden in text["ledger"]:
            fail(f"identity ledger gained value/block/builder dependency: {forbidden}")

    for anchor in (
        "struct PreSsaValueEnvironmentV1",
        "values: BTreeMap<BindingRefV1, ValueId>",
        "fn publish(",
        "fn value(",
        "fn rebind(",
        "fn remove(",
        "fn bindings(",
    ):
        require(text["values"], anchor, "pre-SSA value owner")
    for forbidden in (
        "ASTNode",
        "SourceBindingSite",
        "SourceExprSite",
        "ScopeId",
        "RegionId",
        "RegionFlow",
        "VerifiedResolvedFunction",
        "BindingSsaBuilderV1",
    ):
        if forbidden in text["values"]:
            fail(f"pre-SSA value owner gained semantic/control dependency: {forbidden}")

    combined = text["facade"] + text["ledger"] + text["values"]
    if combined.count("BTreeMap<BindingRefV1, ValueId>") != 1:
        fail("canonical identity box must have exactly one pre-SSA value map")
    for anchor in (
        "ledger: ResolvedIdentityLedgerV2<'a>",
        "values: PreSsaValueEnvironmentV1",
        "impl BranchValueStoreV1 for ResolvedIdentityStateV1",
        "impl DefinedJoinValueStoreV1 for ResolvedIdentityStateV1",
    ):
        require(text["facade"], anchor, "compatibility facade")
    for forbidden in (
        "Option<BindingSsaBuilderV1>",
        "seed_to_ssa",
        "export_values",
        "fallback_read",
        "sync_with_ssa",
    ):
        if forbidden in combined:
            fail(f"old-map/SSA synchronization seam appeared: {forbidden}")

    publish = section(text["facade"], "fn publish_declaration(", "fn variable_value(")
    ordered(
        publish,
        (".adopt_declaration(", "self.values.publish(binding, value)?", ".mark_declaration(site)?"),
        "declaration adoption/value/coverage",
    )
    success = section(text["facade"], "fn retire_scope_success(", "fn retire_scope_error(")
    ordered(
        success,
        (
            "verify_scope_active",
            "self.values.contains",
            "self.values.remove",
            "self.ledger.retire_scope_success(&unique)",
        ),
        "scope-success preflight/value/ledger commit",
    )
    if "expect(" in success or "panic!(" in success or "debug_assert!" in success:
        fail("scope-success transaction may not panic")
    error = section(text["facade"], "fn retire_scope_error(", "fn finish(")
    ordered(
        error,
        ("self.values.remove", "self.ledger.retire_materialized"),
        "scope-error value-first retirement",
    )
    require(text["ledger"], "[canonical_coverage/finish_mismatch]", "finish contract")
    if "value_identity_mismatch" in combined:
        fail("SSA-S2 changed the finish error priority/tag")

    if text["tests"].count("#[test]") != 2:
        fail("SSA-S2 must retain two focused behavior-equivalence fixtures")
    for anchor in (
        "duplicate_scope_success_input_preserves_pre_split_behavior",
        "scope_error_cleanup_is_value_first_and_idempotent",
    ):
        require(text["tests"], anchor, "focused fixtures")

    taskboard = (
        root
        / "docs/development/current/main/investigations/"
        "mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md"
    ).read_text()
    for anchor in (
        "### SSA-S2 — identity/value separation — closed",
        "old If value path remains the sole production value owner",
        "production Binding SSA callers = 0",
        "### SSA-E0 — preserved terminal Return contract — closed",
    ):
        require(taskboard, anchor, "taskboard")

    for path in (*paths.values(), Path(__file__)):
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"source/check reached the 800-line stop boundary: {path} ({lines})")

    print("canonical_ssa_s2_identity_owner=claims-and-lifetime-only")
    print("canonical_ssa_s2_value_owner=pre-ssa-old-if-only")
    print("canonical_ssa_s2_binding_ssa_callers=0")
    print("canonical_ssa_s2_behavior_delta=0")
    print("canonical_ssa_s2_focused_fixtures=2")


if __name__ == "__main__":
    main()
