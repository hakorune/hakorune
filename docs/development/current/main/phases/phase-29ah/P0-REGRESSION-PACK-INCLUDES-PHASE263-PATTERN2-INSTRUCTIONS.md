# Phase 29ah P0: Regression pack includes phase263 Pattern2 — Instructions

Status: Ready for execution  
Scope: JoinIR 回帰パックに real-world Pattern2 を追加（仕様不変）

## Goal

JoinIR 回帰パック（`phase29ae_regression_pack_vm.sh`）に、実ログ由来の Pattern2 ケース（Phase 263）を含めて、
最小パックだけでは拾えない regressions を早期に検知できるようにする。

## Non-goals

- 挙動変更（release 既定挙動の変更）
- env var の追加
- fixture 追加（既存の Phase 263 fixture/smoke を再利用）

## Background (SSOT)

- 現行の回帰パック入口: `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- Phase 263 の実ログ系 Pattern2 smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase263_pattern2_seg_realworld_min_vm.sh`

## Implementation Steps

1) 回帰パックに `phase263_pattern2_` を追加
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - `run_filter "pattern2_realworld" "phase263_pattern2_"` を `pattern2` の直後に追加

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
- Phase 263 の Pattern2（real-world）も回帰パックに含まれる
