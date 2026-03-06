# plan/mod.rs 宣言部の層別整理 SSOT

## 目的

`plan/mod.rs` の宣言部（60+モジュール）を責務ベースの「層」に整理し、保守性と可読性を向上させる。

## 現状

- **809行**: 宣言部(156行) + 構造体定義(653行)
- **60+モジュール**: 責務が混在しており、探すのに時間がかかる
- **Phaseコメント順**: 歴史的経緯で並んでいるため、関連するモジュールが離れている

## 層グループ分類

### Layer 1: Core Infrastructure (基盤)
**責務**: MIR lowering の中核インフラ

```
lowerer       - CorePlan → MIR 変換
normalizer    - Recipe→CorePlan compatibility lane（transition-only residue, SSOT）
verifier      - CorePlan 不変条件検証
trace         - 実行トレース (debug/dev)
branchn       - BranchN 分岐処理
facts         - Facts 抽出 SSOT (route/semantic 認識)
recipes       - Recipe-first 基底型
```

### Layer 2: Analysis Layer (観測)
**責務**: AST分析のみ、変更なし

```
canon         - CondBlockView など (analysis-only view)
```

### Layer 3: Skeleton/Feature Layer (分解スロット)
**責務**: route/semantic 認識の分解スロット

```
skeletons     - 分解スロット定義
features      - Feature modules (carrier/phi/etc.)
```

### Layer 4: Route/Semantic-Specific (route buckets)
**責務**: 各 route/semantic の固有処理（numbered labels は traceability-only の履歴ラベル）

```
generic_loop  - Generic loop v0/v1 処理
loop_simple_while / loop_char_map / loop_array_join - simple-while family
loop_break*   - break/orchestration/steps route
if_phi_join*  - if-phi join route
loop_continue_only* - continue-focused route
loop_true_early_exit* / loop_true_* - loop(true) + early-exit family
bool_predicate_scan* - bool-predicate scan route
accum_const_loop* - accum-const-loop route
```

### Layer 5: Loop-Specific (ループ固有)
**責務**: 各ループタイプの固有処理

```
loop_cond_*    - loop condition route variants 統一 (legacy label family: LoopCond*)
loop_*_*       - Scan/Collect/Bundle/True/Count etc.
nested_loop_*  - ネストループ処理
trim_*_*       - Trim 関連
exit_binding_* - Exit binding 処理
```

### Layer 6: Data Structures (データ構造)
**責務**: CorePlan の構成要素

```
parts         - CorePlan 構成要素
steps         - CorePlan ステップ (Effect vocabulary)
recipe_tree   - Recipe-first 型システム
```

### Layer 7: Orchestration (オーケストレーション)
**責務**: 全体の制御・調整

```
planner       - Single planner エントリーポイント
composer      - CorePlan 構成
emit          - MIR emit
normalize      - 正規化 (compatibility lane / Recipe→CorePlan)
single_planner - Plan order 管理
```

### Layer 8: Utilities (ユーティリティ)
**責務**: 共通機能・ポリシー

```
policies      - Plan policies
extractors    - AST 抽出
observability  - FlowBox タグ (debug/dev)
common_init    - 共通初期化
common         - 共通ヘルパー
route_shape_recognizers - route recognizers (legacy directory name: `pattern_recognizers/`)
```

### Layer 9: Legacy/Scaffolding (残骸・足場)
**責務**: 歴史的経緯で残存、将来的には整理予定

```
conversion_pipeline - 変換パイプライン
condition_env_builder - 条件環境ビルダー
ast_feature_extractor - AST feature 抽出
escape_pattern_recognizer - エスケープパターン認識
join_key      - 構造ロック (edgecfg 側在)
plan_build_session - セ�設計セッション
route_prep_pipeline - ルート前処理パイプライン
```

---

## 並び順の原則

1. **観測→境界→受理→残骸**
   - 観測レイヤー (canon, facts) が最初
   - 境界レイヤー (lowerer, verifier, normalizer)
   - 受理レイヤー (planner, emit)
   - 残骸レイヤー (legacy/*)

2. **安定性優先**: 頻繁に変更されるモジュールは後に配置
3. **循環依存回避**: parts → features などの依存関係を考慮

---

## 影響範囲

- **挙動不変**: モジュール順の変更のみ
- **1コミットに限定**: 宣言順の整理 + コメント更新のみ
- **破壊的変更なし**: pub(in crate::mir::builder) 維持

---

## 実装ステップ（設計メモ作成後）

1. 層グループごとにモジュールをまとめる
2. 各グループ内でアルファベット順
3. コメントで層区切りを明示

---

## 相談ポイント

- この層分けが妥当であれば、実際の並び替えを実装
- 調整が必要なら層グループを再検討
