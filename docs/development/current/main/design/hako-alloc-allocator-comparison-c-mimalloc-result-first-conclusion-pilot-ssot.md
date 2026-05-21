# Hako Alloc Allocator Comparison C Mimalloc Result First Conclusion Pilot SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-468A

## Decision: accepted

MIMAP-468A opens the first provisional conclusion pilot over the landed
MIMAP-464A first conclusion preflight report.

The pilot accepts only explicit, accepted preflight reports and records a
provisional memory-side conclusion in model space only. It keeps provisional
performance conclusion state closed because the landed evidence does not yet add
new timing facts.

## Reason Vocabulary

```text
0 = accepted first conclusion pilot
1 = missing preflight report
2 = blocked preflight report
3 = missing stable memory delta evidence
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_pilot_guard.sh --level L2
```
