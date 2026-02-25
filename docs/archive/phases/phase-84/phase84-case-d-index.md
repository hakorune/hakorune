# Phase 84: Case D 完全解決ロードマップ

## 現在の状況

```
Phase 84-2 完了: 12件 → 9件（25%削減）
残り: 9件

内訳:
- GroupA (Loop 制御フロー): 7件
- GroupB (多段 PHI): 2件
- GroupC (await 特殊): 1件
```

## ドキュメント一覧

### 📊 サマリー

- **[Phase 84-2 サマリー](./phase84-2-summary.md)** - 実装完了報告と次のステップ
- **[Phase 84-2 テスト一覧](./phase84-2-test-list.md)** - クイックリファレンス表

### 📖 詳細分析

- **[Phase 84-2 詳細調査](./phase84-2-case-d-investigation.md)** - 9件の分類と解決策提案
- **[Phase 84-2 失敗パターン](./phase84-2-failure-patterns.md)** - 各パターンのコード例と MIR 構造

### 🔧 実装資料

- **[CopyTypePropagator 実装](../../../src/mir/phi_core/copy_type_propagator.rs)** - Phase 84-2 で実装
- **[GenericTypeResolver](../../../src/mir/join_ir/lowering/generic_type_resolver.rs)** - Phase 84-3/4 で拡張予定
- **[lifecycle.rs](../../../src/mir/builder/lifecycle.rs)** - 型推論統合箇所

## クイックリンク

### 🎯 次のタスク

**Phase 84-3: Edge Copy 追跡 PHI 型推論**
- 目標: GroupA の 7件を解決
- 期待: 9件 → 2件（78%削減）
- 期間: 1-2日

**実装内容**:
```rust
// GenericTypeResolver に追加
pub fn resolve_from_phi_with_copy_trace(
    function: &MirFunction,
    ret_val: ValueId,
    types: &BTreeMap<ValueId, MirType>,
) -> Option<MirType> {
    // PHI の incoming 値から Copy を遡る
    for (_, incoming_val) in phi_inputs {
        if let Some(src) = find_copy_src(function, incoming_val) {
            if let Some(ty) = types.get(&src) {
                return Some(ty.clone());
            }
        }
    }
    None
}
```

### 🧪 テスト実行

```bash
# 全 Case D 確認
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D"

# GroupA のみ
cargo test --release --lib loop_continue_break
cargo test --release --lib loop_nested
cargo test --release --lib loop_return
cargo test --release --lib vm_exec_break

# GroupB のみ
cargo test --release --lib mir_stage1_cli

# GroupC のみ
cargo test --release --lib test_lowering_await
```

## 完了条件

### Phase 84-3 完了

```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 2 (GroupB のみ残存)
```

### Phase 84-4 完了

```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 1 (GroupC のみ残存)
```

### Phase 84-5 完了

```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D"
# 期待: 出力なし（0件）

cargo test --release --lib
# 期待: test result: ok
```

## タイムライン

| Phase | 目標 | 期間 | 削減数 | 残存数 |
|-------|-----|-----|-------|-------|
| 84-1 | PHI Fallback 無効化テスト | 完了 | - | 12件 |
| 84-2 | CopyTypePropagator 実装 | 完了 ✅ | 3件 | 9件 |
| 84-3 | Edge Copy 追跡 PHI 型推論 | 1-2日 | 7件 | 2件 |
| 84-4 | 多段 PHI 型推論 | 1-2日 | 2件 | 1件 |
| 84-5 | await 暫定対応 | 30分 | 1件 | 0件 |

**合計**: 2-4日で Case D 完全解決見込み

## グループ別詳細

### GroupA: Loop 制御フロー PHI（7件）

**パターン**: Loop + continue/break による Edge Copy 合流

**テスト一覧**:
1. `loop_with_continue_and_break_edge_copy_merge` (ValueId 56)
2. `nested_loop_with_multi_continue_break_edge_copy_merge` (ValueId 135)
3. `loop_inner_if_multilevel_edge_copy` (ValueId 74)
4. `loop_break_and_early_return_edge_copy` (ValueId 40)
5. `vm_exec_break_inside_if` (ValueId 27)
6. `loop_if_three_level_merge_edge_copy` (ValueId 75)
7. （7件合計）

**解決策**: Edge Copy 追跡 PHI 型推論（Phase 84-3）

**詳細**: [失敗パターン - GroupA](./phase84-2-failure-patterns.md#groupa-loop-制御フロー-phi7件)

### GroupB: 多段 PHI 型推論（2件）

**パターン**: 複数の PHI 命令が連鎖

**テスト一覧**:
1. `mir_stage1_cli_emit_program_min_exec_hits_type_error` (ValueId 7)
2. `mir_stage1_cli_emit_program_min_compiles_and_verifies` (ValueId 7)

**解決策**: 再帰的 PHI 型推論（Phase 84-4）

**詳細**: [失敗パターン - GroupB](./phase84-2-failure-patterns.md#groupb-多段-phi-型推論2件)

### GroupC: await 特殊パターン（1件）

**パターン**: await 式の MIR lowering

**テスト**:
1. `test_lowering_await_expression` (ValueId 2)

**解決策**: await 特殊ケース処理（Phase 84-5）

**詳細**: [失敗パターン - GroupC](./phase84-2-failure-patterns.md#groupc-await-特殊パターン1件)

## ChatGPT Pro 設計相談ポイント

### 相談1: Edge Copy 追跡の最適化

- Copy チェーンの追跡深度は 10 で十分か？
- 循環 Copy 検出は必要か？
- パフォーマンス最適化（キャッシュ戦略）

### 相談2: 多段 PHI の循環検出

- 循環 PHI は実際に発生するか？
- 発生する場合の処理方法（エラー or Unknown）
- visited セットの最適なデータ構造

### 相談3: await 型推論の長期戦略

- Phase 67+ async/await システムの型推論設計
- Safepoint/Checkpoint 命令の型情報統合方法
- 現在の暫定対応が将来の実装を妨げないか

## 実装チェックリスト

### Phase 84-3: Edge Copy 追跡

- [ ] `GenericTypeResolver::resolve_from_phi_with_copy_trace()` 実装
- [ ] `find_copy_src()` ヘルパー関数実装
- [ ] `trace_copy_chain()` ヘルパー関数実装
- [ ] lifecycle.rs 統合（371行目付近）
- [ ] テスト実行: GroupA の 7件を確認
- [ ] ドキュメント更新

### Phase 84-4: 多段 PHI 推論

- [ ] `GenericTypeResolver::resolve_from_phi_recursive()` 実装
- [ ] 循環検出ロジック実装（HashSet<ValueId>）
- [ ] lifecycle.rs 統合
- [ ] テスト実行: GroupB の 2件を確認
- [ ] ドキュメント更新

### Phase 84-5: await 暫定対応

- [ ] lifecycle.rs に await 特殊ケース追加
- [ ] テスト実行: GroupC の 1件を確認
- [ ] ドキュメント更新
- [ ] Phase 67+ 長期計画メモ作成

## 関連リソース

### 過去の分析

- [Phase 84-1 Case D 分析](./phase84-case-d-detailed-analysis.md) - 最初の 12件分析

### 実装ファイル

- `src/mir/phi_core/copy_type_propagator.rs` - Phase 84-2 実装
- `src/mir/join_ir/lowering/generic_type_resolver.rs` - 拡張予定
- `src/mir/builder/lifecycle.rs` - 型推論統合箇所（371行目）

### テストファイル

- `src/tests/loop_continue_break_no_phi_tests.rs` - GroupA-1
- `src/tests/loop_nested_no_phi_tests.rs` - GroupA-2,3
- `src/tests/loop_return_no_phi_tests.rs` - GroupA-4,6
- `src/tests/mir_ctrlflow_break_continue.rs` - GroupA-5
- `src/tests/mir_stage1_cli_emit_program_min.rs` - GroupB
- `src/mir/mod.rs:363` - GroupC

## まとめ

Phase 84-2 の CopyTypePropagator により 12件 → 9件に削減成功。
残り 9件は 3つのパターンに分類され、各々に明確な解決策が提案済み。

Phase 84-3/4/5 の実装により、**Case D を完全解決** できる見込み。

---

**次のアクション**: Phase 84-3 の Edge Copy 追跡 PHI 型推論を実装
Status: Historical
