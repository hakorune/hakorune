---
Status: Active
Decision: accepted
Date: 2026-03-01
Scope: RZ-LOADER-min4 closeout として runtime route residue relock を route-zero stability へ同期する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-245-runtime-route-residue-relock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-251-rz-loader-min4-loader-types-route-reuse-lock-ssot.md
  - CURRENT_TASK.md
---

# 29cc-252 RZ-LOADER-min4 Closeout Route-Zero Stability Lock

## Purpose

`29cc-245` 系列の loader 側残件（RZ-LOADER-min4）を close し、
runtime source-zero lane を「route-zero + stability 維持」の監視状態へ戻す。

## Decision

- route residue relock series に `29cc-251` / `29cc-252` を追加する。
- `active next` は monitor-only（failure-driven reopen）へ戻す。
- no-delete-first を維持し、source 削除は deferred gate（mac portability 安定後）へ据え置く。

## Acceptance

- `tools/checks/dev_gate.sh runtime-exec-zero` green
- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh` green
- `CURRENT_TASK.md` と `phase-29cc/README.md` が同じ active-next 状態を指す

