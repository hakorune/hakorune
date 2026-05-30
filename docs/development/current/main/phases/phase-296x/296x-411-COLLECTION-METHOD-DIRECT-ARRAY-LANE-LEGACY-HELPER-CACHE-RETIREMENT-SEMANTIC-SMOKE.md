---
Status: Landed
Date: 2026-05-30
Scope: smoke the scoped retirement slice before perf owner refresh.
Blocker: COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-RETIREMENT-PERF-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-410-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-409-COLLECTION-METHOD-DIRECT-ARRAY-LANE-LEGACY-HELPER-CACHE-RETIREMENT-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-412-COLLECTION-METHOD-DIRECT-ARRAY-LANE-POST-RETIREMENT-PERF-OWNER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-373-ARRAYBOX-PUBLIC-SEMANTICS-AND-DIRECTARRAY-SPLIT-SSOT.md
  - crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
  - src/llvm_py/tests/test_collection_method_call.py
  - src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_semantic_smoke_guard.sh
---

# 296x-411 Collection Method Direct Array Lane Legacy Helper Cache Retirement Semantic Smoke

## Purpose

Smoke the scoped retirement implementation after row410.

This row proves that:

1. public `ArrayBox` birth still works,
2. `nyash.array.direct_i64.birth_h` still produces DirectArrayI64,
3. DirectArray materialization snapshot still works,
4. the selected-method DirectArray lowering still takes the direct path,
5. the current proof-app readiness summary remains ok.

This row does not claim perf and does not retire more helper/cache surfaces.
Its smoke is landed, and the follow-on perf owner refresh row opens next.

## Contract

```text
output_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-semantic-smoke-v0
input_contract=collection-method-direct-array-lane-legacy-helper-cache-retirement-implementation-v0
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
selected_next=collection_method_call_direct_array_lane_post_retirement_perf_owner_refresh
summary=ok
```

## Mini Task Board

Keep each item small enough for a mini worker. This row is smoke/report only.
Do not open implementation. Treat each task below as independently runnable.
Do not bundle multiple files into one worker pass.

### CSM-001: Direct Birth Smoke

Input:
- `src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py`

Output:
- short note on direct-array birth and origin-fact coverage

Acceptance:
- direct birth remains separate from public ArrayBox birth
- no implementation is proposed

### CSM-002: Collection Method Smoke

Input:
- `src/llvm_py/tests/test_collection_method_call.py`

Output:
- short note on the selected-method direct-array path and helper fallback anchors

Acceptance:
- the tests still pin the direct-array split
- no implementation is proposed

### CSM-003: Runtime Data Policy Smoke

Input:
- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`

Output:
- short note on the remaining policy assertions that still anchor the split

Acceptance:
- runtime data policy still keeps the direct-array route exact-only
- no implementation is proposed

### CSM-004: Guard And Index

Input:
- this card
- current state
- check index

Output:
- passing row411 guard

Acceptance:
- `bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_semantic_smoke_guard.sh` passes
- `git diff --check` passes

## Decision

The scoped retirement slice is structurally smoke-clean. The next row refreshes
the post-retirement perf owner classification and decides whether the legacy
helper/cache surface can be retired further.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_collection_method_direct_array_lane_legacy_helper_cache_retirement_semantic_smoke_guard.sh
```
