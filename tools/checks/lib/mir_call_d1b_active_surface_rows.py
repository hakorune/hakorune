"""Private row-handler owner for the active-surface lifecycle guard.

This module deliberately owns no registry entry and no shell entrypoint.  The
stable parent guard imports it only for the selected current row; all handlers
below are moved verbatim so their validation and diagnostics remain unchanged.
"""

from __future__ import annotations

from pathlib import Path

import mir_call_d1b_active_surface_guard as api

fail = api.fail
require_text = api.require_text
require_text_list = api.require_text_list
git_diff = api.git_diff
git_diff_paths = api.git_diff_paths
changed_added_test_names = api.changed_added_test_names
cargo_test_names = api.cargo_test_names
check_test_coverage = api.check_test_coverage

CARD_REL = api.CARD_REL
STATE_REL = api.STATE_REL
REGISTRY_REL = api.REGISTRY_REL
ENTRY_REL = api.ENTRY_REL
HELPER_REL = api.HELPER_REL
METHOD_ROW = api.METHOD_ROW
RAW_ROOT_ROW = api.RAW_ROOT_ROW
RAW_ROOT_KEY = api.RAW_ROOT_KEY
SCRIPT_ROOT_ROW = api.SCRIPT_ROOT_ROW
SCRIPT_ROOT_KEY = api.SCRIPT_ROOT_KEY
RAW_LEGACY_ROW = api.RAW_LEGACY_ROW
RAW_LEGACY_KEY = api.RAW_LEGACY_KEY
RAW_LEGACY_I0_ROW = api.RAW_LEGACY_I0_ROW
RAW_LEGACY_I0_KEY = api.RAW_LEGACY_I0_KEY
CATALOGED_GC_RETIRE_ROW = api.CATALOGED_GC_RETIRE_ROW
CATALOGED_GC_RETIRE_KEY = api.CATALOGED_GC_RETIRE_KEY
TYPE_FACT_GUARD_PRUNE_S0_ROW = api.TYPE_FACT_GUARD_PRUNE_S0_ROW
TYPE_FACT_GUARD_PRUNE_S0_KEY = api.TYPE_FACT_GUARD_PRUNE_S0_KEY
ORDINARY_NEW_I0_ROW = api.ORDINARY_NEW_I0_ROW
ORDINARY_NEW_I0_KEY = api.ORDINARY_NEW_I0_KEY
ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW = api.ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW
ORDINARY_STATIC_LEGACY_RETIRE_I0_KEY = api.ORDINARY_STATIC_LEGACY_RETIRE_I0_KEY
ACTIVE_SURFACE_ROWS_ROW = api.ACTIVE_SURFACE_ROWS_ROW
ACTIVE_SURFACE_ROWS_KEY = api.ACTIVE_SURFACE_ROWS_KEY


def check_active_surface_rows_s0(
    state: dict, card: dict, root: Path, parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("active-surface rows S0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != ACTIVE_SURFACE_ROWS_ROW:
        fail("active-surface rows S0 is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("active-surface rows S0 must clear current_design_stop")
    if state.get("next_execution_card") != ACTIVE_SURFACE_ROWS_ROW:
        fail("active-surface rows S0 pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("active-surface rows S0 card pointer drifted")

    row = card.get(ACTIVE_SURFACE_ROWS_KEY)
    if not isinstance(row, dict):
        fail(f"{ACTIVE_SURFACE_ROWS_KEY} section is missing")
    if row.get("task_id") != ACTIVE_SURFACE_ROWS_ROW:
        fail("active-surface rows S0 task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("active-surface rows S0 status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        fail("active-surface rows S0 permission/status drifted")

    parent = root / HELPER_REL
    sibling = root / Path("tools/checks/lib/mir_call_d1b_active_surface_rows.py")
    entry = root / ENTRY_REL
    registry = root / REGISTRY_REL
    for path in (parent, sibling, entry, registry):
        if not path.is_file():
            fail(f"active-surface rows owner is missing: {path}")
        if path.suffix == ".py" and len(path.read_text(encoding="utf-8").splitlines()) >= 760:
            fail(f"active-surface rows owner reached the 760-line boundary: {path}")

    shell = entry.read_text(encoding="utf-8")
    if "[[ $# -eq 0 ]]" not in shell:
        fail("stable shell no-argument rejection disappeared")
    if 'mir_call_d1b_active_surface_guard.py' not in shell:
        fail("stable shell no longer dispatches the parent guard")

    parent_text = parent.read_text(encoding="utf-8")
    sibling_text = sibling.read_text(encoding="utf-8")
    handlers = (
        "check_proof_row",
        "check_cataloged_gc_retire_i0",
        "check_raw_root_resume",
        "check_script_root_ret0",
        "check_raw_legacy_resume",
        "check_raw_legacy_i0",
        "check_type_fact_guard_prune_s0",
        "check_ordinary_new_i0",
    )
    for name in handlers:
        if sibling_text.count(f"def {name}(") != 1:
            fail(f"active-surface row handler is not uniquely rehomed: {name}")
        if f"def {name}(" in parent_text:
            fail(f"active-surface row handler remains in parent: {name}")
    if (
        "from mir_call_d1b_active_surface_rows import (" not in parent_text
        and "from mir_call_d1b_active_surface_rows import check_active_surface_rows_s0"
        not in parent_text
    ):
        fail("parent guard does not dispatch through the private rows sibling")
    if "elif row == ACTIVE_SURFACE_ROWS_ROW:" not in parent_text:
        fail("parent guard lacks the active-surface rows S0 dispatch")
    if row.get("status") == "landed":
        base = require_text(row.get("coverage_base_commit"), "active-surface rows coverage_base_commit")
        changed_paths = git_diff_paths(root, base)
        allowed = set(require_text_list(row.get("allowed_files"), "active-surface rows allowed_files"))
        if not changed_paths.issubset(allowed):
            fail(
                "active-surface rows changed paths exceed allowed boundary: "
                f"{sorted(changed_paths - allowed)}"
            )


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


def check_ordinary_static_legacy_retire_i0(
    state: dict, card: dict, root: Path
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("ordinary-static legacy retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW:
        fail("ordinary-static legacy retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        fail("ordinary-static legacy retirement must clear current_design_stop")
    if state.get("next_execution_card") != ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW:
        fail("ordinary-static legacy retirement pointer drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        fail("ordinary-static legacy retirement card pointer drifted")

    row = card.get(ORDINARY_STATIC_LEGACY_RETIRE_I0_KEY)
    if not isinstance(row, dict):
        fail(f"{ORDINARY_STATIC_LEGACY_RETIRE_I0_KEY} section is missing")
    if row.get("task_id") != ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW:
        fail("ordinary-static legacy retirement task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        fail("ordinary-static legacy retirement status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        fail("ordinary-static legacy retirement permission/status drifted")

    source_rel = Path("src/mir/builder/method_call_handlers.rs")
    route_rel = Path("src/mir/builder/calls/member_route.rs")
    tests_rel = Path("src/mir/builder/method_call_handlers_static_legacy_retire_tests.rs")
    readme_rel = Path("src/mir/builder/calls/README.md")
    reference_rel = Path("docs/reference/language/function-call-evaluation.md")
    expected_allowed = {
        str(source_rel),
        str(route_rel),
        str(tests_rel),
        "src/mir/builder/calls/member_route_descent_tests.rs",
        str(readme_rel),
        str(reference_rel),
        str(HELPER_REL),
        str(Path("tools/checks/lib/mir_call_d1b_active_surface_rows.py")),
        str(STATE_REL),
        str(CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        fail("ordinary-static legacy retirement allowed-file boundary drifted")

    for rel in (source_rel, route_rel, tests_rel):
        path = root / rel
        if not path.is_file():
            fail(f"ordinary-static legacy retirement owner is missing: {rel}")
        if len(path.read_text(encoding="utf-8").splitlines()) >= 760:
            fail(f"ordinary-static legacy retirement owner reached 760 lines: {rel}")

    if row.get("status") != "landed":
        return

    source = (root / source_rel).read_text(encoding="utf-8")
    route = (root / route_rel).read_text(encoding="utf-8")
    tests = (root / tests_rel).read_text(encoding="utf-8")
    reference = (root / reference_rel).read_text(encoding="utf-8")
    for token in (
        "UnissuedStaticCallRetirementV1",
        "GenericCompatibility",
        "[freeze:contract][static-call/legacy-fallback-retired]",
        "qualified_math_compatibility_owner",
        "PreparedMeReceiverV1::Static",
        "PreparedMeReceiverV1::Instance",
    ):
        if token not in source:
            fail(f"ordinary-static legacy retirement evidence is missing: {token}")
    if "legacy static compatibility edge retires before argument effects" not in reference:
        fail("ordinary-static language reference receipt is missing")
    if "static legacy compatibility" not in (root / readme_rel).read_text(encoding="utf-8"):
        fail("ordinary-static calls README receipt is missing")

    method_marker = "pub(in crate::mir::builder) fn handle_static_method_call_with_descent"
    method_body = source.split(method_marker, 1)
    if len(method_body) != 2:
        fail("ordinary-static handler is missing")
    method_body = method_body[1].split("\n    /// Handle TypeOp method calls", 1)
    if len(method_body) != 2:
        fail("ordinary-static handler boundary is missing")
    handler = method_body[0]
    retirement = handler.find("UnissuedStaticCallRetirementV1::GenericCompatibility.error")
    descent = handler.find("completion.lower_all(self)?")
    if retirement < 0 or descent < 0 or retirement > descent:
        fail("ordinary-static retirement is not proven before generic argument descent")
    for name in (
        "unissued_static_route_retires_before_argument_descent",
        "qualified_math_static_route_keeps_compatibility_owner",
        "static_this_retires_before_argument_descent",
        "me_static_fallback_retires_before_argument_descent",
        "lowered_global_static_retires_before_argument_descent",
    ):
        if tests.count(f"fn {name}(") != 1:
            fail(f"ordinary-static legacy retirement test is missing or duplicated: {name}")

    check_test_coverage(root, row)
    base = require_text(row.get("coverage_base_commit"), "ordinary-static coverage_base_commit")
    changed_paths = git_diff_paths(root, base)
    if not changed_paths.issubset(expected_allowed):
        fail(
            "ordinary-static legacy retirement changed paths exceed allowed boundary: "
            f"{sorted(changed_paths - expected_allowed)}"
        )
