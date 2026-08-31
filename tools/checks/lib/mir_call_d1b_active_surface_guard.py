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
DECLARED_INSTANCE_RELATION_I0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-RESOLVER-RELATION-I0"
)
DECLARED_INSTANCE_RELATION_ISSUER_D0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-RELATION-ISSUER-D0"
)
DECLARED_INSTANCE_RELATION_ISSUER_D0_KEY = (
    "mir_call_me_declared_instance_relation_issuer_d0_2026_08_31"
)
DECLARED_INSTANCE_EFFECT_ISSUER_D0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-EFFECT-ISSUER-D0"
)
DECLARED_INSTANCE_EFFECT_ISSUER_D0_KEY = (
    "mir_call_me_declared_instance_effect_issuer_d0_2026_08_31"
)
DECLARED_INSTANCE_EFFECT_ISSUER_I0_ROW = (
    "LANG-ORDINARY-DECLARED-INSTANCE-CALL-EFFECT-ISSUER-I0"
)
DECLARED_INSTANCE_EFFECT_ISSUER_I0_KEY = (
    "lang_ordinary_declared_instance_call_effect_issuer_i0_2026_08_31"
)
DECLARED_INSTANCE_PACKAGE_COSEAL_D0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-PACKAGE-COSEAL-D0"
)
DECLARED_INSTANCE_PACKAGE_COSEAL_D0_KEY = (
    "mir_call_me_declared_instance_package_coseal_d0_2026_08_31"
)
DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-PACKAGE-PRIVATE-LOCATOR-I0"
)
DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_KEY = (
    "mir_call_me_declared_instance_package_private_locator_i0_2026_08_31"
)
DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0"
)
DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_KEY = (
    "mir_call_me_declared_instance_selected_c_admission_d0_2026_08_31"
)
DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-LOCATOR-INSTALL-BRIDGE-I0"
)
DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_KEY = (
    "mir_call_me_declared_instance_locator_install_bridge_i0_2026_08_31"
)
SELECTED_C_STACK_ROW = "NY-LLVMC-SELECTED-LAUNCH-SNAPSHOT-STACK-RETIRE-R0"
SELECTED_C_STACK_KEY = "ny_llvmc_selected_launch_snapshot_stack_retire_r0_2026_08_31"
CSE_SAME_BLOCK_ROW = "MIR-CSE-SAME-BLOCK-STATS-DETERMINISM-R0"
CSE_SAME_BLOCK_KEY = "mir_cse_same_block_stats_determinism_r0_2026_09_01"


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


def check_declared_instance_relation_issuer_d0(state: dict, card: dict) -> None:
    if state.get("work_mode") != "design_stop":
        fail("DeclaredInstance relation issuer must remain design_stop")
    if state.get("current_execution_row") != DECLARED_INSTANCE_RELATION_ISSUER_D0_ROW:
        fail("DeclaredInstance relation issuer row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != DECLARED_INSTANCE_RELATION_ISSUER_D0_ROW:
        fail("DeclaredInstance relation issuer design stop drifted")
    if state.get("next_design_card") != DECLARED_INSTANCE_RELATION_ISSUER_D0_ROW:
        fail("DeclaredInstance relation issuer next design card drifted")
    if state.get("next_execution_card") != "none":
        fail("DeclaredInstance relation issuer design stop must keep next_execution_card=none")
    row = card.get(DECLARED_INSTANCE_RELATION_ISSUER_D0_KEY)
    if not isinstance(row, dict):
        fail(f"{DECLARED_INSTANCE_RELATION_ISSUER_D0_KEY} section is missing")
    if row.get("task_id") != DECLARED_INSTANCE_RELATION_ISSUER_D0_ROW:
        fail("DeclaredInstance relation issuer task id drifted")
    if row.get("status") != "accepted_design_stop":
        fail("DeclaredInstance relation issuer must remain an accepted design stop")
    if row.get("implementation_permission") is not False:
        fail("DeclaredInstance relation issuer cannot permit production implementation")
    child = card.get("mir_call_me_declared_instance_resolver_relation_i0_2026_08_31")
    if not isinstance(child, dict) or child.get("status") != "landed":
        fail("DeclaredInstance relation issuer requires the source relation child to be landed")


def check_declared_instance_effect_issuer_d0(state: dict, card: dict) -> None:
    if state.get("work_mode") != "design_stop":
        fail("DeclaredInstance effect issuer must remain design_stop")
    if state.get("current_execution_row") != DECLARED_INSTANCE_EFFECT_ISSUER_D0_ROW:
        fail("DeclaredInstance effect issuer row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != DECLARED_INSTANCE_EFFECT_ISSUER_D0_ROW:
        fail("DeclaredInstance effect issuer design stop drifted")
    if state.get("next_design_card") != DECLARED_INSTANCE_EFFECT_ISSUER_D0_ROW:
        fail("DeclaredInstance effect issuer next design card drifted")
    if state.get("next_execution_card") != "none":
        fail("DeclaredInstance effect issuer design stop must keep next_execution_card=none")
    row = card.get(DECLARED_INSTANCE_EFFECT_ISSUER_D0_KEY)
    if not isinstance(row, dict):
        fail(f"{DECLARED_INSTANCE_EFFECT_ISSUER_D0_KEY} section is missing")
    if row.get("task_id") != DECLARED_INSTANCE_EFFECT_ISSUER_D0_ROW:
        fail("DeclaredInstance effect issuer task id drifted")
    if row.get("status") != "accepted_design_stop":
        fail("DeclaredInstance effect issuer must remain an accepted design stop")
    if row.get("implementation_permission") is not False:
        fail("DeclaredInstance effect issuer cannot permit implementation")
    relation = card.get(DECLARED_INSTANCE_RELATION_ISSUER_D0_KEY)
    if not isinstance(relation, dict):
        fail("DeclaredInstance effect issuer requires the relation design section")
    child = card.get("mir_call_me_declared_instance_resolver_relation_i0_2026_08_31")
    if not isinstance(child, dict) or child.get("status") != "landed":
        fail("DeclaredInstance effect issuer requires the source relation child to be landed")
    result = card.get("mir_normal_callable_result_contract_retention_d0_i0_2026_08_31")
    if not isinstance(result, dict) or not str(result.get("status", "")).startswith("landed"):
        fail("DeclaredInstance effect issuer requires result/completion retention")


def check_declared_instance_effect_issuer_i0(
    state: dict, card: dict, root: Path
) -> None:
    if state.get("work_mode") != "fast":
        fail("DeclaredInstance effect issuer I0 must be fast")
    if state.get("current_execution_row") != DECLARED_INSTANCE_EFFECT_ISSUER_I0_ROW:
        fail("DeclaredInstance effect issuer I0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("DeclaredInstance effect issuer I0 must clear current_design_stop")
    if state.get("next_execution_card") != DECLARED_INSTANCE_EFFECT_ISSUER_I0_ROW:
        fail("DeclaredInstance effect issuer I0 next_execution_card drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("DeclaredInstance effect issuer I0 card path drifted")
    row = card.get(DECLARED_INSTANCE_EFFECT_ISSUER_I0_KEY)
    if not isinstance(row, dict):
        fail(f"{DECLARED_INSTANCE_EFFECT_ISSUER_I0_KEY} section is missing")
    if row.get("task_id") != DECLARED_INSTANCE_EFFECT_ISSUER_I0_ROW:
        fail("DeclaredInstance effect issuer I0 task id drifted")
    if row.get("status") != "selected_fast":
        fail("DeclaredInstance effect issuer I0 must be selected_fast")
    if row.get("implementation_permission") is not True:
        fail("DeclaredInstance effect issuer I0 must permit only its bounded implementation")
    d0 = card.get(DECLARED_INSTANCE_EFFECT_ISSUER_D0_KEY)
    if not isinstance(d0, dict) or d0.get("status") != "accepted_design_stop":
        fail("DeclaredInstance effect issuer I0 requires the accepted D0 design")
    relation = card.get(DECLARED_INSTANCE_RELATION_ISSUER_D0_KEY)
    if not isinstance(relation, dict):
        fail("DeclaredInstance effect issuer I0 requires the relation design section")
    child = card.get("mir_call_me_declared_instance_resolver_relation_i0_2026_08_31")
    if not isinstance(child, dict) or child.get("status") != "landed":
        fail("DeclaredInstance effect issuer I0 requires the source relation child")
    result = card.get("mir_normal_callable_result_contract_retention_d0_i0_2026_08_31")
    if not isinstance(result, dict) or not str(result.get("status", "")).startswith("landed"):
        fail("DeclaredInstance effect issuer I0 requires result/completion retention")
    check_declared_instance_effect_issuer_structure(root)


def check_declared_instance_package_coseal_d0(state: dict, card: dict) -> None:
    if state.get("work_mode") != "design_stop":
        fail("DeclaredInstance package co-seal must remain design_stop")
    if state.get("current_execution_row") != DECLARED_INSTANCE_PACKAGE_COSEAL_D0_ROW:
        fail("DeclaredInstance package co-seal row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != DECLARED_INSTANCE_PACKAGE_COSEAL_D0_ROW:
        fail("DeclaredInstance package co-seal design stop drifted")
    if state.get("next_design_card") != DECLARED_INSTANCE_PACKAGE_COSEAL_D0_ROW:
        fail("DeclaredInstance package co-seal next design card drifted")
    if not str(state.get("next_execution_card", "")).startswith("none"):
        fail("DeclaredInstance package co-seal must keep next_execution_card=none")
    row = card.get(DECLARED_INSTANCE_PACKAGE_COSEAL_D0_KEY)
    if not isinstance(row, dict):
        fail(f"{DECLARED_INSTANCE_PACKAGE_COSEAL_D0_KEY} section is missing")
    if row.get("task_id") != DECLARED_INSTANCE_PACKAGE_COSEAL_D0_ROW:
        fail("DeclaredInstance package co-seal task id drifted")
    if row.get("status") != "accepted_design_stop":
        fail("DeclaredInstance package co-seal must remain an accepted design stop")
    if row.get("implementation_permission") is not False:
        fail("DeclaredInstance package co-seal cannot permit implementation")
    relation = card.get(DECLARED_INSTANCE_RELATION_ISSUER_D0_KEY)
    if not isinstance(relation, dict):
        fail("DeclaredInstance package co-seal requires the relation design section")
    relation_child = card.get("mir_call_me_declared_instance_resolver_relation_i0_2026_08_31")
    if not isinstance(relation_child, dict) or relation_child.get("status") != "landed":
        fail("DeclaredInstance package co-seal requires the source relation child")
    result = card.get("mir_normal_callable_result_contract_retention_d0_i0_2026_08_31")
    if not isinstance(result, dict) or not str(result.get("status", "")).startswith("landed"):
        fail("DeclaredInstance package co-seal requires result/completion retention")
    effect = card.get(DECLARED_INSTANCE_EFFECT_ISSUER_I0_KEY)
    if not isinstance(effect, dict) or effect.get("status") != "landed":
        fail("DeclaredInstance package co-seal requires the landed effect issuer")


def check_declared_instance_package_locator_i0(
    state: dict, card: dict, root: Path
) -> None:
    if state.get("work_mode") != "fast":
        fail("DeclaredInstance private locator I0 must be fast")
    if state.get("current_execution_row") != DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_ROW:
        fail("DeclaredInstance private locator I0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("DeclaredInstance private locator I0 must clear current_design_stop")
    if state.get("next_execution_card") != DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_ROW:
        fail("DeclaredInstance private locator I0 next_execution_card drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("DeclaredInstance private locator I0 card path drifted")
    row = card.get(DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_KEY)
    if not isinstance(row, dict):
        fail(f"{DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_KEY} section is missing")
    if row.get("task_id") != DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_ROW:
        fail("DeclaredInstance private locator task id drifted")
    if row.get("status") != "selected_fast":
        fail("DeclaredInstance private locator must be selected_fast")
    if row.get("implementation_permission") is not True:
        fail("DeclaredInstance private locator must permit only its bounded implementation")
    package = card.get(DECLARED_INSTANCE_PACKAGE_COSEAL_D0_KEY)
    if not isinstance(package, dict) or package.get("status") != "accepted_design_stop":
        fail("private locator requires the accepted package co-seal design")
    if package.get("implementation_permission") is not False:
        fail("package co-seal must remain closed while locator is selected")
    for key, label in (
        ("mir_call_me_declared_instance_resolver_relation_i0_2026_08_31", "relation"),
        ("mir_normal_callable_result_contract_retention_d0_i0_2026_08_31", "result"),
        (DECLARED_INSTANCE_EFFECT_ISSUER_I0_KEY, "effect"),
    ):
        child = card.get(key)
        if not isinstance(child, dict) or not str(child.get("status", "")).startswith("landed"):
            fail(f"private locator requires landed {label} product")
    source_files = {
        "src/mir/normal_callable_semantic_package/declared_instance_locator.rs",
        "src/mir/normal_callable_semantic_package/mod.rs",
        "src/mir/normal_callable_semantic_package/model.rs",
        "src/mir/normal_callable_semantic_package/issuer.rs",
        "src/mir/normal_callable_semantic_package/install.rs",
    }
    for rel in source_files:
        path = root / rel
        if not path.is_file():
            fail(f"private locator owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 760:
            fail(f"private locator source reached the 760-line boundary: {rel}")
    locator = (root / "src/mir/normal_callable_semantic_package/declared_instance_locator.rs").read_text(
        encoding="utf-8"
    )
    if "OwnedExprSiteV1" not in locator or "ValueId" in locator or "Callee" in locator:
        fail("private locator must contain only source-site/slot locator data")
    if "NoRootDeclaredInstanceCall" not in locator or "finish_empty" in locator:
        fail("private locator must be explicit no-root/ready data, not a loan consumer")
    issuer = (root / "src/mir/normal_callable_semantic_package/issuer.rs").read_text(
        encoding="utf-8"
    )
    if "issue_declared_instance_call_package_locator_v1" not in issuer:
        fail("package issuer does not invoke the sole private locator issuer")
    allowed = set(require_text_list(row.get("allowed_files"), "private locator allowed_files"))
    required = source_files | {
        "src/mir/normal_callable_semantic_package/result_contract.rs",
        "src/mir/normal_callable_semantic_package/declared_instance_locator_tests.rs",
        "src/mir/normal_callable_semantic_package/README.md",
        str(HELPER_REL),
        str(STATE_REL),
        str(CARD_REL),
    }
    if not required <= allowed:
        fail(f"private locator allowed_files omit {sorted(required - allowed)}")


def check_declared_instance_selected_c_admission_d0(state: dict, card: dict) -> None:
    if state.get("work_mode") != "design_stop":
        fail("selected-C admission must remain design_stop")
    if state.get("current_execution_row") != DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_ROW:
        fail("selected-C admission row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_ROW:
        fail("selected-C admission design stop drifted")
    if state.get("next_design_card") != DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_ROW:
        fail("selected-C admission next design card drifted")
    if not str(state.get("next_execution_card", "")).startswith("none"):
        fail("selected-C admission must keep next_execution_card=none")
    row = card.get(DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_KEY)
    if not isinstance(row, dict):
        fail(f"{DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_KEY} section is missing")
    if row.get("task_id") != DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_ROW:
        fail("selected-C admission task id drifted")
    if row.get("status") != "accepted_design_stop":
        fail("selected-C admission must remain an accepted design stop")
    if row.get("implementation_permission") is not False:
        fail("selected-C admission cannot permit implementation")
    locator = card.get(DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_KEY)
    if not isinstance(locator, dict) or locator.get("status") != "landed":
        fail("selected-C admission requires the landed package locator")
    if locator.get("implementation_permission") is not False:
        fail("landed package locator must not retain implementation permission")
    package = card.get(DECLARED_INSTANCE_PACKAGE_COSEAL_D0_KEY)
    if not isinstance(package, dict) or package.get(
        "selected_c_admission_status"
    ) != "NoSafeSlice__existing_source_backed_capability_count_zero":
        fail("selected-C admission capability count must remain zero")
    bridge = card.get(DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_KEY)
    if not isinstance(bridge, dict) or bridge.get("status") != "landed":
        fail("selected-C admission requires the landed locator install bridge")
    if bridge.get("implementation_permission") is not False:
        fail("landed locator install bridge must not retain implementation permission")


def check_declared_instance_locator_install_bridge_i0(
    state: dict, card: dict, root: Path
) -> None:
    if state.get("work_mode") != "fast":
        fail("DeclaredInstance locator install bridge must be fast")
    if state.get("current_execution_row") != DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_ROW:
        fail("DeclaredInstance locator install bridge row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("DeclaredInstance locator install bridge must clear current_design_stop")
    if state.get("next_execution_card") != DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_ROW:
        fail("DeclaredInstance locator install bridge next_execution_card drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("DeclaredInstance locator install bridge card path drifted")
    row = card.get(DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_KEY)
    if not isinstance(row, dict):
        fail(f"{DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_KEY} section is missing")
    if row.get("task_id") != DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_ROW:
        fail("DeclaredInstance locator install bridge task id drifted")
    if row.get("status") != "selected_fast":
        fail("DeclaredInstance locator install bridge must be selected_fast")
    if row.get("implementation_permission") is not True:
        fail("DeclaredInstance locator install bridge must permit only its bounded transport")
    locator = card.get(DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_KEY)
    if not isinstance(locator, dict) or locator.get("status") != "landed":
        fail("locator install bridge requires the landed private locator")
    if locator.get("implementation_permission") is not False:
        fail("landed private locator must not retain implementation permission")
    selected = card.get(DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_KEY)
    if not isinstance(selected, dict) or selected.get("status") != "accepted_design_stop":
        fail("locator install bridge requires selected-C design stop")
    if selected.get("implementation_permission") is not False:
        fail("selected-C admission must remain closed while locator transport is selected")
    source_files = {
        "src/mir/normal_callable_semantic_package/declared_instance_locator.rs",
        "src/mir/normal_callable_semantic_package/mod.rs",
        "src/mir/normal_callable_semantic_package/model.rs",
        "src/mir/normal_callable_semantic_package/install.rs",
        "src/mir/normal_callable_semantic_package/declared_instance_locator_tests.rs",
        "src/mir/builder/normal_callable_package_bridge.rs",
    }
    for rel in source_files:
        path = root / rel
        if not path.is_file():
            fail(f"locator install bridge owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 760:
            fail(f"locator install bridge source reached the 760-line boundary: {rel}")
    install = (root / "src/mir/normal_callable_semantic_package/install.rs").read_text(
        encoding="utf-8"
    )
    if "declared_instance_call_locators," not in install:
        fail("installed package does not retain the locator disposition")
    if "with_declared_instance_call_locators" not in install:
        fail("installed package does not expose a callback-scoped locator view")
    bridge = (root / "src/mir/builder/normal_callable_package_bridge.rs").read_text(
        encoding="utf-8"
    )
    if "with_declared_instance_call_locators" not in bridge:
        fail("Builder package bridge does not forward the locator view")
    locator_source = (
        root / "src/mir/normal_callable_semantic_package/declared_instance_locator.rs"
    ).read_text(encoding="utf-8")
    if "ValueId" in locator_source or "Callee" in locator_source:
        fail("locator install bridge must not introduce target or receiver meaning")
    if "Clone" in locator_source or "Copy" in locator_source:
        fail("locator install bridge view must remain non-Clone/non-Copy")
    allowed = set(require_text_list(row.get("allowed_files"), "locator install bridge allowed_files"))
    required = source_files | {
        str(HELPER_REL),
        str(STATE_REL),
        str(CARD_REL),
    }
    if not required <= allowed:
        fail(f"locator install bridge allowed_files omit {sorted(required - allowed)}")


def check_selected_c_stack_row(state: dict, card: dict, root: Path) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("selected-C stack row requires fast or closeout work_mode")
    if state.get("current_execution_row") != SELECTED_C_STACK_ROW:
        fail("selected-C stack row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("selected-C stack row must clear current_design_stop")
    if state.get("next_execution_card") != SELECTED_C_STACK_ROW:
        fail("selected-C stack row pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("selected-C stack row card pointer drifted")
    row = card.get(SELECTED_C_STACK_KEY)
    if not isinstance(row, dict) or row.get("task_id") != SELECTED_C_STACK_ROW:
        fail("selected-C stack row is missing")
    status = row.get("status")
    if status not in {"selected_fast", "landed"}:
        fail("selected-C stack row status is not finite")
    if row.get("implementation_permission") is not (status == "selected_fast"):
        fail("selected-C stack row permission/status drifted")
    source = root / "lang/c-abi/shims/hako_llvmc_ffi_selected_launch_emit.inc"
    definition_source = root / "lang/c-abi/shims/hako_llvmc_ffi_same_module_function_definition_emit.inc"
    definition_seam = root / "lang/c-abi/shims/hako_llvmc_ffi_same_module_function_emit.inc"
    guard = root / "tools/checks/stage1_emit_program_json_runtime_helper_guard.sh"
    for path in (source, definition_source, definition_seam, guard):
        if not path.is_file():
            fail(f"selected-C stack owner is missing: {path}")
    if sum(1 for _ in source.open(encoding="utf-8")) >= 760:
        fail("selected-C launch owner reached the 760-line boundary")
    if sum(1 for _ in definition_source.open(encoding="utf-8")) >= 760:
        fail("selected-C definition owner reached the 760-line boundary")
    if sum(1 for _ in definition_seam.open(encoding="utf-8")) >= 760:
        fail("selected-C definition seam reached the 760-line boundary")
    allowed = set(require_text_list(row.get("allowed_files"), "selected-C allowed_files"))
    required = {
        "lang/c-abi/shims/hako_llvmc_ffi_selected_launch_emit.inc",
        "lang/c-abi/shims/hako_llvmc_ffi_same_module_function_definition_emit.inc",
        "lang/c-abi/shims/hako_llvmc_ffi_same_module_function_emit.inc",
        "lang/c-abi/shims/hako_llvmc_ffi_same_module_function_context.inc",
        "lang/c-abi/shims/README.md",
        "tools/checks/stage1_emit_program_json_runtime_helper_guard.sh",
        str(HELPER_REL),
        str(STATE_REL),
        str(CARD_REL),
    }
    if not required <= allowed:
        fail(f"selected-C allowed_files omit {sorted(required - allowed)}")
    if status == "landed":
        base = require_text(row.get("base_commit"), "selected-C base_commit")
        changed = git_diff_paths(root, base)
        if not changed <= allowed:
            fail(f"selected-C changed paths escaped: {sorted(changed - allowed)}")


def check_declared_instance_effect_issuer_structure(root: Path) -> None:
    effect_path = root / "src/mir/resolved_semantics/declared_instance_call_effect.rs"
    parser_path = root / "src/parser/callable_contract_syntax.rs"
    loan_path = root / "src/parser/normal_callable_program_source/semantic_syntax_loan.rs"
    batch_model_path = root / "src/mir/callable_semantic_batch/model.rs"
    batch_issuer_path = root / "src/mir/callable_semantic_batch/issuer.rs"
    for path in (
        effect_path,
        parser_path,
        loan_path,
        batch_model_path,
        batch_issuer_path,
    ):
        if not path.exists():
            fail(f"DeclaredInstance effect issuer source is missing: {path}")

    effect = effect_path.read_text(encoding="utf-8")
    required_effect = (
        "DeclaredInstanceCallSemanticEffectV1",
        "OpaqueObservable",
        "DeclaredQuery",
        "DeclaredInstanceCallEffectIssuerV1",
        "TargetSyntaxMissing",
        "TargetSyntaxDuplicate",
    )
    for token in required_effect:
        if token not in effect:
            fail(f"DeclaredInstance effect issuer missing required token: {token}")
    for token in ("EffectMask", "FunctionSignature", "ValueId", "Callee::", "resolve_call_target"):
        if token in effect:
            fail(f"DeclaredInstance effect issuer illegally depends on {token}")

    parser = parser_path.read_text(encoding="utf-8")
    for token in (
        "CallableContractSourceDispositionV1",
        "OutsideDirectDeclaredInstanceMethod",
        "DirectDeclaredInstanceMethod",
    ):
        if token not in parser:
            fail(f"parser contract disposition token is missing: {token}")
    loan = loan_path.read_text(encoding="utf-8")
    if "callable_contract_source" not in loan:
        fail("final callable syntax loan does not carry the contract disposition")
    batch_model = batch_model_path.read_text(encoding="utf-8")
    if "declared_instance_call_effect_source" not in batch_model:
        fail("semantic batch does not retain the effect sibling")
    batch_issuer = batch_issuer_path.read_text(encoding="utf-8")
    if "DeclaredInstanceCallEffectIssuerV1::issue" not in batch_issuer:
        fail("semantic batch does not invoke the sole effect issuer")


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
    from mir_call_d1b_active_surface_dispatch import dispatch

    dispatch(row, state, card, proof, root, api)
    print(f"[{TAG}] row={row} ok")


if __name__ == "__main__":
    main()
