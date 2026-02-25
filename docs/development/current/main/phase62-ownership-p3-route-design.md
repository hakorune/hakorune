# Phase 62: Ownership → P3 (if-sum) Production Route Design (dev-only)

## Goal

本番の MIR→JoinIR ルート（`src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`）に対して、
Ownership-Relay の契約（owned / captures / relay_writes）を SSOT として差し込み、
P3(if-sum) の boundary inputs（host_inputs / join_inputs）と carrier set を「推測」から「解析結果」へ移行する。

前提: 既定挙動は不変。`normalized_dev` feature + dev 実行ガード下で段階接続する。

## Context

- Phase 56–60: OwnershipPlan / relay / plan_to_lowering を整備し、fixtures ルートで段階的に接続済み。
- Phase 61: Break(P2) への by-name 混入を撤去し、if-sum+break は別箱で構造的に導入した。
- Phase 63: 本番 AST（`ASTNode`）から OwnershipPlan を生成できる `AstOwnershipAnalyzer` を追加した（dev-only, analysis-only）。

本番の P3(if-sum) lowering は、OwnershipPlan を受け取らず、carrier set/inputs が解析 SSOT で固定されていない。

## Scope (This Phase)

このフェーズは **設計のみ**（SSOT 文書）。コード変更は Phase 64 で行う。

### In scope

- 本番 P3(if-sum) ルートで OwnershipPlan を導入する「接続点」の決定
- OwnershipPlan と既存コンポーネント（ConditionEnv / ExitMeta / CarrierInfo）の責務境界を明文化
- dev-only の段階接続計画（Fail-Fast 条件/回帰テスト方針）を決める

### Out of scope

- multi-hop relay / merge relay（別フェーズで意味論設計が必要）
- 新しい言語仕様・パターン拡張
- 既定挙動変更（canonical への昇格など）

## Current Production Route (P3 if-sum)

入口: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`

現状の流れ（簡略）:

1. `build_pattern_context(...)` で `PatternPipelineContext` を構築
2. `ctx.is_if_sum_pattern()` のとき `lower_pattern3_if_sum(...)` を実行
3. `ConditionEnvBuilder` で `ConditionEnv + condition_bindings` を構築
4. `lower_if_sum_pattern(...)` が JoinIR を生成し、`fragment_meta.exit_meta` を返す
5. `ExitMetaCollector::collect(..., Some(&ctx.carrier_info), ...)` で exit_bindings を作る
6. `JoinInlineBoundaryBuilder` に `with_inputs(...)` / `with_condition_bindings(...)` / `with_exit_bindings(...)` を渡して conversion pipeline を実行

課題:
- carrier set が OwnershipPlan（owned/relay/capture）と一致する保証がない
- 解析 SSOT が無い状態で境界 inputs が構築され、混線を早期に検出しづらい

## Target Architecture (Ownership as SSOT)

### SSOT

- **OwnershipPlan** を SSOT とし、P3(if-sum) の「管理（carrier）」と「参照（capture）」を固定する。
- `carriers = writes ∩ owned`（Ownership-Relay の不変条件）を P3 本番ルートでも維持する。

### Key Idea

P3(if-sum) で必要なのは次の 2 点:

1. **carrier set**: boundary inputs / exit bindings の対象集合を固定する
2. **capture set**: 条件式が読むだけの外部値を condition_bindings に限定し、carrier と混ぜない

（carrier order は JoinIR 側の exit_meta / 既存 carrier_info と整合チェックで段階的に移行する）

## Integration Point (Where to Compute OwnershipPlan)

候補は 1 点に絞る:

- `cf_loop_pattern3_with_if_phi` 内で `build_pattern_context(...)` の直後
  - `PatternPipelineContext` は loop_var（name/id）と body（AST）を持ち、P3 判定も済んでいる
  - ここで OwnershipPlan を生成し、以後の boundary 構築の整合チェックに使う

## Required Interface (Already available)

- `AstOwnershipAnalyzer`（Phase 63）: `ASTNode` から `Vec<OwnershipPlan>` を生成
  - 実装サマリ: `docs/development/current/main/PHASE_63_SUMMARY.md`

## Fail-Fast Contracts (Production Route, dev-only)

OwnershipPlan を導入する際、次を Fail-Fast で固定する:

1. **Single-hop relay only**: `relay_path.len() > 1` は Err（段階移行の安全策）
2. **Carrier set alignment**:
   - OwnershipPlan から得た carriers 集合が、`ctx.carrier_info` および `exit_meta.exit_values` の集合と一致しない場合は Err
3. **No by-name switching**:
   - 関数名/Box名で意味論を変える分岐は禁止
   - ルートの切替は既存の pattern 判定（`ctx.is_if_sum_pattern()`）と構造チェックのみ

## Migration Plan (Next Phase)

### Phase 64: P3 本番ルートへ dev-only 接続 ✅ 実装済み

- ✅ `pattern3_with_if_phi.rs` に dev-only で OwnershipPlan を導入
  - `analyze_loop()` helper API を追加（`ast_analyzer.rs`）
  - `lower_pattern3_if_sum()` で OwnershipPlan を生成し整合チェック実行
- ✅ boundary inputs / exit bindings に対して carrier set の整合チェックを追加
  - `check_ownership_plan_consistency()` 関数を実装
  - Fail-Fast: multi-hop relay rejection (`relay_path.len() > 1`)
  - Warn-only: carrier set mismatch（order SSOT は後続フェーズ）
- ✅ 回帰テスト追加（`normalized_joinir_min.rs`）
  - `test_phase64_p3_ownership_prod_integration()`: 基本的な P3 ループ解析
  - `test_phase64_p3_multihop_relay_rejection()`: multi-hop relay 検出

**実装サマリ**: `docs/development/current/main/PHASE_64_SUMMARY.md`

### Phase 65+: 後続課題

- Multi-hop relay サポート（`relay_path.len() > 1` 制限の撤廃）
- Carrier order SSOT（OwnershipPlan を carrier 順序の SSOT に昇格、warn → error）
- Owner-based init（legacy `FromHost` から owner ベース初期化へ移行）

## References

- Ownership SSOT: `docs/development/current/main/phase56-ownership-relay-design.md`
- Phase 61 summary: `docs/development/current/main/PHASE_61_SUMMARY.md`
- Phase 63 summary: `docs/development/current/main/PHASE_63_SUMMARY.md`
- Production P3 route: `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
