---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-219
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/segment_arena_backing_pointer_derived_lookup_execution_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_pointer_derived_lookup_execution_pilot_guard.sh
---

# 294x-223 Hako Alloc Usize Pointer-Derived Lookup Reject Counter Selection

## Decision

Select the remaining owner-local
`HakoAllocSegmentArenaBackingPointerDerivedLookupExecutionPilot` accept/reject
counters as `HAKO-ALLOC-USIZE-FIELD-GROUP-220`:

- `accepted_count`
- `reject_count`
- `missing_handle_reject_count`
- `rejected_handle_reject_count`
- `invalid_pointer_token_reject_count`
- `invalid_handle_reject_count`
- `invalid_lookup_reject_count`
- `closed_execution_reject_count`

These fields are monotonic pointer-derived lookup execution pilot counters
initialized to `0`. They complement the already exact `lookup_count` counter
without widening tokens, status vocabularies, byte-count payloads, report
mirrors, or downstream execution seams.

## Stop Line

Do not migrate:

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

`HAKO-ALLOC-USIZE-FIELD-GROUP-220` should migrate only the selected owner-local
accept/reject counters to exact `usize` storage and update the pointer-derived
lookup execution pilot guard to assert that `last_reason` and all report mirrors
remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
