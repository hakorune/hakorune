---
Status: Active
Date: 2026-04-02
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
     - semantic proof/archive recut is landed: `phase2044`, `phase2120`, and archive replay evidence now live in semantic homes
     - the selfhost compat stack wording is fixed as `payload -> transport wrapper -> pack orchestrator`
     - the generic `llvm_codegen::emit_object_from_mir_json(...)` export is gone; the remaining direct helper caller is the archive-later surrogate under `module_string_dispatch/compat/`
     - `compat_codegen_receiver.rs` no longer calls the helper directly; it now keeps the text contract on top of the shared no-helper primitive
     - `compat/llvm_backend_surrogate.rs` also no longer calls the helper directly; it now reads the MIR(JSON) file and forwards text into the same primitive
     - `29x-98` owns the final helper-deletion watch, now split into the keep chokepoint watch and the archive-later surrogate watch; `29x-99` owns landed re-cut history and move order
     - the adopted watch strategy is one Rust-side no-helper `MIR(JSON text) -> object path` primitive first; the surrogate follows later as `json_path -> read_to_string -> same primitive`
     - owner-facade slimming is landed: `compile_obj(json_path)` now reads as an explicit compatibility path-entry shim over the root-first compile core
     - `99W1` is landed: caller groups and reduction order are fixed
     - `99W2` is landed: the single Rust-side no-helper text primitive is explicit and the keep chokepoint uses it
     - `99X1` and `99X2` are landed
     - current active micro task is `99Y final explicit helper deletion decision`
     - next queued micro task is `next optimization restart`
     - watch-1 caller reduction order is `loader-cold extern -> hostbridge dispatch -> plugin-loader env.codegen`
     - review intake detail stays in `29x-99`; the live watch stays in `29x-98`
     - axis and lane detail is canonical in the SSOTs and backend-lane docs
   - phase-29x backend owner cutover prep table:

     | Item | State |
     | --- | --- |
     | Now | `phase-29x backend owner cutover prep` |
     | Blocker | `none` |
     | Next | `99Y -> next optimization restart` |
   - cleanup bands:

     | Band | State |
     | --- | --- |
     | Now | `99Y final explicit helper deletion decision` |
     | Next | `next optimization restart` |
     | Later | `none` |
   - cleanup waves:

     | Wave | Status | Read as |
     | --- | --- | --- |
     | `W1 docs-first path-truth pass` | landed | target buckets and move order |
     | `W2 mixed-file split pass` | landed | split owner-looking mixed files |
     | `W3 smoke/proof filesystem recut` | landed | semantic homes replace phase-number homes |
     | `W4 Hako-side caller drain prep` | landed | exact replacement proof is green; direct Hako caller demotion is complete |
     | `W5 Rust compat receiver collapse` | landed | one compat receiver chokepoint |
     | `W6 final delete/archive sweep` | landed | misleading legacy front-door naming/export is gone; remaining compat helper stays explicit under `29x-98` |
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
