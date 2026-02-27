---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P4-min6（.hako-only roadmap P4）として pilot shape 判定を shape table（箱化）へ整理し、parity gate を table 経由で lock する。
Related:
  - src/backend/wasm/shape_table.rs
  - src/backend/wasm/mod.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min6_shape_table_lock_vm.sh
  - tools/checks/dev_gate.sh
  - docs/development/current/main/phases/phase-29cc/29cc-158-wsm-p4-min5-neg-const-parity-lock-ssot.md
---

# 29cc-159 WSM-P4-min6 Shape Table Lock

## Purpose
P4 pilot の shape 判定を backend 本体から分離し、shape table 経由の単一入口へ固定する。  
`return const` 最小形の判定規則を箱化し、将来 shape 追加時の責務混線を防ぐ。

## Implemented
1. shape table 追加:
   - `src/backend/wasm/shape_table.rs`
   - shape id: `wsm.p4.main_return_i32_const.v0`
2. backend 入口整理:
   - `WasmBackend::compile_module` は `shape_table::match_pilot_shape` のみ参照。
   - 旧 `try_extract_minimal_main_i32_const` は撤去。
3. contract tests:
   - `wasm_shape_table_matches_min_const_return_contract`
   - `wasm_shape_table_rejects_non_const_return_contract`
4. smoke/gate:
   - `phase29cc_wsm_p4_min6_shape_table_lock_vm.sh`
   - `tools/checks/dev_gate.sh wasm-boundary-lite` に統合。

## Invariants
1. pilot shape 判定の判断源は shape table のみ（backend 側の二重判定禁止）。
2. shape table は fail-fast（非一致は `None`）で fallback 判定を増やさない。
3. P4 の既存 parity lock（min4/min5）を壊さない。

## Next
- `WSM-P5-min1` は `29cc-160` で完了（default cutover docs-first lock）。
- `WSM-P5-min2` は `29cc-161` で完了（default/legacy route policy lock）。
- `WSM-P5-min3` は `29cc-162` で完了（default=hako-lane bridge lock）。
- 次は `WSM-P5-min4`: bridge 依存を縮退し、`.hako` 実体路を 1 shape ずつ lock。
