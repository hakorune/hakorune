#!/usr/bin/env python3
"""Select the owner for the selected-method ArraySlot direct op keeper."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    lines = [
        "output_contract=selected-method-array-slot-direct-op-owner-selection-v0",
        "input_contract=mir-array-slot-residence-selected-method-guard-surface-v0",
        "selected_method=HakoAllocPageModel.acquire_usize/1",
        "selected_block=45",
        "selected_owner=c_abi_same_module_array_slot_direct_op_fusion",
        "implementation_owner_file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc",
        "declaration_owner_file=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc",
        "runtime_export_owner=crates/nyash_kernel/src/plugin/array_direct_slot_op.rs",
        "runtime_mod_owner=crates/nyash_kernel/src/plugin/mod.rs",
        "selected_reason=same_block_get_copy_set_pattern_requires_block_sequence_owner",
        "rejected_owner_0=boxcall_runtime_data_individual_get_set_lowering",
        "rejected_reason_0=cannot_erase_previously_lowered_get_when_set_is_seen_later",
        "rejected_owner_1=generic_mir_array_residence_transform",
        "rejected_reason_1=too_broad_before_selected_method_keeper",
        "rejected_owner_2=hako_alloc_by_name_source_rewrite",
        "rejected_reason_2=would_workaround_lowering_in_source",
        "planned_fused_runtime_symbol=nyash.array.slot_load_store_i64_hihi",
        "planned_erased_get_set_helper_calls=2",
        "planned_added_fused_helper_calls=1",
        "planned_net_helper_call_delta=1",
        "generic_array_residence_open=0",
        "by_name_hako_alloc_special_case=0",
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
