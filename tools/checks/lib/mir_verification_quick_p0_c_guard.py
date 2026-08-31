"""Guard the deterministic full-lib baseline runner/quick wiring row."""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


ROW = "DEV-GATE-QUICK-LIB-BASELINE-P0-C-RUNNER-WIRE-R0"
KEY = "verification_health_quick_lib_baseline_p0_c_runner_wire_r0_2026_09_01"
BASELINE = Path("tools/checks/manifests/cargo_lib_red_baseline.toml")
INVENTORY = Path("tools/checks/manifests/cargo_lib_red_baseline.tests.txt")
FAILURES = Path("tools/checks/manifests/cargo_lib_red_baseline.failures.txt")
QUICK_STEPS = Path("tools/checks/lib/dev_gate_quick_steps.sh")


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"P0-C field is missing: {name}")
    return value


def _list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"P0-C list is malformed: {name}")
    return list(value)


def _check_landed_files(root: Path, row: dict) -> None:
    for path in (BASELINE, INVENTORY, FAILURES, QUICK_STEPS):
        if not (root / path).is_file():
            api.fail(f"P0-C landed owner is missing: {path}")
    quick = (root / QUICK_STEPS).read_text(encoding="utf-8")
    if quick.count("cargo_lib_red_baseline.py") != 1:
        api.fail("P0-C quick wiring must invoke the runner exactly once")
    if "dev_gate_cmd_step" not in quick:
        api.fail("P0-C quick wiring must use a command step")
    evidence = _text(row, "implementation_evidence")
    for token in ("three", "exact", "cargo_lib_red_baseline.py"):
        if token not in evidence.lower():
            api.fail(f"P0-C implementation evidence lacks {token}")


def check_verification_quick_p0_c_runner_wire_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("P0-C requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        api.fail("P0-C is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("P0-C must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        api.fail("P0-C execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("P0-C card path drifted")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"P0-C section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("P0-C task id drifted")
    if _text(row, "parent_row") != "DEV-GATE-QUICK-LIB-BASELINE-P0":
        api.fail("P0-C parent row drifted")
    status = _text(row, "status")
    if status not in {"fast_open", "landed"}:
        api.fail("P0-C status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("P0-C permission/status drifted")

    decision = _text(row, "decision").lower()
    for token in ("fixed full-lib", "deterministic baseline", "known failure"):
        if token not in decision:
            api.fail(f"P0-C decision lacks {token}")
    for token in ("cargo test result summary", "cargo --list inventory", "sorted baseline"):
        if token not in _text(row, "source_authority"):
            api.fail(f"P0-C source authority lacks {token}")
    for token in ("added", "disappeared", "stack abort"):
        if token not in _text(row, "fail_fast_boundary"):
            api.fail(f"P0-C fail-fast boundary lacks {token}")
    for token in ("No claim that dev_gate quick is wholly green", "no DeclaredInstance", "no Call-schema"):
        if token not in _text(row, "non_claims"):
            api.fail(f"P0-C non-claims lack {token}")

    allowed = set(_list(row, "allowed_files"))
    expected_allowed = {
        "tools/checks/lib/cargo_lib_red_baseline.py",
        "tools/checks/lib/tests/test_cargo_lib_red_baseline.py",
        str(BASELINE),
        str(INVENTORY),
        str(FAILURES),
        str(QUICK_STEPS),
        "tools/checks/lib/mir_verification_quick_p0_c_guard.py",
        str(api.HELPER_REL),
        "docs/tools/check-scripts-index.md",
        str(api.STATE_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        str(api.CARD_REL),
    }
    if allowed != expected_allowed:
        api.fail("P0-C allowed-file boundary drifted")

    filters = _list(row, "focused_test_filters")
    if filters != ["test_cargo_lib_red_baseline"]:
        api.fail("P0-C focused test filter drifted")
    if status == "landed":
        if _text(row, "implementation_commit") == "pending":
            api.fail("P0-C landed row still has pending implementation")
        _check_landed_files(root, row)
    print(f"[{api.TAG}] P0-C baseline runner contract ok status={status}")
