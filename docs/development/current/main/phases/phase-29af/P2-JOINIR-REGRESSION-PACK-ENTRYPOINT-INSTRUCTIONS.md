# Phase 29af P2: JoinIR Regression Pack Entrypoint — Instructions

Status: Ready for execution  
Scope: 回帰パックの実行導線を 1 コマンドに収束（仕様不変）

## Goal

JoinIR の回帰確認を “1本のスクリプト” に固定し、semantic route family の再実行を迷わず回せる状態にする。

## Non-goals

- 挙動変更（release 既定挙動の変更）
- env var の追加
- fixture/smoke の増加（新規ケースは不要）

## Implementation Steps

1) 回帰パックの entrypoint script を追加
   - `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
   - 内容: `loop_break*` / `scan_with_init*` / `split_scan*` / `nested_loop_minimal*` の semantic wrapper を順に呼ぶだけ
   - historical replay stem の exact basename は retirement SSOT / inventory lane にだけ残す

2) docs を一本化
   - `docs/development/current/main/phases/phase-29ae/README.md`
     - Commands を上記 script 1 本に収束
   - `docs/development/current/main/10-Now.md`
     - “JoinIR 回帰確認はこの 1 本” を追記

## Verification

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile quick`

## Acceptance Criteria

- 回帰パックが 1 コマンドで PASS
- quick 154/154 PASS（不変）
