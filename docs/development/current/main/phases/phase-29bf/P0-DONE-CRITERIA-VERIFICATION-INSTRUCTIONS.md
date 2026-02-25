---
Status: Ready
Scope: docs-only (verification commands)
---

# Phase 29bf P0: Done criteria verification（docs-first）

## Goal

`docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md` の “Done” を、現状のコードに対して
チェックリストとして実行可能にする。

## SSOT

- Done criteria: `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`
- Gate: `docs/development/current/main/phases/phase-29ae/README.md`

## Verification checklist

### 1) Gate green

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

### 2) Routing invariants (spot check)

- JoinIR legacy loop table が存在しないこと（router が plan/composer のみ）
  - `rg -n \"legacy loop\" src/mir/builder/control_flow/joinir/patterns/router.rs`
  - `rg -n \"legacy table\" src/mir/builder/control_flow/joinir/patterns/router.rs`

### 3) Release adopt presence (documented)

- Done criteria に記載の “release adopt” が gate smokes でカバーされていること
  - `docs/development/current/main/phases/phase-29ae/README.md` を確認（該当 smoke が listed）

## Deliverables

- `docs/development/current/main/phases/phase-29bf/README.md` に P0 ✅ を付ける（結果メモを 3 行以内で追記）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` /
  `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md` を Phase 29bf に切替

