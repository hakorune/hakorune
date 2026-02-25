---
Status: Ready
Scope: docs-only
---

# Phase 29bf P1: Selfhost handoff doc（docs-only）

## Goal

CorePlan 移行が gate を満たしている状態で、selfhost 開発へ戻るための「入口ドキュメント」を 1 枚で固定する。

この P1 は実装を進めない（docs-only）。以後の作業で迷子にならないことが目的。

## Preconditions

- Phase 29bf P0 が ✅（Done criteria verification 済み）
- Gate green:
  - `cargo build --release`
  - `./tools/smokes/v2/run.sh --profile quick`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Deliverables

1. Handoff doc を追加
   - `docs/development/current/main/phases/phase-29bf/SELFHOST_HANDOFF.md`
     - 「いま何が Done か（SSOT参照）」と「次にやること（selfhost）」を短く固定
     - Gate コマンドを SSOT として明記
     - “CorePlan 周りを触る条件” を明文化（例: gate が赤になった時だけ）

2. Phase README を更新
   - `docs/development/current/main/phases/phase-29bf/README.md`
     - P1 を ✅ にする
     - Instructions に P2 closeout 指示書へのリンクを追加

3. Now/Backlog/roadmap の Next 更新
   - `docs/development/current/main/10-Now.md`: Next を `Phase 29bf P2 (closeout)` に
   - `docs/development/current/main/30-Backlog.md`: Planned を P2 closeout に寄せる
   - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`: Next を P2 closeout に同期

## Acceptance

- docs-only（テスト不要）
- `git status -sb` が clean

## Commit

- `git add -A`
- `git commit -m "docs(phase29bf): add selfhost handoff doc"`

