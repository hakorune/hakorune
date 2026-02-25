# Phase 33 コード重複マップ - 視覚化ガイド

## 🎯 目的

Pattern 1-4の共通コードを視覚的に理解し、箱化の対象を明確にする。

---

## 📊 重複コード分布図

```
Pattern Lowerer Structure (Before Optimization)
================================================

pattern1_minimal.rs (176行)          pattern2_with_break.rs (219行)
┌─────────────────────────────┐     ┌─────────────────────────────┐
│ ✅ can_lower()              │     │ ✅ can_lower()              │
│ ✅ lower()                  │     │ ✅ lower()                  │
│                             │     │                             │
│ ⚠️ DUPLICATE INIT (50行)   │     │ ⚠️ DUPLICATE INIT (50行)   │
│   - extract_loop_var        │     │   - extract_loop_var        │
│   - variable_map lookup     │     │   - variable_map lookup     │
│   - trace::varmap()         │     │   - trace::varmap()         │
│                             │     │                             │
│ ⚠️ DUPLICATE CONVERT (30行)│     │ ⚠️ DUPLICATE CONVERT (30行)│
│   - convert_join_to_mir     │     │   - convert_join_to_mir     │
│   - trace::joinir_stats()   │     │   - trace::joinir_stats()   │
│   - JoinInlineBoundary      │     │   - JoinInlineBoundary      │
│   - merge_joinir_blocks     │     │   - merge_joinir_blocks     │
│                             │     │                             │
│ ✅ Pattern 1 specific (96行)│     │ ✅ Pattern 2 specific (139行)│
└─────────────────────────────┘     └─────────────────────────────┘

pattern3_with_if_phi.rs (165行)     pattern4_with_continue.rs (343行)
┌─────────────────────────────┐     ┌─────────────────────────────┐
│ ✅ can_lower()              │     │ ✅ can_lower()              │
│ ✅ lower()                  │     │ ✅ lower()                  │
│                             │     │                             │
│ ⚠️ DUPLICATE INIT (50行)   │     │ ⚠️ DUPLICATE INIT (50行)   │
│   - extract_loop_var        │     │   - extract_loop_var        │
│   - variable_map lookup     │     │   - variable_map lookup     │
│   - trace::varmap()         │     │   - trace::varmap()         │
│                             │     │                             │
│ ⚠️ DUPLICATE CONVERT (30行)│     │ ⚠️ DUPLICATE CONVERT (30行)│
│   - convert_join_to_mir     │     │   - convert_join_to_mir     │
│   - trace::joinir_stats()   │     │   - trace::joinir_stats()   │
│   - JoinInlineBoundary      │     │   - JoinInlineBoundary      │
│   - merge_joinir_blocks     │     │   - merge_joinir_blocks     │
│                             │     │                             │
│ ✅ Pattern 3 specific (85行) │     │ ✅ Pattern 4 specific (263行)│
│                             │     │   + LoopUpdateAnalyzer      │
│                             │     │   + ContinueNormalizer      │
└─────────────────────────────┘     └─────────────────────────────┘

⚠️ = 重複コード（削減対象）
✅ = パターン固有ロジック（維持）
```

---

## 🔥 重複コードの詳細内訳

### 重複箇所1: 初期化ロジック（4箇所×50行 = 200行）

**Pattern 1の例**:
```rust
// src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs:64-79
let loop_var_name = self.extract_loop_variable_from_condition(condition)?;
let loop_var_id = self
    .variable_map
    .get(&loop_var_name)
    .copied()
    .ok_or_else(|| {
        format!(
            "[cf_loop/pattern1] Loop variable '{}' not found in variable_map",
            loop_var_name
        )
    })?;

trace::trace().varmap("pattern1_start", &self.variable_map);

// Pattern 2, 3, 4でも同一コード
```

**重複パターン**:
- Pattern 1: `pattern1_minimal.rs:64-79` (16行)
- Pattern 2: `pattern2_with_break.rs:56-71` (16行)
- Pattern 3: `pattern3_with_if_phi.rs:56-71` (16行)
- Pattern 4: `pattern4_with_continue.rs:115-130` (16行)

**合計**: 4箇所 × 16行 = **64行重複**

---

### 重複箇所2: JoinIR変換パイプライン（4箇所×30行 = 120行）

**Pattern 1の例**:
```rust
// src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs:100-130
let mir_module = convert_join_module_to_mir_with_meta(&join_module, &empty_meta)
    .map_err(|e| format!("[cf_loop/joinir/pattern1] MIR conversion failed: {:?}", e))?;

trace::trace().joinir_stats(
    "pattern1",
    join_module.functions.len(),
    mir_module.blocks.len(),
);

let boundary = JoinInlineBoundary::new_inputs_only(
    vec![ValueId(0)],
    vec![loop_var_id],
);

let _ = self.merge_joinir_mir_blocks(&mir_module, Some(&boundary), debug)?;

// Pattern 2, 3, 4でも同一フロー（細部のみ異なる）
```

**重複パターン**:
- Pattern 1: `pattern1_minimal.rs:100-130` (31行)
- Pattern 2: `pattern2_with_break.rs:120-150` (31行)
- Pattern 3: `pattern3_with_if_phi.rs:105-135` (31行)
- Pattern 4: `pattern4_with_continue.rs:200-230` (31行)

**合計**: 4箇所 × 31行 = **124行重複**

---

## 🎯 箱化後の理想構造

```
After Optimization (CommonPatternInitializer + JoinIRConversionPipeline)
=========================================================================

pattern1_minimal.rs (126行 → 28%削減)
┌─────────────────────────────┐
│ ✅ can_lower()              │
│ ✅ lower()                  │
│                             │
│ 📦 CommonPatternInitializer │  ← 新しい箱！
│    .extract_loop_context()  │  ← 1行で50行分の処理
│                             │
│ 📦 JoinIRConversionPipeline │  ← 新しい箱！
│    .convert_and_merge()     │  ← 1行で30行分の処理
│                             │
│ ✅ Pattern 1 specific (96行)│  ← 変更なし
└─────────────────────────────┘

pattern2_with_break.rs (169行 → 23%削減)
pattern3_with_if_phi.rs (115行 → 30%削減)
pattern4_with_continue.rs (293行 → 15%削減)
                          ↑
                全パターン同様に削減！
```

---

## 📊 削減インパクト分析

### Before / After 比較表

| ファイル | Before | After | 削減行数 | 削減率 |
|---------|-------|-------|---------|-------|
| pattern1_minimal.rs | 176行 | 126行 | -50行 | 28% |
| pattern2_with_break.rs | 219行 | 169行 | -50行 | 23% |
| pattern3_with_if_phi.rs | 165行 | 115行 | -50行 | 30% |
| pattern4_with_continue.rs | 343行 | 293行 | -50行 | 15% |
| **patterns/ 合計** | **1,801行** | **1,601行** | **-200行** | **11%** |

### 新規追加ファイル

| ファイル | 行数 | 役割 |
|---------|-----|-----|
| common_init.rs | 60行 | CommonPatternInitializer実装 |
| conversion_pipeline.rs | 50行 | JoinIRConversionPipeline実装 |

**実質削減**: 200行 - 110行 = **90行削減** + 保守性大幅向上

---

## 🔍 コード重複検出コマンド

```bash
# 重複箇所1: Loop variable extraction
grep -A 10 "extract_loop_variable_from_condition" \
  src/mir/builder/control_flow/joinir/patterns/pattern*.rs

# 重複箇所2: JoinIR conversion
grep -A 15 "convert_join_module_to_mir_with_meta" \
  src/mir/builder/control_flow/joinir/patterns/pattern*.rs

# 重複箇所3: Merge call
grep -A 5 "merge_joinir_mir_blocks" \
  src/mir/builder/control_flow/joinir/patterns/pattern*.rs
```

---

## 🎯 実装順序（推奨）

### Phase 1: CommonPatternInitializer (1時間)

```bash
# Step 1: 新規ファイル作成
touch src/mir/builder/control_flow/joinir/patterns/common_init.rs

# Step 2: Pattern 1で動作確認
cargo test --release loop_min_while

# Step 3: Pattern 2, 3, 4に適用
# （各10分）

# Step 4: 全体テスト
cargo test --release loop_min_while loop_with_break \
  loop_with_if_phi_sum loop_with_continue
```

### Phase 2: JoinIRConversionPipeline (1時間)

```bash
# Step 1: 新規ファイル作成
touch src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs

# Step 2: Pattern 1で動作確認
cargo test --release loop_min_while

# Step 3: Pattern 2, 3, 4に適用
# （各10分）

# Step 4: 全体テスト
cargo test --release
```

---

## ✅ 成功基準

1. **ビルド成功**: 0エラー・0警告
2. **テスト全PASS**: Pattern 1-4の既存テスト全て通過
3. **SSA-undefゼロ**: MIRレベルのエラーなし
4. **削減達成**: patterns/モジュール全体で200行削減
5. **保守性向上**: 重複コードゼロ、単一責任の原則適用

---

## 🚨 リスク管理

### 潜在的リスク

1. **テスト失敗**: 初期化ロジックの微妙な差異を見落とす可能性
   - **対策**: Pattern毎に個別テスト実行、段階的移行

2. **デバッグ困難化**: エラー時のスタックトレースが深くなる
   - **対策**: 適切なエラーメッセージ、pattern_name引数の維持

3. **将来の拡張性**: Pattern 5/6で異なる初期化が必要になる可能性
   - **対策**: CommonPatternInitializerを柔軟に設計（オプション引数）

### 緊急時のロールバック手順

```bash
# Step 1: 変更前のコミットに戻る
git revert HEAD

# Step 2: テスト確認
cargo test --release

# Step 3: 原因分析
# → phase33-post-analysis.md の「テスト計画」を参照
```

---

## 📚 関連ドキュメント

- [Phase 33-19 実装完了レポート](phase33-17-implementation-complete.md)
- [Phase 33-21 Parameter remapping fix](../../../private/) (未作成)
- [JoinIRアーキテクチャ概要](joinir-architecture-overview.md)
- [Pattern Router設計](phase33-16-INDEX.md)

---

## 📝 変更履歴

- 2025-12-07: 初版作成（Phase 33-21完了後の調査結果）
Status: Historical
