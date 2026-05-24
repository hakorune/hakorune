---
Status: Landed
Date: 2026-05-24
Scope: facade huge page-model route owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-222
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-225-HAKO-ALLOC-USIZE-FACADE-HUGE-PAGE-MODEL-ROUTE-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako
  - tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
---

# 294x-226 Hako Alloc Usize Facade Huge Page-Model Route Counters

## Decision

Migrate only the selected owner-local
`HakoAllocObjectLifecycleFacadeHugePageModelRoute` counters to exact `usize`
storage:

- `huge_attempt_count`
- `huge_success_count`
- `huge_failure_count`
- `small_forward_count`
- `fallback_attempt_count`
- `success_count`
- `failure_count`

## Stop Line

This row does not migrate:

- `HakoAllocObjectLifecycleFacadeHugePageModelReport`;
- report mirror fields, final result fields, status/reason vocabularies,
  page/block ids, pointer-like payloads, requested/committed size mirrors, or
  fallback source fields;
- `HakoAllocHugePageModel` fields already owned by earlier rows;
- huge release, unregister, unreserve, decommit, OSVM page-source execution,
  remote-free, worker/TLS, atomics, providers, backend matchers, provider
  package / DLL generation, hooks, host replacement, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
