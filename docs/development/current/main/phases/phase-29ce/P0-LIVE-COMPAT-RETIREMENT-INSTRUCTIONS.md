---
Status: Active
Phase: 29ce
Lane: live compat retirement
---

# P0: live compat retirement instructions

## Goal

`SMOKES_SELFHOST_FILTER` / by-name fixture key / semantic fixture alias の live contract を
current lane と inventory lane に分離し、retire authority を固定する。

## Rules

- active docs / active gates は semantic route substring または semantic fixture alias を先頭に置く
- exact historical basename は inventory pointer に寄せる
- by-name fixture key は caller inventory なしで削除しない
- archive replay lane の判断は `phase-29cd` に持ち込まない

## Fixed order

1. `SMOKES_SELFHOST_FILTER` current examples を semantic-first に保つ
2. by-name fixture key の live set / retired set / inventory-only set を確認する
3. semantic fixture alias を current gate/selfhost subset の正本として維持する
4. `CURRENT_TASK.md` に slice と verification を同期する

## Acceptance

- `git diff --check` PASS
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
