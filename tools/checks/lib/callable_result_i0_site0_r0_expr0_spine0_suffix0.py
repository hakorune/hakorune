#!/usr/bin/env python3
"""Structural checks for the disconnected exact body-suffix classifier."""

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


def _production_callers(root: Path, needle: str) -> int:
    total = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if path.name.endswith("_tests.rs") or "/tests/" in relative:
            continue
        total += path.read_text(encoding="utf-8").count(needle)
    return total


def _production_occurrences(root: Path, needle: str) -> int:
    total = 0
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if path.name.endswith("_tests.rs") or "/tests/" in relative:
            continue
        total += path.read_text(encoding="utf-8").count(needle)
    return total


def _located_suffix_constructor_consumers(root: Path) -> list[str]:
    consumers = []
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if path.name.endswith("_tests.rs") or "/tests/" in relative:
            continue
        text = path.read_text(encoding="utf-8")
        if (
            ".body_suffix(" in text
            and "VerifiedCallableResultLegacySourceViewV1" in text
            and not relative.endswith("callable_result_representation/located_legacy.rs")
        ):
            consumers.append(relative)
    return consumers


def check_suffix0_s0(root: Path) -> str:
    policy_path = "src/mir/resolved_semantics/source_path_policy.rs"
    location_path = "src/mir/callable_result_representation/located_legacy.rs"
    error_path = "src/mir/callable_result_representation/located_legacy_error.rs"
    ledger_path = "src/mir/callable_result_representation/caller_ledger.rs"
    tests_path = (
        "src/mir/callable_result_representation/tests/caller_ledger_suffix.rs"
    )
    readme_path = "src/mir/callable_result_representation/README.md"
    block_driver_path = "src/mir/builder/stmts/block_driver.rs"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_suffix0.py"
    )

    policy = _read(root, policy_path)
    location = _read(root, location_path)
    errors = _read(root, error_path)
    ledger = _read(root, ledger_path)
    tests = _read(root, tests_path)
    readme = _read(root, readme_path)
    block_driver = _read(root, block_driver_path)

    _count(policy, "pub(crate) fn owned_item_index", 1, "body item-index owner")
    _count(
        policy,
        "self.owned_item_index(segment).is_some()",
        1,
        "owns-item delegation",
    )
    _count(location, "struct LocatedLegacyBodySuffixV1", 1, "suffix carrier")
    _count(
        location,
        "#[derive(Debug)]\npub(crate) struct LocatedLegacyBodySuffixV1",
        1,
        "non-Clone suffix carrier declaration",
    )
    _count(location, "pub(crate) fn body_suffix(", 1, "suffix constructor")
    _count(location, "fn into_activation_parts", 1, "consuming suffix projection")
    _count(ledger, "struct VerifiedCallableResultInactiveBodySuffixV1", 1, "inactive proof")
    _count(
        ledger,
        "#[derive(Debug)]\npub(crate) struct VerifiedCallableResultInactiveBodySuffixV1",
        1,
        "non-Clone inactive proof declaration",
    )
    _count(ledger, "enum CallableResultBodySuffixDecisionV1", 1, "suffix decision")
    _count(ledger, "pub(crate) fn classify_body_suffix(", 1, "suffix classifier")
    _count(ledger, "fn body_suffix_contains(", 1, "suffix membership owner")
    _count(
        ledger,
        "impl AsRef<[ASTNode]> for VerifiedCallableResultInactiveBodySuffixV1",
        1,
        "verified suffix view",
    )
    _count(
        location,
        "impl AsRef<[ASTNode]> for LocatedLegacyBodySuffixV1",
        0,
        "unverified raw suffix view",
    )
    carrier = location[
        location.index("pub(crate) struct LocatedLegacyBodySuffixV1") : location.index(
            "impl<'plan> LegacyBodyInputV1", location.index("pub(crate) struct LocatedLegacyBodySuffixV1")
        )
    ]
    proof = ledger[
        ledger.index("pub(crate) struct VerifiedCallableResultInactiveBodySuffixV1") : ledger.index(
            "impl AsRef<[ASTNode]>",
            ledger.index("pub(crate) struct VerifiedCallableResultInactiveBodySuffixV1"),
        )
    ]
    for label, product in (("suffix carrier", carrier), ("inactive proof", proof)):
        if "statements: &'plan [ASTNode]" not in product:
            _fail(f"{label} lost exact borrowed statements")
        if "Clone" in product:
            _fail(f"{label} became Clone")
    _count(
        location,
        "impl Clone for LocatedLegacyBodySuffixV1",
        0,
        "suffix carrier Clone impl",
    )
    _count(
        ledger,
        "impl Clone for VerifiedCallableResultInactiveBodySuffixV1",
        0,
        "inactive proof Clone impl",
    )

    for variant in (
        "BodySuffixIndexOverflow",
        "BodySuffixLengthOverflow",
        "BodySuffixStartOutOfBounds",
    ):
        if variant not in errors:
            _fail(f"missing typed suffix location error: {variant}")

    classifier = ledger[
        ledger.index("pub(crate) fn classify_body_suffix(") : ledger.index(
            "pub(crate) fn prove_stmt_inactive(",
            ledger.index("pub(crate) fn classify_body_suffix("),
        )
    ]
    for forbidden in (
        ".claim(",
        "claimed.insert",
        "RowsUnderPrefix",
        "MirBuilder",
        "SourcePathV1",
        "ASTNode::",
        "retry",
        "fallback",
    ):
        if forbidden in classifier:
            _fail(f"suffix classifier owns forbidden authority: {forbidden}")

    membership = ledger[
        ledger.index("fn body_suffix_contains(") : ledger.index(
            "fn body_root_diagnostic_site(",
            ledger.index("fn body_suffix_contains("),
        )
    ]
    for forbidden in ("SourcePathV1", "ASTNode", "root_segment", "item_segment("):
        if forbidden in membership:
            _fail(f"suffix membership reconstructs source authority: {forbidden}")
    _count(
        membership,
        "kind.owned_item_index(item)",
        1,
        "suffix membership item-index authority",
    )
    _count(membership, "index >= start", 1, "suffix membership start boundary")

    if _production_callers(root, ".classify_body_suffix(") != 0:
        _fail("SUFFIX0-S0 classifier has a production consumer")
    if _production_occurrences(root, "classify_body_suffix(") != 1:
        _fail("SUFFIX0-S0 classifier has a UFCS or alternate production consumer")
    constructor_consumers = _located_suffix_constructor_consumers(root)
    if constructor_consumers:
        _fail(f"SUFFIX0-S0 suffix constructor consumers: {constructor_consumers}")
    if _production_occurrences(root, "::body_suffix(") != 0:
        _fail("SUFFIX0-S0 suffix constructor has a UFCS production consumer")
    for forbidden in ("LocatedLegacyBodySuffixV1", "classify_body_suffix"):
        if forbidden in block_driver:
            _fail(f"SUFFIX0-S0 changed BLK0 driver: {forbidden}")

    for fixture in (
        "inactive_root_start_zero_borrows_the_complete_body",
        "root_suffix_scans_all_rows_and_inactive_proof_borrows_exact_slice",
        "condition_only_row_stays_outside_actual_empty_branch_body",
        "nested_rows_belong_to_their_item_without_crossing_sibling_bodies",
        "suffix_location_rejects_unlocated_overflow_and_out_of_bounds",
        "suffix_carriers_reject_foreign_plan_and_foreign_caller",
    ):
        if fixture not in tests:
            _fail(f"missing SUFFIX0-S0 fixture: {fixture}")
    for evidence in (
        "body.statements().len()",
        "inactive.as_ref().as_ptr(), expected.as_ptr()",
        "ledger.claim(&call_at(&view, &body, 0))",
        "ledger.finish().unwrap()",
        "BodySuffixStartOutOfBounds",
        "BodySuffixIndexOverflow",
        "UnlocatedCannotProveInactive",
        "ForeignPlan",
        "ForeignCaller",
        "SourcePathSegmentV1::IfThen(0)",
        "SourcePathSegmentV1::LoopBody(0)",
        "then_first.node().segments()",
        "then_body.statements().is_empty()",
        "inactive.as_ref().as_ptr(), then_body.statements().as_ptr()",
    ):
        if evidence not in tests:
            _fail(f"missing SUFFIX0-S0 exact evidence: {evidence}")

    for phrase in (
        "checked, non-raw carrier",
        "normal decision, never a caught proof error or retry boundary",
        "unverified carrier exposes no statement slice",
        "SUFFIX0-S0 remains",
        "Builder, BLK0 routing, production roots",
    ):
        if phrase not in readme:
            _fail(f"missing SUFFIX0-S0 README boundary: {phrase}")

    touched = (
        policy_path,
        location_path,
        error_path,
        ledger_path,
        tests_path,
        readme_path,
        block_driver_path,
        helper_path,
    )
    oversized = [path for path in touched if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        _fail(f"SUFFIX0-S0 source/check files reached 800 lines: {oversized}")
    return "suffix_carrier=1 suffix_classifier=1 suffix_consumers=0"
