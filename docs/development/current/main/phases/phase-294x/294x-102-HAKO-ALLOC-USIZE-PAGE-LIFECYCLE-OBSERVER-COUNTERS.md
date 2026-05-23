---
Status: Landed
Date: 2026-05-23
Scope: page lifecycle observer owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-101-HAKO-ALLOC-USIZE-PAGE-LIFECYCLE-OBSERVER-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako
  - apps/hako-alloc-page-lifecycle-invariant-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_page_lifecycle_invariant_guard.sh
---

# 294x-102 Hako Alloc Usize Page Lifecycle Observer Counters

## Decision

Migrate only the selected `HakoAllocPageLifecycleInvariantObserver`
owner-local monotonic counters to exact `usize` storage:

- `observe_count`
- `missing_count`
- `active_count`
- `retired_count`
- `decommitted_count`
- `recommitted_count`

The M207 page lifecycle invariant guard now asserts these fields are exact
`usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `HakoAllocPageLifecycleInvariantReport` fields, because they are status,
  page-id, state, flag, count snapshot, generation, and byte payload
  vocabulary;
- `last_page_id`, because it uses the `-1` signed sentinel;
- `last_state`, because it is state vocabulary;
- heap/page queues, page-source adapters, heap/page mutation, OSVM
  byte/pointer payloads, provider / hook / global-allocator rows, TLS,
  atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_page_lifecycle_invariant_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
