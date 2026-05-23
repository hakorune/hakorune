---
Status: Landed
Date: 2026-05-24
Scope: C mimalloc result presentation extension follow-on extension follow-on extension follow-on pilot owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-140-HAKO-ALLOC-USIZE-C-MIMALLOC-RESULT-PRESENTATION-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-EXTENSION-FOLLOW-ON-PILOT-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_pilot_box.hako
  - apps/hako-alloc-allocator-comparison-c-mimalloc-result-presentation-extension-follow-on-extension-follow-on-extension-follow-on-pilot-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh
---

# 294x-141 Hako Alloc Usize C Mimalloc Result Presentation Extension Follow-On Extension Follow-On Extension Follow-On Pilot Counters

## Decision

Migrate only the selected
`HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnPilot`
owner-local monotonic counters to exact `usize` storage:

- `extension_follow_on_extension_follow_on_extension_follow_on_count`
- `accepted_count`
- `blocked_count`
- `missing_pilot_reject_count`
- `blocked_pilot_reject_count`
- `missing_follow_on_input_reject_count`
- `closed_stop_line_reject_count`

The MIMAP-516A C mimalloc result presentation extension follow-on extension
follow-on extension follow-on pilot guard now asserts these fields are exact
`usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- comparison payloads such as allocation counts, requested bytes, RSS bytes,
  or deltas;
- `last_reason`, because it is reason vocabulary;
- report fields and `ReportFields` mirrors, because they remain signed
  comparison payload/mirror seams until their own row;
- performance conclusions, memory conclusions, repeated benchmark execution,
  process allocator replacement, hooks, backend matcher additions, provider
  package generation, worker/TLS, threads, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
