# 293x-1116 MIMAP-486A Allocator Comparison C Mimalloc Result Presentation Extension Pilot

Status: landed
Date: 2026-05-22

## Purpose

Open the presentation extension pilot over the landed MIMAP-480A presentation
follow-on pilot report.

This row may publish an extension-ready report from the landed broader
presentation fields only. It must not change the provisional conclusion outcome
or reopen closed allocator/provider seams.

## Scope

- Consume the landed MIMAP-480A presentation follow-on pilot report.
- Accept only explicit, accepted presentation follow-on pilot reports.
- Publish a narrow extension-ready report over the landed provisional conclusion
  fields and metrics snapshot.
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
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Validation profile: `L2 scalar-mir`.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_pilot_guard.sh --level L2
```

## Task Order

1. Add the presentation extension owner over the landed MIMAP-480A pilot report.
2. Add a proof app and focused guard for accepted vs blocked extension states.
3. Keep benchmark reruns and allocator/provider ladders closed.
4. Select a later closeout or follow-on row only after the pilot guard is green.

## Completed

- Added `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionPilot`
  as the presentation extension owner over the landed MIMAP-480A
  presentation follow-on pilot report.
- Added a manifest-backed proof app and focused L2 guard.
- Preserved the landed provisional memory-side conclusion fields and metric
  snapshot while keeping benchmark reruns and allocator/provider ladders closed.
- Selected MIMAP-487A as the next row-selection card.

## Result

Landed. MIMAP-487A is selected as the next row-selection card.

## Next

MIMAP-487A should choose whether the next row is a presentation extension
closeout, a presentation extension follow-on plan, or a presentation-only
extension row.
