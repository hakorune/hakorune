---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-215
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/segment_arena_backing_no_escape_pointer_residence_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_pointer_residence_pilot_guard.sh
---

# 294x-217 Hako Alloc Usize No-Escape Pointer Residence Counter Selection

## Decision

Select the owner-local
`HakoAllocSegmentArenaBackingNoEscapePointerResidencePilot` counters as
`HAKO-ALLOC-USIZE-FIELD-GROUP-216`:

- `residence_count`
- `accepted_count`
- `reject_count`
- `missing_ledger_reject_count`
- `rejected_ledger_reject_count`
- `invalid_token_reject_count`
- `escape_reject_count`
- `closed_execution_reject_count`

These fields are monotonic no-escape pointer residence pilot / reject counters
initialized to `0`. The selected group records model-space residence accounting
only; pointer-derived lookup, dereference, arena release/recycle, segment-map
mutation, atomic bitmap execution, OSVM, worker/TLS, providers, backend
matchers, provider package / DLL generation, and `#[global_allocator]` remain
closed.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocSegmentArenaBackingNoEscapePointerResidencePilotReportFields`;
- `HakoAllocSegmentArenaBackingNoEscapePointerResidencePilotReport`;
- private pointer token, no-escape scope, payload, report mirror, byte-count,
  would-execute, bool-like, or signed sentinel fields;
- pointer-derived lookup, dereference, arena release/recycle, segment-map
  mutation, atomic bitmap execution, OSVM, worker/TLS, providers, backend
  matchers, provider package / DLL generation, or `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-216` should migrate only the selected owner-local
counters and update the no-escape pointer residence pilot guard to assert exact
`usize` storage while report mirrors and payload fields remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
