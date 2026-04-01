---
Status: Active
Decision: provisional
Date: 2026-04-02
Scope: de-Rust runtime integration lane の active front page。failure-driven selfhost lane と structure-first runtime cutover lane を分離しつつ統合運用する。
Related:
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
  - docs/development/current/main/design/runtime-decl-manifest-v0.toml
  - docs/development/current/main/phases/phase-29x/29x-90-integration-checklist.md
  - docs/development/current/main/phases/phase-29x/29x-91-task-board.md
  - docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-98-legacy-route-retirement-investigation-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-99-structure-recut-wave-plan-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
---

# Phase 29x: De-Rust Runtime Integration (Active Plan)

## Goal

- failure-driven 運用を維持しつつ、runtime de-Rust task packs を前進させる。
- 既存 SSOT（ABI / RC / observability / GC policy）を崩さず、Rust VM 依存を段階縮退する。
- selfhost lane と structure-first runtime cutover lane を混線させない。

## Current Override

- current active front is `backend owner cutover prep`
- canonical seam stays MIR
- do not open `AST -> LLVM` direct lowering in this wave
- docs-first beauty-first cleanup planning is allowed while `29x-98` stop-line stays unchanged
- fixed order is:
  1. `backend-owner-cutover-ssot.md`
  2. `runtime-decl-manifest-v0.toml`
  3. `recipe-facts-v0`
  4. `.hako ll emitter` min v0
  5. explicit compare bridge
  6. boundary-only narrow owner flip
  7. archive/delete sweep
- current reading:
  - `.hako ll emitter` min v0 is already the daily owner for the currently flipped narrow shapes
  - compare bridge smoke is archive-suite only
  - remaining active work is subtraction-first, structure-first, and exact
  - structural perf remains the only allowed carry-over from `phase-29ck`

## Non-goals

- cycle collector / tracing の新規実装
- selfhost unblock のための言語仕様追加
- silent fallback の黙認
- broad cache / GC / VM widening before the current structure-first front is closed

## Baseline And Timebox

- blocker=`none` の通常日は quick/probe を優先する
- failure-driven work is limited to actual FAILs
- 60 分を超える場合は `CURRENT_TASK.md` に詰まりメモを残し、本 lane の planned task へ戻る
- 1 task = 1 accepted shape = fixture + gate = 1 commit

Milestone commands:

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`
3. `bash tools/checks/abi_lane_guard.sh`

## Lanes

- Lane A: selfhost stability
  - failure-driven only
  - no new fixture growth while green
- Lane B: lifecycle core
  - ABI + RC insertion + observability
- Lane C: VM route cutover
  - strict/dev-first staged route demotion
- Lane G: post-29x cache build extension
- Lane H: post-X46 runtime handoff
- Lane I: post-X53 runtime core extension

Current operational truth:

- active exact front is still structure-first backend owner cutover
- Lane G/H/I remain sequenced follow-up lanes, not current implementation fronts

## Canonical Child Docs

- current checklist:
  - `29x-90-integration-checklist.md`
- current task board:
  - `29x-91-task-board.md`
- legacy / demotion ledger:
  - `29x-96-backend-owner-legacy-ledger-ssot.md`
- compare bridge retirement prep:
  - `29x-97-compare-bridge-retirement-prep-ssot.md`
- legacy route retirement investigation:
  - `29x-98-legacy-route-retirement-investigation-ssot.md`
- structure recut wave plan:
  - `29x-99-structure-recut-wave-plan-ssot.md`
- cache extension sequence:
  - `29x-67` through `29x-72`
- runtime handoff sequence:
  - `29x-73` through `29x-89`
- optimization lane sequence:
  - `29x-92` through `29x-95`

## Exact Current Front

- current structure-first parent order lives in:
  - `backend-owner-cutover-ssot.md`
  - `runtime-decl-manifest-v0.toml`
- current landed slice is subtraction-first:
  - daily owner moved to `.hako ll emitter` min v0 for the accepted narrow rows
  - compare bridge is explicit and archive-only
  - legacy `.inc` remains daily owner only for unflipped shapes
- current allowed carry-over work is limited to:
  - attrs centralization
  - facts visibility
  - copy-transparency / bool-i1 cleanliness
  - compare ledger / verifier evidence

## Exact Next

1. keep the backend owner cutover reading fixed
2. keep compare bridge explicit and archive-only
3. keep legacy owner inventory/demotion in `29x-96`
4. keep temp-path / compare-bridge retirement prep in `29x-97`
5. keep `29x-98` as the stop-line owner for delete-readiness
6. use `29x-99` for macro cleanup waves and micro-task sequencing
7. reopen deeper runtime core work only after the current narrow owner-cutover front is closed

## Acceptance Summary

- daily LLVM-only gate remains green
- LLVM C ABI link minimum remains green
- ABI lane guard remains green
- the current daily owner reading stays `.hako ll emitter` first, bridge/archive explicit, legacy owner inventory visible
- this README is front-page only; schedules, lane-by-lane detail, and archive notes stay in child docs

## Entry Points

1. `29x-90-integration-checklist.md`
2. `29x-91-task-board.md`
3. `backend-owner-cutover-ssot.md`
4. `runtime-decl-manifest-v0.toml`
5. `29x-96-backend-owner-legacy-ledger-ssot.md`
6. `29x-97-compare-bridge-retirement-prep-ssot.md`
7. `29x-98-legacy-route-retirement-investigation-ssot.md`
8. `29x-99-structure-recut-wave-plan-ssot.md`

## Detail Owners

- old milestone schedule / historical progress:
  - task-board and numbered `29x-*` child docs
- archive/demotion detail:
  - `29x-96-backend-owner-legacy-ledger-ssot.md`
- compare bridge residue:
  - `29x-97-compare-bridge-retirement-prep-ssot.md`
- future cache/runtime/optimization extension detail:
  - `29x-67` onward
