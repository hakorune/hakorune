# Self Current Task — Overview (main)

> **Historical(2026-11-17、メンテナ決定):** この文書は、古い設計か作業の記録である。**新しい作業を選ぶ根拠にしない。** MirBuilder(source → MIR)の設計は [MirBuilder の設計図](design/mirbuilder-blueprint.md) を読む。食い違う時は、設計図が勝つ。

Status: Historical (was Active)
Date: 2026-03-25
Scope: `main` の current lane 入口だけを短く示す overview。
Related:
- CURRENT_TASK.md
- docs/development/current/main/15-Workstream-Map.md
- docs/development/current/main/10-Now.md

## Purpose

- `main` の current reading を最短で掴むための overview。
- 実装 detail はここに置かず、root anchor / workstream map / phase README へ流す。

## Current Shape

- active implementation lane: `phase-29bq`
- active lane reading:
  - selfhost `.hako` migration
  - `mirbuilder first / parser later`
  - failure-driven with `blocker=none` after `JIR-PORT-08`
- close-synced boundary-retire lane: `phase-29ci`
- close-synced Rune lane: `phase-29cu`
- close-synced bootstrap-retire lane: `phase-29cj`
- parked runtime lane: `phase-29y`
- stop-line reached substrate lane: `phase-29ct`

## Read First

1. `CURRENT_TASK.md`
2. `docs/development/current/main/15-Workstream-Map.md`
3. `docs/development/current/main/10-Now.md`

## JoinIR / Selfhost Entrypoints

- JoinIR / Selfhost index: `docs/development/current/main/01-JoinIR-Selfhost-INDEX.md`
- docs layout SSOT: `docs/development/current/main/DOCS_LAYOUT.md`
- JoinIR architecture SSOT: `docs/development/current/main/joinir-architecture-overview.md`
