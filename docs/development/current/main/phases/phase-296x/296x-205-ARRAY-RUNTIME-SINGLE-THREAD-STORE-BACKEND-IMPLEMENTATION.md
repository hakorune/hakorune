---
Status: Current
Date: 2026-05-28
Scope: implement the ArrayBox helper-side single-thread exact backend selected by row204.
Blocker: ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/design/array-runtime-single-thread-store-backend-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-204-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-SSOT.md
---

# 296x-205 Array Runtime Single-Thread Store Backend Implementation

## Purpose

Implement the diagnostic exact-EXE ArrayBox slot backend selected by row204
without changing exported helper symbols or default ArrayBox behavior.

This row keeps the backend seam on the runtime helper side:

```text
array_runtime_set_idx_i64
  -> array_slot_store_i64
      -> selected backend:
           safe_rwlock
           single_thread_exact
```

## Decision

```text
Decision: accepted

output_contract=array-runtime-single-thread-store-backend-v0
default_backend=SafeRwLockArrayBox
selected_backend=SingleThreadExactArrayStore
selection_env=HAKO_ARRAY_SLOT_STORE
allowed_values=safe_rwlock|single_thread_exact
invalid_backend_fail_fast=1

code_owner=crates/nyash_kernel/src/plugin/array_slot_backend.rs
helper_store_owner=crates/nyash_kernel/src/plugin/array_slot_store.rs
helper_load_owner=crates/nyash_kernel/src/plugin/array_slot_load.rs
arraybox_public_storage_changed=0
exported_abi_unchanged=1
default_visible_arraybox_semantics_unchanged=1

single_thread_exact_store_i64_path=implemented
single_thread_exact_load_i64_path=implemented
append_at_end_semantics=preserved_for_helper_path
negative_index_semantics=preserved_for_helper_path
oob_semantics=preserved_for_helper_path
handle_cache_validity=preserved
boxed_fallback_semantics=not_changed_by_default_backend

provider_activation=0
allocator_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Backend Boundary

`safe_rwlock` remains the default and calls the existing `ArrayBox` raw helper
methods. `single_thread_exact` is a diagnostic exact-EXE helper backend for the
numeric i64 slot path only. It keeps an index-backed i64 sidecar per helper
handle and fails fast when the initial ArrayBox slots are not readable as i64.

The diagnostic backend is not a public ArrayBox storage replacement. Mixed use
through general ArrayBox visible APIs is out of scope for this row; default
behavior remains the public semantics owner.

## Evidence

```text
safe_rwlock_smoke=ok
single_thread_exact_smoke=ok
invalid_backend_fail_fast=ok
safe_rwlock_body_elapsed_ns_scout=215000000
single_thread_exact_body_elapsed_ns_scout=120000000
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_array_runtime_single_thread_store_backend_implementation_guard.sh
```
