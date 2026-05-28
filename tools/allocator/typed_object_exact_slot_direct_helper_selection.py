#!/usr/bin/env python3
"""Select the exact-lane typed-object slot direct helper seam."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    lines = [
        "output_contract=typed-object-exact-slot-direct-helper-selection-v0",
        "input_contract=typed-object-field-helper-subowner-refresh-v0",
        "selected_owner_family=typed_object_exact_slot_direct_helper",
        "selected_reason=field_helper_branch_validation_dominates_after_array_direct_op",
        "default_helper_abi=unchanged",
        "new_helper_symbols=separate",
        "default_exact_helper_emission=0",
        "implementation_primary_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc",
        "implementation_generic_owner_file=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc",
        "declaration_owner_file=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc",
        "runtime_abi_owner_file=crates/nyash_kernel/src/exports/typed_object.rs",
        "runtime_store_owner_file=crates/nyash_kernel/src/exports/typed_object_store.rs",
        "python_compat_owner_file=src/llvm_py/instructions/field_access.py",
        "selected_symbol_0=nyash.object.exact_slot_get_i64_hii",
        "selected_symbol_1=nyash.object.exact_slot_set_i64_hii",
        "selected_symbol_2=nyash.object.exact_slot_get_u64_hii",
        "selected_symbol_3=nyash.object.exact_slot_set_u64_hiu",
        "selected_symbol_4=nyash.object.exact_slot_get_handle_hii",
        "selected_symbol_5=nyash.object.exact_slot_set_handle_hii",
        "selected_symbol_count=6",
        "direct_storage_allowed_0=i64",
        "direct_storage_allowed_1=u64",
        "direct_storage_allowed_2=usize_if_target_pointer_width_64",
        "direct_storage_allowed_3=handle",
        "direct_storage_rejected_0=i8_i16_i32",
        "direct_storage_rejected_1=u8_u16_u32",
        "direct_storage_rejected_2=isize_unless_explicitly_proven_target_compatible",
        "direct_storage_rejected_3=unknown_storage",
        "direct_storage_rejected_4=dynamic_slot",
        "direct_storage_rejected_5=weak_field",
        "direct_storage_rejected_6=no_exact_field_plan",
        "lowering_gate_0=HAKO_TYPED_OBJECT_STORE_single_thread_exact",
        "lowering_gate_1=HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER_1",
        "lowering_gate_2=exact_field_plan_for_receiver_present",
        "lowering_gate_3=slot_constant",
        "lowering_gate_4=same_module_exact_exe_lane",
        "runtime_helper_env_check=0",
        "runtime_helper_safe_mutex_fallback=0",
        "runtime_helper_memory_safety_bounds=preserved",
        "unsupported_storage_fallback_reported=1",
        "selected_hot_method_target_unsupported_storage_fails_row=1",
        "rejected_owner_0=existing_helper_mutation",
        "rejected_reason_0=would_mix_default_semantics_with_exact_lane_fast_path",
        "rejected_owner_1=generic_typed_field_residence_retry",
        "rejected_reason_1=previous_selected_method_residence_net_helper_call_delta_zero",
        "rejected_owner_2=hako_alloc_by_name_special_case",
        "rejected_reason_2=would_workaround_lowering_in_source",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
