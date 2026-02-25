---
Status: SSOT
Scope: `MirBuilder` の生emit API（`emit_instruction` / `emit_extern_call*`）の可視性と層境界
Related:
- docs/development/current/main/design/compiler-pipeline-ssot.md
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
- src/mir/builder/builder_emit.rs
- tools/checks/no_cross_layer_builder_emit.sh
---

# Builder Emit Facade / Visibility SSOT

目的: 「勝手に emit できる」状態をなくし、`Verifier/Lower/Emitter` の責務境界を compile-time で固定する。

## Rule (single layer ownership)

- `MirBuilder::emit_instruction(...)` は `src/mir/builder/**` 内部専用（`pub(in crate::mir::builder)`）。
- `MirBuilder::emit_extern_call*` も同様に `src/mir/builder/**` 内部専用。
- `src/mir/builder/**` の外側は、生emitを呼ばない。
  - 代わりに facade/SSOT 経路を使う（例: `build_expression`, `cf_common::set_jump`, `insert_phi_at_head_spanned`）。

## Boundary intent

- 生emitの責務は builder 層に限定する。
- 外側の層は「何を lower するか」を決めるだけにし、命令列の直接編集を避ける。
- これにより、ValueId lifecycle/PHI lifecycle/diagnostics の観測点を builder 層へ集約する。

## Drift check

- Script: `tools/checks/no_cross_layer_builder_emit.sh`
- 受け入れ基準:
  - `builder_emit.rs` の可視性シグネチャが SSOT と一致する。
  - `src/mir/builder/**` 外で `.emit_instruction(` が 0 件。

## Change policy

- 新規に builder 外から生emitが必要になった場合は、まず facade API を設計する。
- そのうえで SSOT と drift check を同コミットで更新する。
