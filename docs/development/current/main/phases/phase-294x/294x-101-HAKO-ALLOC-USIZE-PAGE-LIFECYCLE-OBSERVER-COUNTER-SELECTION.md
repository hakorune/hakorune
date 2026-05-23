---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the heap reuse priority counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-100-HAKO-ALLOC-USIZE-HEAP-REUSE-PRIORITY-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako
  - tools/checks/k2_wide_hako_alloc_page_lifecycle_invariant_guard.sh
---

# 294x-101 Hako Alloc Usize Page Lifecycle Observer Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocPageLifecycleInvariantObserver` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-123`:

- `observe_count`
- `missing_count`
- `active_count`
- `retired_count`
- `decommitted_count`
- `recommitted_count`

These fields count M207 page lifecycle observer visits and state
classifications. They do not carry page identity, lifecycle state vocabulary,
report flags, free-count snapshots, generation counters, or backing byte
payloads.

## Stop Line

This selection does not migrate:

- `HakoAllocPageLifecycleInvariantReport` fields, because they are status,
  page-id, state, flag, count snapshot, generation, and byte payload
  vocabulary;
- `last_page_id`, because it uses the `-1` signed sentinel;
- `last_state`, because it is state vocabulary;
- heap/page queues, page-source adapters, heap/page mutation, OSVM
  byte/pointer payloads, provider / hook / global-allocator rows, TLS,
  atomics, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
