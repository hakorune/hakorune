"""Current-row dispatch for the stable D1B active-surface guard."""

from __future__ import annotations

from pathlib import Path

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
from mir_call_d1b_cataloged_print_guard import (
    CATALOGED_PRINT_RETIRE_ROW,
    CATALOGED_PRINT_TARGET_ARM_PRUNE_ROW,
    check_cataloged_print_caller_zero_retire_i0,
    check_cataloged_print_target_arm_prune_r0,
)
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
from mir_call_d1b_test_inventory_guard import (
    ROW as TEST_INVENTORY_BINDING_SHADOW_ROW,
    check_binding_shadow_dedup_r0,
    PLANNER_CONTEXT_ROW as TEST_INVENTORY_PLANNER_CONTEXT_ROW,
    check_planner_context_dedup_r0,
    LOOP_IF_EXIT_ROW as TEST_INVENTORY_LOOP_IF_EXIT_ROW,
    check_loop_if_exit_dedup_r0,
    LEGACY_TESTS_RETIRE_ROW,
    check_legacy_tests_retire_r0,
    LEXICAL_PARITY_MATRIX_ROW,
    check_normal_script_lexical_parity_matrix_s0,
)
from mir_call_d1b_rewrite_known_guard import (
    ROW as REWRITE_KNOWN_CALLER_ZERO_PRUNE_S0_ROW,
    check_rewrite_known_caller_zero_s0,
)
from mir_call_d1b_rewrite_known_policy_guard import (
    ROW as REWRITE_KNOWN_POLICY_RETIRE_I0_ROW,
    check_rewrite_known_policy_retire_i0,
)
from mir_call_d1b_same_module_target_only_guard import (
    ORDINARY_STATIC_TARGET_ONLY_I0_ROW,
    ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_ROW,
    check_ordinary_static_target_only_i0,
    check_ordinary_static_target_only_residual_i0,
)
from mir_call_d1b_extern_route_spec_boxshape_guard import (
    ROW as EXTERN_ROUTE_SPEC_BOXSHAPE_ROW,
    check_extern_route_spec_boxshape_s0,
)

SCRIPT_ENTRYPOINT_MODE_ROW = "MIR-TOOLS-CANONICAL-ENTRYPOINT-MODE-I0"

FREE_FUNCTION_PUBLICATION_D0_ROW = "MIR-CALL-FREE-FUNCTION-PUBLICATION-D0"
FREE_FUNCTION_PUBLICATION_D0_KEY = "mir_call_free_function_publication_d0_2026_09_02"
FREE_FUNCTION_PUBLICATION_I0_ROW = "MIR-CALL-FREE-FUNCTION-PUBLICATION-I0"
FREE_FUNCTION_PUBLICATION_I0_KEY = "mir_call_free_function_publication_i0_2026_09_02"

BACKEND_OWNER_ROW = "BACKEND-OWNER-DECLARED-INSTANCE-METHOD-CUTOVER-D0"
RECEIVER_VALUE_OWNER_ROW = "MIR-CALL-ME-DECLARED-INSTANCE-RECEIVER-VALUE-OWNER-D0"
VERIFICATION_P0_A_ROW = "DEV-GATE-QUICK-LIB-BASELINE-P0-A-INC-DEBT-RECONCILE-R0"
VERIFICATION_P0_C_ROW = "DEV-GATE-QUICK-LIB-BASELINE-P0-C-RUNNER-WIRE-R0"
VERIFICATION_REFRESH_ROW = "DEV-GATE-LIB-BASELINE-REFRESH-R0"
VERIFICATION_VARMAP_RECONCILE_ROW = "DEV-GATE-COREPLAN-VARMAP-BOUNDARY-RECONCILE-D0"
VERIFICATION_VARMAP_ROLE_CENSUS_ROW = "DEV-GATE-COREPLAN-VARMAP-ROLE-CENSUS-PRUNE-R0"
VERIFICATION_VARMAP_RESEAL_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-GENERIC-BODY-V1-R0"
VERIFICATION_VARMAP_CARRIER_PIPELINE_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-CARRIER-PIPELINE-R0"
VERIFICATION_VARMAP_LOOP_COND_BC_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-COND-BC-R0"
VERIFICATION_VARMAP_LOOP_TRUE_BC_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-TRUE-BC-R0"
VERIFICATION_VARMAP_LOOP_COND_CONTINUE_ONLY_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-COND-CONTINUE-ONLY-R0"
VERIFICATION_VARMAP_LOOP_COND_CONTINUE_WITH_RETURN_PHI_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-COND-CONTINUE-WITH-RETURN-PHI-R0"
VERIFICATION_VARMAP_LOOP_COND_RETURN_IN_BODY_PHI_ROW = "DEV-GATE-COREPLAN-VARMAP-RESEAL-LOOP-COND-RETURN-IN-BODY-PHI-R0"
CROSSWALK_D0_ROW = "MIR-CALL-ME-DECLARED-INSTANCE-LOCATOR-VALUE-CROSSWALK-D0"
CROSSWALK_I0_ROW = "MIR-CALL-ME-DECLARED-INSTANCE-LOCATOR-VALUE-CROSSWALK-I0"
EXACT_BINDING_VALUE_ACCESSOR_S0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-EXACT-BINDING-VALUE-ACCESSOR-S0"
)
RAW_ROOT_BODY_CLEANUP_ROW = "MIRBUILDER-CLEANUP-T1-S0-RAW-ROOT-BODY-P0"
RAW_ROOT_DRAIN_CLEANUP_ROW = "MIRBUILDER-CLEANUP-T1-S0-RAW-ROOT-DRAIN-P0"
RRF_COVERAGE_REEXPORT_CLEANUP_ROW = "MIRBUILDER-CLEANUP-T1-S1-RRF-COVERAGE-REEXPORT-RETIRE-R0"
RCF_IFCONTROL_REEXPORT_CLEANUP_ROW = "MIRBUILDER-CLEANUP-T1-S1-RCF-IFCONTROL-LOCATED-REEXPORT-RETIRE-R0"
RCF_IFCONTROL_COVERAGE_USE_REEXPORT_CLEANUP_ROW = "MIRBUILDER-CLEANUP-T1-S1-RCF-IFCONTROL-COVERAGE-USE-REEXPORT-RETIRE-R0"
RCF_IFCONTROL_ERROR_REEXPORT_CLEANUP_ROW = "MIRBUILDER-CLEANUP-T1-S1-RCF-IFCONTROL-ERROR-REEXPORT-RETIRE-R0"
T3_S0_CANONICAL_SESSION_LIFECYCLE_OBSERVER_RETIRE_ROW = "MIRBUILDER-CLEANUP-T3-S0-CANONICAL-SESSION-LIFECYCLE-OBSERVER-RETIRE-R0"
T3_S0_LEXICAL_PUSH_TEST_FACADE_RETIRE_ROW = "MIRBUILDER-CLEANUP-T3-S0-LEXICAL-PUSH-TEST-FACADE-RETIRE-R0"
T3_S0_DYNAMIC_V2_ADMISSION_GUARD_RETARGET_ROW = "MIRBUILDER-CLEANUP-DYNAMIC-V2-ADMISSION-GUARD-RETARGET-R0"
T3_S0_DYNAMIC_V2_SCHEDULE_TEST_FACADE_RETIRE_ROW = "MIRBUILDER-CLEANUP-DYNAMIC-V2-SCHEDULE-TEST-FACADE-RETIRE-R0"
RAW_ROOT_CLEANUP = {
    RAW_ROOT_BODY_CLEANUP_ROW: ("mirbuilder_cleanup_t1_s0_raw_root_body_p0_2026_09_03", Path("tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_body0_s0_guard.py")),
    RAW_ROOT_DRAIN_CLEANUP_ROW: ("mirbuilder_cleanup_t1_s0_raw_root_drain_p0_2026_09_03", Path("tools/checks/lib/cut0_i0_root0_raw_source0_lower_final0_guard.py")),
    RRF_COVERAGE_REEXPORT_CLEANUP_ROW: ("mirbuilder_cleanup_t1_s1_rrf_coverage_reexport_retire_r0_2026_09_03", Path("tools/checks/resolved_region_flow_r0_seam_guard.sh")),
    RCF_IFCONTROL_REEXPORT_CLEANUP_ROW: ("mirbuilder_cleanup_t1_s1_rcf_ifcontrol_located_reexport_retire_r0_2026_09_03", Path("tools/checks/resolved_if_control_structure_r0_guard.sh")),
    RCF_IFCONTROL_COVERAGE_USE_REEXPORT_CLEANUP_ROW: ("mirbuilder_cleanup_t1_s1_rcf_ifcontrol_coverage_use_reexport_retire_r0_2026_09_03", Path("tools/checks/resolved_if_control_structure_r0_guard.sh")),
    RCF_IFCONTROL_ERROR_REEXPORT_CLEANUP_ROW: ("mirbuilder_cleanup_t1_s1_rcf_ifcontrol_error_reexport_retire_r0_2026_09_03", Path("tools/checks/resolved_if_control_structure_r0_guard.sh")),
    T3_S0_CANONICAL_SESSION_LIFECYCLE_OBSERVER_RETIRE_ROW: ("mirbuilder_cleanup_t3_s0_canonical_session_lifecycle_observer_retire_r0_2026_09_03", Path("tools/checks/common_v2_s6c_structure_guard.sh")),
    T3_S0_LEXICAL_PUSH_TEST_FACADE_RETIRE_ROW: ("mirbuilder_cleanup_t3_s0_lexical_push_test_facade_retire_r0_2026_09_03", Path("tools/checks/resolved_region_flow_r0_seam_guard.sh")),
    T3_S0_DYNAMIC_V2_ADMISSION_GUARD_RETARGET_ROW: ("mirbuilder_cleanup_dynamic_v2_admission_guard_retarget_r0_2026_09_03", Path("tools/checks/dynamic_v2_physical_input_authority_guard.sh")),
    T3_S0_DYNAMIC_V2_SCHEDULE_TEST_FACADE_RETIRE_ROW: ("mirbuilder_cleanup_dynamic_v2_schedule_test_facade_retire_r0_2026_09_03", Path("tools/checks/dynamic_v2_physical_input_authority_guard.sh")),
    "MIRBUILDER-CLEANUP-ASN0-L0-STALE-GUARD-CONTRACT-RETIRE-R0": ("mirbuilder_cleanup_asn0_l0_stale_guard_contract_retire_r0_2026_09_03", Path("tools/checks/lib/callable_result_i0_site0_r0_expr0_spine0_stmt0_assignment.py")),
    "MIRBUILDER-CLEANUP-ACCESS0-MEHEADER-LOCATED-ADAPTER-GUARD-RETIRE-R0": ("mirbuilder_cleanup_access0_meheader_located_adapter_guard_retire_r0_2026_09_03", Path("tools/checks/lib/me_call_header_observation_guard.py")),
}
def _check_raw_root_cleanup(row: str, key: str, guard: Path, card: dict, root: Path, api) -> None:
    item = card.get(key)
    if not isinstance(item, dict) or item.get("status") not in {"selected_fast", "landed"}:
        api.fail(f"{row} manifest entry is not selected_fast or landed")
    runner = "bash" if guard.suffix == ".sh" else "python3"
    if api.subprocess.run([runner, str(root / guard)], cwd=root).returncode:
        api.fail(f"{row} delegated guard failed")
def _dispatch_coreplan_varmap_reseal_row(
    row: str, state: dict, card: dict, root: Path, api
) -> None:
    """Dispatch future varmap reseal rows through one manifest-driven checker."""
    from mir_verification_quick_p0_c_guard import (
        _check_coreplan_varmap_reseal_single_site,
        _coreplan_varmap_reseal_allowed_files,
    )

    matches = [
        (key, value)
        for key, value in card.items()
        if isinstance(value, dict) and value.get("task_id") == row
    ]
    if len(matches) != 1:
        api.fail(f"CorePlan varmap reseal row must have one manifest entry: {row!r}")
    row_key, manifest_row = matches[0]
    target_paths = manifest_row.get("target_paths")
    if not isinstance(target_paths, list) or not target_paths or not all(
        isinstance(path, str) and path.strip() for path in target_paths
    ):
        api.fail(f"CorePlan varmap reseal target_paths are malformed: {row!r}")
    expected_direct_sites = manifest_row.get("expected_direct_sites")
    if not isinstance(expected_direct_sites, int) or expected_direct_sites <= 0:
        api.fail(f"CorePlan varmap reseal expected_direct_sites is malformed: {row!r}")
    expected_direct_sites_token = manifest_row.get("expected_direct_sites_token")
    label = manifest_row.get("label")
    parent_row = manifest_row.get("parent_row")
    if not all(isinstance(value, str) and value.strip() for value in (expected_direct_sites_token, label, parent_row)):
        api.fail(f"CorePlan varmap reseal metadata is malformed: {row!r}")
    _check_coreplan_varmap_reseal_single_site(
        state,
        card,
        root,
        row_name=row,
        row_key=row_key,
        parent_row=parent_row,
        label=label,
        target_paths=set(target_paths),
        expected_direct_sites=expected_direct_sites,
        expected_direct_sites_token=expected_direct_sites_token,
        allowed_files=_coreplan_varmap_reseal_allowed_files(target_paths[0]),
    )
def _check_free_function_publication_d0(
    state: dict, card: dict, root: Path, api
) -> None:
    """Keep the true-FreeFunction census at an explicit design stop.

    This is a lane check, not an implementation permission.  It verifies that
    the finite source-to-publication boundary is recorded before any caller or
    backend code can be selected.
    """
    if state.get("work_mode") != "design_stop":
        api.fail("true FreeFunction publication D0 must remain design_stop")
    if state.get("current_execution_row") != FREE_FUNCTION_PUBLICATION_D0_ROW:
        api.fail("true FreeFunction publication D0 is not selected by CURRENT_STATE")
    if state.get("next_design_card") != FREE_FUNCTION_PUBLICATION_D0_ROW:
        api.fail("true FreeFunction publication D0 next design card drifted")
    if not str(state.get("next_execution_card", "")).startswith("none"):
        api.fail("true FreeFunction publication D0 must keep next_execution_card=none")
    stop = state.get("current_design_stop")
    if not isinstance(stop, str) or not stop.startswith(FREE_FUNCTION_PUBLICATION_D0_ROW):
        api.fail("true FreeFunction publication D0 design stop is missing")

    row = card.get(FREE_FUNCTION_PUBLICATION_D0_KEY)
    if not isinstance(row, dict):
        api.fail(f"{FREE_FUNCTION_PUBLICATION_D0_KEY} section is missing")
    if row.get("task_id") != FREE_FUNCTION_PUBLICATION_D0_ROW:
        api.fail("true FreeFunction publication D0 task id drifted")
    if row.get("status") != "accepted_design_stop":
        api.fail("true FreeFunction publication D0 status is not a design stop")
    if row.get("implementation_permission") is not False:
        api.fail("true FreeFunction publication D0 must not authorize implementation")
    for field in (
        "decision",
        "source_authority",
        "canonical_issuer",
        "fail_fast_boundary",
        "census_boundary",
        "acceptance",
        "no_safe_slice",
        "non_claims",
    ):
        value = row.get(field)
        if not isinstance(value, str) or not value.strip():
            api.fail(f"true FreeFunction publication D0 field is missing: {field}")
    states = row.get("finite_states")
    if not isinstance(states, list) or not states or not all(
        isinstance(item, str) and item.strip() for item in states
    ):
        api.fail("true FreeFunction publication D0 finite state table is missing")

def _check_free_function_publication_i0(
    state: dict, card: dict, root: Path, api
) -> None:
    """Open only the one selected TopLevel FreeFunction vertical.

    The D0 remains the design record.  This transition guard makes the
    implementation permission explicit and finite: the existing source,
    collector, publisher, and typed backend owners may be connected, but no
    second builder or semantic receipt may be introduced.
    """
    if state.get("work_mode") != "fast":
        api.fail("true FreeFunction publication I0 must run in fast")
    if state.get("current_execution_row") != FREE_FUNCTION_PUBLICATION_I0_ROW:
        api.fail("true FreeFunction publication I0 is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("true FreeFunction publication I0 must clear current_design_stop")
    if state.get("next_execution_card") != FREE_FUNCTION_PUBLICATION_I0_ROW:
        api.fail("true FreeFunction publication I0 next_execution_card drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("true FreeFunction publication I0 card path drifted")

    row = card.get(FREE_FUNCTION_PUBLICATION_I0_KEY)
    if not isinstance(row, dict):
        api.fail(f"{FREE_FUNCTION_PUBLICATION_I0_KEY} section is missing")
    if row.get("task_id") != FREE_FUNCTION_PUBLICATION_I0_ROW:
        api.fail("true FreeFunction publication I0 task id drifted")
    if row.get("status") != "selected_fast":
        api.fail("true FreeFunction publication I0 status is not selected_fast")
    if row.get("implementation_permission") is not True:
        api.fail("true FreeFunction publication I0 must permit only its bounded implementation")
    if row.get("branch_scope") != "branch_or_worktree_only":
        api.fail("true FreeFunction publication I0 must remain branch/worktree scoped")
    if row.get("base_head") != "f669e0271d":
        api.fail("true FreeFunction publication I0 base head drifted")

    d0 = card.get(FREE_FUNCTION_PUBLICATION_D0_KEY)
    if not isinstance(d0, dict) or d0.get("status") != "accepted_design_stop":
        api.fail("true FreeFunction publication I0 requires the accepted D0 design")
    if d0.get("implementation_permission") is not False:
        api.fail("true FreeFunction publication D0 must remain closed")

    for field in (
        "decision",
        "source_authority",
        "canonical_issuer",
        "fail_fast_boundary",
        "census_boundary",
        "first_cohort",
        "acceptance",
        "no_safe_slice",
        "migration_red",
    ):
        value = row.get(field)
        if not isinstance(value, str) or not value.strip():
            api.fail(f"true FreeFunction publication I0 field is missing: {field}")
    for field in ("non_authority", "old_edge_delete_set", "negative_cases", "implementation_files", "allowed_files", "forbidden_files", "focused_tests"):
        value = row.get(field)
        if not isinstance(value, list) or not value or not all(
            isinstance(item, str) and item.strip() for item in value
        ):
            api.fail(f"true FreeFunction publication I0 list is missing: {field}")

    allowed = set(row["allowed_files"])
    required = {
        "src/mir/builder/callable_declaration_catalog/source_backed.rs",
        "src/mir/builder/normal_top_level_function_admission.rs",
        "src/mir/normal_callable_semantic_package/issuer.rs",
        "src/mir/function/published_backend_view.rs",
        "tools/checks/lib/mir_call_d1b_active_surface_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
    }
    if not required <= allowed:
        api.fail(f"true FreeFunction publication I0 allowed_files omit {sorted(required - allowed)}")

    for rel in row["implementation_files"]:
        if "**" in rel:
            continue
        path = root / rel
        if not path.is_file():
            api.fail(f"true FreeFunction publication I0 implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"true FreeFunction publication I0 implementation owner reached 800 lines: {rel}")

def _check_r6_post_group_b_reader_census_c0(
    state: dict, root: Path, api
) -> None:
    """Keep the closed post-Group-B census and exact reopen trigger aligned."""
    row = api.POST_GROUP_B_READER_CENSUS_C0_ROW
    if state.get("work_mode") != "closeout":
        api.fail(f"{row} must remain closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if state.get("current_design_stop") != (
        "NoSafeSlice__NoSingleRemainingCanonicalReaderFamily"
    ):
        api.fail(f"{row} design-stop result drifted")
    if state.get("next_design_card") != "none":
        api.fail(f"{row} must not schedule another census")
    if state.get("next_execution_card") != "none":
        api.fail(f"{row} must not grant implementation permission")
    if state.get("latest_card_path") != str(api.FINAL_PIPELINE_REL):
        api.fail(f"{row} requires the final-pipeline SSOT as its owner")

    card_path = root / api.FINAL_PIPELINE_REL
    card_text = card_path.read_text(encoding="utf-8")
    marker = "### Exact reopen trigger"
    if marker not in card_text:
        api.fail(f"{row} exact reopen section is missing")
    section = card_text.split(marker, 1)[1].split("\n### ", 1)[0]
    for token in (
        "No further broad census is permitted",
        "exactly one existing source authority",
        "exactly one canonical typed issuer before argument descent",
        "exactly one lossless selected-product/publication consumer",
        "exactly one real production caller",
        "one named fail-fast typed reject boundary",
        "one finite family-exclusive old-edge delete set",
        "every compatibility reissuer/reader outside the selected route enumerated",
        "every touched or new owner below the 760-line source trigger",
        "canonical rejection re-entry/fallback/retry/reselection = 0",
        "existing focused positive/negative and lane-guard owners named",
        "migration red classified separately from the known-red baseline",
    ):
        if token not in section:
            api.fail(f"{row} contract is missing: {token}")
    if f"{row}:" not in card_text:
        api.fail(f"{row} tombstone is missing")
    if "NoSafeSlice__NoSingleRemainingCanonicalReaderFamily" not in card_text:
        api.fail(f"{row} result is missing")
    if len(card_text.splitlines()) > 1000:
        api.fail(f"{row} final-pipeline SSOT exceeds the 1000-line hard limit")
    print(f"[{api.TAG}] row={row} delegated=post-group-b-no-safe-slice")

WASM_METHOD_LEGACY_READER_STOP_R0_ROW = "MIR-CALL-LEGACY-READER-STOP-WASM-METHOD-R0"


def _check_wasm_legacy_reader_stop_r0(
    state: dict,
    root: Path,
    api,
    *,
    row: str,
    reader: str,
    stop_tag: str,
    required: tuple[str, ...],
    owners: tuple[str, ...],
) -> None:
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if state.get("current_design_stop") != "none":
        api.fail(f"{row} must clear current_design_stop")
    if state.get("next_design_card") != "none":
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if state.get("next_execution_card") != expected_next:
        api.fail(f"{row} next_execution_card drifted")
    final_rel = str(api.FINAL_PIPELINE_REL)
    for key in ("next_execution_card_path", "latest_card_path"):
        if state.get(key) != final_rel:
            api.fail(f"{row} {key} drifted")
    card_text = (root / api.FINAL_PIPELINE_REL).read_text(encoding="utf-8")
    for token in (row, reader, stop_tag, *required):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")
    status = "status = fast_open" if mode == "fast" else "status = landed"
    permission = "implementation permission = true" if mode == "fast" else "implementation permission = false"
    for token in (status, permission):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")
    if len(card_text.splitlines()) > 1000:
        api.fail(f"{row} final-pipeline SSOT exceeds the 1000-line hard limit")
    for rel in owners:
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")


def _check_wasm_legacy_global_reader_stop_r0(state: dict, root: Path, api) -> None:
    _check_wasm_legacy_reader_stop_r0(
        state, root, api,
        row=api.WASM_LEGACY_GLOBAL_READER_STOP_R0_ROW,
        reader="LegacyCallV0(Global)",
        stop_tag="[freeze:contract][wasm/legacy-global-call-stopped]",
        required=("before shape matching, WAT planning, binary emission", "or selection of a Rust fallback route"),
        owners=("src/backend/wasm/mod.rs", "src/backend/wasm/codegen/instructions.rs", "src/backend/wasm/codegen/mod.rs", "src/backend/wasm/tests.rs"),
    )
    print(f"[{api.TAG}] row={api.WASM_LEGACY_GLOBAL_READER_STOP_R0_ROW} delegated=wasm-legacy-global-stop")


VM_GLOBAL_CANONICAL_CUTOVER_R0_ROW = "MIR-CALL-VM-GLOBAL-CANONICAL-CUTOVER-R0"
WASM_EXTERN_LEGACY_READER_STOP_R0_ROW = "MIR-CALL-LEGACY-READER-STOP-WASM-EXTERN-R0"
VM_VALUE_LEGACY_READER_STOP_R0_ROW = "MIR-CALL-LEGACY-READER-STOP-VM-VALUE-R0"
VM_METHOD_LEGACY_READER_STOP_R0_ROW = "MIR-CALL-LEGACY-READER-STOP-VM-METHOD-R0"


def _check_vm_global_canonical_cutover_r0(
    state: dict, root: Path, api
) -> None:
    """Dispatch the one bounded VM Global canonical-reader cutover.

    This extends an already landed canonical consumer and removes only its
    legacy Global reader.  The stable guard deliberately checks the finite
    owner surface without creating a second task ledger or semantic receipt.
    """
    row = VM_GLOBAL_CANONICAL_CUTOVER_R0_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if state.get("current_design_stop") != "none":
        api.fail(f"{row} must clear current_design_stop")
    if state.get("next_design_card") != "none":
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if state.get("next_execution_card") != expected_next:
        api.fail(f"{row} next_execution_card drifted")
    if state.get("next_execution_card_path") != str(api.FINAL_PIPELINE_REL):
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != str(api.FINAL_PIPELINE_REL):
        api.fail(f"{row} requires the final-pipeline SSOT as its owner")

    card_text = (root / api.FINAL_PIPELINE_REL).read_text(encoding="utf-8")
    required = (
        row,
        "base_head = 36c7c15d87",
        "MirInstruction::Call(MirCall)",
        "rejecting `LegacyCallV0(Global)`",
        "execute_global_target",
        "legacy Global arm is zero",
        "No Call R6 schema deletion",
        "existing active-surface and pointer guards are reused",
    )
    for token in required:
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")
    if mode == "fast":
        for token in ("status = selected_fast", "implementation permission = true"):
            if token not in card_text:
                api.fail(f"{row} fast contract is missing: {token}")
    else:
        for token in (
            "status = landed",
            "implementation permission = false",
            "implementation commit = 111216b539",
            "focused evidence = 5 canonical dispatch + 1 legacy reject + 8 parameter + 9 return tests",
        ):
            if token not in card_text:
                api.fail(f"{row} closeout contract is missing: {token}")
    for rel in (
        "src/backend/mir_interpreter/handlers/mod.rs",
        "src/backend/mir_interpreter/handlers/calls/mod.rs",
        "src/backend/mir_interpreter/handlers/calls/global.rs",
        "src/backend/mir_interpreter/exec/parameter_contracts/tests.rs",
        "src/backend/mir_interpreter/exec/return_contracts/tests.rs",
    ):
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=vm-global-canonical-cutover")


def _check_wasm_extern_legacy_reader_stop_r0(state: dict, root: Path, api) -> None:
    _check_wasm_legacy_reader_stop_r0(
        state, root, api,
        row=WASM_EXTERN_LEGACY_READER_STOP_R0_ROW,
        reader="LegacyCallV0(Callee::Extern)",
        stop_tag="[freeze:contract][wasm/legacy-extern-call-stopped]",
        required=("preflight before shape/WAT/binary/fallback", "EXTERN_CALL_MAP", "do not add a canonical WASM Extern consumer", "No Call R6 schema"),
        owners=("src/backend/wasm/mod.rs", "src/backend/wasm/codegen/instructions.rs", "src/backend/wasm/codegen/tests.rs", "src/backend/wasm/tests.rs", "src/backend/wasm/extern_contract.rs"),
    )
    instructions = (root / "src/backend/wasm/codegen/instructions.rs").read_text(encoding="utf-8")
    contract = (root / "src/backend/wasm/extern_contract.rs").read_text(encoding="utf-8")
    for stale in ("extern_import_name", "supported_extern_calls_csv"):
        if stale in instructions or stale in contract:
            api.fail(f"{WASM_EXTERN_LEGACY_READER_STOP_R0_ROW} stale WASM Extern helper remains: {stale}")
    print(f"[{api.TAG}] row={WASM_EXTERN_LEGACY_READER_STOP_R0_ROW} delegated=wasm-extern-reader-stop")


def _check_wasm_method_legacy_reader_stop_r0(state: dict, root: Path, api) -> None:
    _check_wasm_legacy_reader_stop_r0(
        state, root, api,
        row=WASM_METHOD_LEGACY_READER_STOP_R0_ROW,
        reader="LegacyCallV0(Callee::Method)",
        stop_tag="[freeze:contract][wasm/legacy-method-call-stopped]",
        required=("preflight before shape/WAT/binary/fallback", "no canonical WASM Method consumer", "No Call R6 schema"),
        owners=("src/backend/wasm/mod.rs", "src/backend/wasm/codegen/instructions.rs", "src/backend/wasm/codegen/tests.rs", "src/backend/wasm/tests.rs"),
    )
    if state.get("work_mode") == "closeout":
        instructions = (root / "src/backend/wasm/codegen/instructions.rs").read_text(encoding="utf-8")
        if "MirInstruction::LegacyCallV0" in instructions and "Callee::Method" in instructions:
            api.fail(f"{WASM_METHOD_LEGACY_READER_STOP_R0_ROW} stale Method codegen reader remains")
        if (root / "src/backend/wasm/codegen/builtins.rs").exists():
            api.fail(f"{WASM_METHOD_LEGACY_READER_STOP_R0_ROW} obsolete BoxCall builtin owner remains")
    print(f"[{api.TAG}] row={WASM_METHOD_LEGACY_READER_STOP_R0_ROW} delegated=wasm-method-reader-stop")

def dispatch(row: object, state: dict, card: dict, proof: dict, root: Path, api) -> None:
    if row == api.PERFORMANCE_SNAPSHOT_ROW:
        api.check_delegated_performance_row(state, root)
    elif row == api.PUBLISHED_VIEW_NEGATIVE_COVERAGE_B_S0_ROW:
        api.check_delegated_performance_evidence_row(state, root, row)
    elif row == api.MUTABLE_ACCUMULATOR_DUPLICATE_RETIRE_R0_ROW:
        api.check_delegated_performance_cleanup_row(state, root, row)
    elif row == api.PUBLISHED_C_DUAL_CONSUMER_PREPARE_BOXSHAPE_S0_ROW:
        api.check_delegated_published_c_boxshape_row(state, root, row)
    elif row == api.PRINT_PRODUCER_COVERAGE_S0_ROW:
        api.check_delegated_print_producer_coverage_row(state, root, row)
    elif row == api.POST_GROUP_B_READER_CENSUS_C0_ROW:
        _check_r6_post_group_b_reader_census_c0(state, root, api)
    elif row == api.WASM_LEGACY_GLOBAL_READER_STOP_R0_ROW:
        _check_wasm_legacy_global_reader_stop_r0(state, root, api)
    elif row == VM_GLOBAL_CANONICAL_CUTOVER_R0_ROW:
        _check_vm_global_canonical_cutover_r0(state, root, api)
    elif row == WASM_EXTERN_LEGACY_READER_STOP_R0_ROW:
        _check_wasm_extern_legacy_reader_stop_r0(state, root, api)
    elif row == VM_VALUE_LEGACY_READER_STOP_R0_ROW:
        _check_wasm_legacy_reader_stop_r0(
            state, root, api,
            row=VM_VALUE_LEGACY_READER_STOP_R0_ROW,
            reader="LegacyCallV0(Callee::Value)",
            stop_tag="[vm-reference/legacy-call/value-stopped]",
            required=("before execute_callee_call/register load/dispatch", "no Call R6 schema"),
            owners=("src/backend/mir_interpreter/handlers/calls/mod.rs", "src/backend/mir_interpreter/handlers/mod.rs"),
        )
    elif row == VM_METHOD_LEGACY_READER_STOP_R0_ROW:
        _check_wasm_legacy_reader_stop_r0(state, root, api, row=row, reader="LegacyCallV0(Callee::Method)", stop_tag="[vm-reference/legacy-call/method-stopped]", required=("before trace/hostbridge/direct-array/reg_load/dispatch", "no canonical VM Method consumer", "No Call R6 schema"), owners=("src/backend/mir_interpreter/handlers/calls/mod.rs", "src/backend/mir_interpreter/handlers/mod.rs", "src/backend/mir_interpreter/exec/block.rs"))
    elif row == WASM_METHOD_LEGACY_READER_STOP_R0_ROW:
        _check_wasm_method_legacy_reader_stop_r0(state, root, api)
    elif row == api.STATIC_PUBLICATION_SPINE_ROW:
        api.check_static_publication_spine_landed(state, card)
    elif row == api.FREE_STATIC_PUBLICATION_SPINE_ROW:
        api.check_free_static_publication_spine_i0(state, card)
    elif row == api.BUILTIN_PRINT_PUBLICATION_SPINE_ROW:
        api.check_builtin_print_publication_spine_i0(state, card)
    elif row == FREE_FUNCTION_PUBLICATION_D0_ROW:
        _check_free_function_publication_d0(state, card, root, api)
    elif row == FREE_FUNCTION_PUBLICATION_I0_ROW:
        _check_free_function_publication_i0(state, card, root, api)
    elif row == api.METHOD_ROW:
        check_proof_row(state, card, proof, root)
    elif row == api.RAW_ROOT_ROW:
        check_raw_root_resume(state, card, proof, root)
    elif row in RAW_ROOT_CLEANUP:
        key, guard = RAW_ROOT_CLEANUP[row]
        _check_raw_root_cleanup(row, key, guard, card, root, api)
    elif row == api.SCRIPT_ROOT_ROW:
        check_script_root_ret0(state, card, root)
    elif row == api.METHOD_CORRIDOR_D0_ROW:
        check_method_corridor_d0(state, card, api)
    elif row == api.METHOD_RESOLUTION_RET0_ROW:
        check_method_resolution_ret0(state, card, root, api)
    elif row == GUARD_SPLIT_ROW:
        check_guard_split_s0(state, card, root, api)
    elif row == TEST_SPLIT_ROW:
        check_test_split_s0(state, card, root, api)
    elif row == TEST_INVENTORY_BINDING_SHADOW_ROW:
        check_binding_shadow_dedup_r0(state, card, root, api)
    elif row == TEST_INVENTORY_PLANNER_CONTEXT_ROW:
        check_planner_context_dedup_r0(state, card, root, api)
    elif row == TEST_INVENTORY_LOOP_IF_EXIT_ROW:
        check_loop_if_exit_dedup_r0(state, card, root, api)
    elif row == LEGACY_TESTS_RETIRE_ROW:
        check_legacy_tests_retire_r0(state, card, root, api)
    elif row == LEXICAL_PARITY_MATRIX_ROW:
        check_normal_script_lexical_parity_matrix_s0(state, card, root, api)
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
    elif row == api.CATALOGED_GC_RETIRE_ROW:
        check_cataloged_gc_retire_i0(state, card, root)
    elif row == CATALOGED_PRINT_RETIRE_ROW:
        check_cataloged_print_caller_zero_retire_i0(state, card, root, api)
    elif row == CATALOGED_PRINT_TARGET_ARM_PRUNE_ROW:
        check_cataloged_print_target_arm_prune_r0(state, card, root, api)
    elif row == ORDINARY_STATIC_TARGET_ONLY_I0_ROW:
        check_ordinary_static_target_only_i0(state, card, root, api)
    elif row == ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_ROW:
        check_ordinary_static_target_only_residual_i0(state, card, root, api)
    elif row == api.RAW_LEGACY_ROW:
        check_raw_legacy_resume(state, card)
    elif row == api.RAW_LEGACY_I0_ROW:
        check_raw_legacy_i0(state, card, root)
    elif row == api.TYPE_FACT_GUARD_PRUNE_S0_ROW:
        check_type_fact_guard_prune_s0(state, card, root)
    elif row == api.OPERATOR_ROW:
        from mir_call_d1b_operator_retirement_guard import check_operator_retirement_i0

        check_operator_retirement_i0(state, card, root)
    elif row == api.ORDINARY_NEW_I0_ROW:
        check_ordinary_new_i0(state, card, root)
    elif row == api.ORDINARY_STATIC_LEGACY_RETIRE_I0_ROW:
        check_ordinary_static_legacy_retire_i0(state, card, root)
    elif row == api.BARE_ERROR_RETIRE_ROW:
        from mir_call_d1b_bare_error_retire_guard import check_bare_error_retire_i0

        check_bare_error_retire_i0(state, card, root, api)
    elif row == api.BARE_NOW_RETIRE_ROW:
        from mir_call_d1b_bare_error_retire_guard import check_bare_now_retire_i0

        check_bare_now_retire_i0(state, card, root, api)
    elif row == api.ACTIVE_SURFACE_ROWS_ROW:
        check_active_surface_rows_s0(state, card, root, api)
    elif row == REWRITE_KNOWN_CALLER_ZERO_PRUNE_S0_ROW:
        check_rewrite_known_caller_zero_s0(state, card, root, api)
    elif row == REWRITE_KNOWN_POLICY_RETIRE_I0_ROW:
        check_rewrite_known_policy_retire_i0(state, card, root, api)
    elif row == api.ME_METHOD_CANONICAL_I0_ROW:
        from mir_call_d1b_me_method_cutover_guard import check_me_method_canonical_i0

        check_me_method_canonical_i0(state, card, root, api)
    elif row == api.DECLARED_INSTANCE_RELATION_I0_ROW:
        from mir_call_d1b_declared_instance_relation_guard import (
            check_pointer as check_declared_instance_relation_pointer,
            check_structure as check_declared_instance_relation_structure,
        )

        relation_row = check_declared_instance_relation_pointer(state, card, root)
        check_declared_instance_relation_structure(root, relation_row)
    elif row == api.DECLARED_INSTANCE_RELATION_ISSUER_D0_ROW:
        api.check_declared_instance_relation_issuer_d0(state, card)
    elif row == api.DECLARED_INSTANCE_EFFECT_ISSUER_D0_ROW:
        api.check_declared_instance_effect_issuer_d0(state, card)
    elif row == api.DECLARED_INSTANCE_EFFECT_ISSUER_I0_ROW:
        api.check_declared_instance_effect_issuer_i0(state, card, root)
    elif row == api.DECLARED_INSTANCE_PACKAGE_COSEAL_D0_ROW:
        api.check_declared_instance_package_coseal_d0(state, card)
    elif row == api.DECLARED_INSTANCE_PACKAGE_LOCATOR_I0_ROW:
        api.check_declared_instance_package_locator_i0(state, card, root)
    elif row == api.DECLARED_INSTANCE_LOCATOR_INSTALL_BRIDGE_I0_ROW:
        api.check_declared_instance_locator_install_bridge_i0(state, card, root)
    elif row == api.DECLARED_INSTANCE_METHOD_SOME_VERTICAL_I0_ROW:
        api.check_declared_instance_method_some_vertical_i0(state, card, root)
    elif row == api.DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_ROW:
        api.check_declared_instance_selected_c_admission_d0(state, card)
    elif row == api.SELECTED_C_USERBOX_COMPAT_QUARANTINE_R0_ROW:
        api.check_selected_c_userbox_compat_quarantine_r0(state, card, root)
    elif row == api.HAKO_SAME_MODULE_INSTANCE_PHYSICAL_INGRESS_D0_ROW:
        api.check_hako_same_module_instance_physical_ingress_d0(state, card)
    elif row == api.REPO_LIFECYCLE_BASELINE_REFRESH_R0_ROW:
        api.check_repo_lifecycle_baseline_refresh_r0(state, card, root)
    elif row == api.DOCS_HISTORY_RETIRE_R0_ROW:
        api.check_docs_history_retire_r0(state, card, root)
    elif row == api.TEST_LOCAL_CONTRACT_FACT_DUPLICATE_RETIRE_R0_ROW:
        api.check_test_local_contract_fact_duplicate_retire_r0(state, card, root)
    elif row == api.SELECTED_C_STACK_ROW:
        api.check_selected_c_stack_row(state, card, root)
    elif row == api.CALLTARGET_GUARD_REHOME_ROW:
        api.check_calltarget_guard_rehome_r0(state, card, root)
    elif row == api.CSE_SAME_BLOCK_ROW:
        from mir_cse_same_block_guard import check_cse_same_block_r0

        check_cse_same_block_r0(state, card, root, api)
    elif row == BACKEND_OWNER_ROW:
        from mir_backend_owner_declared_instance_method_d0_guard import (
            check_backend_owner_declared_instance_method_d0,
        )

        check_backend_owner_declared_instance_method_d0(state, card, root, api)
    elif row == RECEIVER_VALUE_OWNER_ROW:
        from mir_declared_instance_receiver_value_owner_d0_guard import (
            check_declared_instance_receiver_value_owner_d0,
        )

        check_declared_instance_receiver_value_owner_d0(state, card, root, api)
    elif row == CROSSWALK_D0_ROW:
        from mir_declared_instance_locator_value_crosswalk_d0_guard import (
            check_declared_instance_locator_value_crosswalk_d0,
        )

        check_declared_instance_locator_value_crosswalk_d0(state, card, root, api)
    elif row == CROSSWALK_I0_ROW:
        from mir_declared_instance_locator_value_crosswalk_d0_guard import (
            check_declared_instance_locator_value_crosswalk_i0,
        )

        check_declared_instance_locator_value_crosswalk_i0(state, card, root, api)
    elif row == EXACT_BINDING_VALUE_ACCESSOR_S0_ROW:
        from mir_declared_instance_exact_binding_value_accessor_s0_guard import (
            check_exact_binding_value_accessor_s0,
        )

        check_exact_binding_value_accessor_s0(state, card, root, api)
    elif row == EXTERN_ROUTE_SPEC_BOXSHAPE_ROW:
        check_extern_route_spec_boxshape_s0(state, card, root, api)
    elif row == SCRIPT_ENTRYPOINT_MODE_ROW:
        from mir_script_entrypoint_mode_guard import check as check_script_entrypoint_mode

        check_script_entrypoint_mode(state, card, root)
    elif row == VERIFICATION_P0_A_ROW:
        from mir_verification_quick_p0_a_guard import (
            check_verification_quick_p0_a_inc_debt_reconcile_r0,
        )

        check_verification_quick_p0_a_inc_debt_reconcile_r0(state, card, root, api)
    elif row == VERIFICATION_P0_C_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_quick_p0_c_runner_wire_r0,
        )

        check_verification_quick_p0_c_runner_wire_r0(state, card, root, api)
    elif row == VERIFICATION_REFRESH_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_quick_lib_baseline_refresh_r0,
        )

        check_verification_quick_lib_baseline_refresh_r0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_RECONCILE_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_boundary_reconcile_d0,
        )

        check_verification_coreplan_varmap_boundary_reconcile_d0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_ROLE_CENSUS_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_role_census_prune_r0,
        )

        check_verification_coreplan_varmap_role_census_prune_r0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_RESEAL_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_reseal_generic_body_v1_r0,
        )

        check_verification_coreplan_varmap_reseal_generic_body_v1_r0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_CARRIER_PIPELINE_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_reseal_carrier_pipeline_r0,
        )

        check_verification_coreplan_varmap_reseal_carrier_pipeline_r0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_LOOP_COND_BC_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_reseal_loop_cond_bc_r0,
        )

        check_verification_coreplan_varmap_reseal_loop_cond_bc_r0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_LOOP_TRUE_BC_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_reseal_loop_true_bc_r0,
        )

        check_verification_coreplan_varmap_reseal_loop_true_bc_r0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_LOOP_COND_CONTINUE_ONLY_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_reseal_loop_cond_continue_only_r0,
        )

        check_verification_coreplan_varmap_reseal_loop_cond_continue_only_r0(state, card, root, api)
    elif row == VERIFICATION_VARMAP_LOOP_COND_CONTINUE_WITH_RETURN_PHI_ROW:
        from mir_verification_quick_p0_c_guard import (
            check_verification_coreplan_varmap_reseal_loop_cond_continue_with_return_phi_r0,
        )

        check_verification_coreplan_varmap_reseal_loop_cond_continue_with_return_phi_r0(state, card, root, api)
    elif isinstance(row, str) and row.startswith("DEV-GATE-COREPLAN-VARMAP-RESEAL-"):
        _dispatch_coreplan_varmap_reseal_row(row, state, card, root, api)
    else:
        api.fail(f"unsupported current row for this stable guard: {row!r}")
