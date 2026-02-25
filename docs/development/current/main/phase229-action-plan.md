# Phase 229 アクションプラン: ConditionAlias 削除

## 🎯 目標

**ConditionAlias 型を削除し、CarrierInfo を簡素化する**

- 削減: CarrierInfo のフィールド 6 → 5
- 効果: データ整合性チェック箇所 3 → 1
- 時間: 1〜2時間
- リスク: 低

## 📋 実装手順

### Step 1: resolve 関数実装 (15分)

**ファイル**: `src/mir/join_ir/lowering/carrier_info.rs`

```rust
impl CarrierInfo {
    /// Phase 229: Resolve promoted LoopBodyLocal carrier name
    ///
    /// # Arguments
    ///
    /// * `old_name` - Original LoopBodyLocal variable name (e.g., "digit_pos")
    ///
    /// # Returns
    ///
    /// * `Some(carrier_name)` - If this variable was promoted (e.g., "is_digit_pos")
    /// * `None` - If not a promoted variable
    ///
    /// # Design
    ///
    /// Uses naming convention: "var_name" → "is_var_name"
    /// Alternative: Linear search in carriers with original_name field
    pub fn resolve_promoted_carrier(&self, old_name: &str) -> Option<&str> {
        // Check if this variable was promoted
        if !self.promoted_loopbodylocals.contains(&old_name.to_string()) {
            return None;
        }

        // Naming convention: "digit_pos" → "is_digit_pos"
        let expected_name = format!("is_{}", old_name);
        self.carriers.iter()
            .find(|c| c.name == expected_name)
            .map(|c| c.name.as_str())
    }
}
```

**追加箇所**: line 420 付近（CarrierInfo impl ブロックの最後）

---

### Step 2: pattern2_with_break.rs 修正 (20分)

**ファイル**: `src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs`

**削除** (line 354-391):
```rust
// Phase 224-D: Add condition aliases to ConditionEnv
// This allows promoted variables to be referenced by their original names in conditions
for alias in &carrier_info.condition_aliases {
    // ... 約40行削除
}
```

**追加** (line 354):
```rust
// Phase 229: Resolve promoted LoopBodyLocal variables dynamically
// This replaces condition_aliases with on-demand resolution
for promoted_var in &carrier_info.promoted_loopbodylocals {
    if let Some(carrier_name) = carrier_info.resolve_promoted_carrier(promoted_var) {
        // Find carrier's join_id in carriers list (BEFORE filtering, with join_ids)
        if let Some(carrier) = carriers_with_join_ids.iter().find(|c| c.name == carrier_name) {
            if let Some(join_id) = carrier.join_id {
                // Add dynamic mapping: old_name → carrier's join_id
                env.insert(promoted_var.clone(), join_id);
                eprintln!(
                    "[pattern2/phase229] Resolved promoted variable '{}' → carrier '{}' (join_id={:?})",
                    promoted_var, carrier_name, join_id
                );
            } else {
                eprintln!(
                    "[pattern2/phase229] WARNING: Promoted carrier '{}' has no join_id yet!",
                    carrier_name
                );
            }
        } else {
            eprintln!(
                "[pattern2/phase229] WARNING: Promoted carrier '{}' not found in carriers list",
                carrier_name
            );
        }
    }
}
```

---

### Step 3: Promoter 修正 (20分)

#### 3-1. Trim Promoter

**ファイル**: `src/mir/loop_pattern_detection/loop_body_carrier_promoter.rs`

**削除** (line 87-90):
```rust
carrier_info.condition_aliases.push(crate::mir::join_ir::lowering::carrier_info::ConditionAlias {
    old_name: trimmed_var_name.to_string(),
    carrier_name: carrier_var_name.clone(),
});
```

**コメント追加** (line 87):
```rust
// Phase 229: promoted_loopbodylocals already records this promotion
// No need for condition_aliases - resolved dynamically via CarrierInfo::resolve_promoted_carrier()
```

#### 3-2. DigitPos Promoter

**ファイル**: `src/mir/loop_pattern_detection/loop_body_digitpos_promoter.rs`

**削除** (line 203-206):
```rust
carrier_info.condition_aliases.push(crate::mir::join_ir::lowering::carrier_info::ConditionAlias {
    old_name: var_name.to_string(),
    carrier_name: carrier_var_name.clone(),
});
```

**コメント追加** (line 203):
```rust
// Phase 229: promoted_loopbodylocals already records this promotion
// No need for condition_aliases - resolved dynamically via CarrierInfo::resolve_promoted_carrier()
```

---

### Step 4: ConditionAlias 型削除 (10分)

**ファイル**: `src/mir/join_ir/lowering/carrier_info.rs`

**削除** (line 85-101):
```rust
/// Phase 224-D: Condition alias for promoted LoopBodyLocal variables
///
/// Maps old variable names to their promoted carrier names for condition resolution.
/// ...
#[derive(Debug, Clone)]
pub struct ConditionAlias {
    pub old_name: String,
    pub carrier_name: String,
}
```

**削除** (line 195):
```rust
pub condition_aliases: Vec<ConditionAlias>,
```

**削除** (各 constructor で):
- line 259: `condition_aliases: Vec::new(),`
- line 320: `condition_aliases: Vec::new(),`
- line 348: `condition_aliases: Vec::new(),`
- line 410-414: `merge_other()` の condition_aliases マージコード

---

### Step 5: pattern4_carrier_analyzer.rs 修正 (5分)

**ファイル**: `src/mir/builder/control_flow/joinir/patterns/pattern4_carrier_analyzer.rs`

**削除** (line 76):
```rust
condition_aliases: all_carriers.condition_aliases.clone(), // Phase 224-D
```

**削除** (line 299):
```rust
condition_aliases: Vec::new(), // Phase 224-D
```

---

### Step 6: pattern_pipeline.rs 修正 (5分)

**ファイル**: `src/mir/builder/control_flow/joinir/patterns/pattern_pipeline.rs`

**削除** (line 412):
```rust
condition_aliases: Vec::new(), // Phase 224-D
```

**削除** (line 452):
```rust
condition_aliases: Vec::new(), // Phase 224-D
```

---

## ✅ テスト手順

### Level 1: ビルド確認 (5分)

```bash
cargo build --release 2>&1 | tee /tmp/phase229-build.log
# エラーがないことを確認
```

### Level 2: 単体テスト (5分)

```bash
# CarrierInfo resolve テスト（手動確認）
cargo test --lib carrier_info -- --nocapture

# Pattern detection テスト
cargo test --lib loop_pattern_detection -- --nocapture
```

### Level 3: パターンテスト (10分)

```bash
# Trim pattern
cargo test --release test_mir_joinir_funcscanner_trim 2>&1 | tee /tmp/phase229-trim.log

# DigitPos pattern
cargo test --release test_loopbodylocal_digitpos 2>&1 | tee /tmp/phase229-digitpos.log

# Pattern 2 (break) integration
cargo test --release test_loop_with_break 2>&1 | tee /tmp/phase229-pattern2.log
```

### Level 4: 決定性テスト (10分)

```bash
# 3回実行して ValueId が変わらないことを確認
for i in 1 2 3; do
    echo "=== Run $i ===" | tee -a /tmp/phase229-determinism.log
    cargo test --release test_loop_with_break 2>&1 | grep -E "ValueId|test result" | tee -a /tmp/phase229-determinism.log
done

# 3回の出力が同一であることを確認
```

### Level 5: E2E テスト (20分)

```bash
# Smoke tests (loop 関連のみ)
tools/smokes/v2/run.sh --profile quick --filter "loop_*" 2>&1 | tee /tmp/phase229-smoke.log

# Full MIR test suite
cargo test --release --test '*' 2>&1 | grep -E "(test result|FAILED)" | tee /tmp/phase229-full.log
```

---

## 🎯 成功条件

### ビルド
- ✅ cargo build --release が成功
- ✅ 警告 0 件（unused imports, dead code など）

### テスト
- ✅ Trim pattern テストが PASS
- ✅ DigitPos pattern テストが PASS
- ✅ Pattern 2 integration テストが PASS
- ✅ 決定性テスト（3回実行で同じ ValueId）

### コード品質
- ✅ CarrierInfo のフィールド数: 6 → 5
- ✅ condition_aliases 参照: 15箇所 → 0箇所
- ✅ 新しいドキュメントコメント追加

---

## 📝 コミットメッセージ

```
refactor(joinir): Phase 229 - Remove redundant ConditionAlias

Problem:
- ConditionAlias duplicates information already in promoted_loopbodylocals
- Maintaining 3 data structures (promoted_loopbodylocals, carriers, condition_aliases) is error-prone
- confusion: "Why do we need condition_aliases when we have promoted_loopbodylocals?"

Solution:
- Add CarrierInfo::resolve_promoted_carrier() for dynamic resolution
- Remove ConditionAlias type and all usages
- Use promoted_loopbodylocals + naming convention ("var" → "is_var")

Impact:
- CarrierInfo fields: 6 → 5
- Maintenance cost: 3 data structures → 2
- No functional changes (all tests pass)

Files changed:
- carrier_info.rs: Added resolve_promoted_carrier(), removed ConditionAlias
- pattern2_with_break.rs: Dynamic resolution instead of condition_aliases
- loop_body_carrier_promoter.rs: Removed condition_aliases.push()
- loop_body_digitpos_promoter.rs: Removed condition_aliases.push()
- pattern4_carrier_analyzer.rs: Removed condition_aliases fields
- pattern_pipeline.rs: Removed condition_aliases fields

Tests:
- ✅ test_mir_joinir_funcscanner_trim
- ✅ test_loopbodylocal_digitpos
- ✅ test_loop_with_break
- ✅ Determinism test (3 runs with same ValueId)
```

---

## 🚨 リスク管理

### 低リスク要因

1. **既存のテストで検証可能**
   - Trim pattern テスト
   - DigitPos pattern テスト
   - Pattern 2 integration テスト

2. **影響範囲が明確**
   - CarrierInfo とその使用箇所のみ
   - MIR generation ロジックは変更なし

3. **後方互換性**
   - promoted_loopbodylocals は残る
   - CarrierVar.role は残る
   - 外部インターフェース変更なし

### もしもの時の対処

**ビルドエラー**:
```bash
# condition_aliases の参照が残っていたら
rg "condition_aliases" src/mir --type rust
# 漏れを修正
```

**テスト失敗**:
```bash
# ログを確認
cat /tmp/phase229-pattern2.log | grep -E "ERROR|FAILED|panic"

# resolve が失敗していたら
# → 命名規則の確認（"is_" prefix）
# → promoted_loopbodylocals に記録されているか確認
```

**決定性失敗**:
```bash
# 3回実行で ValueId が異なる場合
# → Phase 229 の変更は関係なし（元々の問題）
# → git log で最近の HashMap 変更をチェック
```

---

## 📊 見積もり

| タスク | 時間 |
|-------|-----|
| Step 1: resolve 実装 | 15分 |
| Step 2: pattern2 修正 | 20分 |
| Step 3: Promoter 修正 | 20分 |
| Step 4-6: 型削除 | 20分 |
| テスト実行 | 50分 |
| ドキュメント更新 | 15分 |
| **合計** | **2時間20分** |

---

## ✅ チェックリスト

実装時にこのリストを確認：

- [ ] Step 1: CarrierInfo::resolve_promoted_carrier() 実装
- [ ] Step 2: pattern2_with_break.rs の condition_aliases ループ削除
- [ ] Step 3-1: loop_body_carrier_promoter.rs の condition_aliases.push() 削除
- [ ] Step 3-2: loop_body_digitpos_promoter.rs の condition_aliases.push() 削除
- [ ] Step 4: ConditionAlias 型削除
- [ ] Step 5: pattern4_carrier_analyzer.rs 修正
- [ ] Step 6: pattern_pipeline.rs 修正
- [ ] Level 1: ビルド確認
- [ ] Level 2: 単体テスト
- [ ] Level 3: パターンテスト（Trim, DigitPos, Pattern 2）
- [ ] Level 4: 決定性テスト（3回実行）
- [ ] Level 5: E2E テスト（Smoke + Full）
- [ ] コミットメッセージ作成
- [ ] CURRENT_TASK.md 更新

---

**作成日**: 2025-12-10
**対象**: Phase 229 実装
**前提**: Phase 227-228 完了
Status: Active  
Scope: アクションプラン（JoinIR/ExprLowerer ライン）
