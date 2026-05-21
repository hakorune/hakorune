# Hako Alloc Allocator Comparison C Mimalloc Result Presentation Extension Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-22
Owner: MIMAP-486A

## Decision: accepted

MIMAP-486A opens the first presentation extension pilot over the landed
MIMAP-480A presentation follow-on pilot report.

The pilot accepts only explicit, accepted presentation follow-on pilot reports
and reshapes the landed provisional memory-side conclusion plus metrics snapshot
into an extension-ready report. It must not change the provisional outcome or
reopen benchmark, allocator, or provider stop lines.

## Reason Vocabulary

```text
0 = accepted presentation extension pilot
1 = missing presentation follow-on pilot report
2 = blocked presentation follow-on pilot report
3 = missing extension input
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_extension_pilot_guard.sh --level L2
```
