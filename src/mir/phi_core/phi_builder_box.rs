//! PhiBuilderBox - PHI生成の単一責務箱（SSOT）
//!
//! # 箱理論の適用
//!
//! - **箱にする**: PHI生成ロジックを1箱に集約
//! - **境界を作る**: ControlFormで If/Loop を統一インターフェース化
//! - **Fail-Fast**: BodyLocal/Carrier/Pinned 分類を明示的に
//!
//! # アーキテクチャ
//!
//! ```text
//! PhiBuilderBox (SSOT)
//!    ↑
//!    ├─ ControlForm::If  → If PHI生成
//!    └─ ControlForm::Loop → Loop PHI生成
//!        ├─ ExitPhiBuilder (Phase 26-D完成)
//!        └─ HeaderPhiBuilder (Phase 26-C-2完成)
//! ```
//!
//! # Phase 36 Responsibility Classification
//!
//! ## Loop-only methods (if-in-loop PHI)
//! - (Phase 61-6.1: set_if_context削除、if_context直接代入に)
//!
//! ## If-only methods (Phase 37 target)
//! - `generate_if_phis()`: Generate If PHI nodes
//! - `compute_modified_names_if()`: Detect modified variables in If
//! - `get_conservative_if_values()`: Get conservative values for If PHI
//!
//! ## Common methods (stable API)
//! - `new()`: Create PhiBuilderBox instance
//! - `generate_phis()`: Unified PHI generation entry point (If + Loop)
//!
//! ## Loop PHI methods (stub, Phase 37+ implementation)
//! - `generate_loop_phis()`: Loop PHI generation (currently empty stub)
//!
//! ## Usage Pattern
//! - **If-in-loop PHI**: PhiBuilderBox is used for if statements inside loops (loop_builder.rs)
//! - **Pure If PHI**: PhiBuilderBox is used via facade function (phi_core/mod.rs)
//! - **Loop PHI**: Handled by LoopFormBuilder directly (NOT via PhiBuilderBox)
//!
//! # Phase 26-E: PhiBuilderBox統一化
//!
//! - **Phase 1**: 骨格作成（ControlForm対応インターフェース）
//! - **Phase 2**: If側移行（if_phi.rs統合）
//! - **Phase 3**: Loop側移行（loopform_builder.rs統合）
//! - **Phase 4**: Legacy削除（loop_phi.rs 287行削除）
//!
//! # 削減見込み
//!
//! - Phase 2: -80行（If側重複削除）
//! - Phase 3: -456行（Loop側重複削除）
//! - Phase 4: -287行（Legacy削除）
//! - **合計**: -623行（純削減）

use crate::mir::control_form::{ControlForm, ControlKind, IfShape, LoopShape};
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

/// PhiBuilderBox - ControlForm対応の統一PHI生成箱
///
/// # Responsibility
///
/// - If/Loop両対応のPHI生成エントリーポイント
/// - 既存Box群（ExitPhiBuilder, HeaderPhiBuilder等）のオーケストレーション
/// - ControlFormベースの統一インターフェース提供
///
/// # 箱理論
///
/// - **単一責務**: PHI生成のみ（CFG構築・変数管理は別箱）
/// - **状態最小**: Context保持は最小限（将来拡張用）
/// - **ピュア関数的**: 入力 → PHI生成 → 出力（副作用最小化）
pub struct PhiBuilderBox {
    /// If PHI生成時のコンテキスト（将来拡張用）
    /// Phase 61-6.1: pub化（set_if_context削除、直接代入可能に）
    pub if_context: Option<IfPhiContext>,
    // Phase 30: loop_context 削除（完全未使用、将来 JoinIR で代替予定）
}

/// If PHI生成コンテキスト（Phase 26-F-2: 箱理論による責務分離）
///
/// # Phase 26-F-3: ループ内if-merge対応（ChatGPT設計）
///
/// ## 責務分離の原則
/// - **If側（この箱）**: ループコンテキストを「名前セット」だけで受け取る
/// - **Loop側（LoopBuilder）**: LoopScopeShape から「キャリア変数名」を抽出
/// - **橋渡し**: このIfPhiContext経由で最小限の情報伝達
///
/// ## なぜこの設計？
/// - Phase 35-5: if_body_local_merge.rs 削除、ロジックを PhiBuilderBox に吸収
/// - ループ内かどうかの「シグナル」だけを受け取る
/// - 箱の境界を守りながら、ループ内if-breakパターンに対応
#[derive(Debug, Clone)]
pub struct IfPhiContext {
    /// ループbody内のif-mergeかどうか
    pub in_loop_body: bool,

    /// ループキャリア変数名（Pinned + Carrier）
    ///
    /// # 用途
    /// - ループ内if-mergeで「片腕のみの変数」もPHI候補にする判定
    /// - 例: `if i >= n { break }` でthen腕に`i`なし → でもcarrierなのでPHI必要
    ///
    /// # 決定箇所
    /// - LoopBuilder側でLoopScopeShapeから抽出
    /// - Pinned（ループ越しパラメータ） + Carrier（ループ修正変数）
    pub loop_carrier_names: std::collections::BTreeSet<String>,
}

// Phase 30: LoopPhiContext 削除（完全未使用、将来 JoinIR で代替予定）

impl PhiBuilderBox {
    /// 新しいPhiBuilderBoxを作成
    pub fn new() -> Self {
        Self { if_context: None }
    }

    // Phase 61-6.1: set_if_context() 削除（薄いラッパー）
    // 呼び出し側（loop_builder/if_lowering.rs）で直接 IfPhiContext を生成
    //
    // Before:
    //   phi_builder.set_if_context(true, carrier_names.clone());
    // After:
    //   phi_builder.if_context = Some(IfPhiContext {
    //       in_loop_body: true,
    //       loop_carrier_names: carrier_names.clone(),
    //   });

    // Phase 26-F-2: set_body_local_filter() 削除
    // Phase 35-5: if_body_local_merge.rs削除、ロジックをcompute_modified_names_if()内に直接実装

    /// ControlFormベースの統一PHI生成エントリーポイント
    ///
    /// # Responsibility: Common (If + Loop)
    ///
    /// This is the STABLE API for PHI generation across both If and Loop structures.
    /// Both If-side (phi_core/mod.rs) and Loop-side (loop_builder.rs) call this method.
    ///
    /// ## Phase 36 Note
    /// - Kept as direct call (no wrapper)
    /// - Internal delegation to generate_if_phis() or generate_loop_phis()
    ///
    /// # Arguments
    ///
    /// - `ops`: PHI生成操作インターフェース
    /// - `form`: 制御構造形式（If/Loop）
    /// - `pre_snapshot`: 制御構造前の変数スナップショット
    /// - `post_snapshots`: 各経路の終了時変数スナップショット
    ///
    /// # Returns
    ///
    /// - `Ok(())`: PHI生成成功
    /// - `Err(String)`: PHI生成失敗（詳細メッセージ）
    ///
    /// # 箱理論: Fail-Fast原則
    ///
    /// - フォールバックなし
    /// - エラーは即座に明示的に失敗
    /// - 不正な状態での継続を防ぐ
    pub fn generate_phis<O: PhiBuilderOps>(
        &mut self,
        ops: &mut O,
        form: &ControlForm,
        pre_snapshot: &BTreeMap<String, ValueId>,
        post_snapshots: &[BTreeMap<String, ValueId>],
    ) -> Result<(), String> {
        match &form.kind {
            ControlKind::If(if_shape) => {
                self.generate_if_phis(ops, if_shape, pre_snapshot, post_snapshots)
            }
            ControlKind::Loop(loop_shape) => {
                self.generate_loop_phis(ops, loop_shape, pre_snapshot, post_snapshots)
            }
        }
    }

    /// If PHI生成（Phase 2で実装）
    ///
    /// # Responsibility: If-only (Phase 37 target)
    ///
    /// Internal If PHI generation. Called from generate_phis() when ControlKind::If.
    ///
    /// ## Phase 36 Scope
    /// - No changes (If-side deferred to Phase 37)
    /// - Used by both pure If and if-in-loop cases
    ///
    /// ## Phase 37+ Refactor Candidates
    /// - compute_modified_names_if() internalization
    /// - Conservative strategy simplification
    /// - Void emission optimization
    ///
    /// # Phase 2実装
    ///
    /// - `if_phi.rs::merge_modified_at_merge_with` の機能を統合
    /// - Conservative戦略適用
    ///
    /// # アーキテクチャ
    ///
    /// ```text
    /// If PHI生成
    ///   ├─ compute_modified_names: 変更変数検出（決定的順序）
    ///   ├─ Conservative値取得: void emission含む
    ///   └─ PHI生成 or 直接バインド
    /// ```
    fn generate_if_phis<O: PhiBuilderOps>(
        &mut self,
        ops: &mut O,
        if_shape: &IfShape,
        pre_snapshot: &BTreeMap<String, ValueId>,
        post_snapshots: &[BTreeMap<String, ValueId>],
    ) -> Result<(), String> {
        // Phase 2実装: If PHI生成

        // post_snapshots validation
        if post_snapshots.is_empty() {
            return Err("If PHI: post_snapshots is empty".to_string());
        }

        let then_end = &post_snapshots[0];
        let else_end_opt = if post_snapshots.len() > 1 {
            Some(&post_snapshots[1])
        } else {
            None
        };

        let merge_bb = if_shape.merge_block;

        // Trace if enabled
        let trace = std::env::var("NYASH_IF_TRACE").ok().as_deref() == Some("1");
        if trace {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[PhiBuilderBox/if] merge_bb={:?} then={:?} else={:?}",
                merge_bb, if_shape.then_block, if_shape.else_block
            ));
        }

        // Compute modified variables (決定的順序: BTreeSet使用)
        let modified_vars =
            self.compute_modified_names_if(pre_snapshot, then_end, &else_end_opt, if_shape);

        for var_name in modified_vars {
            // Conservative strategy: get values with void fallback
            let (then_v, else_v) = self.get_conservative_if_values(
                &var_name,
                pre_snapshot,
                then_end,
                &else_end_opt,
                if_shape,
                ops,
            )?;

            if trace {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[PhiBuilderBox/if] var={} then_v={:?} else_v={:?}",
                    var_name, then_v, else_v
                ));
            }

            // If values are identical, direct bind (no PHI needed)
            if then_v == else_v {
                ops.update_var(var_name, then_v);
                continue;
            }

            // Generate PHI
            let phi_dst = ops.new_value();

            // Collect predecessors (決定的順序)
            let mut inputs = Vec::new();
            if let Some(then_pred) = if_shape.then_block.into() {
                inputs.push((then_pred, then_v));
            }
            if let Some(else_block) = if_shape.else_block {
                inputs.push((else_block, else_v));
            }

            // Sort inputs for determinism
            inputs.sort_by_key(|(bb, _)| bb.0);

            ops.emit_phi(merge_bb, phi_dst, inputs)?;
            ops.update_var(var_name, phi_dst);
        }

        Ok(())
    }

    /// Compute modified variable names for If
    ///
    /// # Arguments
    /// * `pre_snapshot` - if直前の変数スナップショット
    /// * `then_end` - thenブランチ終端の変数スナップショット
    /// * `else_end_opt` - elseブランチ終端の変数スナップショット（なければNone）
    /// * `if_shape` - IfShape（reachable_preds取得用）
    ///
    /// # Returns
    ///
    /// ソート済みの変更変数名リスト（決定的順序: BTreeSet使用）
    ///
    /// # Phase 35-5: if_body_local_merge.rs を PhiBuilderBox に吸収
    /// 元 IfBodyLocalMergeBox::compute_if_merge_phi_candidates のロジックを直接実装
    ///
    /// # Logic (2モード対応)
    ///
    /// ## 通常if（ループ外、in_loop_body=false）
    /// 1. 両腕に存在する変数を抽出（intersection）
    /// 2. pre_if と比較して値が変わっているものだけを候補にする
    ///
    /// ## ループ内if-merge（in_loop_body=true）
    /// 1. 両腕に存在する変数（既存ロジック）
    /// 2. **loop_carrier_names に含まれていて、片腕にでも存在する変数**
    /// 3. pre_if と比較して値が変わっているものだけを候補にする
    fn compute_modified_names_if(
        &self,
        pre_snapshot: &BTreeMap<String, ValueId>,
        then_end: &BTreeMap<String, ValueId>,
        else_end_opt: &Option<&BTreeMap<String, ValueId>>,
        _if_shape: &IfShape,
    ) -> Vec<String> {
        use std::collections::BTreeSet;

        // else_end_optをOption<BTreeMap>に変換
        let else_end_owned = else_end_opt.map(|m| m.clone());

        // empty else の場合: 何も絞らない（phi_builderに任せる）
        let Some(else_end) = else_end_owned.as_ref() else {
            return Vec::new();
        };

        // Phase 26-F-3: IfPhiContextを取得（デフォルト: ループ外）
        let if_context = self
            .if_context
            .as_ref()
            .cloned()
            .unwrap_or_else(|| IfPhiContext {
                in_loop_body: false,
                loop_carrier_names: std::collections::BTreeSet::new(),
            });

        // 1. 両腕に存在する変数名を収集（決定的順序）
        let then_names: BTreeSet<&String> = then_end.keys().collect();
        let else_names: BTreeSet<&String> = else_end.keys().collect();
        let common_names: BTreeSet<&String> =
            then_names.intersection(&else_names).copied().collect();

        // Phase 26-F-3: ループ内モードで片腕のみのcarrier変数も追加
        let mut candidate_names_set = common_names.clone();

        if if_context.in_loop_body {
            // ループキャリア変数で、片腕にでも存在するものを追加
            for carrier_name in &if_context.loop_carrier_names {
                let in_then = then_names.contains(carrier_name);
                let in_else = else_names.contains(carrier_name);

                // 少なくとも片腕に存在すればPHI候補に
                if in_then || in_else {
                    candidate_names_set.insert(carrier_name);
                }
            }
        }

        // 2. pre_if と比較して値が変わっているものだけを候補にする
        let mut candidates = Vec::new();
        for &name in &candidate_names_set {
            let pre_val = pre_snapshot.get(name);
            let then_val = then_end.get(name);
            let else_val = else_end.get(name);

            // 値が変わっているかチェック
            let changed_in_then = then_val != pre_val;
            let changed_in_else = else_val != pre_val;

            if changed_in_then || changed_in_else {
                candidates.push(name.clone());
            }
        }

        // Debug trace
        if std::env::var("NYASH_IF_TRACE").ok().as_deref() == Some("1") {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[PhiBuilderBox/if] if_merge logic applied, {} candidates (in_loop={})",
                candidates.len(),
                if_context.in_loop_body
            ));
        }

        candidates
    }

    /// Conservative strategy: Get if values with branch-local void fallback
    ///
    /// # Conservative Rules
    ///
    /// 1. Both defined: use both values
    /// 2. Only then: use then + void
    /// 3. Only else: use void + else
    /// 4. Neither: use pre (fallback to predecessor)
    fn get_conservative_if_values<O: PhiBuilderOps>(
        &self,
        var_name: &str,
        pre_snapshot: &BTreeMap<String, ValueId>,
        then_end: &BTreeMap<String, ValueId>,
        else_end_opt: &Option<&BTreeMap<String, ValueId>>,
        if_shape: &IfShape,
        ops: &mut O,
    ) -> Result<(ValueId, ValueId), String> {
        // Phase 35-5: phi_invariants.rs deleted, Fail-Fast check removed
        // Rationale: JoinIR Verifier (verify_select_minimal) now handles invariant checks
        // for new JoinIR path. Legacy MIR Builder path continues with conservative fallback.

        let pre_val = pre_snapshot.get(var_name).copied();

        // Fallback to predecessor value if not defined in a branch
        let then_v_opt = then_end.get(var_name).copied().or(pre_val);
        let else_v_opt = else_end_opt
            .and_then(|m| m.get(var_name).copied())
            .or(pre_val);

        let mut restore_block: Option<BasicBlockId> = None;
        match (then_v_opt, else_v_opt) {
            (Some(tv), Some(ev)) => Ok((tv, ev)),
            (Some(tv), None) => {
                // Only then: emit void for else (elseブロックに紐づく値として配置)
                if std::env::var("NYASH_IF_HOLE_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[PhiBuilderBox/if] void fallback (else missing) var={} then_bb={:?} else_bb={:?} merge_bb={:?}",
                        var_name,
                        if_shape.then_block,
                        if_shape.else_block,
                        if_shape.merge_block
                    ));
                }
                if let Some(else_bb) = if_shape.else_block {
                    restore_block = Some(if_shape.merge_block);
                    ops.set_current_block(else_bb)?;
                }
                let void_val = ops.emit_void()?;
                if let Some(merge_bb) = restore_block.take() {
                    ops.set_current_block(merge_bb)?;
                }
                Ok((tv, void_val))
            }
            (None, Some(ev)) => {
                // Only else: emit void for then（thenブロックに紐づく値として配置）
                if std::env::var("NYASH_IF_HOLE_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[PhiBuilderBox/if] void fallback (then missing) var={} then_bb={:?} else_bb={:?} merge_bb={:?}",
                        var_name,
                        if_shape.then_block,
                        if_shape.else_block,
                        if_shape.merge_block
                    ));
                }
                restore_block = Some(if_shape.merge_block);
                ops.set_current_block(if_shape.then_block)?;
                let void_val = ops.emit_void()?;
                if let Some(merge_bb) = restore_block.take() {
                    ops.set_current_block(merge_bb)?;
                }
                Ok((void_val, ev))
            }
            (None, None) => unreachable!(
                "If PHI invariant violated (Phase 35-5: check moved to JoinIR Verifier)"
            ),
        }
    }

    /// Loop PHI生成（Phase 3で実装）
    ///
    /// # Responsibility: Loop-only (Phase 37+ implementation target)
    ///
    /// ## Current State (Phase 36)
    /// - Empty stub (returns Err immediately)
    /// - Loop PHI is handled by LoopFormBuilder directly
    ///
    /// ## Phase 37+ Implementation Plan
    /// - Migrate LoopFormBuilder::seal_phis() logic here (~90 lines)
    /// - Migrate LoopFormBuilder::build_exit_phis() logic here (~210 lines)
    /// - Unify Loop PHI generation under PhiBuilderBox
    ///
    /// # Phase 3-A 実装状況
    ///
    /// **設計上の課題**:
    /// - PhiBuilderOps (7メソッド) vs LoopFormOps (12メソッド) の違い
    /// - If PHI生成とLoop PHI生成で必要な操作が大きく異なる
    /// - ExitPhiBuilder/HeaderPhiBuilderはLoopFormOpsを要求
    ///
    /// **Pragmatic Solution (Phase 3-A):**
    /// - このメソッドは未使用（generate_phis経由の統一呼び出しは実装せず）
    /// - 代わりに、loop_builder.rsから直接ExitPhiBuilder/HeaderPhiBuilderを使用
    /// - Phase 4で統一インターフェース設計を見直し予定
    ///
    /// # アーキテクチャ（Phase 4目標）
    ///
    /// ```text
    /// Loop PHI生成
    ///   ├─ Header PHI: HeaderPhiBuilder使用
    ///   │   ├─ Pinned変数のPHI
    ///   │   └─ Carrier変数のPHI
    ///   ├─ Exit PHI: ExitPhiBuilder使用
    ///   │   ├─ BodyLocalPhiBuilder（要否判定）
    ///   │   ├─ LoopScopeShape（変数分類）
    ///   │   └─ LocalScopeInspectorBox（定義追跡）
    ///   └─ Seal: PhiInputCollector使用
    /// ```
    fn generate_loop_phis<O: PhiBuilderOps>(
        &mut self,
        _ops: &mut O,
        _loop_shape: &LoopShape,
        _pre_snapshot: &BTreeMap<String, ValueId>,
        _post_snapshots: &[BTreeMap<String, ValueId>],
    ) -> Result<(), String> {
        // Phase 3-A: PhiBuilderOps vs LoopFormOps の設計上の制約により未実装
        // loop_builder.rsから直接ExitPhiBuilder/HeaderPhiBuilderを使用
        // Phase 4で統一インターフェース設計を再検討予定
        Err("Loop PHI generation requires LoopFormOps, not PhiBuilderOps. Use ExitPhiBuilder/HeaderPhiBuilder directly.".to_string())
    }
}

/// PhiBuilderOps - PHI生成操作の抽象化インターフェース
///
/// # 箱理論: 境界を作る
///
/// - MIR操作とPHI生成ロジックを分離
/// - テスタビリティ向上（モック可能）
/// - 実装の詳細を隠蔽
pub trait PhiBuilderOps {
    /// 新しいValueIdを生成
    fn new_value(&mut self) -> ValueId;

    /// PHI命令を発行
    ///
    /// # Arguments
    ///
    /// - `block`: PHIを配置するブロック
    /// - `dst`: PHI結果の格納先ValueId
    /// - `inputs`: 各先行ブロックからの入力 `[(pred_bb, value)]`
    fn emit_phi(
        &mut self,
        block: BasicBlockId,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String>;

    /// 変数バインディングを更新
    ///
    /// # Arguments
    ///
    /// - `name`: 変数名
    /// - `value`: 新しい値
    fn update_var(&mut self, name: String, value: ValueId);

    /// ブロックの先行ブロックを取得
    ///
    /// # Arguments
    ///
    /// - `block`: 対象ブロック
    ///
    /// # Returns
    ///
    /// 先行ブロックのリスト（決定的順序：ソート済み）
    fn get_block_predecessors(&self, block: BasicBlockId) -> Vec<BasicBlockId>;

    /// void定数を発行（Conservative戦略用）
    ///
    /// # Returns
    ///
    /// void定数のValueId
    fn emit_void(&mut self) -> Result<ValueId, String>;

    // Phase 3-A: Loop PHI生成用メソッド追加

    /// 現在のブロックを設定（Loop PHI生成時に使用）
    ///
    /// # Arguments
    ///
    /// - `block`: 設定するブロックID
    fn set_current_block(&mut self, block: BasicBlockId) -> Result<(), String>;

    /// ブロックが存在するか確認（Phantom block判定用）
    ///
    /// # Arguments
    ///
    /// - `block`: 確認するブロックID
    ///
    /// # Returns
    ///
    /// true: ブロックが存在, false: 存在しない（Phantom）
    fn block_exists(&self, block: BasicBlockId) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// モックPhiBuilderOps（テスト用）
    struct MockOps {
        value_counter: u32,
    }

    impl MockOps {
        fn new() -> Self {
            Self { value_counter: 0 }
        }
    }

    impl PhiBuilderOps for MockOps {
        fn new_value(&mut self) -> ValueId {
            let v = ValueId::new(self.value_counter);
            self.value_counter += 1;
            v
        }

        fn emit_phi(
            &mut self,
            _block: BasicBlockId,
            _dst: ValueId,
            _inputs: Vec<(BasicBlockId, ValueId)>,
        ) -> Result<(), String> {
            Ok(())
        }

        fn update_var(&mut self, _name: String, _value: ValueId) {}

        fn get_block_predecessors(&self, _block: BasicBlockId) -> Vec<BasicBlockId> {
            vec![]
        }

        fn emit_void(&mut self) -> Result<ValueId, String> {
            Ok(ValueId::new(999))
        }

        fn set_current_block(&mut self, _block: BasicBlockId) -> Result<(), String> {
            Ok(())
        }

        fn block_exists(&self, _block: BasicBlockId) -> bool {
            true
        }
    }

    #[test]
    fn test_phi_builder_box_creation() {
        let builder = PhiBuilderBox::new();
        assert!(builder.if_context.is_none());
        // Phase 30: loop_context 削除済み
    }

    #[test]
    fn test_mock_ops_value_generation() {
        let mut ops = MockOps::new();
        let v1 = ops.new_value();
        let v2 = ops.new_value();
        assert_eq!(v1, ValueId::new(0));
        assert_eq!(v2, ValueId::new(1));
    }

    // Phase 2/3でテスト追加予定
    // - test_generate_if_phis()
    // - test_generate_loop_phis()
}
