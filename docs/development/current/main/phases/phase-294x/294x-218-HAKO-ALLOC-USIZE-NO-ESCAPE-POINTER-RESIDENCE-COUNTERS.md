---
Status: Landed
Date: 2026-05-24
Scope: no-escape pointer residence pilot owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-216
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-217-HAKO-ALLOC-USIZE-NO-ESCAPE-POINTER-RESIDENCE-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/segment_arena_backing_no_escape_pointer_residence_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_pointer_residence_pilot_guard.sh
---

# 294x-218 Hako Alloc Usize No-Escape Pointer Residence Counters

## Decision

Migrate only the selected owner-local
`HakoAllocSegmentArenaBackingNoEscapePointerResidencePilot` counters to exact
`usize` storage:

- `residence_count`
- `accepted_count`
- `reject_count`
- `missing_ledger_reject_count`
- `rejected_ledger_reject_count`
- `invalid_token_reject_count`
- `escape_reject_count`
- `closed_execution_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocSegmentArenaBackingNoEscapePointerResidencePilotReportFields`;
- `HakoAllocSegmentArenaBackingNoEscapePointerResidencePilotReport`;
- private pointer token, no-escape scope, payload, report mirror, byte-count,
  would-execute, bool-like, or signed sentinel fields;
- pointer-derived lookup, dereference, arena release/recycle, segment-map
  mutation, atomic bitmap execution, OSVM, worker/TLS, providers, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_pointer_residence_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
