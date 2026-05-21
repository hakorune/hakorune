# 293x-1073 MIMAP-451A Allocator Comparison C Mimalloc Explicit Runner Execution Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open the first narrow C mimalloc comparison execution seam using an explicit
runner/tool contract.

## Scope

- Use the MIMAP-448A / MIMAP-449A readiness package.
- Execute only an explicit C mimalloc comparison runner if the row provides one.
- Capture stable output and memory-use evidence.
- Keep the execution distinct from process allocator replacement.
- Record explicit runner evidence in `.hako` model space without making the
  runner part of Hakorune runtime execution.
- Keep Hakorune provider package / DLL generation parked. The future ABI and
  package contract is documented in
  `docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md`.

## Design

SSOT:

```text
docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-execution-pilot-ssot.md
```

Tool:

```text
tools/allocator/c_mimalloc_explicit_runner.sh
tools/allocator/c_mimalloc_explicit_runner.c
```

## Stop Lines

- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No implicit C mimalloc execution or hidden runner discovery.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

This is a first execution seam. It should define exact runner / output evidence
before any heavy or repeated benchmark run is added.

Required guard:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh --level L2
```

## Completed

- Added the explicit C mimalloc runner tool:
  `tools/allocator/c_mimalloc_explicit_runner.sh`.
- Added the runner C workload:
  `tools/allocator/c_mimalloc_explicit_runner.c`.
- Added the `.hako` evidence owner:
  `allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_box.hako`.
- Added the proof app and manifest row for MIMAP-451A.
- Kept process allocator replacement, hooks, backend matcher additions,
  `#[global_allocator]`, provider package / DLL generation, hidden runtime
  discovery, and worker/thread execution closed.

## Next

MIMAP-452A should add observer diagnostics for explicit runner evidence without
repeating heavy benchmark execution.
