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
    lowering_path = "src/mir/builder/located_legacy_lowering.rs"
    integration_path = "src/mir/builder/located_legacy_body_domain_tests.rs"
    assignment_path = "src/mir/builder/located_legacy_assignment_tests.rs"
    policy_path = "src/mir/resolved_semantics/source_path_policy.rs"
    helper_path = "tools/checks/lib/callable_result_i0_site0_r0_bodydomain0.py"

    lowering = _read(root, lowering_path)
    integration = _read(root, integration_path)
    assignment = _read(root, assignment_path)
    policy = _read(root, policy_path)

    _count(policy, "fn owns_item_segment(", 1, "body item-family authority")
    _count(
        lowering,
        "VerifiedCallableResultInactiveBodyV1",
        2,
        "body proof import and raw-delegate requirement",
    )

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
        lowering_path,
        integration_path,
        assignment_path,
        policy_path,
        helper_path,
    )
    oversized = [path for path in touched if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        _fail(f"BODYDOMAIN0 source/check files reached 800 lines: {oversized}")
    return "retained_body_domain_checks=1"
