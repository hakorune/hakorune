# 293x-1073 MIMAP-451A Allocator Comparison C Mimalloc Explicit Runner Execution Pilot

Status: selected current
Date: 2026-05-21

## Purpose

Open the first narrow C mimalloc comparison execution seam using an explicit
runner/tool contract.

## Scope

- Use the MIMAP-448A / MIMAP-449A readiness package.
- Execute only an explicit C mimalloc comparison runner if the row provides one.
- Capture stable output and memory-use evidence.
- Keep the execution distinct from process allocator replacement.
- Keep Hakorune provider package / DLL generation parked. The future ABI and
  package contract is documented in
  `docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md`.

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
