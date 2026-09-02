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


def dispatch(row: object, state: dict, card: dict, proof: dict, root: Path, api) -> None:
    if row == api.STATIC_PUBLICATION_SPINE_ROW:
        api.check_static_publication_spine_landed(state, card)
    elif row == api.FREE_STATIC_PUBLICATION_SPINE_ROW:
        api.check_free_static_publication_spine_i0(state, card)
    elif row == api.BUILTIN_PRINT_PUBLICATION_SPINE_ROW:
        api.check_builtin_print_publication_spine_i0(state, card)
    elif row == FREE_FUNCTION_PUBLICATION_D0_ROW:
        _check_free_function_publication_d0(state, card, root, api)
    elif row == api.METHOD_ROW:
        check_proof_row(state, card, proof, root)
    elif row == api.RAW_ROOT_ROW:
        check_raw_root_resume(state, card, proof, root)
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
    elif row == api.DECLARED_INSTANCE_SELECTED_C_ADMISSION_D0_ROW:
        api.check_declared_instance_selected_c_admission_d0(state, card)
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
