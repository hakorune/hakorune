"""Reusable guard for the caller-zero rewrite/known cleanup slice.

The active-surface entry dispatches this checker; it is intentionally not a
new registry row.  The slice removes only the no-destination compatibility
facades after a source caller scan, while keeping the destination-aware
writers and their live unified/special callers.
"""

from __future__ import annotations

import subprocess
from pathlib import Path


ROW = "MIR-CALL-SAME-MODULE-REWRITE-KNOWN-CALLER-ZERO-PRUNE-S0"
KEY = "same_module_rewrite_known_caller_zero_prune_s0_2026_08_30"
PARENT_KEY = "same_module_all_producer_disposition_r0_2026_08_30"
D0_KEY = "same_module_rewrite_known_issuer_boundary_d0_2026_08_30"
KNOWN_REL = "src/mir/builder/rewrite/known.rs"
README_REL = "src/mir/builder/rewrite/README.md"
LEGACY_GUARD_REL = "tools/checks/lib/rewrite_header_p0_guard.py"
GUARD_REL = "tools/checks/lib/mir_call_d1b_rewrite_known_guard.py"

NO_DST = (
    "rewrite_call_args_for_signature",
    "try_known_rewrite",
    "try_known_rewrite_with_lookup",
    "try_unique_suffix_rewrite",
    "try_unique_suffix_rewrite_with_lookup",
    "try_known_or_unique",
    "try_known_or_unique_with_lookup",
)
LIVE_TO_DST = (
    "try_known_rewrite_to_dst",
    "try_known_rewrite_to_dst_with_lookup",
    "try_unique_suffix_rewrite_to_dst",
    "try_unique_suffix_rewrite_to_dst_with_lookup",
    "try_known_or_unique_to_dst",
    "try_known_or_unique_to_dst_with_lookup",
)


def _grep(root: Path, token: str) -> list[str]:
    result = subprocess.run(
        [
            "git",
            "grep",
            "-n",
            "-E",
            rf"{token}[[:space:]]*\(",
            "--",
            "src",
        ],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode not in (0, 1):
        raise SystemExit(
            f"[mir-call-d1b-rewrite-known] git grep failed for {token}: "
            f"{result.stderr.strip()}"
        )
    return [line for line in result.stdout.splitlines() if line.strip()]


def _paths(matches: list[str]) -> set[str]:
    return {line.split(":", 1)[0] for line in matches}


def _fail(api: object, message: str) -> None:
    api.fail(f"rewrite-known caller-zero S0: {message}")


def check_rewrite_known_caller_zero_s0(
    state: dict, card: dict, root: Path, api: object
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        _fail(api, "requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        _fail(api, "row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        _fail(api, "current_design_stop must be none")
    if state.get("next_execution_card") != ROW:
        _fail(api, "execution pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        _fail(api, "execution card path drifted")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict) or parent.get("status") != "accepted_with_open_blockers":
        _fail(api, "parent SameModule census is not blocker-open")
    d0 = card.get(D0_KEY)
    if not isinstance(d0, dict) or d0.get("status") != "accepted_design_stop_successor_parity_required":
        _fail(api, "rewrite/known D0 is not the accepted parity-open parent")
    row = card.get(KEY)
    if not isinstance(row, dict) or row.get("task_id") != ROW:
        _fail(api, "active row is missing")
    if row.get("status") not in {"fast_open", "landed"}:
        _fail(api, "row status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        _fail(api, "row permission/status drifted")

    expected_allowed = {
        KNOWN_REL,
        README_REL,
        LEGACY_GUARD_REL,
        GUARD_REL,
        str(api.HELPER_REL),
        str(api.STATE_REL),
        str(api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    declared = row.get("allowed_files")
    if not isinstance(declared, list) or set(declared) != expected_allowed:
        _fail(api, "allowed-file boundary drifted")

    known = root / KNOWN_REL
    readme = root / README_REL
    legacy_guard = root / LEGACY_GUARD_REL
    for path in (known, readme, legacy_guard, root / GUARD_REL):
        if not path.is_file():
            _fail(api, f"missing owner: {path.relative_to(root)}")
    if len(known.read_text(encoding="utf-8").splitlines()) >= 760:
        _fail(api, "known.rs reached the 760-line split boundary")
    if "caller-zero" not in readme.read_text(encoding="utf-8"):
        _fail(api, "README does not record the caller-zero boundary")

    for token in LIVE_TO_DST:
        if f"fn {token}(" not in known.read_text(encoding="utf-8"):
            _fail(api, f"retained _to_dst writer disappeared: {token}")
    live_external = {
        "try_known_rewrite_to_dst_with_lookup": "src/mir/builder/rewrite/special.rs",
        "try_unique_suffix_rewrite_to_dst_with_lookup": "src/mir/builder/rewrite/special.rs",
        "try_known_or_unique_to_dst_with_lookup": "src/mir/builder/calls/unified_emitter.rs",
    }
    for token, path in live_external.items():
        if path not in _paths(_grep(root, token)):
            _fail(api, f"retained live caller is missing: {path}::{token}")

    for token in NO_DST:
        matches = _grep(root, token)
        if row.get("status") == "fast_open":
            if not matches or _paths(matches) != {KNOWN_REL}:
                _fail(api, f"pre-delete caller-zero census is not exact: {token}")
        elif matches:
            _fail(api, f"deleted no-dst symbol remains in source: {token}")

    if row.get("status") == "landed":
        base = api.require_text(row.get("coverage_base_commit"), "rewrite-known coverage_base_commit")
        changed = api.git_diff_paths(root, base)
        if not changed.issubset(expected_allowed):
            _fail(api, f"changed paths escaped: {sorted(changed - expected_allowed)}")
