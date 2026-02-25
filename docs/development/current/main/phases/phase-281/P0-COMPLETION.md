# Phase 281 P0 Completion (2025-12-23)

Target:
- Pattern7（SplitScan）の hand-rolled Frag を、body 分岐（cond_match）について `compose::if_()` に置換する。

## Change Summary

- Modified: `src/mir/builder/control_flow/plan/normalizer.rs`
  - body_bb の `cond_match` 分岐（then/else→step join）を `compose::if_()` に置換
  - header_bb の `cond_loop` 分岐、および step_bb の back-edge（step→header）は手組みのまま維持
  - `EdgeArgs` は `empty_args` を明示的に維持（implicit 省略をしない）

## Verification

- VM smoke: `tools/smokes/v2/profiles/integration/apps/phase256_p0_split_vm.sh` PASS（exit=3）
- LLVM smoke: `tools/smokes/v2/profiles/integration/apps/phase256_p0_split_llvm_exe.sh` PASS（exit=3）

## Notes

- Phase 280 の “行動は最小” 方針に従い、差分は Pattern7 の body 分岐に限定した。
- Pattern6（early exit）は Phase 281 P1 以降で段階移行する。

