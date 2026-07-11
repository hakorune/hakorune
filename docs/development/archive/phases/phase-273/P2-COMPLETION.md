# Phase 273 P2 Completion (2025-12-22)

P2 Goal:
- Pattern7（split scan）を Plan ライン（DomainPlan→CorePlan→Lowerer）へ移行し、Lowerer の “pattern知識” を増やさない。

## Delivered

- CoreEffectPlan: 副作用対応
  - `MethodCall` が `dst: Option<ValueId>` を持てる（`push` 等の void-return 対応）
  - `effects: EffectMask` を CorePlan から指定できる（PURE ではない呼び出しを明示）

- CoreLoopPlan: 一般化（Pattern6/7 の収束点）
  - `block_effects: Vec<(BasicBlockId, Vec<CoreEffectPlan>)>`
  - `phis: Vec<CorePhiInfo>`
  - `frag: Frag`（terminator SSOT: `emit_frag()`）
  - `final_values: Vec<(String, ValueId)>`（variable_map 更新の SSOT）

- Pattern7: Plan ライン移行
  - Extractor は pure（builder を触らない）
  - Normalizer が split 固有のブロック構造/PHI/Frag を決定（SSOT）
  - Lowerer は CorePlan を吐くだけ（pattern-agnostic）

## Regression / Validation

- VM:
  - `tools/smokes/v2/profiles/integration/apps/archive/phase256_p0_split_vm.sh` PASS（Pattern7）
  - `tools/smokes/v2/profiles/integration/apps/archive/phase258_p0_index_of_string_vm.sh` PASS（Pattern6）

## Notes

- P2 は “generalized 経路” と “legacy fallback” を共存させた（移行中の安全策）。
- 次の P3 で Pattern6 も generalized 経路へ移し、legacy fallback を撤去すると収束が完成する。

