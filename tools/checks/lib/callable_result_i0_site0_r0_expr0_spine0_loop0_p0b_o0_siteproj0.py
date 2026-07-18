#!/usr/bin/env python3
"""Structural guard for LOOP0-P0b-O0-SITEPROJ0."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _production, _read


LEGACY = "src/mir/callable_result_representation/located_legacy.rs"
PORT = "src/mir/builder/control_flow/plan/expression_port.rs"
PORT_TESTS = "src/mir/builder/control_flow/plan/expression_port_tests.rs"
BASE = "src/mir/builder/control_flow/plan/generic_loop/located_representation"
MODULE = f"{BASE}/mod.rs"
TESTS = f"{BASE}/site_projection_tests.rs"


def _function(text: str, signature: str) -> str:
    start = text.find(signature)
    if start < 0:
        raise RuntimeError(f"LOOP0-P0b-O0-SITEPROJ0 missing function: {signature}")
    brace = text.find("{", start)
    if brace < 0:
        raise RuntimeError(f"LOOP0-P0b-O0-SITEPROJ0 missing body: {signature}")
    depth = 0
    for index in range(brace, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise RuntimeError(f"LOOP0-P0b-O0-SITEPROJ0 unterminated body: {signature}")


def _count(text: str, token: str, expected: int, label: str) -> None:
    actual = text.count(token)
    if actual != expected:
        raise RuntimeError(
            f"LOOP0-P0b-O0-SITEPROJ0 {label} drift: "
            f"expected={expected} actual={actual}"
        )


def check_loop0_p0b_o0_siteproj0(root: Path) -> str:
    legacy = _production(_read(root, LEGACY))
    port = _production(_read(root, PORT))
    port_tests = _read(root, PORT_TESTS)
    module = _read(root, MODULE)
    tests = _read(root, TESTS)

    _count(legacy, "fn project_compact_body_stmt(", 1, "projection owner")
    projection = _function(legacy, "fn project_compact_body_stmt(")
    for required in (
        "LegacyBodyInputV1::Located(body)",
        "self.require_carrier(body.plan_identity, body.caller)?",
        "body.domain_parent.as_ref()",
        "u32::try_from(index)",
        "body.statements.get(index as usize)",
        "SourcePathV1::from_node(domain_parent)",
        ".child(body.kind.item_segment(index))",
    ):
        if required not in projection:
            raise RuntimeError(
                f"LOOP0-P0b-O0-SITEPROJ0 projection law drift: {required}"
            )
    for forbidden in (
        "strip_prefix",
        "activation_plan",
        "rows_for",
        "ASTNode::",
        "ValueId",
        "fallback",
        "retry",
    ):
        if forbidden in projection:
            raise RuntimeError(
                f"LOOP0-P0b-O0-SITEPROJ0 projection authority leak: {forbidden}"
            )

    generic = _function(legacy, "fn body_stmt(")
    for retained in ("body.parent", "body.kind.item_segment(index)"):
        if retained not in generic:
            raise RuntimeError(
                f"LOOP0-P0b-O0-SITEPROJ0 generic body_stmt drift: {retained}"
            )
    if "domain_parent" in generic or "project_compact_body_stmt" in generic:
        raise RuntimeError("LOOP0-P0b-O0-SITEPROJ0 generic body_stmt became compact")

    direct_consumers = []
    for path in (root / "src").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        count = _production(path.read_text(encoding="utf-8")).count(
            ".project_compact_body_stmt("
        )
        direct_consumers.extend([relative] * count)
    if direct_consumers != [PORT]:
        raise RuntimeError(
            "LOOP0-P0b-O0-SITEPROJ0 direct consumer drift: "
            f"{direct_consumers}"
        )
    exact_body_stmt = _function(port, "fn exact_body_stmt(")
    _count(
        exact_body_stmt,
        ".project_compact_body_stmt(body, index)",
        1,
        "Loop-port bridge",
    )

    required_tests = (
        "located_loop_body_item_projection_is_compact_and_port_owned",
        "actual_loop_projection_matches_selected_site_and_nested_branch_paths",
        "compact_projection_rejects_foreign_unlocated_root_and_invalid_ordinals",
    )
    for name in required_tests:
        _count(port_tests + tests, f"fn {name}(", 1, f"focused test {name}")
    _count(module, "mod site_projection_tests;", 1, "test module registration")
    if "#[ignore]" in tests:
        raise RuntimeError("LOOP0-P0b-O0-SITEPROJ0 focused tests must not be ignored")

    touched = (LEGACY, PORT, PORT_TESTS, MODULE, TESTS, __file__)
    oversized = []
    for path in touched:
        relative = str(path) if isinstance(path, str) else str(Path(path).relative_to(root))
        if len(_read(root, relative).splitlines()) >= 800:
            oversized.append(relative)
    if oversized:
        raise RuntimeError(
            f"LOOP0-P0b-O0-SITEPROJ0 source/check files reached 800 lines: {oversized}"
        )

    return (
        "loop0_p0b_o0_siteproj0=green projection_owners=1 "
        "direct_loop_port_consumers=1 generic_body_stmt_delta=0 "
        "activation_row_delta=0 builder=0 ledger=0"
    )
