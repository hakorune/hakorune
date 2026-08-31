"""Guard the bounded .inc no-growth baseline row.

The row records existing analysis debt; it does not bless or remove the C/MIR
sources that contain it.  The executable source guard remains the authority for
the observed counts.  This checker only keeps the active card and its finite
baseline boundary honest.
"""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api


ROW = "DEV-GATE-QUICK-LIB-BASELINE-P0-A-INC-DEBT-RECONCILE-R0"
KEY = "verification_health_quick_lib_baseline_p0_a_inc_debt_reconcile_r0_2026_09_01"
ALLOWLIST_REL = Path("tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv")

EXPECTED = {
    "lang/c-abi/shims/hako_llvmc_ffi_checked_callout_lowering.inc": 3,
    "lang/c-abi/shims/hako_llvmc_ffi_checked_callout_predecessor_projection.inc": 2,
    "lang/c-abi/shims/hako_llvmc_ffi_pinned_text_backend_frame.inc": 3,
    "lang/c-abi/shims/hako_llvmc_ffi_pinned_text_selected_preflight.inc": 4,
}


def _text(row: dict, name: str) -> str:
    value = row.get(name)
    if not isinstance(value, str) or not value.strip():
        api.fail(f"P0-A field is missing: {name}")
    return value


def _list(row: dict, name: str) -> list[str]:
    value = row.get(name)
    if not isinstance(value, list) or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        api.fail(f"P0-A list is malformed: {name}")
    return list(value)


def _read_allowlist(root: Path) -> dict[str, int]:
    path = root / ALLOWLIST_REL
    if not path.is_file():
        api.fail(f"P0-A allowlist is missing: {ALLOWLIST_REL}")
    result: dict[str, int] = {}
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) != 2:
            api.fail(f"P0-A allowlist row is malformed: {raw}")
        try:
            count = int(parts[1])
        except ValueError:
            api.fail(f"P0-A allowlist count is not an integer: {raw}")
        if parts[0] in result:
            api.fail(f"P0-A allowlist row is duplicated: {parts[0]}")
        result[parts[0]] = count
    return result


def check_verification_quick_p0_a_inc_debt_reconcile_r0(
    state: dict, card: dict, root: Path, _parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("P0-A requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        api.fail("P0-A is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("P0-A must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        api.fail("P0-A execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("P0-A card path drifted")

    row = card.get(KEY)
    if not isinstance(row, dict):
        api.fail(f"P0-A section is missing: {KEY}")
    if _text(row, "task_id") != ROW:
        api.fail("P0-A task id drifted")
    if _text(row, "parent_row") != "DEV-GATE-QUICK-LIB-BASELINE-P0":
        api.fail("P0-A parent row drifted")
    if _text(row, "status") not in {"fast_open", "landed"}:
        api.fail("P0-A status is not finite")
    expected_permission = _text(row, "status") == "fast_open"
    if row.get("implementation_permission") is not expected_permission:
        api.fail("P0-A permission/status drifted")
    for token in ("no-growth allowlist", "future files", "C/MIR code"):
        if token not in _text(row, "decision"):
            api.fail(f"P0-A decision lacks {token}")
    for token in ("new .inc debt owner", "per-file count growth", "does not make quick"):
        if token not in _text(row, "fail_fast_boundary"):
            api.fail(f"P0-A fail-fast boundary lacks {token}")
    for token in ("No claim that quick is green", "no cargo-test", "no Call-schema change"):
        if token not in _text(row, "non_claims"):
            api.fail(f"P0-A non-claims lack {token}")

    allowed = set(_list(row, "allowed_files"))
    expected_allowed = {
        str(ALLOWLIST_REL),
        "tools/checks/lib/mir_verification_quick_p0_a_guard.py",
        str(api.HELPER_REL),
        str(api.STATE_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        str(api.CARD_REL),
    }
    if allowed != expected_allowed:
        api.fail("P0-A allowed-file boundary drifted")

    observed = _read_allowlist(root)
    if _text(row, "status") == "landed" and observed != EXPECTED:
        api.fail(f"P0-A landed allowlist drifted: {observed!r}")
    if _text(row, "status") == "fast_open" and any(
        path not in EXPECTED or count < 0 for path, count in observed.items()
    ):
        api.fail("P0-A open allowlist contains an unknown or negative row")
    print(f"[{api.TAG}] P0-A .inc baseline contract ok status={row['status']}")
