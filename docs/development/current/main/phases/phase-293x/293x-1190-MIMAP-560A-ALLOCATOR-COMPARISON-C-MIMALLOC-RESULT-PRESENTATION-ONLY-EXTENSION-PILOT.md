# 293x-1190 MIMAP-560A Allocator Comparison C Mimalloc Result Presentation-Only Extension Pilot

Status: landed
Date: 2026-05-22

## Purpose

Open the presentation-only extension pilot over the landed MIMAP-552A
comparison-ready pilot report and the closed MIMAP-550A explicit C mimalloc
comparison plan seam.

This row may publish a narrow presentation-only extension report from the landed
comparison-ready fields only. It must not change the stabilized comparison
contract or reopen closed allocator/provider/explicit-runner seams.

## Scope

- Consume the landed MIMAP-552A comparison-ready pilot report.
- Accept only explicit, accepted comparison-ready pilot reports.
- Publish a narrow presentation-only extension report over the landed shared
  contract fields.
- Keep benchmark reruns and allocator/provider ladders closed.

## Stop Lines

- No repeated or heavy benchmark pack.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No explicit C mimalloc runner execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `L2 scalar-mir`.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh --level L2
```

## Task Order

1. Add the presentation-only extension owner over the landed MIMAP-552A
   comparison-ready pilot report.
2. Add a proof app and focused guard for accepted vs blocked presentation-only
   extension states.
3. Keep benchmark reruns and allocator/provider ladders closed.
4. Select a later closeout or follow-on row only after the pilot guard is green.

## Completed

- Added `HakoAllocAllocatorComparisonCMimallocResultPresentationOnlyExtensionPilot`
  as the narrow presentation-only extension owner over the landed MIMAP-552A
  comparison-ready pilot report.
- Added a manifest-backed proof app and focused L2 guard.
- Preserved the landed comparison-ready shared contract fields while keeping
  benchmark reruns, allocator/provider ladders, and explicit runner execution
  closed.
- Selected MIMAP-561A as the next row-selection card.

## Result

Landed. MIMAP-561A is selected as the next row-selection card.

## Next

MIMAP-561A should choose whether the next row is a presentation-only extension
closeout, a deeper explicit C mimalloc runner planning row, or a
presentation-only extension follow-on row.
