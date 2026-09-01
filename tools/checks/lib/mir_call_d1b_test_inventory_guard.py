"""Active-row guard for one production-neutral test retirement cohort."""

from __future__ import annotations

from pathlib import Path
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
    if status == "landed" and "three tests remain" not in queue:
        parent_api.fail("structure queue lacks the post-retirement three-test receipt")

    baseline = tomllib.loads((root / BASELINE_REL).read_text(encoding="utf-8"))
    expected_total = 7562 if status == "fast_open" else 7561
    expected_passed = 7394 if status == "fast_open" else 7393
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
