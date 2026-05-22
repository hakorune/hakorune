# 293x-1196 MIMAP-566A Allocator Comparison C Mimalloc Result Explicit Runner Planning Pilot

Status: landed
Date: 2026-05-22

## Purpose

Fix the terminal explicit-runner planning pilot contract while keeping
execution seams closed.

This row validates planning readiness only. It must not execute explicit C
mimalloc runner paths.

## Scope

- Add explicit-runner planning pilot owner/proof/guard.
- Fix accepted/blocked/missing/stop-line/accidental-execution reason vocabulary.
- Keep benchmark reruns, allocator/provider ladders, and runner execution closed.

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

## Validation

Validation profile: `terminal planning pilot L2 pack`.

Validated:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_explicit_runner_planning_pilot_guard.sh
```

## Task Order

1. Re-run MIMAP-564A planning follow-on guard as the anchor.
2. Fix terminal planning pilot reason/state contract.
3. Keep all execution seams closed.

## Completed

- Added explicit-runner terminal planning pilot owner/proof/guard.
- Locked reason vocabulary (`0..5`) for accepted/missing/stop-line/accidental.
- Selected MIMAP-567A as the close-criteria row.

## Next

MIMAP-567A fixes and locks phase-293x close criteria synchronized with:

```text
docs/development/current/main/design/mimalloc-blueprint-lane-close-criteria-ssot.md
```
