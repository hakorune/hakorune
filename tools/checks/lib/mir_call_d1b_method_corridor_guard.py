"""Private Method-corridor handlers for the active D1B guard."""

from pathlib import Path


GUARD_SPLIT_ROW = "MIR-CALL-RUST-METHODIZE-RETIRE-GUARD-S0"
GUARD_SPLIT_KEY = "rust_exact1_methodize_retire_guard_s0_2026_08_30"
TEST_SPLIT_ROW = "MIR-CALL-RUST-METHODIZE-RETIRE-TEST-S0"
TEST_SPLIT_KEY = "rust_exact1_methodize_retire_test_s0_2026_08_30"


def check_guard_split_s0(state: dict, card: dict, root: Path, api: object) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("guard split S0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != GUARD_SPLIT_ROW:
        api.fail("guard split S0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("guard split S0 must clear current_design_stop")
    if state.get("next_execution_card") != GUARD_SPLIT_ROW:
        api.fail("guard split S0 pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("guard split S0 card pointer drifted")

    row = card.get(GUARD_SPLIT_KEY)
    if not isinstance(row, dict):
        api.fail(f"{GUARD_SPLIT_KEY} section is missing")
    if row.get("task_id") != GUARD_SPLIT_ROW:
        api.fail("guard split S0 task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        api.fail("guard split S0 status is not finite")
    expected_permission = row.get("status") == "fast_open"
    if row.get("implementation_permission") is not expected_permission:
        api.fail("guard split S0 permission/status drifted")

    sibling_rel = Path("tools/checks/lib/mir_call_d1b_method_corridor_guard.py")
    parent = root / api.HELPER_REL
    sibling = root / sibling_rel
    for path in (parent, sibling):
        if not path.is_file():
            api.fail(f"guard split S0 owner is missing: {path}")
        if sum(1 for _ in path.open()) >= 760:
            api.fail(f"guard split S0 owner reached the 760-line boundary: {path}")
    parent_text = parent.read_text()
    sibling_text = sibling.read_text()
    if "mir_call_d1b_method_corridor_guard" not in parent_text:
        api.fail("active guard does not import the private Method corridor sibling")
    for token in (
        "def check_method_corridor_d0(",
        "def check_method_resolution_ret0(",
        "def check_guard_split_s0(",
    ):
        if token not in sibling_text:
            api.fail(f"private Method corridor guard lacks {token}")

    allowed = row.get("allowed_files")
    expected_allowed = {
        str(api.HELPER_REL),
        str(sibling_rel),
        str(api.STATE_REL),
        str(api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        api.fail("guard split S0 allowed-file boundary drifted")


def check_test_split_s0(state: dict, card: dict, root: Path, api: object) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("test split S0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != TEST_SPLIT_ROW:
        api.fail("test split S0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("test split S0 must clear current_design_stop")
    if state.get("next_execution_card") != TEST_SPLIT_ROW:
        api.fail("test split S0 pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("test split S0 card pointer drifted")

    row = card.get(TEST_SPLIT_KEY)
    if not isinstance(row, dict):
        api.fail(f"{TEST_SPLIT_KEY} section is missing")
    if row.get("task_id") != TEST_SPLIT_ROW:
        api.fail("test split S0 task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        api.fail("test split S0 status is not finite")
    expected_permission = row.get("status") == "fast_open"
    if row.get("implementation_permission") is not expected_permission:
        api.fail("test split S0 permission/status drifted")

    source_rel = Path("src/mir/builder/module_lifecycle_capture_tests.rs")
    sibling_rel = Path("src/mir/builder/module_lifecycle_ingress_tests.rs")
    expected_allowed = {
        str(source_rel),
        str(sibling_rel),
        str(api.STATE_REL),
        str(api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        api.fail("test split S0 allowed-file boundary drifted")

    source = root / source_rel
    if not source.is_file():
        api.fail(f"test split S0 source owner is missing: {source_rel}")
    if row.get("status") != "landed":
        return

    sibling = root / sibling_rel
    if not sibling.is_file():
        api.fail(f"test split S0 sibling owner is missing: {sibling_rel}")
    for path in (source, sibling):
        if sum(1 for _ in path.open()) >= 800:
            api.fail(f"test split S0 owner reached the 800-line hard stop: {path}")
    source_text = source.read_text()
    sibling_text = sibling.read_text()
    if 'mod module_lifecycle_ingress_tests;' not in source_text:
        api.fail("test split S0 source does not link the focused ingress sibling")
    for test_name in (
        "mirbuilder_minimal_literal_integer_path_smoke",
        "module_ingress_snapshots_explicit_methodize_policy_before_lowering",
        "normal_default_ingress_snapshots_explicit_methodize_policy",
        "module_ingress_snapshots_canonical_policy_for_unset_and_zero",
        "invalid_methodize_selector_rejects_before_normal_module_mutation",
    ):
        if sibling_text.count(f"fn {test_name}(") != 1:
            api.fail(f"test split S0 did not move exactly one {test_name}")
        if source_text.count(f"fn {test_name}(") != 0:
            api.fail(f"test split S0 left duplicate {test_name} in the source owner")


def check_method_corridor_d0(state: dict, card: dict, api: object) -> None:
    if state.get("work_mode") != "design_stop":
        api.fail("Method corridor producer census must remain design_stop")
    if state.get("current_execution_row") != api.METHOD_CORRIDOR_D0_ROW:
        api.fail("Method corridor producer census row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != api.METHOD_CORRIDOR_D0_ROW:
        api.fail("Method corridor producer census design stop drifted")
    if state.get("next_design_card") != api.METHOD_CORRIDOR_D0_ROW:
        api.fail("Method corridor producer census next design card drifted")
    if not str(state.get("next_execution_card", "")).startswith("none"):
        api.fail("Method corridor design stop must keep next_execution_card=none")

    row = card.get(api.METHOD_CORRIDOR_D0_KEY)
    if not isinstance(row, dict):
        api.fail(f"{api.METHOD_CORRIDOR_D0_KEY} section is missing")
    if row.get("task_id") != api.METHOD_CORRIDOR_D0_ROW:
        api.fail("Method corridor producer census task id drifted")
    if row.get("status") not in {"accepted_policy_b_in_progress", "design_stop"}:
        api.fail("Method corridor producer census status is not an active design stop")
    if row.get("implementation_permission") is not False:
        api.fail("Method corridor producer census must keep implementation closed")
    if row.get("current_disposition") != "CutoverBlockerOpen":
        api.fail("Method corridor producer census must remain blocker-open")
    if row.get("in_scope_inventory_count") != 11:
        api.fail("Method corridor producer inventory count drifted")

    open_b = row.get("d0_b_open_rows")
    open_c = row.get("d0_c_open_rows")
    if not isinstance(open_b, list) or not open_b:
        api.fail("Method corridor producer census has no open D0-B rows")
    if not isinstance(open_c, list) or not open_c:
        api.fail("Method corridor producer census has no open D0-C rows")
    for label, values in (("D0-B", open_b), ("D0-C", open_c)):
        if "raw_legacy_origin" in values or "script_root_origin" in values:
            api.fail(f"{label} still lists a landed compatibility origin")
        for required in (
            "resolved_compatibility_consumer",
            "builder_method_none_publication_terminal",
            "unified_emitter_methodize_reissuer",
        ):
            if required not in values:
                api.fail(f"{label} lost remaining Method corridor blocker: {required}")
    closed = row.get("d0_b_closed_rows")
    if not isinstance(closed, list):
        api.fail("Method corridor producer census closed rows are missing")
    for required in (
        "raw_legacy_origin",
        "script_root_origin",
        "method_resolution_static_none",
    ):
        if required not in closed:
            api.fail(f"Method corridor producer census did not close {required}")
    ret0 = card.get(api.METHOD_RESOLUTION_RET0_KEY)
    if not isinstance(ret0, dict) or ret0.get("status") != "landed":
        api.fail("Method corridor producer census lacks landed static-none RET0")
    if ret0.get("implementation_permission") is not False:
        api.fail("landed static-none RET0 must keep implementation closed")


def check_method_resolution_ret0(
    state: dict, card: dict, root: Path, api: object
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("method-resolution RET0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != api.METHOD_RESOLUTION_RET0_ROW:
        api.fail("method-resolution RET0 row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("method-resolution RET0 must clear current_design_stop")
    if state.get("next_execution_card") != api.METHOD_RESOLUTION_RET0_ROW:
        api.fail("method-resolution RET0 pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("method-resolution RET0 card pointer drifted")

    row = card.get(api.METHOD_RESOLUTION_RET0_KEY)
    if not isinstance(row, dict):
        api.fail(f"{api.METHOD_RESOLUTION_RET0_KEY} section is missing")
    if row.get("task_id") != api.METHOD_RESOLUTION_RET0_ROW:
        api.fail("method-resolution RET0 task id drifted")
    if row.get("status") not in {"fast_open", "landed"}:
        api.fail("method-resolution RET0 status is not finite")
    expected_permission = row.get("status") == "fast_open"
    if row.get("implementation_permission") is not expected_permission:
        api.fail("method-resolution RET0 permission/status drifted")

    source_rel = Path("src/mir/builder/calls/method_resolution.rs")
    utils_rel = Path("src/mir/builder/calls/utils.rs")
    source = (root / source_rel).read_text()
    utils = (root / utils_rel).read_text()
    for path in (source_rel, utils_rel):
        if sum(1 for _ in (root / path).open()) >= 760:
            api.fail(
                f"method-resolution RET0 source reached the 760-line split boundary: {path}"
            )
    if "pub fn resolve_call_target(" not in source:
        api.fail("method-resolution RET0 lost the remaining generic resolver")
    if row.get("status") == "landed":
        for token in (
            "current_static_box",
            "has_method(",
            "Callee::Method",
            "is_commonly_shadowed_method",
            "generate_self_recursion_warning",
            "classify_callee_box_kind",
        ):
            if token in source:
                api.fail(f"method-resolution RET0 stale static-none token remains: {token}")
        if "current_static_box" in utils:
            api.fail("method-resolution wrapper still carries current_static_box")
        if "method_resolution_never_issues_receiverless_static_method" not in source:
            api.fail("method-resolution RET0 negative test evidence is missing")

    allowed = row.get("allowed_files")
    expected_allowed = {
        str(source_rel),
        str(utils_rel),
        str(api.HELPER_REL),
        str(api.STATE_REL),
        str(api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
        "src/mir/builder/calls/README.md",
    }
    if not isinstance(allowed, list) or set(allowed) != expected_allowed:
        api.fail("method-resolution RET0 allowed-file boundary drifted")

    if row.get("status") == "landed":
        base = api.require_text(
            row.get("coverage_base_commit"), "method-resolution RET0 coverage_base_commit"
        )
        changed = api.changed_added_test_names(api.git_diff(root, base))
        expected = set(
            api.require_text_list(
                row.get("changed_test_names"), "method-resolution RET0 changed_test_names"
            )
        )
        if changed != expected:
            api.fail(
                "method-resolution RET0 changed test inventory drifted; "
                f"diff={sorted(changed)}, card={sorted(expected)}"
            )
        filters = api.require_text_list(
            row.get("focused_test_filters"), "method-resolution RET0 focused_test_filters"
        )
        listed = api.cargo_test_names(root)
        for name in sorted(changed):
            full_names = [item for item in listed if item.endswith("::" + name)]
            if len(full_names) != 1:
                api.fail(f"method-resolution RET0 changed test {name} is not uniquely listed")
            if not any(token in full_names[0] for token in filters):
                api.fail(f"method-resolution RET0 changed test {name} has no focused filter")
        for token in filters:
            if not any(token in item for item in listed):
                api.fail(f"method-resolution RET0 focused filter has zero matches: {token}")
        changed_paths = api.git_diff_paths(root, base)
        if not changed_paths.issubset(expected_allowed):
            api.fail(
                "method-resolution RET0 changed paths exceed allowed boundary: "
                f"{sorted(changed_paths - expected_allowed)}"
            )
