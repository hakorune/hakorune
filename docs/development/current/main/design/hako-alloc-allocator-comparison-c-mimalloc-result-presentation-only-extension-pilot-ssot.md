# Hako Alloc Allocator Comparison C Mimalloc Result Presentation-Only Extension Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-22
Owner: MIMAP-560A

## Decision: accepted

MIMAP-560A opens the presentation-only extension pilot over the landed
MIMAP-552A comparison-ready pilot report and the closed MIMAP-550A explicit C
mimalloc comparison contract seam.

The pilot accepts only explicit, accepted comparison-ready pilot reports and
reshapes the landed shared comparison contract into a narrow presentation-only
extension report. It must not change the comparison contract or reopen
benchmark, allocator, provider, or explicit runner stop lines.

## Reason Vocabulary

```text
0 = accepted presentation-only extension pilot
1 = missing comparison-ready pilot report
2 = blocked comparison-ready pilot report
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
- No explicit C mimalloc runner execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh --level L2
```
