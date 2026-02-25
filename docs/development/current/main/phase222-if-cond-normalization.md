# Phase 222: If Condition Normalization Design

## 概要

Phase 221で発見した制約「if condition pattern: if-sum mode は `var CmpOp literal` のみ」を解消し、
左辺変数・右辺変数の両方をサポートする条件正規化を実装する。

## 目標

以下のパターンを全て「simple condition」として扱えるようにする：

1. **基本形（既にサポート済み）**:
   - `i > 0`, `i < len`, `i == 5` （左辺=変数、右辺=リテラル）

2. **左右反転（新規対応）**:
   - `0 < i`, `len > i`, `5 == i` （左辺=リテラル、右辺=変数）
   - → `i > 0`, `i < len`, `i == 5` に正規化

3. **変数同士（新規対応）**:
   - `i > j`, `i < end`, `start == pos` （両辺=変数）
   - → simple condition として扱う（ConditionEnv 経由で解決）

4. **複雑条件（引き続き NG）**:
   - `i % 2 == 1`, `i + 1 > len`, `f(x) == 0` 等
   - → legacy P3 経路へフォールバック

## 設計方針

### 1. ConditionPatternBox の責務拡張

**現在の API**:
```rust
pub fn is_simple_comparison(cond: &ASTNode) -> bool
```

**拡張後の API**:
```rust
pub enum ConditionPattern {
    SimpleComparison,  // var CmpOp literal/var
    Complex,           // BinaryOp, MethodCall, etc.
}

pub fn analyze_condition_pattern(cond: &ASTNode) -> ConditionPattern

pub fn is_simple_comparison(cond: &ASTNode) -> bool  // 互換性維持
```

**新規追加**:
```rust
/// Normalize condition to canonical form (var on left)
pub fn normalize_comparison(cond: &ASTNode) -> Option<NormalizedCondition>

pub struct NormalizedCondition {
    pub left_var: String,      // 左辺変数名
    pub op: CompareOp,         // 比較演算子
    pub right: ConditionValue, // 右辺（変数 or リテラル）
}

pub enum ConditionValue {
    Variable(String),
    Literal(i64),
}
```

### 2. 正規化ルール

#### Rule 1: 左右反転（literal on left → var on left）

| Input | Normalized | 演算子変換 |
|-------|-----------|----------|
| `0 < i` | `i > 0` | `<` → `>` |
| `len > i` | `i < len` | `>` → `<` |
| `5 == i` | `i == 5` | `==` （不変） |
| `10 != i` | `i != 10` | `!=` （不変） |

**実装箇所**: `ConditionPatternBox::normalize_comparison()`

#### Rule 2: 変数同士は正規化不要

| Input | Normalized | 理由 |
|-------|-----------|-----|
| `i > j` | `i > j` | 既に canonical form |
| `i < end` | `i < end` | 既に canonical form |
| `j > i` | `j > i` | 左辺変数名の辞書順は気にしない |

### 3. ConditionEnv との統合

**Phase 220-D で実装済み**: loop 条件で変数を ConditionEnv から解決する機能

```rust
// Phase 220-D: extract_loop_condition() の拡張版
fn extract_loop_condition<F>(
    cond: &ASTNode,
    alloc_value: &mut F,
    cond_env: &ConditionEnv,
) -> Result<(String, CompareOp, ValueId, Vec<JoinInst>), String>
```

**Phase 222 での拡張**: if 条件でも同様の処理を行う

```rust
// 新規追加: extract_if_condition()
fn extract_if_condition<F>(
    cond: &ASTNode,
    alloc_value: &mut F,
    cond_env: &ConditionEnv,
) -> Result<(String, CompareOp, ConditionValue, Vec<JoinInst>), String>
```

### 4. is_if_sum_pattern() の拡張

**現在の実装**（Phase 219-fix）:
```rust
pub fn is_if_sum_pattern(&self) -> bool {
    // 1. if statement 存在チェック
    let if_stmt = self.extract_if_statement();
    if if_stmt.is_none() { return false; }

    // 2. Phase 219-fix: if 条件が simple comparison かチェック
    if let Some(ASTNode::If { condition, .. }) = if_stmt {
        use crate::mir::join_ir::lowering::condition_pattern::is_simple_comparison;
        if !is_simple_comparison(condition) {
            return false;  // 複雑条件 → legacy mode
        }
    }

    // 3. Carrier structure check (既存ロジック)
    // ...
}
```

**Phase 222 での拡張**:
```rust
pub fn is_if_sum_pattern(&self) -> bool {
    // 1. if statement 存在チェック（既存）
    let if_stmt = self.extract_if_statement();
    if if_stmt.is_none() { return false; }

    // 2. Phase 222: if 条件を正規化して simple comparison かチェック
    if let Some(ASTNode::If { condition, .. }) = if_stmt {
        use crate::mir::join_ir::lowering::condition_pattern::{
            analyze_condition_pattern, normalize_comparison
        };

        // (a) パターン判定
        let pattern = analyze_condition_pattern(condition);
        if pattern != ConditionPattern::SimpleComparison {
            return false;  // 複雑条件 → legacy mode
        }

        // (b) 正規化可能性チェック（optional: 詳細バリデーション）
        if normalize_comparison(condition).is_none() {
            return false;  // 正規化失敗 → legacy mode
        }
    }

    // 3. Carrier structure check（既存ロジック）
    // ...
}
```

## 実装戦略

### Phase 222-2: BoolExprLowerer/ConditionPatternBox 拡張

1. **ConditionPatternBox 拡張** (`condition_pattern.rs`):
   - `normalize_comparison()` 関数追加
   - `NormalizedCondition` / `ConditionValue` 型追加
   - 左右反転ロジック実装（演算子マッピング）

2. **BoolExprLowerer 統合** (`bool_expr_lowerer.rs`):
   - 正規化後の条件を lowering する経路を追加
   - ConditionEnv で変数同士の比較を解決

3. **condition_to_joinir 統合** (`condition_to_joinir.rs`):
   - loop 条件・if 条件の統一的な処理経路を確立

### Phase 222-3: if-sum 判定に統合

1. **PatternPipelineContext 更新** (`pattern_pipeline.rs`):
   - `is_if_sum_pattern()` で正規化 API を使用

2. **if-sum lowerer 更新** (`loop_with_if_phi_if_sum.rs`):
   - `extract_if_condition()` を正規化ベースに変更
   - ConditionValue::Variable / ConditionValue::Literal の両方をサポート

### Phase 222-4: E2E & 回帰テスト

1. **既存テスト確認**:
   - phase212_if_sum_min.hako: RC=2 維持
   - loop_if_phi.hako: sum=9 維持（複雑条件 → legacy mode）

2. **新規テスト作成**:
   - phase222_if_cond_left_literal.hako: `if 0 < i { sum = sum + 1 }`
   - phase222_if_cond_var_var.hako: `if i > j { sum = sum + 1 }`

### Phase 222-5: ドキュメント更新

1. **joinir-architecture-overview.md**:
   - Section 2.2 条件式ライン: ConditionPatternBox の正規化機能を追加
   - Section 4.3 JsonParser 実戦カバレッジ: Phase 222 成果を追記

2. **CURRENT_TASK.md**:
   - Phase 222 サマリー追加（3行）

## 期待される成果

1. **言語の自然性向上**:
   - `if 0 < i`, `if i > j` のような自然な条件式が if-sum パターンで使える

2. **制約の明確化**:
   - 「simple condition」の定義が明確になる（正規化可能な比較式）
   - 「complex condition」との境界が自明（BinaryOp, MethodCall 等）

3. **コードの局所性**:
   - 変更は ConditionPatternBox と if-sum lowerer のみ
   - JoinIR の芯（P1-P5, ExitLine, PHI contract）は一切変更なし

## 非目標（Non-Goals）

1. **論理演算子のサポート**:
   - `i > 0 && i < len` → Phase 223+ で対応予定
   - Phase 222 では単一比較式のみ

2. **算術式のサポート**:
   - `i + 1 > len`, `i * 2 < max` → Phase 223+ で対応予定
   - Phase 222 では変数・リテラルの直接比較のみ

3. **MethodCall のサポート**:
   - `f(x) > 0`, `s.length() < 10` → 別フェーズで対応
   - Phase 222 では変数のみ

## 参照

- Phase 219-fix: ConditionPatternBox 初版実装
- Phase 220-D: loop 条件変数サポート（ConditionEnv 統合）
- Phase 221: 制約整理（if condition pattern 制約を発見）
Status: Active  
Scope: if 条件正規化（JoinIR v2）
