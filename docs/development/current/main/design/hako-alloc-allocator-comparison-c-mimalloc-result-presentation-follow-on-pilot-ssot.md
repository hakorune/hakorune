# Hako Alloc Allocator Comparison C Mimalloc Result Presentation Follow-On Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-22
Owner: MIMAP-480A

## Decision: accepted

MIMAP-480A opens the first broader presentation pilot over the landed
MIMAP-474A presentation-only conclusion pilot report.

The pilot accepts only explicit, accepted presentation-only pilot reports and
reshapes the landed provisional memory-side conclusion plus metric snapshot into
a broader presentation-ready report. It must not change the provisional outcome
or reopen benchmark, allocator, or provider stop lines.

## Reason Vocabulary

```text
0 = accepted presentation follow-on pilot
1 = missing presentation-only pilot report
2 = blocked presentation-only pilot report
3 = missing follow-on input
4 = closed stop-line violation
```

## Stop Lines

- No repeated benchmark execution.
- No process allocator replacement.
- No hooks.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden discovery or process-global activation.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_follow_on_pilot_guard.sh --level L2
```
