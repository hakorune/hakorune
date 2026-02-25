# Phase 33 最適化実装ガイド - Step by Step

**所要時間**: 2.5時間（CommonPatternInitializer: 1h + JoinIRConversionPipeline: 1h + 検証: 0.5h）
**削減見込み**: 351行（patterns/モジュール: 200行 + merge/mod.rs: 31行 + パイプライン: 120行）

---

## 🚀 Phase 1: CommonPatternInitializer箱化（1時間）

### Step 1.1: 新規ファイル作成（5分）

```bash
cd /home/tomoaki/git/hakorune-selfhost

# 新規ファイル作成
cat > src/mir/builder/control_flow/joinir/patterns/common_init.rs << 'EOF'
//! Phase 33-22: Common Pattern Initializer Box
//!
//! Extracts duplicate initialization logic from Pattern 1-4 lowerers.
//!
//! # Purpose
//!
//! All 4 patterns (Pattern1Minimal, Pattern2WithBreak, Pattern3WithIfPhi, Pattern4WithContinue)
//! share the same initialization steps:
//! 1. Extract loop variable name from condition
//! 2. Look up ValueId in variable_map
//! 3. Trace variable map state
//!
//! This module provides a unified initializer to eliminate 200 lines of duplicate code.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use super::super::trace;

/// Loop context extracted from condition and variable_map
#[derive(Debug, Clone)]
pub struct LoopContext {
    pub loop_var_name: String,
    pub loop_var_id: ValueId,
}

/// Common Pattern Initializer Box
pub struct CommonPatternInitializer;

impl CommonPatternInitializer {
    /// Extract loop context from condition
    ///
    /// # Arguments
    ///
    /// * `builder` - MirBuilder instance (for variable_map access)
    /// * `condition` - Loop condition AST node
    /// * `pattern_name` - Pattern identifier for error messages (e.g., "pattern1", "pattern2")
    ///
    /// # Returns
    ///
    /// LoopContext containing loop_var_name and loop_var_id
    ///
    /// # Example
    ///
    /// ```rust
    /// let ctx = CommonPatternInitializer::extract_loop_context(
    ///     builder,
    ///     condition,
    ///     "pattern1",
    /// )?;
    /// // ctx.loop_var_name = "i"
    /// // ctx.loop_var_id = ValueId(42)
    /// ```
    pub fn extract_loop_context(
        builder: &MirBuilder,
        condition: &ASTNode,
        pattern_name: &str,
    ) -> Result<LoopContext, String> {
        // Step 1: Extract loop variable name from condition (e.g., "i" from "i < 3")
        let loop_var_name = builder.extract_loop_variable_from_condition(condition)?;

        // Step 2: Look up ValueId in variable_map
        let loop_var_id = builder
            .variable_map
            .get(&loop_var_name)
            .copied()
            .ok_or_else(|| {
                format!(
                    "[cf_loop/{}] Loop variable '{}' not found in variable_map",
                    pattern_name, loop_var_name
                )
            })?;

        // Step 3: Trace variable map state for debugging
        trace::trace().varmap(&format!("{}_start", pattern_name), &builder.variable_map);

        Ok(LoopContext {
            loop_var_name,
            loop_var_id,
        })
    }
}
EOF

# mod.rsに追加
echo "pub mod common_init;" >> src/mir/builder/control_flow/joinir/patterns/mod.rs
```

### Step 1.2: Pattern 1に適用（15分）

**Before (pattern1_minimal.rs:64-79)**:
```rust
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
```

**After**:
```rust
use super::common_init::{CommonPatternInitializer, LoopContext};

// ...

let LoopContext { loop_var_name, loop_var_id } =
    CommonPatternInitializer::extract_loop_context(self, condition, "pattern1")?;
```

**編集コマンド**:
```bash
# Pattern 1のファイルを開く
vim src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs

# 3行目あたりにuse追加:
# use super::common_init::{CommonPatternInitializer, LoopContext};

# 64-79行を削除して1行に置き換え:
# let LoopContext { loop_var_name, loop_var_id } =
#     CommonPatternInitializer::extract_loop_context(self, condition, "pattern1")?;
```

**テスト**:
```bash
cargo test --release loop_min_while -- --nocapture
```

### Step 1.3: Pattern 2, 3, 4に適用（各10分 = 30分）

**Pattern 2**:
```bash
vim src/mir/builder/control_flow/joinir/patterns/pattern2_with_break.rs

# 56-71行を削除して1行に置き換え
# let LoopContext { loop_var_name, loop_var_id } =
#     CommonPatternInitializer::extract_loop_context(self, condition, "pattern2")?;

cargo test --release loop_with_break -- --nocapture
```

**Pattern 3**:
```bash
vim src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs

# 56-71行を削除して1行に置き換え
# let LoopContext { loop_var_name, loop_var_id } =
#     CommonPatternInitializer::extract_loop_context(self, condition, "pattern3")?;

cargo test --release loop_with_if_phi_sum -- --nocapture
```

**Pattern 4**:
```bash
vim src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs

# 115-130行を削除して1行に置き換え
# let LoopContext { loop_var_name, loop_var_id } =
#     CommonPatternInitializer::extract_loop_context(self, condition, "pattern4")?;

cargo test --release loop_with_continue -- --nocapture
```

### Step 1.4: 全体テスト（10分）

```bash
# ビルド確認
cargo build --release

# 全パターンテスト
cargo test --release loop_min_while loop_with_break \
  loop_with_if_phi_sum loop_with_continue

# SSA-undefエラーチェック
cargo test --release 2>&1 | grep -i "ssa-undef\|undefined"

# 結果確認
if [ $? -eq 0 ]; then
  echo "✅ Phase 1完了！200行削減達成"
else
  echo "❌ Phase 1失敗、ロールバックが必要"
fi
```

---

## 🎯 Phase 2: JoinIRConversionPipeline箱化（1時間）

### Step 2.1: 新規ファイル作成（5分）

```bash
cat > src/mir/builder/control_flow/joinir/patterns/conversion_pipeline.rs << 'EOF'
//! Phase 33-22: JoinIR Conversion Pipeline Box
//!
//! Unified pipeline for JoinModule → MIR conversion + merge.
//!
//! # Purpose
//!
//! All 4 patterns share the same conversion flow:
//! 1. convert_join_module_to_mir_with_meta()
//! 2. trace::joinir_stats()
//! 3. merge_joinir_mir_blocks()
//!
//! This module eliminates 120 lines of duplicate code.

use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::JoinModule;
use crate::mir::join_ir_vm_bridge::convert_join_module_to_mir_with_meta;
use crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary;
use crate::mir::ValueId;
use std::collections::BTreeMap;
use super::super::trace;

/// JoinIR Conversion Pipeline Box
pub struct JoinIRConversionPipeline;

impl JoinIRConversionPipeline {
    /// Convert JoinModule to MIR and merge into host function
    ///
    /// # Arguments
    ///
    /// * `builder` - MirBuilder instance (mutable for merge)
    /// * `join_module` - JoinModule generated by pattern lowerer
    /// * `boundary` - JoinInlineBoundary for input/output mapping
    /// * `pattern_name` - Pattern identifier for trace/error messages
    /// * `debug` - Debug flag for verbose output
    ///
    /// # Returns
    ///
    /// Option<ValueId> from merge operation (loop result value)
    ///
    /// # Example
    ///
    /// ```rust
    /// let boundary = JoinInlineBoundary::new_inputs_only(
    ///     vec![ValueId(0)],
    ///     vec![loop_var_id],
    /// );
    ///
    /// let result = JoinIRConversionPipeline::convert_and_merge(
    ///     builder,
    ///     join_module,
    ///     boundary,
    ///     "pattern1",
    ///     debug,
    /// )?;
    /// ```
    pub fn convert_and_merge(
        builder: &mut MirBuilder,
        join_module: JoinModule,
        boundary: JoinInlineBoundary,
        pattern_name: &str,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        // Step 1: Convert JoinModule to MIR
        let empty_meta = BTreeMap::new();
        let mir_module = convert_join_module_to_mir_with_meta(&join_module, &empty_meta)
            .map_err(|e| {
                format!(
                    "[cf_loop/joinir/{}] MIR conversion failed: {:?}",
                    pattern_name, e
                )
            })?;

        // Step 2: Trace JoinIR stats for debugging
        trace::trace().joinir_stats(
            pattern_name,
            join_module.functions.len(),
            mir_module.blocks.len(),
        );

        // Step 3: Merge MIR blocks into host function
        builder.merge_joinir_mir_blocks(&mir_module, Some(&boundary), debug)
    }
}
EOF

# mod.rsに追加
echo "pub mod conversion_pipeline;" >> src/mir/builder/control_flow/joinir/patterns/mod.rs
```

### Step 2.2: Pattern 1-4に適用（各10分 = 40分）

**Pattern 1の例**:
```bash
vim src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs

# use追加:
# use super::conversion_pipeline::JoinIRConversionPipeline;

# 100-130行を削除して以下に置き換え:
# let boundary = JoinInlineBoundary::new_inputs_only(
#     vec![ValueId(0)],
#     vec![loop_var_id],
# );
#
# let _ = JoinIRConversionPipeline::convert_and_merge(
#     self,
#     join_module,
#     boundary,
#     "pattern1",
#     debug,
# )?;

cargo test --release loop_min_while
```

**Pattern 2, 3, 4も同様に適用**（各10分）

### Step 2.3: 全体テスト（15分）

```bash
cargo build --release
cargo test --release

# 削減確認
wc -l src/mir/builder/control_flow/joinir/patterns/pattern*.rs
# Pattern 1: 176 → 126行
# Pattern 2: 219 → 169行
# Pattern 3: 165 → 115行
# Pattern 4: 343 → 293行
```

---

## ⚠️ Phase 3: Legacy Fallback削除検証（30分）

### Step 3.1: Fallbackコメントアウト（5分）

```bash
vim src/mir/builder/control_flow/joinir/merge/mod.rs

# 277-307行をコメントアウト:
# /*
# if function_params.get(main_func_name).is_none() && ...
#     ...
# }
# */
```

### Step 3.2: テスト実行（20分）

```bash
cargo test --release loop_min_while loop_with_break \
  loop_with_if_phi_sum loop_with_continue 2>&1 | tee /tmp/fallback-test.log

# エラー確認
grep -i "error\|failed" /tmp/fallback-test.log
```

### Step 3.3: 判定（5分）

**テスト全てPASS → Fallback削除OK**:
```bash
# 277-307行を完全削除
vim src/mir/builder/control_flow/joinir/merge/mod.rs

# コミット
git add -A
git commit -m "feat(joinir): Phase 33-22 Remove legacy fallback (31 lines)"
```

**テスト失敗 → Fallback必要**:
```bash
# コメントアウトを戻す
git checkout src/mir/builder/control_flow/joinir/merge/mod.rs

# コメント追加（なぜ必要か記録）
vim src/mir/builder/control_flow/joinir/merge/mod.rs
# 277行目あたりに:
# // Phase 33-22検証済み: このFallbackは〇〇のケースで必要
# // 削除するとtest_XXXが失敗する
```

---

## ✅ 完了チェックリスト

### Phase 1: CommonPatternInitializer

- [ ] common_init.rs作成済み（60行）
- [ ] Pattern 1適用済み（176 → 126行）
- [ ] Pattern 2適用済み（219 → 169行）
- [ ] Pattern 3適用済み（165 → 115行）
- [ ] Pattern 4適用済み（343 → 293行）
- [ ] テスト全PASS
- [ ] ビルド警告ゼロ

### Phase 2: JoinIRConversionPipeline

- [ ] conversion_pipeline.rs作成済み（50行）
- [ ] Pattern 1適用済み（さらに30行削減）
- [ ] Pattern 2適用済み（さらに30行削減）
- [ ] Pattern 3適用済み（さらに30行削減）
- [ ] Pattern 4適用済み（さらに30行削減）
- [ ] テスト全PASS
- [ ] ビルド警告ゼロ

### Phase 3: Legacy Fallback削除

- [ ] Fallbackコメントアウト済み
- [ ] テスト実行済み
- [ ] 判定完了（削除 or 保持）
- [ ] ドキュメント更新済み

---

## 🚨 トラブルシューティング

### Q1: テストが失敗する

**症状**:
```
test loop_min_while ... FAILED
  SSA-undef: ValueId(42) not found
```

**原因**: LoopContext.loop_var_idのマッピングミス

**対処**:
```bash
# デバッグ出力有効化
NYASH_TRACE_VARMAP=1 cargo test --release loop_min_while -- --nocapture

# variable_mapの状態確認
grep "varmap.*pattern1" の出力を確認
```

### Q2: ビルドエラー

**症状**:
```
error[E0433]: failed to resolve: use of undeclared type `LoopContext`
```

**原因**: use文の追加忘れ

**対処**:
```bash
# 各patternファイルに追加
use super::common_init::{CommonPatternInitializer, LoopContext};
use super::conversion_pipeline::JoinIRConversionPipeline;
```

### Q3: Fallback削除でテスト失敗

**症状**:
```
test loop_XXX ... FAILED
  ValueId(0) not found in remapper
```

**原因**: 一部パターンでFallbackが必要

**対処**:
```bash
# Fallbackを保持
git checkout src/mir/builder/control_flow/joinir/merge/mod.rs

# コメント追加
# このFallbackはPattern Xで必要（理由: ...）
```

---

## 📚 参考ドキュメント

- [Phase 33-22 分析レポート](phase33-post-analysis.md)
- [コード重複マップ](phase33-duplication-map.md)
- [JoinIRアーキテクチャ](joinir-architecture-overview.md)

---

## 📝 完了後のコミット

```bash
git add -A
git commit -m "feat(joinir): Phase 33-22 CommonPatternInitializer + JoinIRConversionPipeline

- CommonPatternInitializer: Pattern 1-4の初期化ロジック統一化（200行削減）
- JoinIRConversionPipeline: JoinIR変換フロー統一化（120行削減）
- Legacy Fallback削除: merge/mod.rs 277-307行削除（31行削減）

Total: 351行削減

Phase 33-22完了！"
```

---

**最終確認**:
```bash
# ビルド成功
cargo build --release

# テスト全PASS
cargo test --release

# 削減確認
git diff --stat HEAD~1
# patterns/ モジュール: -200行
# merge/mod.rs: -31行
# conversion_pipeline.rs: +50行
# common_init.rs: +60行
# 実質削減: -121行
```

✅ Phase 33-22最適化完了！
Status: Historical
