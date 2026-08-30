#!/usr/bin/env python3
"""Structural guard for the one-name bare ``error`` retirement row."""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


TAG = "mir-call-d1b-bare-error-retire"
ROW = "MIR-CALL-SAME-MODULE-CATALOGED-PROVIDER-BARE-ERROR-RETIRE-I0"
CARD_KEY = "same_module_cataloged_provider_bare_error_retire_i0_2026_08_30"
CARD_REL = api.CARD_REL
STATE_REL = api.STATE_REL

ROUTE_REL = Path("src/mir/builder/calls/function_call_preflight_route.rs")
TESTS_REL = Path("src/mir/builder/calls/function_call_installed_gc_builtin_tests.rs")
README_REL = Path("src/mir/builder/calls/README.md")
GUARD_REL = Path("tools/checks/lib/mir_call_d1b_bare_error_retire_guard.py")
PARENT_GUARD_REL = Path("tools/checks/lib/mir_call_d1b_active_surface_guard.py")
WORKSTREAM_REL = Path(
    "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md"
)


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] {message}")


def text(root: Path, rel: Path) -> str:
    try:
        return (root / rel).read_text(encoding="utf-8")
    except OSError as exc:
        fail(f"cannot read {rel}: {exc}")


def check_pointer(state: dict) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("bare error retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        fail("bare error retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("bare error retirement must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        fail("bare error retirement pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("bare error retirement card pointer drifted")


def check_card(card: dict) -> dict:
    row = card.get(CARD_KEY)
    if not isinstance(row, dict):
        fail(f"{CARD_KEY} section is missing")
    if row.get("task_id") != ROW:
        fail("bare error retirement task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("bare error retirement status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        fail("bare error retirement permission/status drifted")
    return row


def check_files(root: Path, row: dict) -> set[str]:
    expected = {
        str(ROUTE_REL),
        str(TESTS_REL),
        str(README_REL),
        str(GUARD_REL),
        str(PARENT_GUARD_REL),
        str(STATE_REL),
        str(CARD_REL),
        str(WORKSTREAM_REL),
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected:
        fail("bare error retirement allowed-file boundary drifted")
    for rel in (ROUTE_REL, TESTS_REL, GUARD_REL, PARENT_GUARD_REL):
        path = root / rel
        if not path.is_file():
            fail(f"bare error retirement owner is missing: {rel}")
        if len(path.read_text(encoding="utf-8").splitlines()) >= 760:
            fail(f"bare error retirement owner reached the 760-line boundary: {rel}")
    return expected


def check_fast_open(root: Path, row: dict) -> None:
    """Activation is allowed before the implementation commit, but is not proof."""
    if row.get("changed_test_names") != []:
        fail("fast_open bare error row must not claim changed tests")
    if row.get("focused_test_runs") != []:
        fail("fast_open bare error row must not claim test runs")
    if not text(root, README_REL).strip():
        fail("calls README is empty")


def check_landed(root: Path, row: dict, parent_api) -> None:
    route = text(root, ROUTE_REL)
    tests = text(root, TESTS_REL)
    readme = text(root, README_REL)
    start = route.find("fn prepare_ordinary_function_completion_v1")
    end = route.find("fn is_installed_non_unified_gc_builtin_v1")
    if start < 0 or end <= start:
        fail("ordinary completion owner cannot be located")
    completion = route[start:end]
    error_branch = completion.find('name == "error"')
    caller_branch = completion.find("else if let Some(caller) = caller")
    target_prep = completion.find("prepare_cataloged_target_v1")
    rejected = completion.find("PreparedRawOrdinaryFunctionCompletionV1::Rejected")
    if min(error_branch, caller_branch, target_prep, rejected) < 0:
        fail("bare error pre-effect branch evidence is missing")
    if not (error_branch < rejected < caller_branch < target_prep):
        fail("bare error reject is not before target preparation and caller descent")
    if route.count("[freeze:contract][direct-call/bare-error-unsupported]") != 1:
        fail("bare error terminal tag is not unique")
    for name in (
        "cataloged_bare_error_rejects_before_target_synthesis",
        "cataloged_bare_error_rejection_does_not_descend_or_publish",
    ):
        if tests.count(f"fn {name}(") != 1:
            fail(f"bare error focused test is missing or duplicated: {name}")
    if "bare `error`" not in readme.lower() or "pre-effect" not in readme:
        fail("calls README lacks the bare error contract receipt")

    parent_api.check_test_coverage(root, row)
    base = parent_api.require_text(row.get("coverage_base_commit"), "bare error coverage_base_commit")
    changed = parent_api.git_diff_paths(root, base)
    expected_allowed = check_files(root, row)
    if not changed.issubset(expected_allowed):
        fail(
            "bare error retirement changed paths exceed boundary: "
            f"{sorted(changed - expected_allowed)}"
        )


def check_bare_error_retire_i0(state: dict, card: dict, root: Path, parent_api=api) -> None:
    check_pointer(state)
    row = check_card(card)
    check_files(root, row)
    if row.get("status") == "fast_open":
        check_fast_open(root, row)
    else:
        check_landed(root, row, parent_api)


if __name__ == "__main__":
    import sys
    import tomllib

    if len(sys.argv) != 2:
        fail("usage: mir_call_d1b_bare_error_retire_guard.py ROOT")
    root = Path(sys.argv[1]).resolve()
    with (root / STATE_REL).open("rb") as stream:
        state = tomllib.load(stream)
    with (root / CARD_REL).open("rb") as stream:
        card = tomllib.load(stream)
    check_bare_error_retire_i0(state, card, root)
    print(f"[{TAG}] row={ROW} ok")
