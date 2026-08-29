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
PROOF_KEY = "proof_reliability_followups_2026_08_29"
RAW_ROOT_KEY = "raw_root_main_retire_i0_2026_08_29"
SCRIPT_ROOT_KEY = "method_call_compat_script_root_ret0_2026_08_30"
RAW_LEGACY_KEY = "method_call_compat_raw_legacy_fate_d0_2026_08_30"
RAW_LEGACY_I0_KEY = "method_call_compat_raw_legacy_fate_i0_2026_08_30"
METHOD_CORRIDOR_D0_KEY = "method_corridor_nonstage1_producer_retire_d0_2026_08_29"
METHOD_RESOLUTION_RET0_KEY = "method_call_method_resolution_static_none_ret0_d0_2026_08_30"


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
        "src/mir/resolved_semantics/callable_index.rs",
        "src/mir/resolved_semantics/direct_call_verifier.rs",
        "src/mir/resolved_semantics/tests.rs",
    }
    if not isinstance(allowed, list) or set(allowed) != expected:
        fail("active guard allowed-file boundary drifted")


def check_raw_root_resume(state: dict, card: dict, proof: dict, root: Path) -> None:
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
    if proof.get("status") != "landed" or proof.get("implementation_permission") is not False:
        fail("RawRootMain resume requires the proof row to be landed and closed")
    check_test_coverage(root, proof)


def check_script_root_ret0(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("ScriptRoot RET0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != SCRIPT_ROOT_ROW:
        fail("ScriptRoot RET0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("ScriptRoot RET0 must clear current_design_stop")
    if state.get("next_execution_card") != SCRIPT_ROOT_ROW:
        fail("ScriptRoot RET0 pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("ScriptRoot RET0 card pointer drifted")

    row = card.get(SCRIPT_ROOT_KEY)
    if not isinstance(row, dict):
        fail(f"{SCRIPT_ROOT_KEY} section is missing")
    if row.get("task_id") != SCRIPT_ROOT_ROW:
        fail("ScriptRoot RET0 task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("ScriptRoot RET0 status is not finite")
    expected_permission = row.get("status") == "fast_open"
    if row.get("implementation_permission") is not expected_permission:
        fail("ScriptRoot RET0 permission/status drifted")

    route_rel = Path("src/mir/builder/calls/function_call_preflight_route.rs")
    tests_rel = Path("src/mir/builder/calls/function_call_script_compatibility_tests.rs")
    retention_rel = Path(
        "src/mir/builder/normal_script_semantic_source_call_retention_tests.rs"
    )
    route = (root / route_rel).read_text()
    tests = (root / tests_rel).read_text()
    retention = (root / retention_rel).read_text()
    for path in (route_rel, tests_rel, retention_rel):
        if sum(1 for _ in (root / path).open()) >= 760:
            fail(f"ScriptRoot RET0 source reached the 760-line split boundary: {path}")

    completion_start = route.find("fn prepare_ordinary_function_completion_v1")
    completion_end = route.find("fn is_installed_non_unified_gc_builtin_v1")
    if completion_start < 0 or completion_end < completion_start:
        fail("ordinary completion owner cannot be located")
    completion = route[completion_start:completion_end]
    if "RawCompatibilityOrdinaryCallTerminalV1::ScriptRootRetired" not in completion:
        fail("ScriptRootRetired terminal is not issued by ordinary completion")
    if "RawCompatibilityOrdinaryCallTerminalV1::RawScriptRootRetired" not in completion:
        fail("RawScriptRootRetired precedence is not retained")
    if "RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired" not in completion:
        fail("RawRootMain typed retirement precedence is not retained")
    if "PreparedRawOrdinaryFunctionCompletionV1::Resolved" in completion:
        fail("retired shared Resolved compatibility arm reappeared")

    for token in (
        "script_root_parked_compatibility_retires_before_arguments",
        "raw_script_root_ordinary_call_retires_before_arguments",
        "raw_root_main_ordinary_call_retires_before_arguments",
        "script_root_parked_compatibility_keeps_brand_precedence",
        "raw_script_root_keeps_brand_and_special_precedence",
        "expression_count",
        "events.is_empty()",
        "before_instructions",
        "after_instructions",
    ):
        if token not in tests:
            fail(f"ScriptRoot RET0 test evidence is missing: {token}")
    if "script_function_call_remains_deferred_to_runtime_retirement_terminal" not in retention:
        fail("R4 semantic deferral test evidence is missing")
    if "RetainedExistingTerminal" not in retention:
        fail("R4 retained terminal contract is not recorded")

    allowed = row.get("allowed_files")
    expected_allowed = {
        str(route_rel),
        str(tests_rel),
        str(retention_rel),
        "src/mir/builder/calls/README.md",
        "docs/reference/language/function-call-evaluation.md",
        str(HELPER_REL),
        str(STATE_REL),
        str(CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        fail("ScriptRoot RET0 allowed-file boundary drifted")

    if row.get("status") == "landed":
        base = require_text(row.get("coverage_base_commit"), "ScriptRoot coverage_base_commit")
        changed = changed_added_test_names(git_diff(root, base))
        expected = set(require_text_list(row.get("changed_test_names"), "ScriptRoot changed_test_names"))
        if changed != expected:
            fail(f"ScriptRoot changed test inventory drifted; diff={sorted(changed)}, card={sorted(expected)}")
        filters = require_text_list(row.get("focused_test_filters"), "ScriptRoot focused_test_filters")
        listed = cargo_test_names(root)
        for name in sorted(changed):
            full_names = [item for item in listed if item.endswith("::" + name)]
            if len(full_names) != 1:
                fail(f"ScriptRoot changed test {name} is not uniquely listed by cargo")
            if not any(token in full_names[0] for token in filters):
                fail(f"ScriptRoot changed test {name} has no matching focused filter")
        for token in filters:
            if not any(token in item for item in listed):
                fail(f"ScriptRoot focused test filter has zero cargo-list matches: {token}")
        changed_paths = git_diff_paths(root, base)
        if not changed_paths.issubset(expected_allowed):
            fail(
                "ScriptRoot changed paths exceed allowed boundary: "
                f"{sorted(changed_paths - expected_allowed)}"
            )


def check_raw_legacy_resume(state: dict, card: dict) -> None:
    if state.get("work_mode") != "design_stop":
        fail("RawLegacy fate census must remain design_stop")
    if state.get("current_execution_row") != RAW_LEGACY_ROW:
        fail("RawLegacy fate census row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != RAW_LEGACY_ROW:
        fail("RawLegacy fate census design stop drifted")
    row = card.get(RAW_LEGACY_KEY)
    if not isinstance(row, dict):
        fail(f"{RAW_LEGACY_KEY} section is missing")
    if row.get("task_id") != RAW_LEGACY_ROW:
        fail("RawLegacy fate census task id drifted")
    if row.get("status") != "design_stop":
        fail("RawLegacy fate census is not an active design stop")
    if row.get("implementation_permission") is not False:
        fail("RawLegacy fate census must keep implementation closed")
    census = require_text(row.get("production_reach_census"), "RawLegacy production_reach_census")
    for token in (
        "structural_sites = 1",
        "production_reachable_callers = 1",
        "test_only_authority_injection_helpers = 1",
        "test_only_production_reachable_callers = 0",
        "public_contract_owners = 0",
    ):
        if token not in census:
            fail(f"RawLegacy census lacks {token}")
    if "physical_facade_entries = multiple" not in census:
        fail("RawLegacy census lacks physical facade denominator")
    boundary = require_text(row.get("boundary"), "RawLegacy boundary")
    for token in ("RawLegacy", "shared Resolved", "pre-effect"):
        if token not in boundary:
            fail(f"RawLegacy boundary lacks {token}")


def check_raw_legacy_i0(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("RawLegacy I0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != RAW_LEGACY_I0_ROW:
        fail("RawLegacy I0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("RawLegacy I0 must clear current_design_stop")
    if state.get("next_execution_card") != RAW_LEGACY_I0_ROW:
        fail("RawLegacy I0 pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("RawLegacy I0 card pointer drifted")

    row = card.get(RAW_LEGACY_I0_KEY)
    if not isinstance(row, dict):
        fail(f"{RAW_LEGACY_I0_KEY} section is missing")
    if row.get("task_id") != RAW_LEGACY_I0_ROW:
        fail("RawLegacy I0 task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("RawLegacy I0 status is not finite")
    expected_permission = row.get("status") == "fast_open"
    if row.get("implementation_permission") is not expected_permission:
        fail("RawLegacy I0 permission/status drifted")

    route_rel = Path("src/mir/builder/calls/function_call_preflight_route.rs")
    tests_rel = Path("src/mir/builder/calls/function_call_script_compatibility_tests.rs")
    route = (root / route_rel).read_text()
    tests = (root / tests_rel).read_text()
    for path in (route_rel, tests_rel):
        if sum(1 for _ in (root / path).open()) >= 760:
            fail(f"RawLegacy I0 source reached the 760-line split boundary: {path}")

    if "RawLegacyRetired" not in route:
        fail("RawLegacyRetired terminal is not defined or issued")
    completion_start = route.find("fn prepare_ordinary_function_completion_v1")
    completion_end = route.find("fn is_installed_non_unified_gc_builtin_v1")
    if completion_start < 0 or completion_end < completion_start:
        fail("ordinary completion owner cannot be located")
    completion = route[completion_start:completion_end]
    if "RawLegacyRetired" not in completion:
        fail("RawLegacyRetired is not issued by ordinary completion")
    if "RawCompatibilityOrdinaryCallTerminalV1::RawRootMainRetired" not in completion:
        fail("RawRootMain typed retirement precedence is not retained")
    if "PreparedRawOrdinaryFunctionCompletionV1::Resolved" in completion:
        fail("retired shared Resolved compatibility arm reappeared")
    for token in (
        "raw_legacy_parked_compatibility_retires_before_arguments",
        "raw_root_main_ordinary_call_retires_before_arguments",
        "raw_legacy_port_issues_named_compatibility_provenance",
        "raw_script_root_keeps_brand_and_special_precedence",
        "expression_count",
        "events.is_empty()",
        "before_instructions",
        "after_instructions",
    ):
        if token not in tests:
            fail(f"RawLegacy I0 test evidence is missing: {token}")
    for token in (
        "RawLegacyParkedCompatibility",
        "Brand",
        "TypeOp",
        "Math",
        "FastMem",
        "str/1",
    ):
        if token not in route:
            fail(f"RawLegacy I0 precedence evidence is missing: {token}")

    allowed = row.get("allowed_files")
    expected_allowed = {
        str(route_rel),
        str(tests_rel),
        "src/mir/builder/calls/function_call_preflight_route_tests.rs",
        "src/mir/builder/calls/function_call_installed_gc_builtin_tests.rs",
        str(HELPER_REL),
        str(STATE_REL),
        str(CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        "docs/reference/language/function-call-evaluation.md",
        "src/mir/builder/calls/README.md",
    }
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        fail("RawLegacy I0 allowed-file boundary drifted")

    if row.get("status") == "landed":
        base = require_text(row.get("coverage_base_commit"), "RawLegacy I0 coverage_base_commit")
        changed = changed_added_test_names(git_diff(root, base))
        expected = set(require_text_list(row.get("changed_test_names"), "RawLegacy I0 changed_test_names"))
        if changed != expected:
            fail(f"RawLegacy I0 changed test inventory drifted; diff={sorted(changed)}, card={sorted(expected)}")
        filters = require_text_list(row.get("focused_test_filters"), "RawLegacy I0 focused_test_filters")
        listed = cargo_test_names(root)
        for name in sorted(changed):
            full_names = [item for item in listed if item.endswith("::" + name)]
            if len(full_names) != 1:
                fail(f"RawLegacy I0 changed test {name} is not uniquely listed by cargo")
            if not any(token in full_names[0] for token in filters):
                fail(f"RawLegacy I0 changed test {name} has no matching focused filter")
        for token in filters:
            if not any(token in item for item in listed):
                fail(f"RawLegacy I0 focused test filter has zero cargo-list matches: {token}")
        changed_paths = git_diff_paths(root, base)
        if not changed_paths.issubset(expected_allowed):
            fail(
                "RawLegacy I0 changed paths exceed allowed boundary: "
                f"{sorted(changed_paths - expected_allowed)}"
            )


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
    elif row == RAW_LEGACY_ROW:
        check_raw_legacy_resume(state, card)
    elif row == RAW_LEGACY_I0_ROW:
        check_raw_legacy_i0(state, card, root)
    else:
        fail(f"unsupported current row for this stable guard: {row!r}")
    print(f"[{TAG}] row={row} ok")


if __name__ == "__main__":
    main()
