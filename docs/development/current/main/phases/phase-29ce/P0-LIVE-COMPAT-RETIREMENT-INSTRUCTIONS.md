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
   - `current runtime keep` / `retired Program JSON compat key` / `historical docs-private caller only` /
     `dev-gated compat key` の bucket を SSOT に固定する
3. semantic fixture alias を current gate/selfhost subset の正本として維持する
4. `CURRENT_TASK.md` に slice と verification を同期する

## Current hotspots to inspect first

- `phase29bq_selfhost_planner_required_dev_gate_vm.sh`
  - filter 対象は `fixture + planner_tag + reason`
- `planner_required_selfhost_subset.tsv`
  - `reason` 列の exact token が live compat token になりうる
- `phase29ae_regression_pack_vm.sh`
  - phase-prefixed filter family の live caller を持つ
- `src/mir/join_ir/frontend/ast_lowerer/route.rs`
  - by-name key の live contract authority
  - key bucket authority は `joinir-frontend-legacy-fixture-key-retirement-ssot.md`

## Acceptance

- `git diff --check` PASS
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` PASS
- `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail` PASS
