# 293x-1075 MIMAP-453A Allocator Comparison C Mimalloc Explicit Runner Closeout

Status: landed
Date: 2026-05-21

## Purpose

Close out the MIMAP-451A / MIMAP-452A explicit C mimalloc runner execution and
evidence diagnostics pack before opening a comparison-result ledger row.

## Scope

- Re-run the MIMAP-451A explicit runner execution pilot guard.
- Re-run the MIMAP-452A evidence diagnostics guard.
- Confirm the runner evidence contract, `.hako` evidence ledger, and diagnostic
  reason vocabulary are in sync.
- Select the next narrow comparison-result ledger row.

## Design

SSOT:

```text
docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-closeout-ssot.md
```

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

Planned validation profile: closeout over MIMAP-451A / MIMAP-452A L2 guards.

Required guard:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_closeout_guard.sh
```

## Completed

- Added the closeout SSOT and guard.
- Reused MIMAP-451A / MIMAP-452A L2 guards as the evidence pack.
- Kept repeated benchmark execution, process allocator replacement, hooks,
  backend matcher additions, `#[global_allocator]`, provider package / DLL
  generation, hidden runtime discovery, and worker/thread execution closed.

## Next

MIMAP-454A should open a narrow C-vs-Hako comparison result ledger over explicit
runner evidence and existing Hako representative metrics.
