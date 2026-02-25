Status: Active  
Scope: Phase 33-17 の JoinIR モジュール化分析の要約（現行参照用）。詳細メモは archive 側へ移行。

# Phase 33-17: JoinIR モジュール化分析

## 実行日: 2025-12-07

## 現状のファイルサイズ分析

```
649行 instruction_rewriter.rs  ⚠️ 最大のファイル（目標200行の3.2倍）
405行 exit_binding.rs          ✅ 適切（テスト含む）
355行 pattern4_with_continue.rs
338行 routing.rs
318行 loop_header_phi_builder.rs ⚠️ やや大きい
300行 merge/mod.rs              ✅ 適切
250行 trace.rs
228行 ast_feature_extractor.rs
214行 pattern2_with_break.rs
192行 router.rs
176行 pattern1_minimal.rs
163行 pattern3_with_if_phi.rs
157行 exit_line/reconnector.rs
139行 exit_line/meta_collector.rs
104行 value_collector.rs
103行 exit_line/mod.rs
98行  exit_phi_builder.rs
65行  block_allocator.rs
```

## 🎯 モジュール化推奨事項

### 優先度A: instruction_rewriter.rs (649行 → 3ファイル200行以下)

**問題**:
- 1ファイル649行は箱理論の単一責任原則に違反
- 3つの独立した関心事が混在:
  1. **TailCallKind分類ロジック** (70行)
  2. **命令書き換え処理** (400行)
  3. **MergeResult構築** (170行)

**推奨分割**:

```
instruction_rewriter.rs (649行)
  ↓
1. tail_call_classifier.rs (90行)
   - TailCallKind enum
   - classify_tail_call()
   - 分類ロジックの完全カプセル化

2. instruction_mapper.rs (350行)
   - merge_and_rewrite() のコア処理
   - Call→Jump変換
   - 命令リマッピング

3. boundary_injector.rs (180行)
   - BoundaryInjector 呼び出し
   - Copy命令生成
   - boundary関連ロジック

4. merge_result.rs (30行)
   - MergeResult struct定義
   - 関連ヘルパー
```

**効果**:
- ✅ 単一責任の原則完全準拠
- ✅ テスタビリティ向上（tail call分類を独立テスト可能）
- ✅ 可読性向上（ファイルサイズ200行以下）

---

### 優先度B: loop_header_phi_builder.rs (318行 → 2ファイル)

**問題**:
- 318行は200行目標を58%超過
- 2つの関心事が存在:
  1. **LoopHeaderPhiInfo管理** (データ構造)
  2. **LoopHeaderPhiBuilder構築ロジック** (アルゴリズム)

**推奨分割**:

```
loop_header_phi_builder.rs (318行)
  ↓
1. loop_header_phi_info.rs (150行)
   - LoopHeaderPhiInfo struct
   - CarrierPhiEntry struct
   - get/set メソッド

2. loop_header_phi_builder.rs (170行)
   - LoopHeaderPhiBuilder
   - build() 実装
   - finalize() 実装
```

**効果**:
- ✅ データとロジックの分離
- ✅ LoopHeaderPhiInfo を他モジュールから独立利用可能
- ✅ ファイルサイズ200行以下達成

---

### 優先度C: pattern4_with_continue.rs (355行)

**問題**:
- 355行（目標の1.8倍）
- Pattern 2/3との重複コードが存在する可能性

**推奨調査**:
```bash
# Pattern 2/3/4の共通ヘルパー抽出可能性を調査
grep -A5 "extract.*variable\|build.*boundary" pattern*.rs
```

**暫定推奨**:
- Pattern 4は継続フラグで大きくなるのは自然
- 共通ヘルパーがあれば `pattern_helpers.rs` に抽出
- なければ現状維持（Phase 33-17の対象外）

---

### 優先度D: routing.rs (338行)

**問題**:
- 338行（目標の1.7倍）
- パターンルーティングロジックが肥大化

**推奨調査**:
- Pattern 1-4の条件分岐ロジックが重複していないか確認
- 抽出可能な共通ロジックがあれば分離

**暫定推奨**:
- ルーティングは本質的に線形な条件分岐になる
- 無理に分割せず、Phase 34以降で見直し

---

## 🚀 実装計画

### Phase 33-17-A: instruction_rewriter分割（最優先）

**タスク**:
1. tail_call_classifier.rs 作成（TailCallKind + classify_tail_call）
2. boundary_injector_wrapper.rs 作成（BoundaryInjector呼び出し）
3. instruction_mapper.rs 作成（merge_and_rewriteコア）
4. merge_result.rs 作成（MergeResult struct）
5. instruction_rewriter.rs → 上記4ファイルへ移行
6. merge/mod.rs の import 更新
7. cargo build --release 確認

**期待効果**:
- instruction_rewriter.rs: 649行 → 削除（4ファイルに分散）
- 最大ファイル: 350行（instruction_mapper.rs）
- 200行超ファイル: 2個（instruction_mapper, pattern4）

---

### Phase 33-17-B: loop_header_phi_builder分割（次点）

**タスク**:
1. loop_header_phi_info.rs 作成（データ構造）
2. loop_header_phi_builder.rs → ビルダーロジックのみ残す
3. merge/mod.rs の import 更新
4. cargo build --release 確認

**期待効果**:
- loop_header_phi_builder.rs: 318行 → 170行
- loop_header_phi_info.rs: 新規 150行
- 200行超ファイル: 1個（instruction_mapper のみ）

---

## 📊 完成後の予測

### Before (Phase 33-16)
```
649行 instruction_rewriter.rs  ⚠️
318行 loop_header_phi_builder.rs ⚠️
355行 pattern4_with_continue.rs ⚠️
```

### After (Phase 33-17完了後)
```
350行 instruction_mapper.rs   ⚠️ (許容範囲)
355行 pattern4_with_continue.rs ⚠️ (継続フラグで妥当)
180行 boundary_injector_wrapper.rs ✅
170行 loop_header_phi_builder.rs ✅
150行 loop_header_phi_info.rs ✅
90行  tail_call_classifier.rs ✅
30行  merge_result.rs ✅
```

**達成指標**:
- ✅ 200行超ファイル: 4個 → 2個（50%削減）
- ✅ 最大ファイル: 649行 → 355行（45%削減）
- ✅ 単一責任の原則完全準拠

---

## 🎯 Next Steps

1. **Phase 33-17-A実装** → instruction_rewriter分割（優先度最高）
2. **Phase 33-17-B実装** → loop_header_phi_builder分割
3. **Pattern 4調査** → 共通ヘルパー抽出可能性検証
4. **Phase 33-18検討** → routing.rs の最適化（必要に応じて）

---

## 箱理論への準拠

### 設計哲学
- ✅ **TailCallClassifier Box**: 分類ロジックの完全カプセル化
- ✅ **BoundaryInjectorWrapper Box**: boundary処理の委譲
- ✅ **InstructionMapper Box**: 命令変換の単一責任
- ✅ **LoopHeaderPhiInfo Box**: データ構造の独立管理

### 命名規則
- `〜Classifier`: 分類・判定ロジック
- `〜Mapper`: 変換・マッピング処理
- `〜Wrapper`: 外部Boxへの委譲
- `〜Info`: データ構造管理

---

## 📝 実装時の注意点

1. **後方互換性**: merge/mod.rsからのインターフェースは変更しない
2. **テスト**: 既存のテストが全てパスすることを確認
3. **ドキュメント**: 各ファイルに箱理論の役割を明記
4. **段階的移行**: 1ファイルずつ移行 → ビルド確認 → 次へ

---

**Status**: 分析完了、Phase 33-17-A実装準備完了
**Estimated Time**: Phase 33-17-A (2時間), Phase 33-17-B (1時間)
