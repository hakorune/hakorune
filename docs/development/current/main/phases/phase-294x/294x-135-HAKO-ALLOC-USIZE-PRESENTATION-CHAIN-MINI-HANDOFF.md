---
Status: Landed
Date: 2026-05-24
Scope: Mini handoff for the remaining C mimalloc result presentation-chain `usize` field-group rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
---

# 294x-135 Hako Alloc Usize Presentation Chain Mini Handoff

## Decision

Keep `HAKO-ALLOC-USIZE-FIELD-GROUP-156` as the current blocker, but make the
remaining C mimalloc result presentation-chain work executable by a smaller
model. Each owner still advances as a pair:

1. selection-only row;
2. migration row.

Do not batch multiple owners into one migration commit. The migration rows are
mechanical, but each owner has its own guard and proof app, so one owner remains
the review and rollback unit.

## Mini Execution Loop

For the next owner in the table:

1. Create a selection card.
2. Update `CURRENT_STATE.toml` latest-card fields and the taskboard current
   blocker text.
3. Add or update the pending row in `lang/src/hako_alloc/memory/NUMERIC_FIELDS.md`.
4. Verify the selection row:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

5. Commit the selection row.
6. In the migration row, change only the selected owner-local monotonic counters
   from `i64 = 0` to `usize = 0`.
7. Keep `last_reason`, report mirrors, comparison payloads, ids, byte values,
   deltas, readiness/status values, and conclusion payloads signed or unchanged.
8. Extend the owner guard so it checks:
   - the selection card and migration card exist;
   - selected source fields are `usize = 0`;
   - `last_reason: i64 = 0` remains signed;
   - the MIR typed-object plan records the selected fields as `usize`;
   - the MIR typed-object plan records `last_reason` as `i64`.
9. Verify the migration row:

```bash
bash tools/checks/<selected-owner-guard>.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

10. Commit the migration row.

## Common Counter Set

Most remaining owners use this counter shape:

- the owner-specific primary counter from the table;
- `accepted_count`;
- `blocked_count`;
- owner-specific reject counters already present in the source box;
- `closed_stop_line_reject_count`.

Do not infer extra fields from names. If a field is not part of the owner-local
monotonic counter group, leave it for a later explicit row.

## Stop Line

This handoff does not open:

- C mimalloc execution expansion beyond the existing proof owner;
- performance or memory conclusions;
- repeated benchmark execution;
- comparison payload migration (`allocation_count`, requested bytes, RSS bytes,
  deltas, or derived result payloads);
- report field or `ReportFields` mirror migration;
- process allocator replacement;
- provider package generation;
- hooks or `#[global_allocator]`;
- backend matcher additions;
- worker/TLS or thread behavior.

## Remaining Owner Sequence

| Suggested next owner | Source file | Box owner | Primary counter | Guard |
| --- | --- | --- | --- | --- |
| MIMAP-504A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnPilot` | `extension_follow_on_extension_follow_on_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_pilot_guard.sh` |
| MIMAP-510A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionPilot` | `follow_on_extension_follow_on_extension_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_pilot_guard.sh` |
| MIMAP-516A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnPilot` | `extension_follow_on_extension_follow_on_extension_follow_on_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh` |
| MIMAP-522A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionPilot` | `follow_on_extension_follow_on_extension_follow_on_extension_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_pilot_guard.sh` |
| MIMAP-528A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnPilot` | `extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh` |
| MIMAP-534A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionPilot` | `follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_pilot_guard.sh` |
| MIMAP-540A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnPilot` | `extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh` |
| MIMAP-546A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionPilot` | `follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_pilot_guard.sh` |
| MIMAP-552A | `allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnPilot` | `comparison_ready_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh` |
| MIMAP-560A | `allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_box.hako` | `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot` | `presentation_count` | `tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh` |

## Next Selection Hint

Unless a newer SSOT overrides this card, `HAKO-ALLOC-USIZE-FIELD-GROUP-156`
should select the MIMAP-504A owner:

```text
HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnPilot
```

The expected migration row after that selection should be
`HAKO-ALLOC-USIZE-FIELD-GROUP-157`.
