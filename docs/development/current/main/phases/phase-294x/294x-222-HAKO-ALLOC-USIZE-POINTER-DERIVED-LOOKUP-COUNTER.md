---
Status: Landed
Date: 2026-05-24
Scope: pointer-derived lookup execution pilot owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-218
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-221-HAKO-ALLOC-USIZE-POINTER-DERIVED-LOOKUP-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/segment_arena_backing_pointer_derived_lookup_execution_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_pointer_derived_lookup_execution_pilot_guard.sh
---

# 294x-222 Hako Alloc Usize Pointer-Derived Lookup Counter

## Decision

Migrate only the selected owner-local
`HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilot.lookup_count`
counter to exact `usize` storage.

## Stop Line

This row does not migrate:

- `accepted_count`, `reject_count`, or reject-subreason counters;
- `last_reason`;
- `HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilotReportFields`;
- `HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilotReport`;
- lookup result token, arena handle token, private pointer token, lifecycle
  identity, report mirror, byte-count, would-execute, bool-like, status, or
  signed sentinel fields;
- dereference, arena release/recycle, segment-map mutation, atomic bitmap
  execution, OSVM, worker/TLS, providers, backend matchers, provider package /
  DLL generation, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_pointer_derived_lookup_execution_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
