---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P9-min1 として const-const-binop-return shape を native shape table に追加し、BridgeRustBackend 依存を1件縮退する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-189-wsm-p9-min0-non-native-inventory-lock-ssot.md
  - apps/tests/phase29cc_wsm_p9_min1_const_binop_return.hako
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min1_const_binop_native_lock_vm.sh
  - src/backend/wasm/shape_table.rs
  - tests/wasm_demo_min_fixture.rs
---

# 29cc-190 WSM-P9-min1 Const BinOp Native Shape Lock

## Purpose
`const -> const -> binop -> return` の最小算術shapeを native lane へ吸収し、
`default` ルートでの `BridgeRustBackend` 到達を段階的に減らす。

## Shape Contract
1. shape id: `wsm.p9.main_return_i32_const_binop.v0`
2. function: `main` 単一ブロック
3. instructions: `Const`, `Const`, `BinOp`, `Return`
4. fold 対象: `Add/Sub/Mul/Div/Mod`（不正演算は fail-fast で shape不一致）

## Fixture Contract
1. `apps/tests/phase29cc_wsm_p9_min1_const_binop_return.hako` は native lane に入る。
2. route-trace は `plan=native-shape-table shape_id=wsm.p9.main_return_i32_const_binop.v0` を出す。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p9_min1_const_binop_native_lock_vm.sh`
2. `cargo test --features wasm-backend wasm_demo_default_hako_lane_native_const_binop_shape_contract -- --nocapture`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P9-min2`: loop + canvas primer shape へ拡張する。
