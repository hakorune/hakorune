# Phase29x Direct Route Probe (2026-03-02)

Status: active monitor  
Scope: `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe`

## Purpose

- `--emit-mir-json` direct route の残件を `phase29x-probe` で継続観測する。
- `CURRENT_TASK.md` は latest 数値と先頭 blocker だけを保持し、詳細ログは本ノートを SSOT とする。

## Command

```bash
bash tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe
```

## Snapshot Timeline

1. baseline
   - `emit_fail=61`, `run_nonzero=9`, `run_ok=48`
   - class: `emit:direct-verify=6`, `emit:joinir-reject=41`, `emit:other=13`, `emit:freeze-contract=1`
2. after direct-verify dominance/Phi fix
   - `emit_fail=55`, `run_nonzero=10`, `run_ok=53`
   - class: `emit:direct-verify=0`, `emit:joinir-reject=41`, `emit:other=13`, `emit:freeze-contract=1`
3. after release route gate update (loop_cond)
   - `emit_fail=46`, `run_nonzero=9`, `run_ok=63`
   - class: `emit:joinir-reject=32`, `emit:other=13`, `emit:freeze-contract=1`
4. after allow_extended gate update (loop_cond return)
   - `emit_fail=43`, `run_nonzero=7`, `run_ok=68`
   - class: `emit:joinir-reject=25`, `emit:other=17`, `emit:freeze-contract=1`
5. after release route gate update (loop_true)
   - `emit_fail=39`, `run_nonzero=7`, `run_ok=72`
   - class: `emit:joinir-reject=22`, `emit:other=16`, `emit:freeze-contract=1`
6. after throw fixture split (`throw_reserved` -> parser-only negatives)
   - `emit_fail=27`, `run_nonzero=7`, `run_ok=84`
   - class: `emit:direct-verify=0`, `emit:joinir-reject=0`, `emit:other=27`, `emit:freeze-contract=0`
   - note: runtime lane fixtures `phase29bq_selfhost_try_throw_catch_cleanup_min` / `phase29bq_selfhost_try_loop_throw_catch_min` は throw-free cleanup route へ更新。
7. refresh snapshot（2026-03-03）
   - `emit_fail=20`, `run_nonzero=7`, `run_ok=91`, `route_blocker=0`
   - class: `emit:direct-verify=2`, `emit:freeze-contract=1`, `emit:other=17`
   - head detail:
     - `[normalizer] generic loop v0: nested loop has no plan` = 12
     - single-exit family（`last=Assignment` / `then_last=Return` / `loop_v0 then_last=Assignment`）= 4
     - `emit:direct-verify` residual = 2（`scan_methods_nested_loop_idx19/28`）
     - singletons: `unsupported stmt Call` = 1, `Expected BinOp` = 1
8. after nested loop recipe/exit-if fixes（2026-03-03）
   - `emit_fail=13`, `run_nonzero=9`, `run_ok=96`, `route_blocker=0`
   - class: `emit:direct-verify=2`, `emit:other=11`, `run:vm-error=3`
   - head detail:
     - `Unsupported value AST: MapLiteral` = 7
     - single-exit family（`then_last=Return` / `then_last=Assignment else_last=If`）= 2
     - `emit:direct-verify` residual = 2（`scan_methods_nested_loop_idx19/28`）
     - singletons: `unsupported stmt Call` = 1, `Expected BinOp` = 1
9. after map/or + cond-prelude branch cleanup（2026-03-03）
   - `emit_fail=13`, `run_nonzero=9`, `run_ok=96`, `route_blocker=0`
   - class: `emit:direct-verify=9`, `emit:other=4`, `run:vm-error=3`
   - head detail:
     - `emit:direct-verify` = 9（`scan_methods_nested_loop_idx19/28` + box_member 7）
     - single-exit family（`then_last=Return` / `then_last=Assignment else_last=If`）= 2
     - singletons: `unsupported stmt Call` = 1, `Expected BinOp` = 1
   - note:
     - `Unsupported value AST: MapLiteral` / `Unsupported binary operator: Or` / `if_effect_empty` は解消。
     - box_member cluster は dominance/merge (`Undefined value %290 ... bb55`) へ前進。

## Resolved: emit:direct-verify (6 fixtures)

- `apps/tests/phase29bq_generic_loop_v1_local_def_continue_min.hako`
- `apps/tests/phase29bq_loop_cond_continue_only_no_else_min.hako`
- `apps/tests/phase29bq_loop_cond_continue_with_return_min.hako`
- `apps/tests/phase29bq_selfhost_blocker_parse_program2_loop_continue_if_min.hako`
- `apps/tests/phase29bq_strict_nested_loop_guard_accept_min.hako`
- `apps/tests/phase29bq_strict_nested_loop_guard_min.hako`

## Root-Cause Memo (direct-verify)

- fixture: `apps/tests/phase29bq_loop_cond_continue_only_no_else_min.hako`
- symptom: `%45/%46` use in step block without dominating def
- cause: `continue` edge (`bb7 -> bb5`) bypassed merge (`bb12`)
- result: dominance/Phi violation
- fix direction: continue edge を merge 側へ寄せる / step merge へ incoming 付与

## Current Head Blocker

- class: `[freeze:contract][emit-mir/direct-verify]`（dominance/undefined value）
- count: 9（2026-03-03 latest snapshot）
- representative fixtures:
  - `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_cleanup_min.hako`
  - `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_cleanup_min.hako`
  - `apps/tests/phase29bq_selfhost_blocker_scan_methods_nested_loop_idx19_min.hako`
  - `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_guard_sync_tail_mirror_sync_tail_cleanup_min.hako`

## Guard Canary

```bash
bash tools/checks/phase29ca_direct_verify_dominance_block_canary.sh
```

Expected: `emit_rc=0` and `run_rc=4`
