# Hako Alloc Allocator Comparison C Mimalloc Result First Conclusion Preflight SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-464A

## Decision: accepted

MIMAP-464A adds a guarded first performance / memory-use conclusion preflight
over the landed MIMAP-461A C-vs-Hako result reporting diagnostics.

The preflight accepts only reporting diagnostics that already carry comparison
availability, Hako-ready execution evidence, C-ready evidence, memory evidence,
and stable positive allocation/request-byte deltas while all conclusion and
allocator/provider stop lines remain closed. It does not make the final
performance or memory-use conclusion.

## Reason Vocabulary

```text
0 = accepted reporting diagnostics ready for a later conclusion row
1 = missing reporting diagnostics row
2 = blocked reporting diagnostics row
3 = missing comparison availability evidence
4 = missing Hako-ready execution evidence
5 = missing C-ready evidence
6 = missing memory evidence
7 = missing stable delta evidence
8 = closed stop-line violation
```

## Stop Lines

- No repeated benchmark execution.
- No performance conclusion.
- No memory-use conclusion.
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
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_first_conclusion_preflight_guard.sh --level L2
```
