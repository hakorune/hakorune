# Phase 29af P4: Layout Consistency Contract — Instructions

Status: Ready for execution  
Scope: BoundaryCarrierLayout と LoopHeaderPhiInfo::carrier_order の整合を Fail-Fast で固定（仕様不変）

## Goal

BoundaryCarrierLayout と header PHI の順序がズレたときに、merge 入口で早期に検知する。

## Non-goals

- 挙動変更（release 既定挙動の変更）
- env var の追加
- fixture/smoke の増加

## Implementation Steps

1) contract_checks にレイアウト整合性チェックを追加
   - `src/mir/builder/control_flow/joinir/merge/contract_checks/header_phi_layout.rs`
   - boundary の順序と header PHI の順序が一致することを strict/dev で検証

2) merge 入口でチェックを呼ぶ
   - `src/mir/builder/control_flow/joinir/merge/coordinator.rs`
   - prebuild_header_phis の直後で `verify_header_phi_layout(...)` を呼ぶ

3) boundary_hygiene を強化
   - `src/mir/builder/control_flow/joinir/merge/contract_checks/boundary_hygiene.rs`
   - carrier_info の順序と BoundaryCarrierLayout の順序が同名同順で一致することを検証

4) docs 更新
   - `docs/development/current/main/phases/phase-29af/README.md`
   - `docs/development/current/main/10-Now.md`
   - `docs/development/current/main/30-Backlog.md`

## Verification

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- strict/dev でズレを Fail-Fast できる
- quick 154/154 PASS（不変）
- JoinIR regression pack が PASS
