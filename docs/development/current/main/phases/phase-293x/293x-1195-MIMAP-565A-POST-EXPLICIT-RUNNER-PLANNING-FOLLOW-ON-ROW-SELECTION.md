# 293x-1195 MIMAP-565A Post Explicit Runner Planning Follow-On Row Selection

Status: selected current
Date: 2026-05-22

## Decision

Select the next narrow allocator row after the explicit runner planning
follow-on row. The lane now has a closed presentation-only extension pack and a
closed deeper explicit runner planning boundary while execution seams remain
closed.

## Candidate Next Rows

- explicit runner planning pilot row
- presentation-only extension follow-on row
- another closeout extension

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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_follow_on_guard.sh
```

## Decision Result

Selected:

```text
MIMAP-566A Allocator Comparison C Mimalloc Result Explicit Runner Planning Pilot
```

The narrowest useful next step is to open an explicit runner planning pilot
contract while keeping execution and benchmark seams closed.
