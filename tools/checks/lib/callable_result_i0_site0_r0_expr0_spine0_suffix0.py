#!/usr/bin/env python3
"""Structural checks for the exact body-suffix classifier and borrowed seam."""

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
    readme_path = "src/mir/callable_result_representation/README.md"
    block_driver_path = "src/mir/builder/stmts/block_driver.rs"
    helper_path = (
        "tools/checks/lib/"
        "callable_result_i0_site0_r0_expr0_spine0_suffix0.py"
    )

    policy = _read(root, policy_path)
    readme = _read(root, readme_path)
    block_driver = _read(root, block_driver_path)

    _count(policy, "pub(crate) fn owned_item_index", 1, "body item-index owner")
    _count(
        policy,
        "self.owned_item_index(segment).is_some()",
        1,
        "owns-item delegation",
    )

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
        readme_path,
        block_driver_path,
        helper_path,
    )
    oversized = [path for path in touched if len(_read(root, path).splitlines()) >= 800]
    if oversized:
        _fail(f"SUFFIX0-S0 source/check files reached 800 lines: {oversized}")
    return "retired_suffix_carrier=0 retired_suffix_classifier=0 suffix_consumers=0"

def check_suffix0_i0(root: Path) -> str:
    driver = _read(root, "src/mir/builder/stmts/block_driver.rs")
    raw = _read(root, "src/mir/builder/stmts/block_stmt.rs")
    for retired in (
        "SuffixInput",
        "suffix_route_input",
        "consume_suffix_prefix",
        "NormalizedShadowSuffixRouterBox",
        "try_lower_loop_suffix",
    ):
        _count(driver + raw, retired, 0, f"retired suffix capability {retired}")
    for relative in (
        "src/mir/builder/stmts/block_suffix_parity_reference.rs",
        "src/mir/callable_result_representation/tests/block_suffix_route_parity.rs",
    ):
        if (root / relative).exists():
            _fail(f"retired suffix proof artifact remains: {relative}")
    return "suffix_parity_reference=0 associated_suffix_view=0 production_located_execution_callers=0"
