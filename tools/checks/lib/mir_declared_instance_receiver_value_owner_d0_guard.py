#!/usr/bin/env python3
"""Guard the DeclaredInstance receiver-value owner design stop."""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


ROW = "MIR-CALL-ME-DECLARED-INSTANCE-RECEIVER-VALUE-OWNER-D0"
KEY = "mir_call_me_declared_instance_receiver_value_owner_d0_2026_09_01"
PARENT_KEY = "backend_owner_declared_instance_method_cutover_d0_2026_09_01"


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"receiver-value owner D0 field is missing: {name}")
    return value


def _list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"receiver-value owner D0 list is missing or empty: {name}")
    return value


def _require_tokens(text: str, label: str, tokens: tuple[str, ...]) -> None:
    for token in tokens:
        if token not in text:
            api.fail(f"receiver-value owner D0 {label} lacks {token}")


def check_declared_instance_receiver_value_owner_d0(
    state: dict, card: dict, _root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") != "design_stop":
        api.fail("receiver-value owner D0 must remain design_stop")
    for field in ("current_execution_row", "current_design_stop", "next_design_card"):
        if state.get(field) != ROW:
            api.fail(f"receiver-value owner D0 pointer drifted: {field}")
    if state.get("next_execution_card") != "none":
        api.fail("receiver-value owner D0 must not open implementation")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"receiver-value owner D0 section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("receiver-value owner D0 task id drifted")
    if _text(row, "status") != "accepted_design_stop":
        api.fail("receiver-value owner D0 status drifted")
    if row.get("implementation_permission") is not False:
        api.fail("receiver-value owner D0 implementation permission must be false")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict):
        api.fail(f"receiver-value owner D0 parent is missing: {PARENT_KEY}")
    _require_tokens(
        _text(parent, "option_review_decision"),
        "final option",
        ("Choose B", "final architecture", "temporary UnsupportedBeforeObject"),
    )
    _require_tokens(
        _text(parent, "option_review_reason"),
        "option reason",
        ("ordinary me.method", "C-speed goal", "one MIR semantic authority"),
    )

    _require_tokens(
        _text(row, "decision"),
        "decision",
        ("Option B", "BindingRef", "ValueId", "Option A"),
    )
    _require_tokens(
        _text(row, "source_authority"),
        "source authority",
        ("receiver_binding", "BindingRef", "same-function"),
    )
    _require_tokens(
        _text(row, "canonical_issuer"),
        "canonical issuer",
        (
            "CallableSemanticLoweringState",
            "BindingRef -> ValueId",
            "no new Verified/Prepared receipt",
        ),
    )
    _require_tokens(
        _text(row, "fail_fast_boundary"),
        "fail-fast boundary",
        ("before receiver or argument effects", "MIR publication", "backend admission"),
    )
    _require_tokens(
        _text(row, "census_boundary"),
        "census boundary",
        ("locator callback", "materialization ledger", "first request-local ValueId borrow"),
    )

    non_authority = "\n".join(_list(row, "non_authority"))
    _require_tokens(
        non_authority,
        "non-authority",
        ("variable_map", "param0", "args[0]", "ValueId(0)", "backend metadata"),
    )
    states = _list(row, "finite_states")
    for state_name in (
        "ExactStorageCandidate",
        "ExactExistingOwnerButViewUnavailable",
        "Ready",
        "NoRootDeclaredInstanceCall",
        "OwnerMissing",
        "OwnerForeign",
        "OwnerAmbiguous",
        "BindingMismatch",
        "EntryValueUnavailable",
        "AlreadyTaken",
        "Residual",
        "NestedOrUpvarOutside",
    ):
        if not any(item.startswith(state_name + ":") for item in states):
            api.fail(f"receiver-value owner D0 finite states lack {state_name}")

    tasks = _list(row, "ordered_tasks")
    for prefix in ("D0-A:", "D0-B:", "D0-C:", "I0 later:"):
        if not any(item.startswith(prefix) for item in tasks):
            api.fail(f"receiver-value owner D0 tasks lack {prefix}")
    _require_tokens(
        _text(row, "acceptance"),
        "acceptance",
        (
            "sole physical storage candidate",
            "production locator-to-ledger crosswalk",
            "before effects",
            "no ValueId",
        ),
    )
    _require_tokens(
        _text(row, "no_safe_slice"),
        "NoSafeSlice",
        (
            "locator view",
            "production crosswalk is zero",
            "variable_map",
            "args[0]",
            "family-specific loan",
            "fallback",
            "retry",
        ),
    )
    _require_tokens(
        _text(row, "non_claims"),
        "non-claims",
        ("No Method(Some)", "no selected-C or Hako coverage", "no MIR/Call schema change"),
    )
    _require_tokens(
        _text(row, "worker_audit_result"),
        "worker audit",
        ("Three independent", "CallableSemanticLoweringState", "crosswalk is 0"),
    )
    if row.get("candidate_owner_count") != 1:
        api.fail("receiver-value owner D0 candidate owner count must be one")
    if row.get("production_crosswalk_count") != 0:
        api.fail("receiver-value owner D0 production crosswalk must be zero")
    evidence = "\n".join(_list(row, "worker_audit_evidence"))
    _require_tokens(
        evidence,
        "worker evidence",
        (
            "normal_callable_semantic_lowering_state.rs",
            "declared_instance_locator.rs",
            "method_call_handlers.rs",
        ),
    )
    _require_tokens(
        _text(row, "next_design_slice"),
        "next design slice",
        (
            "LOCATOR-VALUE-CROSSWALK-D0",
            "existing generic consume ledger",
            "without issuing a new receipt",
        ),
    )

    expected = {
        "docs/development/current/main/CURRENT_STATE.toml",
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        "docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml",
        "tools/checks/lib/mir_call_d1b_active_surface_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        "tools/checks/lib/mir_declared_instance_receiver_value_owner_d0_guard.py",
    }
    if set(_list(row, "allowed_files")) != expected:
        api.fail("receiver-value owner D0 allowed-file boundary drifted")
    forbidden = "\n".join(_list(row, "forbidden_files"))
    _require_tokens(forbidden, "forbidden boundary", ("src/mir/", "Call schema", "VM/backend"))
    print(f"[{api.TAG}] DeclaredInstance receiver-value owner D0 contract ok")
