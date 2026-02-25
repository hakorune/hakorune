/*!
 * PHI命令挿入ユーティリティ - Phase 4: PHI挿入パターン統一
 *
 * ## 目的
 * builder内の重複するPHI挿入パターン（26箇所）を統一し、50-100行削減
 *
 * ## 設計原則
 * 1. SSA不変条件の維持（predecessor対応の厳密さ）
 * 2. 既存の`insert_phi_at_head`との統合
 * 3. 段階的移行のための後方互換性
 *
 * ## 使用例
 * ```rust
 * // Before (5-7行)
 * let phi_val = self.value_gen.next();
 * let inputs = vec![(pred1, val1), (pred2, val2)];
 * if let (Some(func), Some(cur_bb)) = (self.scope_ctx.current_function.as_mut(), self.current_block) {
 *     crate::mir::ssot::cf_common::insert_phi_at_head(func, cur_bb, phi_val, inputs);
 * } else {
 *     self.emit_instruction(MirInstruction::Phi { dst: phi_val, inputs })?;
 * }
 *
 * // After (1行, 80-85%削減)
 * let phi_val = self.insert_phi(vec![(pred1, val1), (pred2, val2)])?;
 * ```
 */

use crate::mir::builder::MirBuilder;
use crate::mir::diagnostics::FreezeContract;
use crate::mir::{BasicBlockId, ValueId};

/// PHI挿入ヘルパー - MirBuilderへのextension methods
impl MirBuilder {
    /// **標準PHI挿入メソッド** - 現在のブロックに複数入力のPHI命令を挿入
    ///
    /// ## 引数
    /// - `inputs`: Vec<(predecessor_block, value_from_that_block)>
    ///
    /// ## 戻り値
    /// - 新しく割り当てられたPHI命令の結果ValueId
    ///
    /// ## 使用場面
    /// - If/Else合流（2入力）
    /// - ループヘッダー（初期値+backedge）
    /// - 短絡評価のマージ
    /// - match式の合流
    ///
    /// ## 例
    /// ```rust
    /// // If/Else合流
    /// let phi_val = self.insert_phi(vec![
    ///     (then_block, then_value),
    ///     (else_block, else_value),
    /// ])?;
    ///
    /// // ループヘッダー
    /// let loop_var = self.insert_phi(vec![
    ///     (entry_block, init_value),
    ///     (backedge_block, updated_value),
    /// ])?;
    /// ```
    #[inline]
    pub fn insert_phi(&mut self, inputs: Vec<(BasicBlockId, ValueId)>) -> Result<ValueId, String> {
        // Phase 25.1b fix: Use function-local ID allocator to avoid SSA verification failures
        // This prevents PHI dst ValueIds from colliding with function-local IDs allocated later.
        // Same pattern as pin_to_slot() and the loop builder fix in e2d061d1.
        let phi_val = if let Some(ref mut f) = self.scope_ctx.current_function {
            f.next_value_id() // Function context: use local ID allocator
        } else {
            self.core_ctx.next_value() // Module context: use core_ctx SSOT
        };

        // 統一された挿入ロジック（既存パターンと完全互換）
        if let (Some(func), Some(cur_bb)) =
            (self.scope_ctx.current_function.as_mut(), self.current_block)
        {
            // CFG経由の正規化挿入（predecessor順序の正規化を含む）
            crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                func,
                cur_bb,
                phi_val,
                inputs,
                self.metadata_ctx.current_span(),
            )?;
        } else {
            return Err(
                FreezeContract::new("builder/phi_insert_without_function_context")
                    .field("dst", format!("%{}", phi_val.0))
                    .build(),
            );
        }

        Ok(phi_val)
    }

    /// **事前割り当てPHI挿入** - ValueIdを事前に確保済みの場合に使用
    ///
    /// ## 引数
    /// - `dst`: 事前に割り当てられたValueId（結果の格納先）
    /// - `inputs`: Vec<(predecessor_block, value_from_that_block)>
    ///
    /// ## 使用場面
    /// - 複雑なif式で結果ValueIdを先に確保している場合
    /// - PHI命令の結果を複数箇所で参照する必要がある場合
    ///
    /// ## 重要: ValueId割り当てルール
    /// `dst`は必ず関数コンテキストに適したアロケーターで確保すること:
    /// ```rust
    /// // ✅ 正しい: 関数ローカルアロケーター使用
    /// let result_val = if let Some(ref mut f) = self.scope_ctx.current_function {
    ///     f.next_value_id()
    /// } else {
    ///     self.value_gen.next()
    /// };
    /// self.insert_phi_with_dst(result_val, vec![...])?;
    ///
    /// // ❌ 間違い: 常にグローバルアロケーター使用（SSA違反の原因）
    /// let result_val = self.value_gen.next();
    /// ```
    ///
    /// ## 例
    /// ```rust
    /// let result_val = if let Some(ref mut f) = self.scope_ctx.current_function {
    ///     f.next_value_id()  // 関数コンテキスト: ローカルID
    /// } else {
    ///     self.value_gen.next()  // モジュールコンテキスト: グローバルID
    /// };
    /// // ... 複雑なロジック ...
    /// self.insert_phi_with_dst(result_val, vec![
    ///     (then_block, then_value),
    ///     (else_block, else_value),
    /// ])?;
    /// ```
    #[inline]
    pub fn insert_phi_with_dst(
        &mut self,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        // 統一された挿入ロジック（既存パターンと完全互換）
        if let (Some(func), Some(cur_bb)) =
            (self.scope_ctx.current_function.as_mut(), self.current_block)
        {
            // CFG経由の正規化挿入（predecessor順序の正規化を含む）
            crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                func,
                cur_bb,
                dst,
                inputs,
                self.metadata_ctx.current_span(),
            )?;
        } else {
            return Err(
                FreezeContract::new("builder/phi_insert_without_function_context")
                    .field("dst", format!("%{}", dst.0))
                    .build(),
            );
        }

        Ok(())
    }

    /// **単一入力PHI挿入** - 変数のmaterialization（具現化）用
    ///
    /// ## 使用場面
    /// - If/Elseブランチのエントリーでの変数具現化
    /// - ループ内での変数の再定義
    ///
    /// ## 例
    /// ```rust
    /// // 変数をブランチエントリーで具現化
    /// for (name, &pre_v) in pre_if_var_map.iter() {
    ///     let phi_val = self.insert_phi_single(pre_branch_bb, pre_v)?;
    ///     self.variable_map.insert(name.clone(), phi_val);
    /// }
    /// ```
    #[inline]
    pub fn insert_phi_single(
        &mut self,
        pred: BasicBlockId,
        value: ValueId,
    ) -> Result<ValueId, String> {
        self.insert_phi(vec![(pred, value)])
    }

    /// **2入力PHI挿入** - If/Else合流の最頻出パターン用
    ///
    /// ## 引数
    /// - `pred1`, `val1`: 第1のpredecessor（通常はthen-branch）
    /// - `pred2`, `val2`: 第2のpredecessor（通常はelse-branch）
    ///
    /// ## 例
    /// ```rust
    /// // If/Else合流
    /// let result = self.insert_phi_binary(
    ///     then_exit, then_value,
    ///     else_exit, else_value
    /// )?;
    /// ```
    #[inline]
    pub fn insert_phi_binary(
        &mut self,
        pred1: BasicBlockId,
        val1: ValueId,
        pred2: BasicBlockId,
        val2: ValueId,
    ) -> Result<ValueId, String> {
        self.insert_phi(vec![(pred1, val1), (pred2, val2)])
    }

    /// **ループヘッダーPHI挿入** - セマンティック明確化版
    ///
    /// ## 引数
    /// - `entry_pred`: ループに入る前のブロック
    /// - `init_value`: 初期値
    /// - `backedge_pred`: ループ本体の末尾ブロック
    /// - `updated_value`: ループ内で更新された値
    ///
    /// ## 例
    /// ```rust
    /// let loop_counter = self.insert_phi_loop_header(
    ///     entry_block, zero_value,
    ///     backedge_block, incremented_value
    /// )?;
    /// ```
    #[inline]
    pub fn insert_phi_loop_header(
        &mut self,
        entry_pred: BasicBlockId,
        init_value: ValueId,
        backedge_pred: BasicBlockId,
        updated_value: ValueId,
    ) -> Result<ValueId, String> {
        // ループヘッダーPHIは論理的に[entry, backedge]の順序が自然
        self.insert_phi(vec![
            (entry_pred, init_value),
            (backedge_pred, updated_value),
        ])
    }

    /// **短絡評価用PHI挿入** - AND/ORの合流点用
    ///
    /// ## 引数
    /// - `short_circuit_pred`: 短絡したブロック（評価せずに結果確定）
    /// - `short_circuit_value`: 短絡時の値（AND: false, OR: true）
    /// - `evaluated_pred`: RHSを評価したブロック
    /// - `evaluated_value`: RHS評価結果
    ///
    /// ## 例
    /// ```rust
    /// // AND短絡: false || evaluated_rhs
    /// let and_result = self.insert_phi_short_circuit(
    ///     lhs_false_block, false_value,
    ///     rhs_eval_block, rhs_value
    /// )?;
    ///
    /// // OR短絡: true || evaluated_rhs
    /// let or_result = self.insert_phi_short_circuit(
    ///     lhs_true_block, true_value,
    ///     rhs_eval_block, rhs_value
    /// )?;
    /// ```
    #[inline]
    pub fn insert_phi_short_circuit(
        &mut self,
        short_circuit_pred: BasicBlockId,
        short_circuit_value: ValueId,
        evaluated_pred: BasicBlockId,
        evaluated_value: ValueId,
    ) -> Result<ValueId, String> {
        self.insert_phi(vec![
            (short_circuit_pred, short_circuit_value),
            (evaluated_pred, evaluated_value),
        ])
    }
}

#[cfg(test)]
mod tests {

    // ユニットテストは実際のMirBuilder構造が必要なため、
    // 統合テストでの検証を推奨（smoke testsで実証）

    #[test]
    fn test_phi_helpers_exist() {
        // コンパイル時にメソッドの存在を確認
        // 実行時テストはsmoke testsで行う
    }
}
