#!/usr/bin/env python3
"""Guard the locator-to-ledger crosswalk design stop."""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


ROW = "MIR-CALL-ME-DECLARED-INSTANCE-LOCATOR-VALUE-CROSSWALK-D0"
KEY = "mir_call_me_declared_instance_locator_value_crosswalk_d0_2026_09_02"
PARENT_KEY = "mir_call_me_declared_instance_receiver_value_owner_d0_2026_09_01"


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"locator-value crosswalk D0 field is missing: {name}")
    return value


def _list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"locator-value crosswalk D0 list is missing or empty: {name}")
    return value


def _tokens(text: str, label: str, required: tuple[str, ...]) -> None:
    for token in required:
        if token not in text:
            api.fail(f"locator-value crosswalk D0 {label} lacks {token}")


def check_declared_instance_locator_value_crosswalk_d0(
    state: dict, card: dict, _root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") != "design_stop":
        api.fail("locator-value crosswalk D0 must remain design_stop")
    for field in ("current_execution_row", "current_design_stop", "next_design_card"):
        if state.get(field) != ROW:
            api.fail(f"locator-value crosswalk D0 pointer drifted: {field}")
    if state.get("next_execution_card") != "none":
        api.fail("locator-value crosswalk D0 must not open implementation")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"locator-value crosswalk D0 section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("locator-value crosswalk D0 task id drifted")
    if _text(row, "status") != "accepted_design_stop":
        api.fail("locator-value crosswalk D0 status drifted")
    if row.get("implementation_permission") is not False:
        api.fail("locator-value crosswalk D0 implementation permission must be false")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict):
        api.fail(f"locator-value crosswalk D0 parent is missing: {PARENT_KEY}")
    _tokens(
        _text(parent, "next_design_row"),
        "parent handoff",
        (ROW,),
    )
    _tokens(
        _text(row, "decision"),
        "decision",
        ("non-Clone callback-scoped view", "existing generic take-once value", "family-specific receiver loan"),
    )
    _tokens(
        _text(row, "source_authority"),
        "source authority",
        ("caller_owner", "receiver_site", "receiver_binding"),
    )
    _tokens(
        _text(row, "canonical_issuer"),
        "canonical issuer",
        ("No new issuer", "DeclaredInstanceCallLocatorViewV1", "CallableSemanticLoweringState"),
    )
    _tokens(
        _text(row, "fail_fast_boundary"),
        "fail-fast boundary",
        ("before receiver effects", "argument descent", "MIR publication", "fallback", "retry"),
    )
    _tokens(
        _text(row, "census_boundary"),
        "census boundary",
        ("Installed package locator callback", "live callable session owner", "request-local receiver ValueId"),
    )
    forbidden = "\n".join(_list(row, "non_authority"))
    _tokens(
        forbidden,
        "non-authority",
        ("variable_map", "param0", "args[0]", "ValueId(0)", "family-specific loan", "second resolver"),
    )

    states = _list(row, "finite_states")
    for state_name in (
        "NoRootDeclaredInstanceCall",
        "RelationRowUnavailable",
        "OwnerMismatch",
        "SessionMismatch",
        "BindingMismatch",
        "DuplicateCandidate",
        "EntryValueUnavailable",
        "ReadyToBorrow",
        "AlreadyTaken",
        "Residual",
        "NestedOrUpvarOutside",
    ):
        if not any(item.startswith(state_name + ":") for item in states):
            api.fail(f"locator-value crosswalk D0 finite states lack {state_name}")

    tasks = _list(row, "ordered_tasks")
    for prefix in ("D0-A:", "D0-B:", "D0-C:", "I0 later:"):
        if not any(item.startswith(prefix) for item in tasks):
            api.fail(f"locator-value crosswalk D0 tasks lack {prefix}")
    _tokens(
        _text(row, "acceptance"),
        "acceptance",
        ("finite bidirectional crosswalk", "before effects", "no ValueId"),
    )
    _tokens(
        _text(row, "no_safe_slice"),
        "NoSafeSlice",
        ("locator view", "locator-to-ledger crosswalk", "second authority", "fallback", "retry"),
    )
    _tokens(
        _text(row, "non_claims"),
        "non-claims",
        ("No receiver ValueId production", "no Method(Some)", "no selected-C/Hako/C change"),
    )
    _tokens(
        _text(row, "worker_audit_result"),
        "worker audit",
        ("three D0 audits", "CallableSemanticLoweringState", "crosswalk is zero"),
    )
    evidence = "\n".join(_list(row, "evidence"))
    _tokens(
        evidence,
        "evidence",
        ("declared_instance_locator.rs", "normal_callable_semantic_lowering_state.rs", "normal_callable_semantic_loan_port.rs"),
    )
    if _text(row, "next_design_row") != ROW:
        api.fail("locator-value crosswalk D0 next design row drifted")

    expected = {
        "docs/development/current/main/CURRENT_STATE.toml",
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        "docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        "tools/checks/lib/mir_call_d1b_active_surface_guard.py",
        "tools/checks/lib/mir_declared_instance_locator_value_crosswalk_d0_guard.py",
    }
    if set(_list(row, "allowed_files")) != expected:
        api.fail("locator-value crosswalk D0 allowed-file boundary drifted")
    forbidden_files = "\n".join(_list(row, "forbidden_files"))
    _tokens(forbidden_files, "forbidden boundary", ("src/mir/", "Call schema", "VM/backend/JSON/runtime"))
    print(f"[{api.TAG}] locator-value crosswalk D0 design-stop contract ok")
