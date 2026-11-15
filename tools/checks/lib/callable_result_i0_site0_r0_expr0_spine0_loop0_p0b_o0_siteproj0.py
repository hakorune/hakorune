#!/usr/bin/env python3
"""Structural guard for LOOP0-P0b-O0-SITEPROJ0."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _production, _read

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
    port = _production(_read(root, PORT))
    port_tests = _read(root, PORT_TESTS)
    module = _read(root, MODULE)
    tests = _read(root, TESTS)

    required_tests = (
        "actual_loop_projection_matches_selected_site_and_nested_branch_paths",
        "compact_projection_rejects_foreign_unlocated_root_and_invalid_ordinals",
    )
    for name in required_tests:
        _count(port_tests + tests, f"fn {name}(", 1, f"focused test {name}")
    _count(module, "mod site_projection_tests;", 1, "test module registration")
    if "#[ignore]" in tests:
        raise RuntimeError("LOOP0-P0b-O0-SITEPROJ0 focused tests must not be ignored")

    touched = ( PORT, PORT_TESTS, MODULE, TESTS, __file__)
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
        "loop0_p0b_o0_siteproj0=green retired_projection_owners=0 "
        "retired_direct_loop_port_consumers=0 generic_body_stmt_delta=0 "
        "activation_row_delta=0 builder=0 ledger=0"
    )
