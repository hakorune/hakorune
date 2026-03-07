# Phase 267 — EdgeCFG Branch（BranchStub + emit_frag）

Status: Completed (P0) ✅ / P1 deferred ⏸️  
SSOT (design): `docs/development/current/main/design/edgecfg-fragments.md`

## ゴール
- `Frag` に **Branch（条件分岐）**を第一級で追加し、`wires`（Jump/Return）と同様に **MIR terminator へ落とせる**入口を作る
- 既存の JoinIR/NormalizedShadow/scan_with_init / split_scan / bool_predicate_scan route には触らず、**BasicBlockId 層の test-only PoC**で証明する

## P0（完了）
### 実装
- `BranchStub` を追加（フラット構造: from/cond/then/else + edge-args）
  - `src/mir/builder/control_flow/edgecfg/api/branch_stub.rs`
- `Frag` に `branches: Vec<BranchStub>` を追加（Branch 専用）
  - `wires` は Jump/Return 専用のまま維持
  - `src/mir/builder/control_flow/edgecfg/api/frag.rs`
- `compose::if_` が header→then/else の `BranchStub` を生成するように更新
  - `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
- `emit_frag(function, frag)` を追加（SSOT）
  - `verify_frag_invariants_strict()` を最初に実行
  - `wires` は `emit_wires()` を内部利用
  - `branches` は `set_branch_with_edge_args()` で terminator + successors を同期
  - 1 block = 1 terminator（wire/branch 重複）を Fail-Fast
  - `src/mir/builder/control_flow/edgecfg/api/emit.rs`

### テスト
- `emit_frag` の Branch 生成・衝突検出、`compose::if_` の BranchStub 生成を unit test で固定（lib tests PASS）

## P1（見送り）
- “層を跨がない実適用”の候補調査を行い、現状は抽象化層（emit_conditional/emit_jump 等）へ委譲済みであるため、無理に差し替えず **Phase 268** で体系的に適用する方針

## 次フェーズ（Phase 268）
- Branch の edge-args（then/else entry params）計算を含めた実戦投入
- JoinIR/NormalizedShadow/scan_with_init / split_scan / bool_predicate_scan route 側へ段階的に適用（層境界を守る）
