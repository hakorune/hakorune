#!/usr/bin/env python3
"""Structural checks for the canonical located body-domain prerequisite."""

from __future__ import annotations

from pathlib import Path


def _fail(message: str) -> None:
    raise RuntimeError(message)


def _read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        _fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def _count(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        _fail(f"{label}: expected={expected} actual={actual}")


def check_bodydomain0(root: Path) -> str:
    located_path = "src/mir/callable_result_representation/located_legacy.rs"
    ledger_path = "src/mir/callable_result_representation/caller_ledger.rs"
    ledger_tests_path = "src/mir/callable_result_representation/tests/caller_ledger.rs"
    lowering_path = "src/mir/builder/located_legacy_lowering.rs"
    integration_path = "src/mir/builder/located_legacy_body_domain_tests.rs"
    assignment_path = "src/mir/builder/located_legacy_assignment_tests.rs"
    policy_path = "src/mir/resolved_semantics/source_path_policy.rs"
    helper_path = "tools/checks/lib/callable_result_i0_site0_r0_bodydomain0.py"

    located = _read(root, located_path)
    ledger = _read(root, ledger_path)
    ledger_tests = _read(root, ledger_tests_path)
    lowering = _read(root, lowering_path)
    integration = _read(root, integration_path)
    assignment = _read(root, assignment_path)
    policy = _read(root, policy_path)

    _count(
        located,
        "struct LegacyActivationBodyDomainPartsV1",
        1,
        "typed body-domain parts owner",
    )
    _count(
        located,
        "fn activation_body_domain_parts",
        1,
        "body-domain parts publication",
    )
    _count(ledger, "fn prove_body_domain(", 1, "body-domain ledger decision")
    _count(ledger, "fn body_domain_contains(", 1, "body-domain membership owner")
    _count(policy, "fn owns_item_segment(", 1, "body item-family authority")
    _count(ledger, "kind.owns_item_segment(item)", 1, "body item-family consumer")
    _count(ledger, "fn same_body_item_family(", 0, "duplicate body item-family owner")
    _count(
        ledger,
        ".activation_body_domain_parts()",
        1,
        "one body-domain ledger consumer",
    )
    _count(
        ledger,
        ".activation_prefix_parts()",
        2,
        "stmt and expr literal-prefix consumers",
    )
    _count(
        lowering,
        "VerifiedCallableResultInactiveBodyV1",
        2,
        "body proof import and raw-delegate requirement",
    )

    for forbidden in (
        ".child(root_segment).child(",
        "function_proof",
        "current_static_box",
        "build_if_statement",
        "build_loop",
        "retry",
        "fallback",
    ):
        if forbidden in located + ledger:
            _fail(f"BODYDOMAIN0 owns forbidden authority: {forbidden}")

    for evidence in (
        "typed_body_domains_cover_canonical_items_without_crossing_siblings",
        "body_domains_do_not_cross_siblings_condition_or_other_statements",
        "body_domain_covers_nested_descendants_of_its_direct_item",
        "empty_bodies_and_condition_rows_remain_outside_branch_domains",
        "equal_foreign_plan_location_cannot_claim_or_prove_a_prefix",
        "SourcePathSegmentV1::IfThenBody",
        "SourcePathSegmentV1::IfThen(0)",
        "SourcePathSegmentV1::IfElseBody",
        "SourcePathSegmentV1::IfElse(0)",
        "SourcePathSegmentV1::LoopBodyRoot",
        "SourcePathSegmentV1::LoopBody(0)",
    ):
        if evidence not in ledger_tests:
            _fail(f"missing BODYDOMAIN0 exact fixture evidence: {evidence}")
    for evidence in (
        "active_then_else_and_loop_body_domains_fail_before_raw_effects",
        "RowsUnderPrefix",
        "assert_eq!(effect_snapshot(&builder), before)",
        "core_next_value",
        "core_next_block",
        "assert_eq!(call_count(&builder), 0)",
        "assert_eq!(return_count(&builder), 0)",
        "LocatedLegacyLoweringErrorV1::Poisoned",
    ):
        if evidence not in integration:
            _fail(f"missing BODYDOMAIN0 integration evidence: {evidence}")
    for evidence in (
        "loop_body_assignment_path_seam_fails_closed_until_loop0",
        "child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)",
        "lower_statement(&mut builder, assignment)",
        'contains("Unexpected")',
        'contains("LoopBodyRoot")',
    ):
        if evidence not in assignment:
            _fail(f"missing direct nested Loop fail-closed evidence: {evidence}")

    touched = (
        located_path,
        ledger_path,
        ledger_tests_path,
        lowering_path,
        integration_path,
        assignment_path,
        policy_path,
        helper_path,
    )
    oversized = [path for path in touched if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        _fail(f"BODYDOMAIN0 source/check files reached 800 lines: {oversized}")
    return "body_domain_owner=1 ledger_consumer=1"
