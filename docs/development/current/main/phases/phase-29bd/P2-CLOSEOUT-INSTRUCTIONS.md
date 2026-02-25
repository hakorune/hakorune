---
Status: Ready
Scope: docs-only
---

# Phase 29bd P2: Closeout（docs-only）

## Goal

Phase 29bd（CorePlan purity Stage-2 / fallback→0）の作業を closeout 形式に整え、次フェーズへ安全に受け渡す。

## Preconditions

- P1 が完了し、`docs/development/current/main/phases/phase-29bd/README.md` の `Inventory (P0)` が実装と一致している
- Gate が green:
  - `./tools/smokes/v2/run.sh --profile quick`
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Steps

1. Phase README を Complete へ
   - `docs/development/current/main/phases/phase-29bd/README.md`
     - `Status: Complete`
     - P0/P1/P2 の成果を短く Summary に固定
     - 次フェーズ候補（例: Stage-2 継続 / 追加 inventory）を 1 行で明示

2. Now/Backlog/CURRENT_TASK の切替
   - `docs/development/current/main/10-Now.md`: Current/Next を「次フェーズ選定（TBD）」へ
   - `docs/development/current/main/30-Backlog.md`: Phase 29bd を COMPLETE 側へ整流
   - `CURRENT_TASK.md`: Stage-2 で残っている “fallback 0 収束” の残件を短く列挙
   - `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`: Current/Next を同期

3. Gate SSOT の整合
   - `docs/development/current/main/phases/phase-29ae/README.md` の gate 記述と矛盾しないことを確認

## Acceptance

- `git status -sb` が clean
- `./tools/smokes/v2/run.sh --profile quick` が PASS
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が PASS

## Commit

- `git add -A`
- `git commit -m "docs(phase29bd): closeout purity stage2 inventory"`
