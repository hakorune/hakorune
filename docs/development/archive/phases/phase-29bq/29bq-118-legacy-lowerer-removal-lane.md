---
Status: Closed
Scope: disabled legacy mirbuilder lowerers を mainline / probes / docs から段階撤去する narrow cleanup lane。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29bq/README.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
---

# 29bq-118 — Legacy Lowerer Removal Lane

## Goal

- disabled 済みの legacy lowerer を mainline owner から完全に外す
- 参照を `mainline / probes / docs / archive` に分解して順番に撤去する
- selfhost fast gate を green のまま保つ

## Target

- `LowerReturnMethodArrayMapBox`
- `LowerReturnLoopStrlenSumBox`

## Order

1. mainline owners
   - registry / fallback / min / basic lower
2. live docs and current pointers
3. live probes / monitor scripts
4. reference tests that only pin legacy payload names
5. final tombstone or file deletion

## Non-goals

- 新しい受理形の追加
- planner facts / route widening
- loop owner seam cleanup との同時実施

## Acceptance

- `phase29bq_fast_gate_vm.sh` が green
- disabled legacy lowerer が mainline owner から参照されない
- remaining references are limited to deliberate archive/history only

## Progress

- mainline owner removal: landed
- live `phase2160` arraymap canaries: retired
- legacy `tools/dev/phase29ci_test_runner_*` exact-proof wrappers: retired
- vm_hako historical name/payload pins: retired
- disabled legacy lowerer files: retired from live tree
- remaining references are limited to deliberate archive/history only
