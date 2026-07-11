---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-221
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako
  - tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
---

# 294x-225 Hako Alloc Usize Facade Huge Page-Model Route Counter Selection

## Decision

Select the owner-local
`HakoAllocObjectLifecycleFacadeHugePageModelRoute` counters as
`HAKO-ALLOC-USIZE-FIELD-GROUP-222`:

- `huge_attempt_count`
- `huge_success_count`
- `huge_failure_count`
- `small_forward_count`
- `fallback_attempt_count`
- `success_count`
- `failure_count`

These fields are monotonic facade route counters initialized to `0`. The
selected group is part of the comparison vertical slice huge/OSVM path: it
records route-level accounting for huge allocation attempts, small forwards,
fallback attempts, and final success/failure totals.

## Stop Line

Do not migrate:

- `HakoAllocObjectLifecycleFacadeHugePageModelReport`;
- report mirror fields, final result fields, status/reason vocabularies,
  page/block ids, pointer-like payloads, requested/committed size mirrors, or
  fallback source fields;
- `HakoAllocHugePageModel` fields already owned by earlier rows;
- huge release, unregister, unreserve, decommit, OSVM page-source execution,
  remote-free, worker/TLS, atomics, providers, backend matchers, provider
  package / DLL generation, hooks, host replacement, or `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-222` should migrate only the selected
`HakoAllocObjectLifecycleFacadeHugePageModelRoute` owner-local counters to
exact `usize` storage and update the facade huge page-model guard to assert the
route counter storage while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
