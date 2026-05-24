---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-217
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/segment_arena_backing_pointer_derived_lookup_execution_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_pointer_derived_lookup_execution_pilot_guard.sh
---

# 294x-221 Hako Alloc Usize Pointer-Derived Lookup Counter Selection

## Decision

Select the owner-local
`HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilot.lookup_count`
counter as `HAKO-ALLOC-USIZE-FIELD-GROUP-218`.

This field is a monotonic pointer-derived lookup execution pilot counter
initialized to `0`. The selected group records that the owner attempted its
bounded pointer-derived lookup route. It does not widen pointer payloads,
result tokens, status vocabularies, report mirrors, byte-count payloads, or any
downstream execution seam.

## Stop Line

Do not migrate:

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

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-218` should migrate only `lookup_count` to exact
`usize` storage and update the pointer-derived lookup execution pilot guard to
assert exact `usize` storage while all report mirrors and payload fields remain
signed or otherwise unchanged.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
