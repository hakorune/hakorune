# Phase 29ag P1: Coordinator remappers use boundary.join_inputs — Instructions

Status: Ready for execution  
Scope: coordinator の ValueId(i) fallback を撤去し、boundary.join_inputs を SSOT にする（仕様不変）

## Goal

`src/mir/builder/control_flow/joinir/merge/coordinator.rs` に残っている “ValueId(0), ValueId(1), …” 前提の remap/fallback を
撤去し、boundary が持つ `join_inputs`（JoinIR param slots）を SSOT にして remap する。

狙い:
- by-number のハードコード前提を消す（将来の JoinIR param レイアウト変更に強くする）
- coordinator 内の “順序の起点” を boundary/layout に寄せる（Phase 29af/29ag の延長）

## Non-goals

- 挙動変更（release 既定挙動の変更）
- env var の追加
- contract_checks の追加（Phase 29af P4 までで足りる）
- fixture/smoke の追加（回帰パックで検証）

## Precondition (SSOT)

- carrier order SSOT: `src/mir/builder/control_flow/joinir/merge/boundary_carrier_layout.rs`
- layout consistency contract: `src/mir/builder/control_flow/joinir/merge/contract_checks/header_phi_layout.rs`
- regression pack entrypoint: `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Implementation Steps

### Step 1: main params の remap を BoundaryCarrierLayout 経由に統一

対象: `src/mir/builder/control_flow/joinir/merge/coordinator.rs`

現状の “idx → carrier_order(index)” をやめて、
`BoundaryCarrierLayout::from_boundary(boundary).ordered_names()` を idx→carrier 名の SSOT として使う。

推奨形:
- `let layout_names = BoundaryCarrierLayout::from_boundary(boundary).ordered_names();`
- `for (idx, &main_param) in main_params.iter().enumerate() {`
  - `let Some(carrier_name) = layout_names.get(idx) else { continue; };`
  - `let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(carrier_name) else { continue; };`
  - condition_bindings の上書き回避は現状維持
  - `remapper.set_value(main_param, phi_dst)`

### Step 2: “function_params が無い” fallback を boundary.join_inputs で置き換え

現状の `ValueId(idx as u32)` を撤去し、`boundary.join_inputs[idx]` を使う。

- `let join_ids = boundary.join_inputs.as_slice();`
- `for (idx, carrier_name) in layout_names.iter().enumerate() {`
  - `let Some(&join_id) = join_ids.get(idx) else { continue; };`
  - `let Some(phi_dst) = loop_header_phi_info.get_carrier_phi(carrier_name) else { continue; };`
  - condition_bindings の上書き回避は join_id に対して行う
  - `remapper.set_value(join_id, phi_dst)`

注:
- `host_inputs` 側の placeholder（ValueId(0)）とは独立に、`join_inputs` は常に “JoinIR の param slots” を持つ前提。

### Step 3: docs 更新（P1 完了時）

- `docs/development/current/main/phases/phase-29ag/README.md` に P1 完了を追記
- `docs/development/current/main/10-Now.md` を Phase 29ag に更新（P1 完了を反映）

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- JoinIR 回帰パックが PASS のまま
- coordinator 内の `ValueId(idx)` 前提が消える（レビューで確認できる）
