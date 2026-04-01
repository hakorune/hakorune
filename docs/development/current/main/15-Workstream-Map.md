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
     - `K2-wide next structural follow-up`
     - `zero-rust default`
   - current read:
     - `K-axis` is `K0 / K1 / K2`
     - `K2-core` / `K2-wide` are task packs inside `K2`
     - `K2-core` is closed
     - `K2-wide` is now the next structural lane
     - current `K2-wide` focus is metal keep review / boundary-shrink planning
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
  - `K1 done-enough` stop-line fixation
  - `K2-wide` follow-up
  - `zero-rust` default operationalization
  - artifact contract sync for `K0/K1` binaries vs `K2` bundle reading
- Keep big tasks:
  - `Rune` primitive control plane keep
- Parked big tasks:
  - broad widening beyond the current `K2-wide` narrow slices
  - broad `Map` structural expansion
- Active small tasks:
  - docs ladder sync
  - Map evidence bundle maintenance
  - current metal keep inventory / boundary-shrink planning
- Parked small tasks:
  - warning debt sweep
  - TODO cleanup / ignore triage
  - hotspot cleanup outside the active pilot boundary

## Exact Next

1. keep `policy-refresh` first until stage / naming / order wording stays synced
2. keep `K2-wide` on narrow slices only; do not reopen broad widening
3. keep `phase-29bq` active as failure-driven / blocker-none lane
4. keep `phase-29x` and `phase-29ck` exact and docs-first
5. keep closed lanes closed unless a new exact gap appears

## Active Lane

- `policy-refresh` is active
- current lane rules:
  - keep `stage0/stage1/stage2-mainline/stage2+` as build/distribution vocabulary
  - keep `K0/K1/K2` as build/runtime stage vocabulary
  - keep task packs separate from `K-axis`
  - keep `RuntimeDataBox` facade-only
  - keep `.hako` capability facades distinct from native keep leaf glue
  - keep artifact roots binary/bundle only; migration task notes stay in root/docs/phase owners

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
  - `hako.atomic`
  - `hako.tls`
  - `hako.gc`
  - `hako.osvm`
  - `hako_alloc` handle reuse policy
  - `hako_alloc` GC trigger threshold policy
- current follow-up is metal keep inventory / boundary-shrink planning

## Read Order

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
5. active phase README
