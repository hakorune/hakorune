"""Guard the backend-owner design stop for DeclaredInstance methods.

This row is deliberately documentation-only.  It keeps the selected-C
compatibility owner outside MirBuilder semantic authority until one exact Hako
LLVM-text replacement family and a named caller are proven.
"""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


ROW = "BACKEND-OWNER-DECLARED-INSTANCE-METHOD-CUTOVER-D0"
KEY = "backend_owner_declared_instance_method_cutover_d0_2026_09_01"


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"backend owner D0 field is missing: {name}")
    return value


def _nonempty_list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"backend owner D0 list is missing or empty: {name}")
    return value


def check_backend_owner_declared_instance_method_d0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") != "design_stop":
        api.fail("backend owner D0 must remain design_stop")
    if state.get("current_execution_row") != ROW:
        api.fail("backend owner D0 is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != ROW:
        api.fail("backend owner D0 design-stop pointer drifted")
    if state.get("next_design_card") != ROW:
        api.fail("backend owner D0 next-design pointer drifted")
    if state.get("next_execution_card") != "none":
        api.fail("backend owner D0 must not open an implementation card")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"backend owner D0 section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("backend owner D0 task id drifted")
    if _text(row, "status") != "accepted_design_stop":
        api.fail("backend owner D0 status is not accepted_design_stop")
    if row.get("implementation_permission") is not False:
        api.fail("backend owner D0 implementation permission must remain false")
    if "RetireAfterReplacement" not in _text(row, "decision"):
        api.fail("backend owner D0 does not record RetireAfterReplacement")
    source = _text(row, "source_authority")
    if "MIR remains the semantic SSOT" not in source:
        api.fail("backend owner D0 source authority drifted")
    issuer = _text(row, "canonical_issuer")
    if "Hako LLVM-text emitter" not in issuer:
        api.fail("backend owner D0 future terminal owner is missing")
    boundary = _text(row, "fail_fast_boundary")
    for token in ("before object emission", "fallback/retry", "repair MIR meaning"):
        if token not in boundary:
            api.fail(f"backend owner D0 fail-fast contract lacks {token}")

    inventory = _nonempty_list(row, "finite_inventory")
    states = _nonempty_list(row, "finite_states")
    tasks = _nonempty_list(row, "ordered_tasks")
    for token in ("current compatibility owner", "future terminal owner", "transport only"):
        if not any(token in item for item in inventory):
            api.fail(f"backend owner D0 inventory lacks {token}")
    for token in ("CoverageMissing", "CoverageExact", "CallerSwitched", "RetireReady", "Unsupported"):
        if not any(item.startswith(token) for item in states):
            api.fail(f"backend owner D0 state inventory lacks {token}")
    if not any(item.startswith("D0-A") for item in tasks):
        api.fail("backend owner D0 task inventory lacks D0-A")
    if not any(item.startswith("D0-B") for item in tasks):
        api.fail("backend owner D0 task inventory lacks D0-B")
    if not any(item.startswith("I0 later") for item in tasks):
        api.fail("backend owner D0 task inventory lacks deferred I0")

    non_claims = _text(row, "non_claims")
    for token in ("No selected-C admission token", "No MIR semantic change", "no physical delete"):
        if token not in non_claims:
            api.fail(f"backend owner D0 non-claims lack {token}")
    reopen = _text(row, "reopen_trigger")
    for token in ("exact Hako emitter family", "named live caller", "fallback/retry zero"):
        if token not in reopen:
            api.fail(f"backend owner D0 reopen trigger lacks {token}")

    allowed = row.get("allowed_files")
    expected = {
        "docs/development/current/main/CURRENT_STATE.toml",
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        "docs/development/current/main/investigations/mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml",
        "tools/checks/lib/mir_call_d1b_active_surface_guard.py",
        "tools/checks/lib/mir_backend_owner_declared_instance_method_d0_guard.py",
    }
    if not isinstance(allowed, list) or set(allowed) != expected:
        api.fail("backend owner D0 allowed-file boundary drifted")
    forbidden = _nonempty_list(row, "forbidden_files")
    for token in ("src/mir/", "Call schema", "VM/backend/JSON/runtime"):
        if not any(token in item for item in forbidden):
            api.fail(f"backend owner D0 forbidden boundary lacks {token}")
    print(f"[{api.TAG}] backend owner D0 design-stop contract ok")
