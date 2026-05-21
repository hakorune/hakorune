# 293x-1093 MIMAP-463A Allocator Comparison C Mimalloc Result Presentation / Decision Row Selection

Status: landed
Date: 2026-05-21

## Decision

Choose the next row after the reporting closeout pack without skipping directly
to an unguarded performance or memory-use conclusion.

## Context

The landed reporting pack already proves an accepted reporting path with stable
scalar evidence:

- `comparison_available == 1`
- `memory_evidence_present == 1`
- preserved Hako/C allocation and request-byte metrics plus deltas
- all performance/memory conclusions and process-global allocator/provider
  ladders still closed

That means the remaining risk is not missing presentation shape. The unresolved
question is whether this evidence is sufficient to open a later conclusion row
without rerunning benchmarks or reopening inactive ladders.

## Scope

- Decide whether the next row is:
  - presentation-only, or
  - a guarded first performance / memory-use conclusion preflight.
- Keep the choice grounded in the landed reporting inventory / diagnostics pack.
- Do not rerun benchmarks in this row.
- Do not make the final performance or memory-use conclusion in this row.

## Stop Lines

- No repeated or heavy benchmark pack.
- No performance conclusion.
- No memory-use conclusion.
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

Validation profile: `L0 planning`.

Validated:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Candidate Next Rows

- presentation-only row
- guarded first performance / memory-use conclusion preflight

## Result

Selected MIMAP-464A guarded first performance / memory-use conclusion preflight
as the next row.

The reporting inventory/diagnostics pack already carries the stable scalar
fields a presentation-only row would restate. The more valuable next boundary is
to prove that a future conclusion row can be opened from the landed reporting
evidence while final conclusions, heavy reruns, and inactive allocator/provider
ladders remain closed.

## Next

MIMAP-464A should validate conclusion eligibility from the landed reporting
inventory/diagnostics evidence, keep final performance/memory verdicts closed,
and only open a later conclusion row if the preflight stays green.
