# Phase 29ah: JoinIR regression pack expansion (real-world coverage)

Goal: JoinIR の回帰パックを “最小 + 現実ログ系” に拡張し、見落としを減らす（仕様不変）。

## Status

- P0: ✅ COMPLETE（phase263 Pattern2 を回帰パックへ追加）

## P0: Add Pattern2 real-world case to regression pack

- 指示書: `docs/development/current/main/phases/phase-29ah/P0-REGRESSION-PACK-INCLUDES-PHASE263-PATTERN2-INSTRUCTIONS.md`
- 回帰パック入口: `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- 追加フィルタ: `phase263_pattern2_*`（real-world Pattern2）

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
