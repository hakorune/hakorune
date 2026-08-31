#!/usr/bin/env python3
"""Guard the behavior-neutral exact BindingRef -> ValueId accessor row."""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


ROW = "MIR-CALL-ME-DECLARED-INSTANCE-EXACT-BINDING-VALUE-ACCESSOR-S0"
KEY = "mir_call_me_declared_instance_exact_binding_value_accessor_s0_2026_09_02"
PARENT_KEY = "mir_call_me_declared_instance_locator_value_crosswalk_d0_2026_09_02"


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"exact-binding accessor S0 field is missing: {name}")
    return value


def _list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"exact-binding accessor S0 list is missing or empty: {name}")
    return value


def _contains(text: str, label: str, tokens: tuple[str, ...]) -> None:
    for token in tokens:
        if token not in text:
            api.fail(f"exact-binding accessor S0 {label} lacks {token}")


def check_exact_binding_value_accessor_s0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") != "fast":
        api.fail("exact-binding accessor S0 requires fast work_mode")
    for field in ("current_execution_row", "next_execution_card"):
        if state.get(field) != ROW:
            api.fail(f"exact-binding accessor S0 pointer drifted: {field}")
    if state.get("current_design_stop") != "none":
        api.fail("exact-binding accessor S0 must clear current_design_stop")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"exact-binding accessor S0 section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("exact-binding accessor S0 task id drifted")
    if _text(row, "status") != "active_fast":
        api.fail("exact-binding accessor S0 status must be active_fast")
    if row.get("implementation_permission") is not True:
        api.fail("exact-binding accessor S0 implementation permission must be true")
    if _text(row, "parent_row") != _text(card.get(PARENT_KEY, {}), "task_id"):
        api.fail("exact-binding accessor S0 parent row does not name the D0 owner")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict):
        api.fail(f"exact-binding accessor S0 parent is missing: {PARENT_KEY}")
    if parent.get("status") != "accepted_design_stop":
        api.fail("exact-binding accessor S0 parent must remain an accepted design stop")
    if parent.get("implementation_permission") is not False:
        api.fail("exact-binding accessor S0 parent permission must remain false")

    _contains(
        _text(row, "decision"),
        "decision",
        ("generic exact-binding accessor", "already-materialized ValueId", "receiver-specific loan"),
    )
    _contains(
        _text(row, "source_authority"),
        "source authority",
        ("CallableSemanticLoweringState", "entry_installed", "values map"),
    )
    _contains(
        _text(row, "canonical_issuer"),
        "canonical issuer",
        ("No new issuer", "existing entry materialization path"),
    )
    _contains(
        _text(row, "fail_fast_boundary"),
        "fail-fast boundary",
        ("foreign owner", "foreign binding", "before any future receiver or argument effect"),
    )
    _contains(
        _text(row, "census_boundary"),
        "census boundary",
        ("CallableSemanticLoweringState value lookup", "exact generic BindingRef accessor"),
    )
    non_authority = "\n".join(_list(row, "non_authority"))
    _contains(
        non_authority,
        "non-authority",
        ("variable_map", "param0", "args[0]", "numeric ValueId(0)", "fallback", "retry"),
    )

    states = _list(row, "finite_states")
    for state_name in (
        "OwnerMismatch",
        "ForeignBinding",
        "EntryNotInstalled",
        "ValueUnavailable",
        "Ready",
        "Reusable",
    ):
        if not any(item.startswith(state_name + ":") for item in states):
            api.fail(f"exact-binding accessor S0 finite states lack {state_name}")

    tasks = _list(row, "ordered_tasks")
    for prefix in ("S0-A:", "S0-B:", "S0-C:", "S0-D:"):
        if not any(item.startswith(prefix) for item in tasks):
            api.fail(f"exact-binding accessor S0 tasks lack {prefix}")
    _contains(
        _text(row, "acceptance"),
        "acceptance",
        ("existing variable reader", "repeated reads", "no target", "receiver loan"),
    )
    _contains(
        _text(row, "non_claims"),
        "non-claims",
        ("No locator-to-ledger", "no Method(Some)", "no selected-C/Hako/C change"),
    )

    source = root / "src/mir/builder/normal_callable_semantic_lowering_state.rs"
    accessor = root / "src/mir/builder/normal_callable_semantic_receiver_crosswalk.rs"
    tests = root / "src/mir/builder/normal_callable_semantic_receiver_crosswalk_tests.rs"
    for path in (source, accessor, tests):
        if not path.is_file():
            api.fail(f"exact-binding accessor S0 source is missing: {path}")
        if len(path.read_text(encoding="utf-8").splitlines()) >= 760:
            api.fail(f"exact-binding accessor S0 source reached the 760-line boundary: {path}")

    source_text = source.read_text(encoding="utf-8")
    accessor_text = accessor.read_text(encoding="utf-8")
    tests_text = tests.read_text(encoding="utf-8")
    _contains(
        source_text,
        "state source",
        ("normal_callable_semantic_receiver_crosswalk", "read_variable", "value_for_exact_binding"),
    )
    _contains(
        accessor_text,
        "accessor source",
        ("ExactBindingValueErrorV1", "entry_installed", "value_for_exact_binding"),
    )
    for token in ("variable_map", "args[0]", "ValueId(0)", "Callee", "Method(Some)"):
        if token in accessor_text:
            api.fail(f"exact-binding accessor S0 source contains forbidden token: {token}")
    for test_name in _list(row, "changed_test_names"):
        if f"fn {test_name}(" not in tests_text:
            api.fail(f"exact-binding accessor S0 changed test is missing: {test_name}")

    allowed = set(_list(row, "allowed_files"))
    expected = {
        "src/mir/builder/normal_callable_semantic_lowering_state.rs",
        "src/mir/builder/normal_callable_semantic_receiver_crosswalk.rs",
        "src/mir/builder/normal_callable_semantic_receiver_crosswalk_tests.rs",
        "src/mir/builder/README.md",
        "tools/checks/lib/mir_declared_instance_exact_binding_value_accessor_s0_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        "docs/development/current/main/CURRENT_STATE.toml",
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        "docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml",
    }
    if allowed != expected:
        api.fail(f"exact-binding accessor S0 allowed-file boundary drifted: {sorted(allowed ^ expected)}")
    forbidden = "\n".join(_list(row, "forbidden_files"))
    _contains(forbidden, "forbidden boundary", ("method_call_handlers.rs", "calls/build.rs", "Method(Some)", "selected-C"))
    print(f"[{api.TAG}] exact-binding accessor S0 contract ok")


if __name__ == "__main__":
    import sys

    if len(sys.argv) != 2:
        api.fail("usage: mir_declared_instance_exact_binding_value_accessor_s0_guard.py ROOT")
    root = Path(sys.argv[1]).resolve()
    state = api.load_toml(root / api.STATE_REL)
    card = api.load_toml(root / api.CARD_REL)
    check_exact_binding_value_accessor_s0(state, card, root)
