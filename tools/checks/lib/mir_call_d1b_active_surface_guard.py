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
    check_cataloged_print_caller_zero_retire_i0,
)
from mir_call_d1b_same_module_target_only_guard import (
    ORDINARY_STATIC_TARGET_ONLY_I0_ROW,
    check_ordinary_static_target_only_i0,
)
from mir_call_d1b_rewrite_known_guard import (
    ROW as REWRITE_KNOWN_CALLER_ZERO_PRUNE_S0_ROW,
    check_rewrite_known_caller_zero_s0,
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


def check_cataloged_gc_retire_i0(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("cataloged GC retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != CATALOGED_GC_RETIRE_ROW:
        fail("cataloged GC retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("cataloged GC retirement must clear current_design_stop")
    if state.get("next_execution_card") != CATALOGED_GC_RETIRE_ROW:
        fail("cataloged GC retirement pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("cataloged GC retirement card pointer drifted")

    row = card.get(CATALOGED_GC_RETIRE_KEY)
    if not isinstance(row, dict):
        fail(f"{CATALOGED_GC_RETIRE_KEY} section is missing")
    if row.get("task_id") != CATALOGED_GC_RETIRE_ROW:
        fail("cataloged GC retirement task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("cataloged GC retirement status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        fail("cataloged GC retirement permission/status drifted")

    route_rel = Path("src/mir/builder/calls/function_call_preflight_route.rs")
    build_rel = Path("src/mir/builder/calls/build.rs")
    tests_rel = Path("src/mir/builder/calls/function_call_installed_gc_builtin_tests.rs")
    route = (root / route_rel).read_text()
    tests = (root / tests_rel).read_text()
    completion_start = route.find("fn prepare_ordinary_function_completion_v1")
    completion_end = route.find("fn is_installed_non_unified_gc_builtin_v1")
    if completion_start < 0 or completion_end < completion_start:
        fail("cataloged GC ordinary completion owner cannot be located")
    completion = route[completion_start:completion_end]
    gc_pos = completion.find("is_installed_non_unified_gc_builtin_v1(name)")
    caller_pos = completion.find("else if let Some(caller) = caller")
    if gc_pos < 0 or caller_pos < 0 or gc_pos > caller_pos:
        fail("cataloged GC retirement is not before caller target preparation")
    if "PreparedRawOrdinaryFunctionCompletionV1::Retired" not in completion:
        fail("cataloged GC retirement does not use the typed retirement variant")
    for token in (
        "cataloged_gc_names_reject_before_target_synthesis",
        "installed_gc_names_reject_before_arguments",
        "installed_gc_rejection_does_not_descend_or_publish",
        "RawOrdinaryFunctionRetirementV1::GcGlobal",
    ):
        if token not in tests:
            fail(f"cataloged GC retirement test evidence is missing: {token}")
    for path in (route_rel, build_rel, tests_rel):
        if sum(1 for _ in (root / path).open()) >= 760:
            fail(f"cataloged GC retirement source reached the 760-line boundary: {path}")

    expected_allowed = {
        str(route_rel),
        "src/mir/builder/calls/function_call_preflight_route_tests.rs",
        str(tests_rel),
        "src/mir/builder/calls/build.rs",
        "src/mir/builder/calls/README.md",
        str(HELPER_REL),
        str(STATE_REL),
        str(CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        fail("cataloged GC retirement allowed-file boundary drifted")
    if row.get("status") == "landed":
        check_test_coverage(root, row)


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


def check_type_fact_guard_prune_s0(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("type-fact guard prune requires fast or closeout work_mode")
    if state.get("current_execution_row") != TYPE_FACT_GUARD_PRUNE_S0_ROW:
        fail("type-fact guard prune row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("type-fact guard prune must clear current_design_stop")
    if state.get("next_execution_card") != TYPE_FACT_GUARD_PRUNE_S0_ROW:
        fail("type-fact guard prune pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("type-fact guard prune card pointer drifted")

    row = card.get(TYPE_FACT_GUARD_PRUNE_S0_KEY)
    if not isinstance(row, dict):
        fail(f"{TYPE_FACT_GUARD_PRUNE_S0_KEY} section is missing")
    if row.get("task_id") != TYPE_FACT_GUARD_PRUNE_S0_ROW:
        fail("type-fact guard prune task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("type-fact guard prune status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        fail("type-fact guard prune permission/status drifted")

    expected_files = frozenset(("tools/checks/lib/mirbuilder_type_fact_partition_guard.py", "tools/checks/lib/mirbuilder_type_fact_partition_guard_tests.py", "tools/checks/lib/mirbuilder_type_fact_call_post_success_guard.py", "tools/checks/lib/mirbuilder_type_fact_call_post_success_guard_tests.py", "tools/checks/guard_rows.toml", str(HELPER_REL), "tools/checks/lib/mir_call_d1b_operator_retirement_guard.py", str(STATE_REL), str(CARD_REL), "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md"))
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_files:
        fail("type-fact guard prune allowed-file boundary is missing")

    parent = root / "tools/checks/lib/mirbuilder_type_fact_partition_guard.py"
    sibling = root / "tools/checks/lib/mirbuilder_type_fact_call_post_success_guard.py"
    if len(parent.read_text(encoding="utf-8").splitlines()) >= 760:
        fail("retained type-fact parent reached the 760-line split boundary")
    if len(sibling.read_text(encoding="utf-8").splitlines()) >= 800:
        fail("rehomed type-fact sibling reached the 800-line hard stop")


def check_ordinary_new_i0(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("ordinary-new I0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != ORDINARY_NEW_I0_ROW:
        fail("ordinary-new I0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("ordinary-new I0 must clear current_design_stop")
    if state.get("next_execution_card") != ORDINARY_NEW_I0_ROW:
        fail("ordinary-new I0 pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("ordinary-new I0 card pointer drifted")

    row = card.get(ORDINARY_NEW_I0_KEY)
    if not isinstance(row, dict):
        fail(f"{ORDINARY_NEW_I0_KEY} section is missing")
    if row.get("task_id") != ORDINARY_NEW_I0_ROW:
        fail("ordinary-new I0 task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("ordinary-new I0 status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        fail("ordinary-new I0 permission/status drifted")
    parent = card.get("same_module_ordinary_new_birth_target_d0_2026_08_30")
    if not isinstance(parent, dict) or parent.get("status") != "accepted_design_stop_exact_constructor_relation_required":
        fail("ordinary-new I0 parent design is not accepted")
    allowed = row.get("allowed_files")
    expected_allowed = {
        "src/parser/source_authority/constructor_source.rs",
        "src/parser/constructor_source_catalog.rs",
        "src/parser/normal_callable_program_source/ordinary_new_source.rs",
        "src/mir/instance_constructor_abi.rs",
        "src/mir/mod.rs",
        "src/mir/normal_callable_semantic_package/instance_constructor_semantic.rs",
        "src/mir/normal_callable_semantic_package/ordinary_new_coseal.rs",
        "src/mir/normal_callable_semantic_package/issuer.rs",
        "src/mir/normal_callable_semantic_package/mod.rs",
        "src/mir/builder/module_lowering_invocation.rs",
        "src/mir/builder/normal_instance_constructor_admission.rs",
        "src/mir/builder/ordinary_new_admission.rs",
        "src/mir/builder/raw_root_physical/callable_main_terminal.rs",
        "src/mir/builder/raw_root_physical/child_terminal.rs",
        "src/mir/builder/README.md",
        "src/mir/normal_callable_semantic_package/README.md",
        "src/parser/source_authority.rs",
        "src/parser/public_api.rs",
        "src/parser/source_authority_tests.rs",
        str(HELPER_REL),
        str(STATE_REL),
        str(CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        fail("ordinary-new I0 allowed-file boundary drifted")
    for rel in (
        Path("src/parser/source_authority/constructor_source.rs"),
        Path("src/parser/constructor_source_catalog.rs"),
        Path("src/parser/normal_callable_program_source/ordinary_new_source.rs"),
        Path("src/mir/normal_callable_semantic_package/instance_constructor_semantic.rs"),
        Path("src/mir/normal_callable_semantic_package/ordinary_new_coseal.rs"),
        Path("src/mir/builder/normal_instance_constructor_admission.rs"),
        Path("src/mir/builder/ordinary_new_admission.rs"),
    ):
        if sum(1 for _ in (root / rel).open()) >= 760:
            fail(f"ordinary-new I0 source reached the 760-line split boundary: {rel}")
    abi = root / "src/mir/instance_constructor_abi.rs"
    if abi.exists() and len(abi.read_text(encoding="utf-8").splitlines()) >= 760:
        fail("ordinary-new ABI owner reached the 760-line split boundary")
    if row.get("status") == "landed":
        check_test_coverage(root, row)
        base = require_text(row.get("coverage_base_commit"), "ordinary-new coverage_base_commit")
        changed_paths = git_diff_paths(root, base)
        if not changed_paths.issubset(expected_allowed):
            fail(
                "ordinary-new I0 changed paths exceed allowed boundary: "
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
    elif row == CATALOGED_GC_RETIRE_ROW:
        check_cataloged_gc_retire_i0(state, card, root)
    elif row == CATALOGED_PRINT_RETIRE_ROW:
        check_cataloged_print_caller_zero_retire_i0(state, card, root, api)
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
    elif row == REWRITE_KNOWN_CALLER_ZERO_PRUNE_S0_ROW:
        check_rewrite_known_caller_zero_s0(state, card, root, api)
    else:
        fail(f"unsupported current row for this stable guard: {row!r}")
    print(f"[{TAG}] row={row} ok")


if __name__ == "__main__":
    main()
