---
Status: Active
Date: 2026-04-01
Scope: current mainline / secondary lanes / parked lanes の one-screen map。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md
---

# Workstream Map

## Purpose

- current lane の順番と残りの見通しだけを 1 画面で読む。
- canonical rough task order is owned by `design/kernel-implementation-phase-plan-ssot.md`.
- this file is the operational one-screen mirror, not a competing SSOT.

## Current Order

1. `policy-refresh`
   - active docs/policy lane
   - active sequence:
     - `stage / docs / naming`
     - `K1 done-enough stop-line`
     - `K2-core accepted stop-line`
     - `zero-rust default operationalization` (landed)
     - `stage2plus entry / first optimization wave` (accepted)
     - `phase-29x backend owner cutover prep`
   - current read:
     - `K2-core` is closed
     - `K2-wide` boundary-shrink lock-down is closed enough for handoff
     - `zero-rust default operationalization` is landed
     - `stage2plus entry / first optimization wave` is accepted
     - current active lane is `phase-29x backend owner cutover prep`
     - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64` are already landed
     - boundary audit result: `RuntimeDataBox.delete` is still absent; delete stays on the `MapBox -> RawMap -> nyash.map.delete_hh` lane
     - phase2120 pure canaries are now split by suites: `phase2120-pure-keep` for the 2 active keep pins and `phase2120-pure-historical` for archive-backed replay evidence
     - phase2044 semantics are split by bucket runner; only the llvmlite trio is `monitor-only keep`, owned by a dedicated suite manifest, and the other groups stay bucket-runner only
     - phase2111 and phase251 archive proofs share one replay-evidence suite
     - selfhost compat stack wording is fixed as `payload -> transport wrapper -> pack orchestrator`
     - axis and lane detail is canonical in the SSOTs and backend-lane docs
   - phase-29x backend owner cutover prep table:

     | Item | State |
     | --- | --- |
     | Now | `phase-29x backend owner cutover prep` |
     | Blocker | `none` |
     | Next | `29x-98` stop-line surfaces (`extern_provider` + compat selfhost wrapper) -> proof/example caller sequencing |
   - cleanup bands:

     | Band | State |
     | --- | --- |
     | Now | `lang/src/vm/hakorune-vm/extern_provider.hako` + compat selfhost wrapper stack |
     | Next | proof-only direct `hostbridge.extern_invoke(..., "emit_object", ...)` callers |
     | Later | `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)` / `CodegenBridgeBox.emit_object_args(...)` / Rust dispatch residues |
2. `phase-29bq`
   - active selfhost lane
   - `mirbuilder first / parser later`
   - current blocker: `none`
   - failure-driven steady-state
3. `phase-29x`
   - active structure-first owner-cutover lane
   - keep exact detail in phase README and backend-owner SSOT
4. `phase-29ck`
   - active follow-up / docs-first exact front
5. `phase-29ci`
   - close-synced
6. `phase-29cu`
   - close-synced
7. `phase-29cj`
   - close-synced
8. `phase-29y`
   - parked / monitor-only
9. `phase-29ct`
   - stop-line reached

## Next Horizon Inventory

- Active big tasks:
  - `stage / docs / naming` fixation
  - `zero-rust` default operationalization (landed)
  - `stage2plus` entry / first optimization wave (accepted)
  - `phase-29x` backend owner cutover prep
- Active LLVM tasks:
  - `llvmlite` keep/probe parity
  - `ny-llvm` collection / allocator-handle / dynamic-fallback buckets
- Parked big tasks:
  - broad widening beyond the current `K2-wide` narrow slices
  - broad `Map` structural expansion
- Active small tasks:
  - Map evidence bundle maintenance
  - current `zero-rust` entrypoint sync
- Parked small tasks:
  - warning debt sweep
  - TODO cleanup / ignore triage
  - hotspot cleanup outside the active pilot boundary

## Exact Next

1. keep `phase-29x` exact and docs-first
2. keep `phase-29bq` active as failure-driven / blocker-none lane
3. keep `phase-29ck` exact and docs-first
4. keep closed lanes closed unless a new exact gap appears

## Active Lane

- `policy-refresh` is active
- current lane rules:
  - keep `stage0/stage1/stage2-mainline/stage2+` as build/distribution vocabulary
  - keep `K0/K1/K2` as build/runtime stage vocabulary
  - keep task packs separate from `K-axis`
  - keep `RuntimeDataBox` facade-only
  - keep `.hako` capability facades distinct from native keep leaf glue
  - keep artifact roots binary/bundle only; migration task notes stay in root/docs/phase owners
  - keep backend lane vocabulary in the backend-lane docs

## Parked / Stop-Line

- `phase-29y`
  - reopen only on exact runtime gate / bootstrap-proof failure
- `phase-29ct`
  - docs/task lane only until explicit reopen
- `phase-21_5` perf
  - parked reopen
- `phase-29cs`
  - parked naming cleanup

## Recently Landed

- `K2-core` accepted stop-line is closed
  - `K2-wide` landed rows now include:
    - `RawMap` first slice
    - `RawMap.clear`
    - `RawMap.remove/delete`
    - `hako.atomic.fence_i64`
    - `hako.tls.last_error_text_h`
    - `hako.gc.write_barrier_i64`
    - `hako.osvm.reserve_bytes_i64` / `commit_bytes_i64` / `decommit_bytes_i64`
    - `hako_alloc` handle reuse and GC trigger policy rows
- current follow-up is boundary-shrink lock-down

## Read Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
5. active phase README
