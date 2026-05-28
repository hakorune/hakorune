---
Status: Current
Date: 2026-05-28
Scope: define typed-object storage backend boundaries before single-thread fast-lane implementation.
Blocker: TYPED-OBJECT-STORAGE-BACKEND-SSOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-189-TYPED-OBJECT-HELPER-LOCK-COST-PROBE.md
---

# 296x-190 Typed Object Storage Backend SSOT

## Purpose

Define the runtime storage backend boundary before changing typed-object helper
behavior. Row189 selected `runtime_single_thread_fast_lane` because lock/global
slab cost dominates the field helper probe.

## Decision

```text
Decision: accepted

Default typed-object storage remains SafeMutexStore.
SingleThreadExactStore is a diagnostic/exact-EXE fast lane only.
The exported typed-object ABI stays unchanged.
```

## Storage Backends

```text
SafeMutexStore:
  default
  current semantics
  Mutex<Vec<TypedSlotObject>>
  works for all normal runtime profiles

SingleThreadExactStore:
  exact-EXE diagnostic/perf lane only
  no cross-thread guarantee
  no silent fallback
  same exported helper ABI
  same invalid handle/slot observable behavior
```

## Exported ABI That Must Not Change

```text
nyash.object.field_get_hii(handle, slot) -> i64
nyash.object.field_set_hii(handle, slot, value) -> void
nyash.object.field_get_u64_hii(handle, slot) -> u64
nyash.object.field_set_u64_hiu(handle, slot, value) -> i64
nyash.object.field_get_i64_hii(handle, slot) -> i64
nyash.object.field_set_i64_hii(handle, slot, value) -> i64
nyash.object.new_typed_hi(type_id, field_count) -> handle
nyash.object.register_typed_layout_* -> status
```

## Fail-Fast Boundary

```text
single_thread_exact requested outside exact-EXE/profile gate:
  fail-fast

threaded runtime / worker API observed with single_thread_exact:
  fail-fast

poisoned or impossible store state:
  fail-fast

invalid handle/slot:
  preserve existing helper semantics

backend mismatch:
  fail-fast; do not silently fall back to SafeMutexStore
```

## Implementation Shape

Target module split:

```text
crates/nyash_kernel/src/exports/typed_object.rs
  exported ABI, validation/status contract

crates/nyash_kernel/src/exports/typed_object_store.rs
  SafeMutexStore
  SingleThreadExactStore
  backend selection
```

The implementation row may keep the public file thin and move storage internals
behind a small store API:

```text
new_typed_object(type_id, fields) -> handle
get_legacy_i64(handle, slot) -> Option<i64>
set_legacy_i64(handle, slot, value) -> bool
get_unsigned_u64(handle, slot) -> Option<u64>
set_unsigned_u64(handle, slot, value) -> bool
```

## Non-Goals

```text
- Do not change field_access.py lowering in this SSOT row.
- Do not implement MIR scalar field residence here.
- Do not optimize ArrayBox here.
- Do not add hako_alloc by-name special cases.
- Do not open provider activation, allocator replacement, hooks, globals, or
  winner claims.
```

## Acceptance

```text
storage_backend_ssot=accepted
default_backend=SafeMutexStore
selected_fast_lane_backend=SingleThreadExactStore
exported_abi_unchanged=1
exact_exe_gate_required=1
silent_fallback_allowed=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
