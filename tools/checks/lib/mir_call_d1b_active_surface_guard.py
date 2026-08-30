#!/usr/bin/env python3
"""Fail-closed guard for the live D1B proof surface.

The former lifecycle guard replayed every landed phase at HEAD.  This checker
keeps one registry entry and one current-row dispatch.  Landed phase evidence
is validated from the card as non-executable tombstones.
"""

from __future__ import annotations

import os
from pathlib import Path
import re
import subprocess
import sys
import tomllib

from mir_call_d1b_method_corridor_guard import (
    EXACT1_RETIRE_ROW,
    GUARD_SPLIT_ROW,
    METHOD_NONE_TERMINAL_ROW,
    RESOLVED_RETIRE_ROW,
    SAME_MODULE_PARENT_ROW,
    STATIC_RECEIPT_ROW,
    TEST_SPLIT_ROW,
    check_exact1_retire_i0,
    check_guard_split_s0,
    check_method_corridor_d0,
    check_method_none_terminal_ret0,
    check_method_resolution_ret0,
    check_resolved_retire_ret0,
    check_same_module_parent_r0,
    check_static_receipt_target_before_args_i0,
    check_test_split_s0,
)
from mir_call_d1b_cataloged_print_guard import (
    CATALOGED_PRINT_RETIRE_ROW,
    CATALOGED_PRINT_TARGET_ARM_PRUNE_ROW,
    check_cataloged_print_caller_zero_retire_i0,
    check_cataloged_print_target_arm_prune_r0,
)
from mir_call_d1b_same_module_target_only_guard import (
    ORDINARY_STATIC_TARGET_ONLY_I0_ROW,
    check_ordinary_static_target_only_i0,
)
from mir_call_d1b_rewrite_known_guard import (
    ROW as REWRITE_KNOWN_CALLER_ZERO_PRUNE_S0_ROW,
    check_rewrite_known_caller_zero_s0,
)
from mir_call_d1b_rewrite_known_policy_guard import (
    ROW as REWRITE_KNOWN_POLICY_RETIRE_I0_ROW,
    check_rewrite_known_policy_retire_i0,
)


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
SCRIPT_ROOT_ROW = "MIR-CALL-COMPAT-SCRIPT-ROOT-RET0"
RAW_LEGACY_ROW = "MIR-CALL-COMPAT-RAW-LEGACY-FATE-D0"
RAW_LEGACY_I0_ROW = "MIR-CALL-COMPAT-RAW-LEGACY-FATE-I0"
METHOD_CORRIDOR_D0_ROW = "MIR-CALL-METHOD-CORRIDOR-NONSTAGE1-PRODUCER-RETIRE-D0"
METHOD_RESOLUTION_RET0_ROW = "MIR-CALL-METHOD-RESOLUTION-STATIC-NONE-RET0"
CATALOGED_GC_RETIRE_ROW = "MIR-CALL-SAME-MODULE-CATALOGED-GC-RETIRE-I0"
CATALOGED_GC_RETIRE_KEY = "same_module_cataloged_gc_retire_i0_2026_08_30"
PROOF_KEY = "proof_reliability_followups_2026_08_29"
RAW_ROOT_KEY = "raw_root_main_retire_i0_2026_08_29"
SCRIPT_ROOT_KEY = "method_call_compat_script_root_ret0_2026_08_30"
RAW_LEGACY_KEY = "method_call_compat_raw_legacy_fate_d0_2026_08_30"
RAW_LEGACY_I0_KEY = "method_call_compat_raw_legacy_fate_i0_2026_08_30"
METHOD_CORRIDOR_D0_KEY = "method_corridor_nonstage1_producer_retire_d0_2026_08_29"
METHOD_RESOLUTION_RET0_KEY = "method_call_method_resolution_static_none_ret0_d0_2026_08_30"
TYPE_FACT_GUARD_PRUNE_S0_ROW = "MIRBUILDER-TYPE-FACT-PARTITION-GUARD-PRUNE-S0"
TYPE_FACT_GUARD_PRUNE_S0_KEY = "mirbuilder_type_fact_partition_guard_prune_s0_2026_08_30"
OPERATOR_ROW = "MIR-CALL-SAME-MODULE-OPERATOR-CALL-RETIRE-I0"
ORDINARY_NEW_I0_ROW = "MIR-CALL-SAME-MODULE-ORDINARY-NEW-EXACT-CONSTRUCTOR-CUTOVER-I0"
ORDINARY_NEW_I0_KEY = "same_module_ordinary_new_exact_constructor_cutover_i0_2026_08_30"
ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW = (
    "MIR-CALL-SAME-MODULE-ORDINARY-STATIC-LEGACY-COMPAT-RETIRE-I0"
)
ORDINARY_STATIC_LEGACY_RETIRE_I0_KEY = (
    "same_module_ordinary_static_legacy_compat_retire_i0_2026_08_30"
)
BARE_ERROR_RETIRE_ROW = "MIR-CALL-SAME-MODULE-CATALOGED-PROVIDER-BARE-ERROR-RETIRE-I0"
BARE_NOW_RETIRE_ROW = "MIR-CALL-SAME-MODULE-CATALOGED-PROVIDER-BARE-NOW-RETIRE-I0"
ACTIVE_SURFACE_ROWS_ROW = "MIR-CALL-GUARD-ACTIVE-SURFACE-ROWS-S0"
ACTIVE_SURFACE_ROWS_KEY = "active_surface_guard_rows_s0_2026_08_30"
ME_METHOD_CANONICAL_I0_ROW = (
    "MIR-CALL-SAME-MODULE-STATIC-CURRENT-OWNER-HANDOFF-I0"
)
ME_METHOD_CANONICAL_I0_KEY = "same_module_static_current_owner_handoff_i0_2026_08_30"


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


def require_text_list(value: object, label: str) -> list[str]:
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        fail(f"{label} must be a non-empty string list")
    return list(value)


def git_diff(root: Path, base: str) -> str:
    result = subprocess.run(
        ["git", "diff", "--unified=3", f"{base}..HEAD", "--", "*.rs"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail(f"cannot inspect implementation diff from {base}: {result.stderr.strip()}")
    return result.stdout


def git_diff_paths(root: Path, base: str) -> set[str]:
    result = subprocess.run(
        ["git", "diff", "--name-only", f"{base}..HEAD"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail(f"cannot inspect changed paths from {base}: {result.stderr.strip()}")
    return {line for line in result.stdout.splitlines() if line.strip()}


def changed_added_test_names(diff: str) -> set[str]:
    names: set[str] = set()
    test_attr_pending = False
    for line in diff.splitlines():
        if line.startswith("diff --git "):
            test_attr_pending = False
            continue
        if line.startswith(" "):
            if re.search(r"#\s*\[\s*test\s*\]", line[1:]):
                test_attr_pending = True
            continue
        if not line.startswith("+") or line.startswith("+++"):
            continue
        added = line[1:]
        if re.search(r"#\s*\[\s*test\s*\]", added):
            test_attr_pending = True
            continue
        if test_attr_pending:
            match = re.search(r"\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(", added)
            if match:
                names.add(match.group(1))
                test_attr_pending = False
            elif added.strip() and not added.lstrip().startswith("#"):
                test_attr_pending = False
    return names


def cargo_test_names(root: Path) -> list[str]:
    env = os.environ.copy()
    env.update(
        {
            "CARGO_BUILD_JOBS": "4",
            "CARGO_INCREMENTAL": "0",
            "CARGO_PROFILE_QUICK_CODEGEN_UNITS": "1",
        }
    )
    result = subprocess.run(
        ["cargo", "test", "--profile", "quick", "--lib", "--", "--list"],
        cwd=root,
        env=env,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        fail(f"cargo test -- --list failed: {result.stderr.strip()[-600:]}")
    names = []
    for line in result.stdout.splitlines():
        match = re.match(r"^(.+): test$", line)
        if match:
            names.append(match.group(1))
    if not names:
        fail("cargo test -- --list returned no tests")
    return names


def check_test_coverage(root: Path, proof: dict) -> None:
    if proof.get("status") != "landed":
        return
    base = require_text(proof.get("coverage_base_commit"), "coverage_base_commit")
    changed = changed_added_test_names(git_diff(root, base))
    expected = set(require_text_list(proof.get("changed_test_names"), "changed_test_names"))
    if changed != expected:
        fail(f"changed test inventory drifted; diff={sorted(changed)}, card={sorted(expected)}")
    filters = require_text_list(proof.get("focused_test_filters"), "focused_test_filters")
    listed = cargo_test_names(root)
    for name in sorted(changed):
        full_names = [item for item in listed if item.endswith("::" + name)]
        if len(full_names) != 1:
            fail(f"changed test {name} is not uniquely listed by cargo")
        if not any(token in full_names[0] for token in filters):
            fail(f"changed test {name} has no matching focused filter")
    for token in filters:
        if not any(token in item for item in listed):
            fail(f"focused test filter has zero cargo-list matches: {token}")


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


# Row-specific handlers live in mir_call_d1b_active_surface_rows.py.
# This parent keeps the shared contract, registry, tombstones, and dispatch.

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
    api = sys.modules[__name__]
    sys.modules.setdefault("mir_call_d1b_active_surface_guard", api)
    from mir_call_d1b_active_surface_rows import (
        check_active_surface_rows_s0,
        check_cataloged_gc_retire_i0,
        check_ordinary_new_i0,
        check_ordinary_static_legacy_retire_i0,
        check_proof_row,
        check_raw_legacy_i0,
        check_raw_legacy_resume,
        check_raw_root_resume,
        check_script_root_ret0,
        check_type_fact_guard_prune_s0,
    )
    if row == METHOD_ROW:
        check_proof_row(state, card, proof, root)
    elif row == RAW_ROOT_ROW:
        check_raw_root_resume(state, card, proof, root)
    elif row == SCRIPT_ROOT_ROW:
        check_script_root_ret0(state, card, root)
    elif row == METHOD_CORRIDOR_D0_ROW:
        check_method_corridor_d0(state, card, api)
    elif row == METHOD_RESOLUTION_RET0_ROW:
        check_method_resolution_ret0(state, card, root, api)
    elif row == GUARD_SPLIT_ROW:
        check_guard_split_s0(state, card, root, api)
    elif row == TEST_SPLIT_ROW:
        check_test_split_s0(state, card, root, api)
    elif row == EXACT1_RETIRE_ROW:
        check_exact1_retire_i0(state, card, root, api)
    elif row == METHOD_NONE_TERMINAL_ROW:
        check_method_none_terminal_ret0(state, card, root, api)
    elif row == RESOLVED_RETIRE_ROW:
        check_resolved_retire_ret0(state, card, root, api)
    elif row == STATIC_RECEIPT_ROW:
        check_static_receipt_target_before_args_i0(state, card, root, api)
    elif row == SAME_MODULE_PARENT_ROW:
        check_same_module_parent_r0(state, card, api)
    elif row == CATALOGED_GC_RETIRE_ROW:
        check_cataloged_gc_retire_i0(state, card, root)
    elif row == CATALOGED_PRINT_RETIRE_ROW:
        check_cataloged_print_caller_zero_retire_i0(state, card, root, api)
    elif row == CATALOGED_PRINT_TARGET_ARM_PRUNE_ROW:
        check_cataloged_print_target_arm_prune_r0(state, card, root, api)
    elif row == ORDINARY_STATIC_TARGET_ONLY_I0_ROW:
        check_ordinary_static_target_only_i0(state, card, root, api)
    elif row == RAW_LEGACY_ROW:
        check_raw_legacy_resume(state, card)
    elif row == RAW_LEGACY_I0_ROW:
        check_raw_legacy_i0(state, card, root)
    elif row == TYPE_FACT_GUARD_PRUNE_S0_ROW:
        check_type_fact_guard_prune_s0(state, card, root)
    elif row == OPERATOR_ROW:
        from mir_call_d1b_operator_retirement_guard import check_operator_retirement_i0
        check_operator_retirement_i0(state, card, root)
    elif row == ORDINARY_NEW_I0_ROW:
        check_ordinary_new_i0(state, card, root)
    elif row == ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW:
        check_ordinary_static_legacy_retire_i0(state, card, root)
    elif row == BARE_ERROR_RETIRE_ROW:
        from mir_call_d1b_bare_error_retire_guard import check_bare_error_retire_i0

        check_bare_error_retire_i0(state, card, root, api)
    elif row == BARE_NOW_RETIRE_ROW:
        from mir_call_d1b_bare_error_retire_guard import check_bare_now_retire_i0

        check_bare_now_retire_i0(state, card, root, api)
    elif row == ACTIVE_SURFACE_ROWS_ROW:
        check_active_surface_rows_s0(state, card, root, api)
    elif row == REWRITE_KNOWN_CALLER_ZERO_PRUNE_S0_ROW:
        check_rewrite_known_caller_zero_s0(state, card, root, api)
    elif row == REWRITE_KNOWN_POLICY_RETIRE_I0_ROW:
        check_rewrite_known_policy_retire_i0(state, card, root, api)
    elif row == ME_METHOD_CANONICAL_I0_ROW:
        from mir_call_d1b_me_method_cutover_guard import check_me_method_canonical_i0

        check_me_method_canonical_i0(state, card, root, api)
    else:
        fail(f"unsupported current row for this stable guard: {row!r}")
    print(f"[{TAG}] row={row} ok")


if __name__ == "__main__":
    main()
