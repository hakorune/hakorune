---
Status: Landed
Date: 2026-05-30
Scope: smoke the selected-method ArraySlot NativeDirect lowering and DirectArrayI64 runtime substrate before perf refresh.
Blocker: ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-SEMANTIC-SMOKE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-369-ARRAY-SLOT-NATIVEDIRECT-SELECTED-METHOD-LOWERING-IMPLEMENTATION.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - src/llvm_py/tests/test_collection_method_call.py
---

# 296x-370 ArraySlot NativeDirect Selected-Method Semantic Smoke

## Purpose

Smoke the selected-method DirectArray lowering after row369.

This row verifies two boundaries:

1. Python LLVM lowering emits DirectArrayI64 direct get/set only for selected
   DirectArray-origin receivers.
2. Rust DirectArrayI64 runtime storage preserves stable layout,
   append/OOB behavior, materialization snapshot, and direct handle separation.

This row does not run perf and does not retire legacy helper/cache paths.

## Contract

```text
output_contract=array-slot-nativedirect-selected-method-semantic-smoke-v0
input_contract=array-slot-nativedirect-selected-method-lowering-implementation-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_array_i64_exact
python_lowering_smoke=ok
rust_direct_array_substrate_smoke=ok
direct_array_get_set_lowering_smoke=ok
direct_array_append_oob_storage_smoke=ok
direct_array_materialization_snapshot_smoke=ok
public_arraybox_handle_separation_smoke=ok
default_public_arraybox_helper_path_preserved=1
legacy_retirement_now=0
legacy_retirement_policy=defer_until_post_keeper_perf_owner_refresh
perf_measurement_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=array_slot_nativedirect_post_semantic_perf_owner_refresh
summary=ok
```

## Commands

```bash
PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_collection_method_call.py
HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact cargo test -p nyash_kernel direct_array_i64 --lib -- --nocapture
```

## Decision

The selected-method implementation is structurally smoke-clean. The next row is
a perf owner refresh, not legacy deletion. Helper/cache retirement needs direct
evidence that the DirectArray path has taken over the hot owner.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_selected_method_semantic_smoke_guard.sh
```
