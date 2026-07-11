# Phase 29ag P0: Coordinator uses BoundaryCarrierLayout — Instructions

Status: Ready for execution  
Scope: merge/coordinator の carrier 順序参照を BoundaryCarrierLayout に統一（仕様不変）

## Goal

`src/mir/builder/control_flow/joinir/merge/coordinator.rs` の “carrier順序依存 remap” を
BoundaryCarrierLayout SSOT参照へ統一し、順序の二重管理を解消する。

## Non-goals

- 挙動変更（release 既定挙動の変更）
- env var の追加
- fixture/smoke の増加

## Implementation Steps

1) coordinator の carrier order 参照を置換
   - `loop_step` param の index → carrier 名の対応を BoundaryCarrierLayout から取得
   - fallback の loop_var + carriers の順序も BoundaryCarrierLayout から取得

2) 既存 SSOT への依存は維持
   - header PHI の対応は LoopHeaderPhiInfo の carrier_phis を参照する

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
