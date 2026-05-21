# Hako Alloc Allocator Comparison C Mimalloc Result Presentation-Only Conclusion Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-474A

## Decision: accepted

MIMAP-474A opens the presentation-only conclusion pilot over the landed
MIMAP-468A first conclusion pilot report.

The pilot accepts only explicit, accepted first-conclusion pilot reports and
reshapes the landed provisional memory-side conclusion into a narrow
presentation-only report. It must not change the provisional outcome or reopen
benchmark, allocator, or provider stop lines.

## Reason Vocabulary

```text
0 = accepted presentation-only conclusion pilot
1 = missing first-conclusion pilot report
2 = blocked first-conclusion pilot report
3 = missing presentation input
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_conclusion_pilot_guard.sh --level L2
```
