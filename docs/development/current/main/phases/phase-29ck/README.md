---
Status: Active
Decision: accepted
Date: 2026-04-01
Scope: backend-zero boundary cutover prep の active front page。monitor/evidence lane と fixed task-pack order を docs-ready に保つ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
  - docs/development/current/main/design/runtime-decl-manifest-v0.toml
  - docs/development/current/main/phases/phase-29ck/P14-PURE-FIRST-NO-REPLAY-CUTOVER.md
  - docs/development/current/main/phases/phase-29ck/P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md
  - docs/development/current/main/phases/phase-29ck/P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md
  - docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md
  - docs/development/current/main/phases/phase-29x/29x-63-llvm-cabi-link-gate-ssot.md
---

# Phase 29ck: Backend-Zero Boundary Cutover Preparation

## Goal

- backend-zero を queued phase として読めるように保つ。
- final target を `.hako -> thin backend C ABI/plugin boundary -> object/exe` に固定する。
- current bootstrap seam と final cutover target を混線させない。
- inventory -> task pack -> acceptance/reopen rule を phase 内に閉じる。

## Current Override

- current phase role is `monitor/evidence lane`
- `phase-29x` is the active structure-first parent front
- keep MIR / route / perf evidence readable, but do not reopen leaf-only retune before compare lane stays explicit
- current structure-first parent order lives in:
  - `backend-owner-cutover-ssot.md`
  - `runtime-decl-manifest-v0.toml`
- archive-home is sufficient for retired compare/lock assets
- carry-over work from this phase is limited to:
  - attrs SSOT
  - copy transparency
  - bool-i1 / facts visibility
  - compare ledger / verifier evidence

## Entry Conditions

1. immediate blocker remains compiler authority removal for pure `.hako` hakorune build
2. canonical ABI surface stays two-sided:
   - Core C ABI
   - TypeBox ABI v2
3. `Cranelift` stays explicit keep
4. runtime-zero daily policy (`LLVM-first / vm-hako monitor-only`) does not change here

## Fixed Order

1. `P0-BACKEND-ZERO-OWNER-INVENTORY.md`
2. `P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md`
3. `P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md`
4. `P3-THIN-BACKEND-CUTOVER-LOCK.md`
5. `P4-RUNTIME-PROOF-OWNER-BLOCKER-INVENTORY.md`
6. `P5-COMPAT-PURE-PACK-LOCK.md`
7. `P6-MACOS-PORTABILITY-FFI-CANDIDATE-LOCK.md`
8. `P7-PRE-PERF-RUNWAY-TASK-PACK.md`
9. `P8-PERF-REOPEN-JUDGMENT.md`
10. `P9-METHOD-CALL-ONLY-PERF-ENTRY-INVENTORY.md`
11. `P14-PURE-FIRST-NO-REPLAY-CUTOVER.md`
12. `P15-STAGE1-MIR-DIALECT-INVENTORY.md`
13. `P16-STAGE1-CANONICAL-MIR-CUTOVER.md`
14. `P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md`
15. `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`

## Current Snapshot

- caller-facing daily LLVM route is already boundary-first
- `ny-llvmc` default driver is already on `Boundary`
- unsupported shapes still replay through the explicit boundary compat lane
- compare bridge remains explicit; it is not the daily hidden path
- current optimization reading is structure-first:
  - accepted proof rows are kept
  - rejected perf cuts stay historical evidence only
  - exact next cut remains leaf-quality evidence, not broad route expansion
- leaf-only perf wins remain parked until `phase-29x` reaches narrow owner flip

## Exact Current Front

- keep current evidence focused on:
  - pure-first / no-replay cutover
  - Stage1 canonical MIR dialect
  - AOT-core proof vocabulary
  - live-route debug bundle
- do not reopen broad value-repr, broad optimizer migration, or generic backend rewrites from this phase
- do not promote this lane back above the `phase-29x` structure-first parent front

## Immediate Next

1. keep `phase-29x` as the parent active front
2. keep `29ck` as monitor/evidence only
3. use `P14` / `P17` / `P18` as the current active detail owners
4. reopen only when a new exact route/perf blocker appears or the parent front explicitly pulls this lane forward

## Acceptance / Reopen Summary

- this phase stays healthy while:
  - parent owner-cutover docs stay synced
  - explicit compare bridge remains archive-only
  - current proof bundles remain readable
  - no hidden replay/default owner drift appears
- reopen only on:
  - new exact route blocker
  - new exact boundary replay drift
  - explicit parent-front request from `phase-29x`

## Detail Owners

- exact pure-first / no-replay detail:
  - `P14-PURE-FIRST-NO-REPLAY-CUTOVER.md`
- exact proof vocabulary:
  - `P17-AOT-CORE-PROOF-VOCABULARY-LOCK.md`
- exact live route/debug bundle:
  - `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
- rejected optimization history:
  - `phase29ck-array-substrate-rejected-optimizations-2026-03-27.md`
