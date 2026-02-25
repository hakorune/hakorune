# Phase 93: ConditionOnly Derived Slot（Trim / body-local）

Status: ✅ Done（P0+P1）  
Scope: Pattern2（Loop with Break）で「ConditionOnly（PHIで運ばない派生値）」を毎イテレーション再計算できるようにする。  
Related:
- 設計地図（入口）: `docs/development/current/main/design/joinir-design-map.md`
- Phase 92（ConditionalStep / body-local 条件式）: `docs/development/current/main/phases/phase-92/README.md`
- ExitLine/Boundary 契約（背景）: `docs/development/current/main/joinir-boundary-builder-pattern.md`

## 目的

Trim 系で使う `is_ch_match` のような「body-local から再計算できる bool」を **ConditionOnly** として扱い、
JoinIR で “初回の計算値が固定される” 事故を避ける。

- ConditionOnly は loop carrier（LoopState）ではない（header PHI で運ばない）
- 代わりに **毎イテレーション**で Derived slot として再計算する（SSOT: Recipe）

## 成果（P0）

コミット: `04fdac42 feat(mir): Phase 93 P0 - ConditionOnly Derived Slot実装`

- 新規: `src/mir/join_ir/lowering/common/condition_only_emitter.rs`
  - `ConditionOnlyRecipe`: 再計算レシピ（運搬禁止のSSOT）
  - `ConditionOnlyEmitter`: `LoopBodyLocalEnv` を使って毎イテレーション再計算
- schedule: `src/mir/join_ir/lowering/step_schedule.rs`
  - ConditionOnly がある場合に `body-init → derived → break` を強制（評価順のSSOT）
- Trim: `src/mir/builder/control_flow/joinir/patterns/trim_loop_lowering.rs`
  - ConditionOnly 用 break 生成（反転の有無を明示）

## 成果（P1）: 箱化・語彙のSSOT化

コミット: `c213ecc3 refactor(mir): Phase 93 リファクタリング - 箱化モジュール化`

- schedule:
  - `decide_pattern2_schedule()` に判定を集約し、理由（ConditionOnly / body-local / loop-local / default）をSSOT化
  - 決定→生成を分離（decision→build）してテスト容易性を上げた
- ConditionOnlyRecipe:
  - `BreakSemantics`（WhenMatch / WhenNotMatch）を recipe に保持し、break 条件生成の責務を recipe 側へ移動
  - `trim_loop_lowering.rs` 側の重複ヘルパーを削除
- Debug（新 env 追加なし）:
  - 既存の `NYASH_JOINIR_DEBUG=1` の範囲で、`[phase93/*]` prefix に統一

## 受け入れ基準（P0）

- `apps/tests/loop_min_while.hako` が退行しない（Pattern2 baseline）
- `/tmp/test_body_local_simple.hako` が “毎イテレーション再計算” で期待通り動く
- ConditionOnly を `ConditionBinding`（join input）で運ばない（初回値固定を禁止）

## 次

- P5b の完全E2E（escape skip）に進む場合も、ConditionOnly と schedule の契約は再利用できる
