---
Status: Landed
Date: 2026-05-29
Scope: remove HashMap lookup from the exact-EXE ArrayBox single-thread numeric slot backend.
Blocker: ARRAY-SINGLE-THREAD-EXACT-HANDLE-CACHE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-345-POST-DIRECT-SLOT-SUPPORTED-STORAGE-OWNER-REFRESH.md
  - crates/nyash_kernel/src/plugin/array_slot_backend.rs
---

# 296x-346 Array Single-Thread Exact Handle Cache

## Purpose

Implement the row345 selected owner by replacing the diagnostic
`single_thread_exact` Array slot backend's per-access `HashMap` lookup with a
small handle-entry cache.

This row only changes the exact-EXE diagnostic Array slot backend. It does not
change public `ArrayBox` storage, the default `safe_rwlock` backend, MIRBuilder,
`.hako` source, provider activation, or allocator replacement.

## Contract

```text
output_contract=array-single-thread-exact-handle-cache-v0
input_contract=direct-slot-post-supported-storage-owner-refresh-v0
implemented_owner=array_slot_backend_single_thread_exact_handle_cache
implemented_owner_file=crates/nyash_kernel/src/plugin/array_slot_backend.rs
selected_backend=single_thread_exact
hashmap_lookup_removed=1
small_handle_entry_cache=1
default_backend_semantics_change=0
public_arraybox_storage_change=0
safe_rwlock_path_preserved=1
numeric_i64_slot_semantics_preserved=1
append_at_end_semantics_preserved=1
oob_semantics_preserved=1
invalid_handle_idx_semantics_preserved=1
unsupported_storage_failfast_preserved=1
mirbuilder_changes_allowed=0
hako_source_changes_allowed=0
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Cache Rule

`single_thread_exact` stores numeric i64 slot vectors by ArrayBox handle in a
thread-local entry list:

```text
ArraySlotCacheEntry {
  handle: i64
  values: Vec<i64>
}
```

This keeps the diagnostic backend narrow and removes the `HashMap` hash cost
observed in row345. The first access for a handle still initializes from the
visible `ArrayBox` storage, preserving setup behavior.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_single_thread_exact_handle_cache_guard.sh
```
