---
Status: Landed
Date: 2026-05-28
Scope: define the ArrayBox runtime single-thread store backend boundary before implementation.
Blocker: ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-SSOT-296X-001
Related:
  - docs/development/current/main/design/array-runtime-single-thread-store-backend-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-203-ARRAY-RUNTIME-SLOT-HELPER-COST-PROBE.md
---

# 296x-204 Array Runtime Single-Thread Store Backend SSOT

## Purpose

Accept the ArrayBox runtime single-thread store backend boundary selected by
row203. This row is docs-only and keeps runtime/compiler behavior unchanged.

## Decision

```text
Decision: accepted

array_runtime_single_thread_store_backend_ssot=accepted
design_ssot=docs/development/current/main/design/array-runtime-single-thread-store-backend-ssot.md
default_array_storage_backend=SafeRwLockArrayBox
selected_diagnostic_backend=SingleThreadExactArrayStore
selection_env=HAKO_ARRAY_SLOT_STORE
exported_array_helper_abi=unchanged
visible_arraybox_semantics=unchanged
default_visible_arraybox_semantics=unchanged
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next

```text
row205:
  array_runtime_single_thread_store_backend_implementation

Goal:
  implement a diagnostic exact-EXE ArrayBox slot backend or reject the backend
  if the code owner cannot be kept narrow without changing public ArrayBox
  semantics.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_array_runtime_single_thread_store_backend_ssot_guard.sh
```
