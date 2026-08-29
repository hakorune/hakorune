#!/usr/bin/env python3
"""Fail-closed guard for the live D1B proof surface.

The former lifecycle guard replayed every landed phase at HEAD.  This checker
keeps one registry entry and one current-row dispatch.  Landed phase evidence
is validated from the card as non-executable tombstones.
"""

from __future__ import annotations

from pathlib import Path
import re
import sys
import tomllib


TAG = "mir-call-d1b-active-surface"
CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml"
)
STATE_REL = Path("docs/development/current/main/CURRENT_STATE.toml")
REGISTRY_REL = Path("tools/checks/guard_rows.toml")
ENTRY_REL = Path("tools/checks/mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh")
HELPER_REL = Path("tools/checks/lib/mir_call_d1b_active_surface_guard.py")
METHOD_ROW = "MIR-CALL-GUARD-ACTIVE-SURFACE-PRUNE-R0"
RAW_ROOT_ROW = "MIR-CALL-COMPAT-RAW-ROOT-MAIN-RETIRE-I0"
PROOF_KEY = "proof_reliability_followups_2026_08_29"
RAW_ROOT_KEY = "raw_root_main_retire_i0_2026_08_29"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] {message}")


def load_toml(path: Path) -> dict:
    try:
        with path.open("rb") as stream:
            value = tomllib.load(stream)
    except (OSError, tomllib.TOMLDecodeError) as exc:
        fail(f"cannot load {path}: {exc}")
    if not isinstance(value, dict):
        fail(f"{path} is not a TOML table")
    return value


def require_text(value: object, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        fail(f"{label} must be a non-empty string")
    return value


def check_registry(registry: dict) -> None:
    rows = registry.get("rows")
    if not isinstance(rows, list):
        fail("guard_rows.toml rows table is missing")
    matches = [
        row
        for row in rows
        if isinstance(row, dict)
        and row.get("id") == "mir-call-d1b-cataloged-affine-loan-lifecycle"
    ]
    if len(matches) != 1:
        fail(f"expected one lifecycle registry row, found {len(matches)}")
    row = matches[0]
    if row.get("profiles") != ["pilot", "quick-static"]:
        fail("lifecycle guard profiles drifted")
    if row.get("cmd") != ["bash", str(ENTRY_REL)]:
        fail("lifecycle guard command drifted")
    if sum(
        1
        for item in rows
        if isinstance(item, dict)
        and item.get("id") == "mir-call-d1b-cataloged-affine-loan-lifecycle"
    ) != 1:
        fail("lifecycle guard id is duplicated")


def check_tombstones(proof: dict) -> None:
    tombstones = proof.get("historical_phase_tombstones")
    if not isinstance(tombstones, dict) or not tombstones:
        fail("historical phase tombstones are missing")
    expected = {
        "readiness",
        "bridge_ready",
        "observer_i0",
        "observer_i0_verifier_corrective",
        "cataloged_source_coseal_validation",
        "main_observation_gate_corrective_r0",
        "main_root_owner_forest_validation_r0",
        "main_root_identity_coseal_i0",
        "main_raw_cataloged_handoff_d0",
        "main_raw_cataloged_route_r0",
        "main_raw_lineage_handoff_d1",
        "main_raw_lineage_witness_harden_r0",
        "qualified_method_target_issuer_d0",
        "qualified_method_target_issuer_i0",
        "cataloged_source_relation_affine_loan_i0",
        "installed_nonbrand_pre_effect_reject_r2a",
        "resolved_compatibility_provenance_r2b",
        "resolved_compatibility_provenance_r2c",
        "resolved_compatibility_unclassified_r2d",
        "method_corridor_explicit_compat_ingress_i0",
        "method_corridor_nonstage1_producer_retire_d0",
        "raw_script_root_pre_effect_retire_i0",
    }
    if set(tombstones) != expected:
        missing = sorted(expected - set(tombstones))
        extra = sorted(set(tombstones) - expected)
        fail(f"historical tombstone inventory drifted; missing={missing}, extra={extra}")
    for phase, record in tombstones.items():
        if not isinstance(record, str):
            fail(f"historical tombstone {phase} is not text")
        for token in ("owner=", "landed=", "superseded_by="):
            if token not in record:
                fail(f"historical tombstone {phase} lacks {token}")
        if "superseded_by=" + METHOD_ROW not in record:
            fail(f"historical tombstone {phase} points at a different successor")
        if re.search(r"landed=([0-9a-f]{10,40})", record) is None:
            fail(f"historical tombstone {phase} lacks a git commit id")


def check_proof_row(state: dict, card: dict, proof: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("active proof row requires fast or closeout work_mode")
    if state.get("current_execution_row") != METHOD_ROW:
        fail("active proof row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("active proof row must clear current_design_stop")
    if state.get("next_execution_card") != METHOD_ROW:
        fail("active proof row pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("active proof row card pointer drifted")
    if proof.get("status") not in {"fast_open", "landed"}:
        fail("proof row status is not finite")
    expected_permission = proof.get("status") == "fast_open"
    if proof.get("implementation_permission") is not expected_permission:
        fail("proof row permission/status drifted")
    if card.get("implementation_permission") is not False:
        fail("semantic Method card permission must remain closed")
    helper = root / HELPER_REL
    entry = root / ENTRY_REL
    if not helper.is_file() or not entry.is_file():
        fail("active guard entry/helper is missing")
    for path in (entry, helper):
        if sum(1 for _ in path.open()) >= 760:
            fail(f"active guard owner reached the 760-line split boundary: {path}")
    contract = require_text(proof.get("active_surface_contract"), "active_surface_contract")
    for token in ("registered shell entrypoint", "explicit phase argument", "below 760"):
        if token not in contract:
            fail(f"active guard contract lacks {token}")
    allowed = proof.get("active_surface_allowed_files")
    expected = {
        str(ENTRY_REL),
        str(HELPER_REL),
        str(REGISTRY_REL),
        str(STATE_REL),
        str(CARD_REL),
    }
    if not isinstance(allowed, list) or set(allowed) != expected:
        fail("active guard allowed-file boundary drifted")


def check_raw_root_resume(state: dict, card: dict) -> None:
    if state.get("work_mode") != "design_stop":
        fail("RawRootMain resume must remain design_stop")
    if state.get("current_execution_row") != RAW_ROOT_ROW:
        fail("RawRootMain resume row drifted")
    if state.get("current_design_stop") != RAW_ROOT_ROW:
        fail("RawRootMain resume design stop drifted")
    raw_root = card.get(RAW_ROOT_KEY)
    if not isinstance(raw_root, dict):
        fail("RawRootMain row is missing")
    if raw_root.get("status") != "caller_zero_reconciled":
        fail("RawRootMain caller-zero reconciliation is not closed")
    if raw_root.get("implementation_permission") is not False:
        fail("RawRootMain semantic permission must remain closed")
    evidence = require_text(raw_root.get("caller_zero_evidence"), "RawRootMain caller_zero_evidence")
    for token in ("UnsupportedSurface(Call)", "before physical open", "production FunctionCall reach = 0"):
        if token not in evidence:
            fail(f"RawRootMain evidence lacks {token}")


def main() -> None:
    if len(sys.argv) != 2:
        fail("usage: mir_call_d1b_active_surface_guard.py ROOT")
    root = Path(sys.argv[1]).resolve()
    for rel in (CARD_REL, STATE_REL, REGISTRY_REL, ENTRY_REL, HELPER_REL):
        if not (root / rel).exists():
            fail(f"missing owner {rel}")
    state = load_toml(root / STATE_REL)
    card = load_toml(root / CARD_REL)
    registry = load_toml(root / REGISTRY_REL)
    check_registry(registry)
    proof = card.get(PROOF_KEY)
    if not isinstance(proof, dict):
        fail(f"{PROOF_KEY} section is missing")
    check_tombstones(proof)
    row = state.get("current_execution_row")
    if row == METHOD_ROW:
        check_proof_row(state, card, proof, root)
    elif row == RAW_ROOT_ROW:
        check_raw_root_resume(state, card)
    else:
        fail(f"unsupported current row for this stable guard: {row!r}")
    print(f"[{TAG}] row={row} ok")


if __name__ == "__main__":
    main()
