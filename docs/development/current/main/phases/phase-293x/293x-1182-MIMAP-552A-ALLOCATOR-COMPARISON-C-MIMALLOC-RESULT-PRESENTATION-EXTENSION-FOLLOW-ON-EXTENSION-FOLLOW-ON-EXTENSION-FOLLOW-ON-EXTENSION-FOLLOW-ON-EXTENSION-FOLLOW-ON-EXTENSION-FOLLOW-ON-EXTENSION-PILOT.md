# 293x-1182 MIMAP-552A Allocator Comparison C Mimalloc Result Presentation Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Extension Follow-On Pilot

Status: landed
Date: 2026-05-22

## Purpose

Open the first comparison-ready presentation extension follow-on extension
follow-on extension follow-on extension follow-on extension follow-on
extension follow-on extension follow-on pilot over the landed MIMAP-546A
deeper-extension-ready report and the fixed MIMAP-550A explicit C mimalloc
contract.

This row may publish a narrow comparison-ready report that fixes the explicit C
mimalloc / hako_alloc shared contract fields only. It must not execute a C
runner, rerun benchmarks, or reopen allocator/provider seams.

## Scope

- Consume the landed MIMAP-546A deeper-extension-ready report.
- Accept only explicit, accepted MIMAP-546A reports whose hako_alloc and
  explicit C mimalloc contract fields are present.
- Publish a narrow comparison-ready report with the shared schema fields fixed
  for later runner work.
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_extension_follow_on_pilot_guard.sh --level L2
```

## Task Order

1. Add the comparison-ready presentation extension follow-on extension
   follow-on extension follow-on extension follow-on extension follow-on
   extension follow-on extension follow-on owner over the landed MIMAP-546A
   report.
2. Add a proof app and focused guard for accepted vs blocked comparison-ready
   states.
3. Keep benchmark reruns and allocator/provider ladders closed.
4. Select a later closeout or follow-on row only after the pilot guard is green.

## Completed

- Added
  `HakoAllocAllocatorComparisonCMimallocResultPresentationExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnExtensionFollowOnPilot`
  as the comparison-ready owner over the landed MIMAP-546A report.
- Added a manifest-backed proof app and focused L2 guard.
- Fixed the explicit C mimalloc shared schema fields without executing a runner
  or reopening allocator/provider ladders.
- Selected MIMAP-553A as the next row-selection card.

## Result

Landed. MIMAP-553A is selected as the next row-selection card.

## Next

MIMAP-553A should choose whether the next row is a presentation extension
follow-on extension follow-on extension follow-on extension follow-on
extension follow-on extension follow-on extension follow-on closeout, a
presentation extension follow-on extension follow-on extension follow-on
extension follow-on extension follow-on extension follow-on extension
follow-on plan closeout, or a presentation-only extension row.
