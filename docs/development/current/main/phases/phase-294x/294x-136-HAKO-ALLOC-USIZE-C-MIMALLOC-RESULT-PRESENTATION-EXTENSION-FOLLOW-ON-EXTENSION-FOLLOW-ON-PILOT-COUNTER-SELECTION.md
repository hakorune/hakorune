---
Status: Landed
Date: 2026-05-24
Scope: select the next owner-local production exact `usize` field group after the presentation-chain mini handoff.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-135-HAKO-ALLOC-USIZE-PRESENTATION-CHAIN-MINI-HANDOFF.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_pilot_box.hako
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_pilot_guard.sh
---

# 294x-136 Hako Alloc Usize C Mimalloc Result Presentation Extension Follow-On Extension Follow-On Pilot Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnPilot`
as `HAKO-ALLOC-USIZE-FIELD-GROUP-157`:

- `extension_follow_on_extension_follow_on_count`
- `accepted_count`
- `blocked_count`
- `missing_pilot_reject_count`
- `blocked_pilot_reject_count`
- `missing_follow_on_input_reject_count`
- `closed_stop_line_reject_count`

These fields count the MIMAP-504A C mimalloc result presentation-chain owner
local classifications and reject outcomes. They do not carry allocation-count
payloads, requested-byte payloads, RSS evidence, deltas, presentation
readiness flags, reason vocabulary, conclusions, or provider / host allocator
state.

## Stop Line

This selection does not migrate:

- comparison payloads such as allocation counts, requested bytes, RSS bytes, or
  deltas;
- `last_reason`, because it is reason vocabulary;
- `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnPilotReportFields`
  and
  `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnPilotReport`
  fields, because report mirrors stay signed until their own row;
- performance conclusions, memory conclusions, repeated benchmark execution,
  process allocator replacement, hooks, backend matcher additions, provider
  package generation, worker/TLS, threads, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
