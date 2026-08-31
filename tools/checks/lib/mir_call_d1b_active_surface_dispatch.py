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
    check_ordinary_static_target_only_i0,
)

BACKEND_OWNER_ROW = "BACKEND-OWNER-DECLARED-INSTANCE-METHOD-CUTOVER-D0"
RECEIVER_VALUE_OWNER_ROW = "MIR-CALL-ME-DECLARED-INSTANCE-RECEIVER-VALUE-OWNER-D0"
VERIFICATION_P0_A_ROW = "DEV-GATE-QUICK-LIB-BASELINE-P0-A-INC-DEBT-RECONCILE-R0"
VERIFICATION_P0_C_ROW = "DEV-GATE-QUICK-LIB-BASELINE-P0-C-RUNNER-WIRE-R0"
CROSSWALK_D0_ROW = "MIR-CALL-ME-DECLARED-INSTANCE-LOCATOR-VALUE-CROSSWALK-D0"
EXACT_BINDING_VALUE_ACCESSOR_S0_ROW = (
    "MIR-CALL-ME-DECLARED-INSTANCE-EXACT-BINDING-VALUE-ACCESSOR-S0"
)


def dispatch(row: object, state: dict, card: dict, proof: dict, root: Path, api) -> None:
    if row == api.METHOD_ROW:
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
    elif row == EXACT_BINDING_VALUE_ACCESSOR_S0_ROW:
        from mir_declared_instance_exact_binding_value_accessor_s0_guard import (
            check_exact_binding_value_accessor_s0,
        )

        check_exact_binding_value_accessor_s0(state, card, root, api)
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
    else:
        api.fail(f"unsupported current row for this stable guard: {row!r}")
