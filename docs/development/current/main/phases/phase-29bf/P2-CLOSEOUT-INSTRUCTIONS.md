---
Status: Ready
Scope: docs-only
---

# Phase 29bf P2: Closeout（docs-only）

## Goal

Phase 29bf を closeout 形式に整え、Now/Backlog/roadmap を selfhost 優先へ戻す。

## Steps

1. Phase README
   - `docs/development/current/main/phases/phase-29bf/README.md`
     - `Status: Complete`
     - P0/P1/P2 を ✅ に
     - Summary を 5 行以内で固定
     - Next（TBD または selfhost）を 1 行で明記

2. Now/Backlog/roadmap
   - `docs/development/current/main/10-Now.md`: Phase を selfhost 側へ（TBDでも可）
   - `docs/development/current/main/30-Backlog.md`: Phase 29bf を Complete へ
   - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`: Current/Next を selfhost へ

## Acceptance

- docs-only（テスト不要）
- `git status -sb` が clean

## Commit

- `git add -A`
- `git commit -m "docs(phase29bf): closeout; handoff to selfhost"`

