---
Status: Landed
Date: 2026-05-28
Scope: implement a typed-object single-thread exact storage backend without changing exported helper ABI.
Blocker: TYPED-OBJECT-RUNTIME-SINGLE-THREAD-FAST-LANE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-189-TYPED-OBJECT-HELPER-LOCK-COST-PROBE.md
  - docs/development/current/main/phases/phase-296x/296x-190-TYPED-OBJECT-STORAGE-BACKEND-SSOT.md
---

# 296x-191 Typed Object Runtime Single-Thread Fast Lane

## Purpose

Implement the storage backend seam selected by row190. The row189 probe showed
that typed-object field helpers are dominated by the lock/global-slab boundary,
so this row adds a diagnostic exact-EXE fast lane while keeping the existing
exported helper ABI unchanged.

## Front

```text
front=exact-EXE typed-object field helper runtime
failure_mode=runtime helper lock/global-slab cost dominates field get/set
current_owner=typed_object_storage_backend
hot_transition=slot-indexed scalar field op -> Mutex-protected global Vec<TypedSlotObject>
next_seam=SingleThreadExactStore behind unchanged exported helper ABI
reject_seam=MIR scalar field residence, ArrayBox optimization, provider activation, allocator replacement
```

## Decision

```text
Decision: accepted

HAKO_TYPED_OBJECT_STORE=safe_mutex:
  default behavior

HAKO_TYPED_OBJECT_STORE=single_thread_exact:
  explicit exact-EXE/perf diagnostic lane
  no cross-thread guarantee
  no silent fallback to SafeMutexStore
  exported typed-object ABI unchanged
```

The selection environment variable is the explicit exact-EXE/profile gate for
this diagnostic row. It is documented in `docs/reference/environment-variables.md`
and must not be enabled by default runners.

## Implementation Boundary

```text
crates/nyash_kernel/src/exports/typed_object.rs:
  exported ABI, layout registration, status preservation

crates/nyash_kernel/src/exports/typed_object_store.rs:
  SafeMutexStore
  SingleThreadExactStore
  backend selection
  object field read/write storage operations
```

## Non-Goals

```text
- Do not change LLVM field_access.py lowering.
- Do not implement MIR scalar field residence.
- Do not optimize ArrayBox.
- Do not add hako_alloc by-name special cases.
- Do not open provider activation, allocator replacement, hooks, globals, or
  winner claims.
```

## Acceptance

```text
output_contract=typed-object-runtime-single-thread-fast-lane-v0
default_backend=SafeMutexStore
selected_backend=SingleThreadExactStore
selection_env=HAKO_TYPED_OBJECT_STORE
exported_abi_unchanged=1
safe_mutex_unit_tests=ok
single_thread_exact_smoke=ok
invalid_backend_fail_fast=ok
semantic_summary=ok
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
