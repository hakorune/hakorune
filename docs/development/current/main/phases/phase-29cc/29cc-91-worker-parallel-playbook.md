---
Status: Active
Scope: Phase 29cc worker parallel execution playbook
Related:
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-90-migration-execution-checklist.md
---

# 29cc-91 Worker Parallel Playbook

## Purpose

同一ファイル衝突を避けつつ、M1/M2 を並走で進めるための固定台本。

## Worker roles (fixed)

1. Explorer-Inventory
- ownership: 調査のみ（編集禁止）
- output: `Rust-only residue` / `gate impact` / `next smallest shape`

2. Worker-Parser
- ownership:
  - `lang/src/compiler/parser/**`
  - parser向け fixture/smoke
- rule:
  - 1 shape per commit
  - Rust parser 側の意味変更は禁止

3. Worker-MirBuilder
- ownership:
  - `lang/src/compiler/mirbuilder/**`
  - mirbuilder向け fixture/smoke
- rule:
  - BoxCount/BoxShape 混在禁止
  - parser変更を混ぜない

4. Parent-Integrator
- ownership:
  - SSOT/doc sync
  - gate execution
  - merge conflict resolution

## Collision guard

- `lang/src/compiler/parser/**` と `lang/src/compiler/mirbuilder/**` は同時編集しない。
- `CURRENT_TASK.md` / `10-Now.md` は Parent のみ編集。
- fast gate FAIL 中は `cases.tsv` 更新禁止。

## Ready-to-run order

1. Explorer-Inventory を走らせる
2. Worker-Parser が最小 shape を1件対応
3. Parent が parser gate を実行
4. Worker-MirBuilder が1件対応
5. Parent が bq gate を実行
6. 緑なら PROMOTE、赤なら failure-driven で1件だけ修正
