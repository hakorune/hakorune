# Phase 29af P0: Pattern2 Boundary Hygiene (SSOT)

Goal: Pattern2 の boundary 情報の歪みを SSOT で整理し、将来の回帰を防ぐ（仕様不変）。

## Status

- P0–P5: ✅ COMPLETE（closeout）

## Boundary Contract (SSOT)

- Header PHI 対象:
  - `carrier_info` の carriers（LoopState + ConditionOnly + LoopLocalZero）
- Exit reconnection 対象:
  - LoopState のみ（ConditionOnly は exit_bindings に入れない）
- Host binding 対象:
  - `CarrierInit::FromHost` のみ（BoolConst / LoopLocalZero は host slot 不要）

## Fail-Fast Rules

- exit_bindings の `carrier_name` 重複は禁止（debug_assert）
- `CarrierInit::FromHost` の `host_id=0` は Fail-Fast

## Entry Points

- boundary 構築: `src/mir/builder/control_flow/joinir/patterns/pattern2_steps/emit_joinir_step_box.rs`
- header PHI 事前構築: `src/mir/builder/control_flow/joinir/merge/header_phi_prebuild.rs`

## P1: Contract Checks (merge 入口)

P0 で確定した boundary hygiene を、merge 入口の `contract_checks` に集約する（仕様不変）。

- 実装: `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_hygiene.rs`
- 配線: `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_creation.rs`
- 実行条件: `joinir_strict` または `joinir_dev` のみ Fail-Fast

## P2: JoinIR Regression Pack Entrypoint

JoinIR 回帰確認の導線を 1 コマンドに収束する（仕様不変）。

- Script: `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- SSOT: `docs/development/current/main/phases/phase-29ae/README.md`（Commands）
- 指示書: `docs/development/current/main/phases/phase-29af/P2-JOINIR-REGRESSION-PACK-ENTRYPOINT-INSTRUCTIONS.md`

## P3: BoundaryCarrierLayout SSOT

carrier の順序（loop_var + carriers）を merge 側の SSOT に統合する（仕様不変）。

- SSOT: `src/mir/builder/control_flow/joinir/merge/boundary_carrier_layout.rs`
- 適用: tail_call_policy / latch_incoming_recorder の order 統一
- contract_checks: `phase29af/boundary_hygiene/layout_order`（strict/dev のみ）

## P4: Layout Consistency Contract

BoundaryCarrierLayout と header PHI の順序一致を strict/dev で検証する（仕様不変）。

- contract_checks: `src/mir/builder/control_flow/joinir/merge/contract_checks/header_phi_layout.rs`
- 配線: `src/mir/builder/control_flow/joinir/merge/coordinator.rs`

## P5: Closeout

P0–P4 の SSOT/contract を確定し、入口（README/Now/Backlog）を締めた。

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Notes

- Merge 側の Header PHI Entry/Latch contract は Phase 29ae で SSOT 化済み: `docs/development/current/main/phases/phase-29ae/README.md`
