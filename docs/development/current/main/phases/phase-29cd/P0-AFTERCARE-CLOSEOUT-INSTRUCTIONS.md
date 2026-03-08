---
Status: Active
Phase: 29cd
Lane: aftercare closeout
---

# P0: aftercare closeout instructions

## Goal

archive replay / live compat / dust の残件を、current lane を汚さずに閉じる。

## Rules

- semantic-first を維持する
- exact legacy token は inventory pointer に寄せる
- archive replay lane は caller inventory と keep authority が揃うまで hard-delete しない
- `docs/private` nested repo は別管理

## Fixed order

1. archive replay lane の `fixed keep / retire when` を更新
2. `SMOKES_SELFHOST_FILTER` / by-name fixture key / semantic fixture alias の live contract を点検
3. low-risk dust を削る
4. `CURRENT_TASK.md` に slice と verification を同期

## Acceptance

- `git diff --check` PASS
- `cargo check --tests` PASS
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
