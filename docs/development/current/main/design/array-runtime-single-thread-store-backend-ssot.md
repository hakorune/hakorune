---
Status: Provisional
Date: 2026-05-28
Scope: ArrayBox runtime single-thread store backend boundary for exact-EXE diagnostics.
Related:
  - docs/development/current/main/phases/phase-296x/296x-203-ARRAY-RUNTIME-SLOT-HELPER-COST-PROBE.md
---

# Array Runtime Single-Thread Store Backend SSOT

## Purpose

Define the ArrayBox runtime storage fast-lane boundary after row203 identified
`array_storage_write_lock` as the dominant subowner of the
`array_runtime_set_idx_i64` hot path.

This is not a generic ArrayBox rewrite. It is a diagnostic exact-EXE lane for
index-backed numeric slot helpers.

## Decision

```text
Decision: provisional

default_array_storage_backend=SafeRwLockArrayBox
selected_diagnostic_backend=SingleThreadExactArrayStore
selection_env=HAKO_ARRAY_SLOT_STORE
allowed_values=safe_rwlock|single_thread_exact
exported_array_helper_abi=unchanged
visible_arraybox_semantics=unchanged
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Boundary

Default behavior remains the existing `ArrayBox` implementation:

```text
ArrayBox:
  items: Arc<RwLock<ArrayStorage>>
```

The diagnostic backend may only accelerate these index-backed helper shapes:

```text
array_runtime_set_idx_i64(handle, idx, value)
array_runtime_get_idx(handle, idx)
array_slot_store_i64(handle, idx, value)
array_slot_load_encoded_i64(handle, idx)
```

The backend must preserve:

```text
- invalid handle/index behavior
- append-at-end semantics for set idx == len
- out-of-bounds failure behavior
- handle cache validity
- boxed fallback when a value cannot stay in numeric storage
- helper ABI names and signatures
```

## Fail-Fast

```text
HAKO_ARRAY_SLOT_STORE unset:
  use safe_rwlock

HAKO_ARRAY_SLOT_STORE=safe_rwlock:
  use current behavior

HAKO_ARRAY_SLOT_STORE=single_thread_exact:
  allowed only in exact-EXE diagnostic/perf rows
  no cross-thread guarantee
  no silent fallback after backend selection

unknown value:
  fail-fast
```

## Rejected

```text
hako_alloc_by_name_array_special_case:
  rejected

changing public ArrayBox semantics:
  rejected

provider activation / allocator replacement / hooks / globals:
  rejected

MIR ArrayBox residence transform:
  rejected for this backend row; it needs a separate SSOT and positive net
  helper-call evidence
```

## Required Implementation Row

Before implementation, a row must specify the exact code owner and verification:

```text
output_contract=array-runtime-single-thread-store-backend-v0
default_backend=SafeRwLockArrayBox
selected_backend=SingleThreadExactArrayStore
selection_env=HAKO_ARRAY_SLOT_STORE
exported_abi_unchanged=1
invalid_backend_fail_fast=1
array_slot_store_i64_smoke=ok
array_slot_load_i64_smoke=ok
semantic_summary=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
