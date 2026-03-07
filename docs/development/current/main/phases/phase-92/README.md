# Phase 92: Lowering (ConditionalStep / P5b Escape)

## Status
- ✅ P0: Contract + skeleton-to-lowering wiring (foundations)
- ✅ P1: Boxification / module isolation (ConditionalStep emitter)
- ✅ P2: loop_break route（historical label: Pattern2）へ配線 + body-local 条件式サポート
- ✅ P3: BodyLocal 1変数（read-only）を loop_break route 条件で許可（Fail-Fast）
- ✅ P4: E2E固定（最小）+ 回帰最小化（unit + integration smoke）
- ⏸️ P5: P5b “完全E2E” は promotion 拡張後

## Goal
- Phase 91 で認識した P5b（escape skip: +1 / +2 の条件付き更新）を、JoinIR lowering まで落とせるようにする。
- 既定挙動は不変。Fail-Fast を維持し、未対応は理由付きで止める。

## P0（完了）

- P0-1: ConditionalStep 契約（SSOT）
  - 実装/記録: `src/mir/loop_canonicalizer/skeleton_types.rs`（契約コメント）
  - 記録（歴史）: `docs/development/current/main/phases/phase-92/p0-2-skeleton-to-context.md`
- P0-2: Skeleton → lowering への配線（Option A）
  - legacy context carrier type `LoopPatternContext` に skeleton を optional に通した（後に P1 で境界を整理）
- P0-3: ConditionalStep を JoinIR（Select 等）で表現する基盤を追加

## P1（完了）: 箱化・モジュール化

- ConditionalStep lowering を 1 箱に隔離
  - `src/mir/join_ir/lowering/common/conditional_step_emitter.rs`
  - `src/mir/join_ir/lowering/common.rs` から export
- 目的: loop_break route 本体を肥大化させず、条件付き更新の責務を emitter に閉じ込める
- 境界の整理（SSOT）
  - routing 層から skeleton を取り除き、loop_break route 側で skeleton/recognizer 情報の取得を内部化
  - recognizer は cond/delta 抽出に限定（スコープ/寿命の判断を混ぜない）
- E2E fixture
  - `test_pattern5b_escape_minimal.hako` は用意済み（body-local 対応後に実行固定）

## P2（完了）: loop_break route へ配線 + 条件式の body-local 対応

- `ConditionalStep` の lowering を loop_break route から emitter に委譲（route 本体の肥大化を防ぐ）
- 条件式の値解決に `LoopBodyLocalEnv` を追加し、`ConditionEnv → LoopBodyLocalEnv` の優先順位で解決
- break guard の lowering 順序を修正し、body-local 初期化の後に条件式を lower（`ch == ...` などを解決）

## P3（完了）: BodyLocal 1変数（read-only）を条件式で許可（Fail-Fast）

- 目的: `ch` のような read-only body-local（毎回再計算）を loop_break route の break/escape 条件で参照できるようにする
- 新規箱: `src/mir/join_ir/lowering/common/body_local_slot.rs`
  - 許可: 条件に出る LoopBodyLocal が 1つ、top-level `local <name> = <expr>`、break guard `if` より前、代入なし
  - 禁止: 複数、代入あり、定義が break guard より後、top-level 以外（分岐内など）
  - 破ると `error_tags::freeze(...)` で理由付き停止

## P4（完了）: E2E固定（最小）+ 回帰最小化

- unit: `src/mir/join_ir/lowering/condition_lowerer.rs` に body-local 解決のユニットテストを追加
- integration smoke: `tools/smokes/v2/profiles/integration/apps/archive/phase92_pattern2_baseline.sh`
  - Case A: `apps/tests/loop_min_while.hako`（既存 loop_break route の退行チェック; legacy fixture token retained）
  - Case B: `apps/tests/phase92_conditional_step_minimal.hako`（ConditionalStep の最小確認）
- 詳細ログ（歴史）:
  - `docs/development/current/main/phases/phase-92/P4-E2E-PLAN.md`
  - `docs/development/current/main/phases/phase-92/P4-COMPLETION.md`

## Follow-up（Phase 93）

- Trim の `is_ch_match` など「ConditionOnly（PHIで運ばない派生値）」を毎イテレーション再計算する Derived Slot を追加（初回値固定の根治）。
  - `docs/development/current/main/phases/phase-93/README.md`

## Acceptance
- `NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1` で parity が green のまま
- E2E が 1 本通る（最初は VM でOK）
- 既定挙動不変（フラグOFFで無影響）
