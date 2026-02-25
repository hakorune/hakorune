# Phase 223 Refactoring Tasks

Phase 223-3 実装完了後のコード品質改善タスク一覧だよ。

---

## Task R-1: impl ブロック統合（優先度: 高）

**問題**:
`loop_body_cond_promoter.rs` で同じ struct に対して impl が2箇所に分かれている。

**現状**:
```rust
impl LoopBodyCondPromoter {
    pub fn extract_continue_condition(...) { ... }
    fn contains_continue(...) { ... }
}

impl LoopBodyCondPromoter {
    pub fn try_promote_for_condition(...) { ... }
}
```

**やること**:
1. 2つの impl ブロックを1つに統合
2. メソッドの順序を論理的に整理:
   - Public API (extract_continue_condition, try_promote_for_condition)
   - Private helpers (contains_continue)

**影響範囲**: `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs` のみ

**テスト**: `cargo test --release --lib cond_promoter` (5 tests PASS を確認)

**期待**: コード可読性向上、構造の明確化

---

## Task R-2: Phase コメント整理（優先度: 中）

**問題**:
`pattern4_with_continue.rs` に古い Phase 番号のコメントが残っている。

**現状**:
```rust
// Phase 171-C-3: LoopBodyCarrierPromoter integration (削除済みコードへの参照)
// Phase 223-3: LoopBodyCondPromoter integration (新しいコード)
```

**やること**:
1. Phase 171-C-3 の古いコメントを削除
2. Phase 223-3 のコメントを簡潔に整理
3. 実装の意図が明確になるようコメントを調整

**候補**:
```rust
// Phase 223-3: LoopBodyLocal Condition Promotion
// Check for LoopBodyLocal in loop/continue conditions and attempt promotion.
// Safe Trim patterns (Category A-3: skip_whitespace) are promoted to carriers.
```

**影響範囲**: `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs`

**テスト**: `cargo test --release --lib pattern4` (8 tests PASS を確認)

---

## Task R-3: エラーメッセージの定数化（優先度: 低）

**問題**:
Pattern4 のエラーメッセージが複数箇所にハードコードされている。

**現状**:
```rust
// pattern4_with_continue.rs
return Err(format!(
    "[cf_loop/pattern4] Cannot promote LoopBodyLocal variables {:?}: {}",
    vars, reason
));

// loop_body_cond_promoter.rs
eprintln!(
    "[cond_promoter] Cannot promote LoopBodyLocal variables {:?}: {}",
    vars, reason
);
```

**やること**:
1. `error_messages.rs` に Pattern4 用のエラーメッセージ関数を追加
2. 既存の `format_error_pattern4` パターンに合わせる
3. ハードコードされたメッセージを関数呼び出しに置き換え

**影響範囲**:
- `src/mir/loop_pattern_detection/error_messages.rs` (追加)
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` (置き換え)
- `src/mir/loop_pattern_detection/loop_body_cond_promoter.rs` (置き換え)

**テスト**: `cargo test --release --lib error_messages pattern4 cond_promoter`

---

## Task R-4: Pattern4ConditionAnalyzer Box 化（優先度: 低、将来対応）

**問題**:
Pattern4 の条件分析ロジックが `pattern4_with_continue.rs` に直接書かれている。

**現状**:
```rust
// pattern4_with_continue.rs 内にインライン
let continue_cond = LoopBodyCondPromoter::extract_continue_condition(body_to_analyze);
let conditions_to_analyze: Vec<&ASTNode> = if let Some(cont_cond) = continue_cond {
    vec![condition, cont_cond]
} else {
    vec![condition]
};
let cond_scope = LoopConditionScopeBox::analyze(...);
```

**候補**:
```rust
// 新規 Box: Pattern4ConditionAnalyzer
pub struct Pattern4ConditionAnalyzer;

impl Pattern4ConditionAnalyzer {
    /// Analyze all conditions (header + continue) for Pattern4
    pub fn analyze_all_conditions(
        loop_param_name: &str,
        header_cond: &ASTNode,
        body: &[ASTNode],
        scope: &LoopScopeShape,
    ) -> LoopConditionScope {
        // Extract continue condition
        // Combine with header condition
        // Call LoopConditionScopeBox::analyze
    }
}
```

**やること**:
1. 新規ファイル `src/mir/builder/control_flow/joinir/patterns/pattern4_condition_analyzer.rs` 作成
2. 条件分析ロジックを切り出し
3. Pattern4 から呼び出し
4. ユニットテスト追加

**影響範囲**:
- 新規: `pattern4_condition_analyzer.rs`
- 修正: `pattern4_with_continue.rs`
- 修正: `mod.rs` (モジュール登録)

**テスト**: 新規テスト + 既存 pattern4 テスト

**Note**: 現状で十分動作しているため、Pattern4 の複雑度が増した時に検討。

---

## Task R-5: LoopConditionScopeBox API 改善（優先度: 低、将来対応）

**問題**:
`analyze()` が `&[&ASTNode]` を受け取るため、呼び出し側で Vec を作る必要がある。

**現状**:
```rust
let conditions_to_analyze: Vec<&ASTNode> = ...;
LoopConditionScopeBox::analyze(&loop_var, &conditions_to_analyze, ...);
```

**候補**:
```rust
// Option 1: Iterator ベース
LoopConditionScopeBox::analyze_iter(
    &loop_var,
    [condition, cont_cond].iter().filter_map(|&c| c),
    scope,
);

// Option 2: 複数シグネチャ
impl LoopConditionScopeBox {
    pub fn analyze(...) -> LoopConditionScope { ... }
    pub fn analyze_single(cond: &ASTNode, ...) -> LoopConditionScope { ... }
    pub fn analyze_pair(cond1: &ASTNode, cond2: Option<&ASTNode>, ...) -> LoopConditionScope { ... }
}
```

**影響範囲**: 広範囲（LoopConditionScopeBox を使う全ての箇所）

**Note**: 影響範囲が大きいため、現状維持。必要になったら検討。

---

## 実施方針

1. **Task R-1 (impl 統合)**: 簡単・影響小、すぐ実施可能
2. **Task R-2 (コメント整理)**: 簡単・影響小、R-1 と一緒に実施可能
3. **Task R-3 (エラーメッセージ)**: 中程度、既存パターンあり、実施可能
4. **Task R-4/R-5**: 優先度低、必要になったら検討

## フィードバック項目

- [ ] R-1/R-2 は一緒にやる？
- [ ] R-3 も含める？
- [ ] 他に気になる箇所ある？
- [ ] コメントの書き方（日本語 vs 英語、詳細度）の好み

---

**Note**: 全て既存テスト PASS を前提。退行防止のため各タスク後に `cargo test --release` 実行。
Status: Active  
Scope: Refactoring タスクリスト（JoinIR/ExprLowerer ライン）
