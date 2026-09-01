"""Active-row guard for one production-neutral test retirement cohort."""

from __future__ import annotations

import hashlib
from pathlib import Path
import re
import tomllib

import mir_call_d1b_active_surface_guard as api


ROW = "MIR-BUILDER-TEST-INVENTORY-BINDING-SHADOW-DEDUP-R0"
KEY = "mir_builder_test_inventory_binding_shadow_dedup_r0_2026_09_01"
SOURCE_REL = Path("src/mir/builder/builder_binding_id_tests.rs")
HOME_GUARD_REL = Path("tools/checks/rust_mirbuilder_builder_test_home_r0_guard.sh")
BASELINE_REL = Path("tools/checks/manifests/cargo_lib_red_baseline.toml")
INVENTORY_REL = Path("tools/checks/manifests/cargo_lib_red_baseline.tests.txt")
QUEUE_REL = Path(
    "docs/development/current/main/investigations/"
    "mirbuilder-structure-refactor-queue-d0-2026-08-23.md"
)
DISPATCH_REL = Path("tools/checks/lib/mir_call_d1b_active_surface_dispatch.py")
SELF_REL = Path("tools/checks/lib/mir_call_d1b_test_inventory_guard.py")


def _canonical_lines_sha256(lines: list[str]) -> str:
    payload = "" if not lines else "\n".join(lines) + "\n"
    return hashlib.sha256(payload.encode("utf-8")).hexdigest()

REMAINING_SYMBOLS = (
    "test_binding_map_initialization",
    "test_binding_allocation_sequential",
    "test_valueid_binding_parallel_allocation",
)
SUCCESSOR_NAMES = (
    "mir::builder::vars::lexical_scope::tests::"
    "declaration_and_shadowing_use_one_binding_allocator",
    "mir::builder::vars::lexical_scope::tests::"
    "local_binding_snapshot_restores_values_and_identity_together",
)


def check_binding_shadow_dedup_r0(
    state: dict, card: dict, root: Path, parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        parent_api.fail("binding-shadow test retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        parent_api.fail("binding-shadow test retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        parent_api.fail("binding-shadow test retirement must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        parent_api.fail("binding-shadow test retirement pointer drifted")
    if state.get("next_execution_card_path") != str(parent_api.CARD_REL):
        parent_api.fail("binding-shadow test retirement card path drifted")

    row = card.get(KEY)
    if not isinstance(row, dict):
        parent_api.fail(f"{KEY} section is missing")
    if row.get("task_id") != ROW:
        parent_api.fail("binding-shadow test retirement task id drifted")
    status = row.get("status")
    if status not in {"fast_open", "landed"}:
        parent_api.fail("binding-shadow test retirement status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        parent_api.fail("binding-shadow test retirement permission/status drifted")

    expected_allowed = {
        str(SOURCE_REL),
        str(HOME_GUARD_REL),
        str(BASELINE_REL),
        str(INVENTORY_REL),
        str(QUEUE_REL),
        str(DISPATCH_REL),
        str(SELF_REL),
        str(parent_api.STATE_REL),
        str(parent_api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        parent_api.fail("binding-shadow test retirement allowed-file boundary drifted")

    source = root / SOURCE_REL
    builder = root / "src/mir/builder.rs"
    for path in (source, builder):
        if not path.is_file():
            parent_api.fail(f"binding-shadow test retirement owner is missing: {path}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            parent_api.fail(f"binding-shadow test retirement owner reached 800 lines: {path}")
    source_text = source.read_text(encoding="utf-8")
    if "test_shadowing_binding_restore" in builder.read_text(encoding="utf-8"):
        parent_api.fail("shadowing test leaked into the production barrel")
    expected_candidate_count = 1 if status == "fast_open" else 0
    if source_text.count("test_shadowing_binding_restore") != expected_candidate_count:
        parent_api.fail("shadowing candidate presence does not match the row status")
    for symbol in REMAINING_SYMBOLS:
        if source_text.count(f"fn {symbol}(") != 1:
            parent_api.fail(f"remaining binding test is not unique: {symbol}")

    queue = (root / QUEUE_REL).read_text(encoding="utf-8")
    if ROW not in queue:
        parent_api.fail("structure queue lacks the binding-shadow disposition")
    if status == "landed" and "three binding-id tests remain" not in queue:
        parent_api.fail("structure queue lacks the post-retirement three-test receipt")

    baseline = tomllib.loads((root / BASELINE_REL).read_text(encoding="utf-8"))
    expected_total = 7564 if status == "fast_open" else 7563
    expected_passed = 7396 if status == "fast_open" else 7395
    if baseline.get("expected_passed") != expected_passed:
        parent_api.fail("binding-shadow retirement baseline passed count is not reconciled")
    if baseline.get("expected_failed") != 139 or baseline.get("expected_ignored") != 29:
        parent_api.fail("binding-shadow retirement changed the known-red partition")
    inventory = (root / INVENTORY_REL).read_text(encoding="utf-8").splitlines()
    names = {line for line in inventory if line}
    if len(names) != expected_total:
        parent_api.fail(
            f"binding-shadow retirement inventory count drifted: {len(names)} != {expected_total}"
        )
    candidate_name = "mir::builder::binding_id_tests::test_shadowing_binding_restore"
    if (candidate_name in names) is (status == "landed"):
        parent_api.fail("binding-shadow candidate inventory presence does not match row status")
    if status == "landed":
        for successor in SUCCESSOR_NAMES:
            if successor not in names:
                parent_api.fail(f"binding-shadow successor missing from inventory: {successor}")


PLANNER_CONTEXT_ROW = "MIR-BUILDER-TEST-INVENTORY-PLANNER-CONTEXT-DEDUP-R0"
PLANNER_CONTEXT_KEY = "mir_builder_test_inventory_planner_context_dedup_r0_2026_09_01"
PLANNER_CONTEXT_SOURCE_REL = Path(
    "src/mir/builder/control_flow/plan/facts/loop_tests_parts/planner_ctx.rs"
)
PLANNER_CONTEXT_CANDIDATE = (
    "mir::builder::control_flow::plan::facts::loop_tests::planner_ctx::"
    "loopfacts_ctx_keeps_simple_while_route_even_when_kind_mismatch"
)
PLANNER_CONTEXT_SUCCESSOR = (
    "mir::builder::control_flow::plan::facts::loop_tests::planner_ctx::"
    "loopfacts_ctx_allows_simple_while_route_when_kind_matches"
)
PLANNER_CONTEXT_NEGATIVES = (
    "mir::builder::control_flow::plan::facts::loop_tests::planner_ctx::"
    "loopfacts_ok_none_when_condition_not_supported",
    "mir::builder::control_flow::plan::facts::loop_tests::planner_ctx::"
    "loopfacts_ok_none_when_step_var_differs_from_condition_var",
    "mir::builder::control_flow::plan::facts::loop_simple_while_facts::tests::"
    "loop_simple_while_facts_reject_nested_loop_even_when_step_exists",
)


def check_planner_context_dedup_r0(
    state: dict, card: dict, root: Path, parent_api=api
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        parent_api.fail("planner-context test retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != PLANNER_CONTEXT_ROW:
        parent_api.fail("planner-context test retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        parent_api.fail("planner-context test retirement must clear current_design_stop")
    if state.get("next_execution_card") != PLANNER_CONTEXT_ROW:
        parent_api.fail("planner-context test retirement pointer drifted")
    if state.get("next_execution_card_path") != str(parent_api.CARD_REL):
        parent_api.fail("planner-context test retirement card path drifted")

    row = card.get(PLANNER_CONTEXT_KEY)
    if not isinstance(row, dict):
        parent_api.fail(f"{PLANNER_CONTEXT_KEY} section is missing")
    if row.get("task_id") != PLANNER_CONTEXT_ROW:
        parent_api.fail("planner-context test retirement task id drifted")
    status = row.get("status")
    if status not in {"fast_open", "landed"}:
        parent_api.fail("planner-context test retirement status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        parent_api.fail("planner-context test retirement permission/status drifted")

    expected_allowed = {
        str(PLANNER_CONTEXT_SOURCE_REL),
        str(BASELINE_REL),
        str(INVENTORY_REL),
        str(DISPATCH_REL),
        str(SELF_REL),
        str(parent_api.STATE_REL),
        str(parent_api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        parent_api.fail("planner-context test retirement allowed-file boundary drifted")

    source = root / PLANNER_CONTEXT_SOURCE_REL
    if not source.is_file():
        parent_api.fail(f"planner-context test retirement owner is missing: {source}")
    if sum(1 for _ in source.open(encoding="utf-8")) >= 800:
        parent_api.fail(f"planner-context test retirement owner reached 800 lines: {source}")
    source_text = source.read_text(encoding="utf-8")
    expected_candidate_count = 1 if status == "fast_open" else 0
    if source_text.count(
        "fn loopfacts_ctx_keeps_simple_while_route_even_when_kind_mismatch("
    ) != expected_candidate_count:
        parent_api.fail("planner-context candidate presence does not match the row status")
    if source_text.count(
        "fn loopfacts_ctx_allows_simple_while_route_when_kind_matches("
    ) != 1:
        parent_api.fail("planner-context exact-match successor is not unique")
    for name in (
        "loopfacts_ok_none_when_condition_not_supported",
        "loopfacts_ok_none_when_step_var_differs_from_condition_var",
    ):
        if source_text.count(f"fn {name}(") != 1:
            parent_api.fail(f"planner-context negative is not unique: {name}")

    baseline = tomllib.loads((root / BASELINE_REL).read_text(encoding="utf-8"))
    expected_total = 7563 if status == "fast_open" else 7562
    expected_passed = 7395 if status == "fast_open" else 7394
    if baseline.get("expected_passed") != expected_passed:
        parent_api.fail("planner-context retirement baseline passed count is not reconciled")
    if baseline.get("expected_failed") != 139 or baseline.get("expected_ignored") != 29:
        parent_api.fail("planner-context retirement changed the known-red partition")
    if baseline.get("failures_sha256") != (
        "86b8c383eb3d20f1851f33278e30fd431cae97dcc716aad9ac2fe13b586d9041"
    ):
        parent_api.fail("planner-context retirement failure-name SHA drifted")
    inventory = (root / INVENTORY_REL).read_text(encoding="utf-8").splitlines()
    names = {line for line in inventory if line}
    if len(names) != expected_total:
        parent_api.fail(
            f"planner-context retirement inventory count drifted: {len(names)} != {expected_total}"
        )
    candidate_present = PLANNER_CONTEXT_CANDIDATE in names
    if candidate_present is (status == "landed"):
        parent_api.fail("planner-context candidate inventory presence does not match row status")
    for required in (PLANNER_CONTEXT_SUCCESSOR, *PLANNER_CONTEXT_NEGATIVES):
        if required not in names:
            parent_api.fail(f"planner-context retained contract missing from inventory: {required}")


LOOP_IF_EXIT_ROW = "MIR-BUILDER-TEST-INVENTORY-LOOP-IF-EXIT-DEDUP-R0"
LOOP_IF_EXIT_KEY = "mir_builder_test_inventory_loop_if_exit_dedup_r0_2026_09_01"
LOOP_IF_EXIT_SOURCE_REL = Path(
    "src/mir/control_tree/normalized_shadow/tests/phase143_loop_if_exit_contract.rs"
)
LOOP_IF_EXIT_TESTS_MOD_REL = Path(
    "src/mir/control_tree/normalized_shadow/tests/mod.rs"
)
LOOP_IF_EXIT_PARENT_MOD_REL = Path("src/mir/control_tree/normalized_shadow/mod.rs")
LOOP_IF_EXIT_ROUTE_REL = Path(
    "src/mir/control_tree/normalized_shadow/loop_true_if_break_continue.rs"
)
LOOP_IF_EXIT_SUCCESSOR_REL = Path(
    "src/mir/control_tree/normalized_shadow/common/loop_if_exit_contract.rs"
)
LOOP_IF_EXIT_CANDIDATE_NAMES = (
    "test_shape_p0_break_only",
    "test_shape_validate_p0_break_ok",
    "test_shape_validate_p0_else_not_supported",
    "test_shape_validate_p0_continue_not_supported",
    "test_shape_validate_p1_continue_ok",
    "test_shape_validate_p1_break_ok",
    "test_shape_validate_p2_break_else_continue_ok",
    "test_loop_if_exit_then_eq",
)
LOOP_IF_EXIT_SUCCESSOR_NAMES = (
    *LOOP_IF_EXIT_CANDIDATE_NAMES[:-1],
    "test_shape_validate_p2_continue_else_break_ok",
    LOOP_IF_EXIT_CANDIDATE_NAMES[-1],
)
LOOP_IF_EXIT_PREFIX = (
    "mir::control_tree::normalized_shadow::tests::phase143_loop_if_exit_contract::"
)
LOOP_IF_EXIT_SUCCESSOR_PREFIX = (
    "mir::control_tree::normalized_shadow::common::loop_if_exit_contract::tests::"
)
LOOP_IF_EXIT_OLD_COMMENT = (
    "// Unit tests are in: normalized_shadow/tests/phase143_loop_if_exit_contract.rs"
)
LOOP_IF_EXIT_NEW_COMMENT = (
    "// Contract tests are in: normalized_shadow/common/loop_if_exit_contract.rs"
)


def check_loop_if_exit_dedup_r0(
    state: dict, card: dict, root: Path, parent_api=api
) -> None:
    """Guard one exact duplicate-test deletion without opening loop semantics."""
    if state.get("work_mode") not in {"fast", "closeout"}:
        parent_api.fail("loop-if-exit test retirement requires fast or closeout work_mode")
    if state.get("current_execution_row") != LOOP_IF_EXIT_ROW:
        parent_api.fail("loop-if-exit test retirement row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        parent_api.fail("loop-if-exit test retirement must clear current_design_stop")
    if state.get("next_execution_card") != LOOP_IF_EXIT_ROW:
        parent_api.fail("loop-if-exit test retirement pointer drifted")
    if state.get("next_execution_card_path") != str(parent_api.CARD_REL):
        parent_api.fail("loop-if-exit test retirement card path drifted")

    row = card.get(LOOP_IF_EXIT_KEY)
    if not isinstance(row, dict):
        parent_api.fail(f"{LOOP_IF_EXIT_KEY} section is missing")
    if row.get("task_id") != LOOP_IF_EXIT_ROW:
        parent_api.fail("loop-if-exit test retirement task id drifted")
    status = row.get("status")
    if status not in {"fast_open", "landed"}:
        parent_api.fail("loop-if-exit test retirement status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        parent_api.fail("loop-if-exit test retirement permission/status drifted")

    expected_allowed = {
        str(LOOP_IF_EXIT_SOURCE_REL),
        str(LOOP_IF_EXIT_TESTS_MOD_REL),
        str(LOOP_IF_EXIT_PARENT_MOD_REL),
        str(LOOP_IF_EXIT_ROUTE_REL),
        str(BASELINE_REL),
        str(INVENTORY_REL),
        str(DISPATCH_REL),
        str(SELF_REL),
        str(parent_api.STATE_REL),
        str(parent_api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        parent_api.fail("loop-if-exit test retirement allowed-file boundary drifted")

    source = root / LOOP_IF_EXIT_SOURCE_REL
    successor = root / LOOP_IF_EXIT_SUCCESSOR_REL
    parent_mod = root / LOOP_IF_EXIT_PARENT_MOD_REL
    route = root / LOOP_IF_EXIT_ROUTE_REL
    for path in (successor, parent_mod, route):
        if not path.is_file():
            parent_api.fail(f"loop-if-exit test retirement owner is missing: {path}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            parent_api.fail(f"loop-if-exit test retirement owner reached 800 lines: {path}")
    if sum(1 for _ in successor.open(encoding="utf-8")) >= 800:
        parent_api.fail("loop-if-exit successor reached 800 lines")
    successor_text = successor.read_text(encoding="utf-8")
    for name in LOOP_IF_EXIT_SUCCESSOR_NAMES:
        if successor_text.count(f"fn {name}(") != 1:
            parent_api.fail(f"loop-if-exit successor is not unique: {name}")

    candidate_present = source.is_file()
    if status == "fast_open" and not candidate_present:
        parent_api.fail("loop-if-exit candidate disappeared before deletion")
    if status == "landed" and candidate_present:
        parent_api.fail("loop-if-exit candidate remains after retirement")
    if candidate_present:
        candidate_text = source.read_text(encoding="utf-8")
        for name in LOOP_IF_EXIT_CANDIDATE_NAMES:
            if candidate_text.count(f"fn {name}(") != 1:
                parent_api.fail(f"loop-if-exit candidate is not unique: {name}")

    tests_mod = root / LOOP_IF_EXIT_TESTS_MOD_REL
    if status == "fast_open":
        if not tests_mod.is_file():
            parent_api.fail("loop-if-exit tests/mod.rs disappeared before deletion")
        tests_mod_text = tests_mod.read_text(encoding="utf-8")
        if tests_mod_text.count("mod phase143_loop_if_exit_contract;") != 1:
            parent_api.fail("loop-if-exit test module declaration is not unique")
    elif tests_mod.exists():
        parent_api.fail("loop-if-exit tests/mod.rs remains after retirement")

    parent_text = parent_mod.read_text(encoding="utf-8")
    test_decl_count = len(re.findall(r"(?m)^#\[cfg\(test\)\]\s+mod tests;", parent_text))
    if test_decl_count != (1 if status == "fast_open" else 0):
        parent_api.fail("loop-if-exit parent test declaration does not match row status")
    route_text = route.read_text(encoding="utf-8")
    expected_comment = LOOP_IF_EXIT_OLD_COMMENT if status == "fast_open" else LOOP_IF_EXIT_NEW_COMMENT
    if route_text.count(expected_comment) != 1:
        parent_api.fail("loop-if-exit test-home comment does not match row status")
    if status == "landed" and LOOP_IF_EXIT_OLD_COMMENT in route_text:
        parent_api.fail("retired loop-if-exit test path remains in the route comment")

    baseline = tomllib.loads((root / BASELINE_REL).read_text(encoding="utf-8"))
    expected_total = 7562 if status == "fast_open" else 7554
    expected_passed = 7394 if status == "fast_open" else 7386
    if baseline.get("expected_passed") != expected_passed:
        parent_api.fail("loop-if-exit retirement baseline passed count is not reconciled")
    if baseline.get("expected_failed") != 139 or baseline.get("expected_ignored") != 29:
        parent_api.fail("loop-if-exit retirement changed the known-red partition")
    failure_sha = "86b8c383eb3d20f1851f33278e30fd431cae97dcc716aad9ac2fe13b586d9041"
    if baseline.get("failures_sha256") != failure_sha:
        parent_api.fail("loop-if-exit retirement failure-name SHA drifted")
    inventory = (root / INVENTORY_REL).read_text(encoding="utf-8").splitlines()
    names = tuple(line for line in inventory if line)
    if len(names) != expected_total or names != tuple(sorted(set(names))):
        parent_api.fail("loop-if-exit retirement inventory is not sorted and unique")
    candidate_names = {LOOP_IF_EXIT_PREFIX + name for name in LOOP_IF_EXIT_CANDIDATE_NAMES}
    successor_names = {
        LOOP_IF_EXIT_SUCCESSOR_PREFIX + name for name in LOOP_IF_EXIT_SUCCESSOR_NAMES
    }
    if status == "fast_open" and not candidate_names.issubset(names):
        parent_api.fail("loop-if-exit candidate inventory is incomplete")
    if status == "landed" and candidate_names.intersection(names):
        parent_api.fail("retired loop-if-exit candidate remains in inventory")
    if not successor_names.issubset(names):
        parent_api.fail("loop-if-exit successor inventory is incomplete")
    if _canonical_lines_sha256(list(names)) != baseline.get("inventory_sha256"):
        parent_api.fail("loop-if-exit retirement inventory SHA drifted")
    if _canonical_lines_sha256(
        (root / Path("tools/checks/manifests/cargo_lib_red_baseline.failures.txt")).read_text(
            encoding="utf-8"
        ).splitlines()
    ) != failure_sha:
        parent_api.fail("loop-if-exit retirement failure receipt SHA drifted")
    print(f"[{parent_api.TAG}] loop-if-exit test retirement ok status={status}")
