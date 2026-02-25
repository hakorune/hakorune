# Phase 69-1: Trio 使用箇所の完全棚卸し

## 背景

Phase 48-3〜48-6 で Trio（LoopVarClassBox / LoopExitLivenessBox / LocalScopeInspectorBox）の機能は LoopScopeShape に移行済み。
本フェーズでは完全削除に向けて、残存する使用箇所を調査し、削除難易度を評価する。

## Trio 使用頻度（2025-11-30）

| 箱名 | 使用箇所数 | 主な使用パターン |
|------|-----------|-----------------|
| LoopVarClassBox | 54箇所 | 定義ファイル + テスト + LoopScopeShape内部 |
| LoopExitLivenessBox | 36箇所 | 定義ファイル + テスト + LoopScopeShape内部 |
| LocalScopeInspectorBox | 62箇所 | 定義ファイル + テスト + LoopScopeShape内部 |

## ファイル別使用箇所

### 1. Trio 定義ファイル（削除対象）

| ファイル | LoopVarClass | LoopExitLiveness | LocalScopeInspector | 削除難易度 |
|---------|--------------|------------------|---------------------|-----------|
| `src/mir/phi_core/loop_var_classifier.rs` | 14 | - | 15 | **Easy** |
| `src/mir/phi_core/loop_exit_liveness.rs` | 4 | 14 | - | **Easy** |
| `src/mir/phi_core/local_scope_inspector.rs` | 1 | - | 16 | **Easy** |

**削減見込み**: 3ファイル完全削除（推定 300-400行）

### 2. LoopScopeShape 関連（内部使用、削除時に調整必要）

| ファイル | LoopVarClass | LoopExitLiveness | LocalScopeInspector | 対応 |
|---------|--------------|------------------|---------------------|------|
| `src/mir/join_ir/lowering/loop_scope_shape/builder.rs` | 11 | 11 | 11 | `from_existing_boxes_legacy` を削除・簡略化 |
| `src/mir/join_ir/lowering/loop_scope_shape/shape.rs` | - | - | 1 | struct フィールド削除 |
| `src/mir/join_ir/lowering/loop_scope_shape/tests.rs` | 9 | 9 | - | テスト調整（LoopScopeShape API に統一） |

**対応**: `from_existing_boxes_legacy()` 削除、Trio フィールド削除

### 3. 外部使用箇所（置き換え必要）

| ファイル | LoopVarClass | LoopExitLiveness | LocalScopeInspector | 対応 |
|---------|--------------|------------------|---------------------|------|
| `src/mir/phi_core/loopform_builder.rs` | - | - | 4 (1箇所は new()) | LoopScopeShape API に置き換え |
| `src/mir/phi_core/phi_builder_box.rs` | 3 | - | 1 | LoopScopeShape API に置き換え |
| `src/mir/phi_core/loop_snapshot_merge.rs` | 7 | - | 10 | LoopScopeShape API に置き換え |
| `src/mir/join_ir/lowering/loop_form_intake.rs` | 2 | - | 2 | LoopScopeShape API に置き換え |
| `src/mir/join_ir/lowering/generic_case_a.rs` | 1 | 1 | - | LoopScopeShape API に置き換え |
| `src/mir/join_ir/lowering/loop_to_join.rs` | - | 1 | - | LoopScopeShape API に置き換え |
| `src/mir/loop_builder/loop_form.rs` | 1 | - | - | LoopScopeShape API に置き換え |
| `src/runner/json_v0_bridge/lowering/loop_.rs` | - | - | 2 | LoopScopeShape API に置き換え |
| `src/mir/join_ir/mod.rs` | 1 | - | - | use 文削除 |

**削減見込み**: 8ファイルから Trio 依存を削除（推定 50-150行削減）

## Trio::new() 実際の使用箇所

```bash
# コメント除外の実使用箇所
rg "LoopVarClassBox::new|LoopExitLivenessBox::new|LocalScopeInspectorBox::new" \
  --type rust -n | grep -v "^\s*//"
```

### 実使用箇所（テスト除外）

1. **loopform_builder.rs:985**
   ```rust
   let mut inspector = LocalScopeInspectorBox::new();
   ```
   - **対応**: LoopScopeShape::variable_definitions を直接使用

### テストコード内使用箇所

- **local_scope_inspector.rs**: 12箇所（テストコード内）
- **loop_var_classifier.rs**: 7箇所（テストコード内）

**対応**: LoopScopeShape 統合テストに置き換え、Trio 単体テストは削除

## 削除戦略

### Phase 69-2: LoopScopeShape への完全移行

#### Step 1: 外部使用箇所の置き換え（Easy 優先）

| ファイル | 難易度 | 対応 |
|---------|--------|------|
| loopform_builder.rs | **Easy** | inspector.new() → scope.variable_definitions |
| phi_builder_box.rs | **Easy** | classify() → scope.classify() |
| loop_form_intake.rs | **Easy** | 既に LoopScopeShape を持っている、直接使用に変更 |
| generic_case_a.rs | **Easy** | 既に LoopScopeShape を持っている、直接使用に変更 |
| loop_to_join.rs | **Easy** | 既に LoopScopeShape を持っている、直接使用に変更 |
| loop_form.rs | **Easy** | 既に LoopScopeShape を持っている、直接使用に変更 |
| json_v0_bridge/loop_.rs | **Medium** | LoopScopeShape 取得経路を追加 |

#### Step 2: LoopScopeShape 内部の Trio 依存削除

1. `from_existing_boxes_legacy()` を削除
2. `from_loop_form()` を `from_existing_boxes()` にリネーム
3. Trio フィールドを struct から削除

#### Step 3: テストコード調整

1. Trio 単体テストを削除
2. LoopScopeShape 統合テストに機能を統合

### Phase 69-3: Trio 3箱の削除

1. `loop_var_classifier.rs` 削除（推定 150行）
2. `loop_exit_liveness.rs` 削除（推定 100行）
3. `local_scope_inspector.rs` 削除（推定 150行）
4. `phi_core/mod.rs` から use 文削除

### Phase 69-4: conservative.rs の docs/ 移設

1. `phi_core/conservative.rs`（57行、全てコメント）を削除
2. `docs/development/architecture/phi-conservative-history.md` として移設

## 削減見込み合計

| カテゴリ | 削減見込み |
|---------|-----------|
| Trio 定義ファイル削除 | 300-400行 |
| 外部使用箇所置き換え | 50-150行 |
| LoopScopeShape 簡略化 | 50-100行 |
| conservative.rs 移設 | 57行 |
| **合計** | **457-707行** |

## 次のステップ

Phase 69-2 で Easy 箇所から順次置き換えを開始：
1. loopform_builder.rs（1箇所）
2. phi_builder_box.rs（3箇所）
3. loop_form_intake.rs（2箇所）
4. generic_case_a.rs（1箇所）
5. ...（全8ファイル）

全置き換え完了後、Phase 69-3 で Trio 3箱を完全削除。
Status: Historical
