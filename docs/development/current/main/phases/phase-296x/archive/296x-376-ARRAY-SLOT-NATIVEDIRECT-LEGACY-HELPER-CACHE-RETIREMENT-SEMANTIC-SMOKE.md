---
Status: Landed
Date: 2026-05-30
Scope: smoke the scoped ArraySlot NativeDirect retirement slice before perf owner refresh.
Blocker: ARRAY-SLOT-NATIVEDIRECT-POST-RETIREMENT-PERF-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-375-ARRAY-SLOT-NATIVEDIRECT-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md
  - docs/development/current/main/phases/phase-296x/296x-377-ARRAY-SLOT-NATIVEDIRECT-POST-RETIREMENT-PERF-OWNER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
  - src/llvm_py/tests/test_collection_method_call.py
  - src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py
  - tools/allocator/array_slot_nativedirect_lowering_readiness_inventory.py
  - tools/checks/k2_wide_phase296x_array_slot_nativedirect_legacy_helper_cache_retirement_semantic_smoke_guard.sh
---

# 296x-376 ArraySlot NativeDirect Legacy Helper Cache Retirement Semantic Smoke

## Purpose

Smoke the scoped retirement implementation after row375.

This row proves that:

1. public `ArrayBox` birth still works,
2. `nyash.array.direct_i64.birth_h` still produces DirectArrayI64,
3. DirectArray materialization snapshot still works,
4. the selected-method DirectArray lowering still takes the direct path,
5. the current proof-app readiness summary remains ok.

This row does not claim perf and does not retire more helper/cache surfaces.

## Contract

```text
output_contract=array-slot-nativedirect-legacy-helper-cache-retirement-semantic-smoke-v0
input_contract=array-slot-nativedirect-legacy-helper-cache-retirement-implementation-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_backend=direct_array_i64_exact
default_public_birth_symbol=nyash.array.birth_h
selected_direct_birth_symbol=nyash.array.direct_i64.birth_h
receiver_origin_fact=resolver.direct_array_i64_ids
receiver_origin_fact_required=1
public_arraybox_birth_smoke=ok
direct_array_birth_smoke=ok
direct_array_materialization_snapshot_smoke=ok
selected_method_direct_array_lowering_smoke=ok
proof_app_summary=ok
public_arraybox_handle_reinterpret_as_direct=0
legacy_helper_cache_retirement_now=0
silent_fallback_allowed=0
public_arraybox_behavior_deletion=0
handle_entry_cache_deletion=0
public_helper_abi_removal=0
perf_measurement_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
selected_next=array_slot_nativedirect_post_retirement_perf_owner_refresh
summary=ok
```

## Commands

```bash
PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py
PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_collection_method_call.py
HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact cargo test -p nyash_kernel direct_array_i64 --lib -- --nocapture
python3 tools/allocator/array_slot_nativedirect_lowering_readiness_inventory.py --out "$report"
```

## Decision

The scoped retirement slice is structurally smoke-clean. The next row refreshes
the post-retirement perf owner classification and decides whether the legacy
helper/cache surface can be retired further.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_slot_nativedirect_legacy_helper_cache_retirement_semantic_smoke_guard.sh
```
