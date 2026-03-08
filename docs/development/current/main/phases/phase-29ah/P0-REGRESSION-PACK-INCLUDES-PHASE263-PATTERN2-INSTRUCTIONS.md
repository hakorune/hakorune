# Phase 29ah P0: Regression pack includes phase263 LoopBreak realworld — Instructions

Status: Ready for execution  
Scope: JoinIR 回帰パックに real-world Pattern2 を追加（仕様不変）

## Goal

JoinIR 回帰パック（`phase29ae_regression_pack_vm.sh`）に、実ログ由来の LoopBreak realworld ケース（Phase 263）を含めて、
最小パックだけでは拾えない regressions を早期に検知できるようにする。

## Non-goals

- 挙動変更（release 既定挙動の変更）
- env var の追加
- fixture 追加（既存の Phase 263 fixture/smoke を再利用）

## Background (SSOT)

- 現行の回帰パック入口: `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- current semantic wrapper: `tools/smokes/v2/profiles/integration/joinir/loop_break_realworld_vm.sh`
- historical replay basename: `phase263_pattern2_seg_realworld_min_vm.sh`

## Implementation Steps

1) 回帰パックに `loop_break_realworld_vm` を追加
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - `run_filter "loop_break_realworld_vm" "loop_break_realworld_vm"` を `loop_break` route family の直後に追加

2) docs の SSOT を更新
- `docs/development/current/main/phases/phase-29ae/README.md`
  - Regression pack (SSOT) に `phase263_pattern2_*` を追記
- `docs/development/current/main/10-Now.md`
  - JoinIR 回帰確認のSSOTが “この1本” であることは維持（記述の更新のみ）

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- 回帰パックが PASS のまま
- Phase 263 の LoopBreak realworld も回帰パックに含まれる
