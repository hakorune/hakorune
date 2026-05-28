---
Status: Landed
Date: 2026-05-29
Scope: implement selected-method receiver block-entry copy forwarding.
Blocker: SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-250-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-GUARD-SURFACE.md
---

# 296x-251 Selected Method Receiver Block-Entry Copy Forwarding Implementation

## Purpose

Implement the row250 guard surface without opening a broad LocalSSA or MIR
builder rewrite.

The selected candidates are `copy src=0` receiver materialization copies in
`HakoAllocPageModel.acquire_usize/1`. Other copy aliases already lower as
alias-only in the same-module C ABI emitter, but `src=0` copies were emitted as
LLVM `add i64 %r0, 0`. This row keeps the change at the C ABI same-module
lowering seam and forwards only the selected receiver copies into typed field
receiver references.

## Evidence

```text
output_contract=selected-method-receiver-block-entry-copy-forwarding-implementation-v0
input_contract=selected-method-receiver-block-entry-copy-forwarding-guard-surface-v0
selected_method=HakoAllocPageModel.acquire_usize/1
implementation_owner=c_abi_same_module_receiver_forwarding_alias
receiver_forward_alias_owner=lang/c-abi/shims/hako_llvmc_ffi_compiler_state.inc
body_emit_owner=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc
typed_field_emit_owner=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_object_emit.inc
forwarded_receiver_copy_count=9
remaining_param0_copy_add_count=1
forwarding_scope=selected_method_only
forwarding_source=copy_src_0
forwarding_sink=field_get_or_field_set_receiver
exclude_call_adjacent_receiver_copy=1
exclude_cross_block_rewrite=1
broad_local_ssa_reuse=0
mir_builder_rewrite=0
semantic_proof_summary=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

The single remaining `%r0` copy in `acquire_usize/1` is outside the row250
candidate set. The nine selected receiver copies now feed exact-slot field
helpers through `%r0` directly.

## Decision

```text
selected_owner_family=selected_method_receiver_block_entry_copy_forwarding_measurement
selected_reason=implementation_preserves_proof_and_removes_selected_param0_receiver_copy_adds
next_row=selected_method_receiver_block_entry_copy_forwarding_measurement
optimization_open=0
```

This row is structural implementation only. Keeper acceptance still requires a
separate measurement row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_method_receiver_block_entry_copy_forwarding_implementation_guard.sh
```
