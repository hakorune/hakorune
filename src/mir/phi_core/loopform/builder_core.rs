/*!
 * loopform::builder_core – Core LoopFormBuilder orchestration
 *
 * This module contains the main LoopFormBuilder struct and coordinates
 * the 4-pass PHI generation pipeline.
 *
 * The builder manages:
 * - Carrier and pinned variable collections
 * - Preheader and header block IDs
 * - Coordination between the 4 passes
 */

use crate::mir::phi_core::loop_snapshot_merge::LoopSnapshotMergeBox;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

use super::variable_models::{CarrierVariable, PinnedVariable};

/// LoopForm Meta-Box: Structured representation of loop SSA construction
///
/// Separates loop-visible variables into classes（25.1e/25.2 スコープモデル）:
/// - Carriers: Modified in loop body, need header/exit PHI nodes.
/// - Pinned: Loop-invariant parameters/receivers, need PHI so every header/exit
///   edge has a well-defined value but the logical value never changes.
/// - Invariants: Not tracked here; they keep the preheader ValueId and never
///   participate in PHI construction.
/// - Body-local live-out (BodyLocalInOut): Not stored as dedicated structs, but
///   detected at exit-phi time in `build_exit_phis` and merged via
///   `LoopSnapshotMergeBox::merge_exit`.
///
/// Key idea: All ValueIds for Carriers/Pinned are allocated upfront before any
/// MIR emission, eliminating circular dependency issues in SSA.
#[derive(Debug)]
pub struct LoopFormBuilder {
    pub carriers: Vec<CarrierVariable>,
    pub pinned: Vec<PinnedVariable>,
    pub preheader_id: BasicBlockId,
    pub header_id: BasicBlockId,
    /// Step 5-2: Preheader snapshot for ValueId comparison (選択肢3)
    /// Used in seal_phis() to detect which variables are truly modified vs. invariant
    pub preheader_vars: BTreeMap<String, ValueId>,
}

impl LoopFormBuilder {
    /// Create a new LoopForm builder with specified block IDs
    pub fn new(preheader_id: BasicBlockId, header_id: BasicBlockId) -> Self {
        Self {
            carriers: Vec::new(),
            pinned: Vec::new(),
            preheader_id,
            header_id,
            preheader_vars: BTreeMap::new(), // Will be set in prepare_structure()
        }
    }

    /// Pass 1: Allocate all ValueIds for loop structure
    ///
    /// This is the critical innovation: we allocate ALL ValueIds
    /// (preheader copies and header PHIs) BEFORE emitting any instructions.
    /// This guarantees definition-before-use in SSA form.
    pub fn prepare_structure<O: LoopFormOps>(
        &mut self,
        ops: &mut O,
        current_vars: &BTreeMap<String, ValueId>,
    ) -> Result<(), String> {
        super::passes::pass1_discovery::prepare_structure(self, ops, current_vars)
    }

    /// Pass 2: Emit preheader block instructions
    ///
    /// Emits copy instructions for ALL variables in deterministic order:
    /// 1. Pinned variables first
    /// 2. Carrier variables second
    ///
    /// This ordering ensures consistent ValueId allocation across runs.
    pub fn emit_preheader<O: LoopFormOps>(&self, ops: &mut O) -> Result<(), String> {
        super::passes::pass2_preheader::emit_preheader(self, ops)
    }

    /// Pass 3: Emit header block PHI nodes (incomplete)
    ///
    /// Creates incomplete PHI nodes with only preheader input.
    /// These will be completed in seal_phis() after loop body is lowered.
    pub fn emit_header_phis<O: LoopFormOps>(&mut self, ops: &mut O) -> Result<(), String> {
        super::passes::pass3_header_phi::emit_header_phis(self, ops)
    }

    /// Pass 4: Complete PHI nodes with latch values
    ///
    /// Seals both pinned and carrier PHI nodes by finding latch values
    /// and updating PHI inputs.
    pub fn seal_phis<O: LoopFormOps>(
        &mut self,
        ops: &mut O,
        latch_id: BasicBlockId,
        continue_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
        _writes: &std::collections::HashSet<String>, // Step 5-1/5-2: Reserved for future optimization
        header_bypass: bool,                         // Phase 27.4C: Header φ バイパスフラグ
    ) -> Result<(), String> {
        super::passes::pass4_seal::seal_phis(
            self,
            ops,
            latch_id,
            continue_snapshots,
            _writes,
            header_bypass,
        )
    }

    /// Build exit PHIs for break/continue merge points
    ///
    /// Similar to header PHIs, but merges:
    /// - Header fallthrough (normal loop exit)
    /// - Break snapshots (early exit from loop body)
    ///
    /// # Parameters
    /// - `branch_source_block`: The ACTUAL block that emitted the branch to exit
    ///   (might differ from header_id if condition evaluation created new blocks)
    /// Option C: Build exit PHIs with variable classification
    ///
    /// ## 引数
    ///
    /// - `inspector`: LocalScopeInspectorBox（変数定義位置追跡）
    ///   - 各ブロックでどの変数が定義されているか追跡
    ///   - BodyLocalInternal 変数（全exit predsで定義されていない）を検出
    ///   - これらの変数は exit PHI を生成しない（PHI pred mismatch防止）
    pub fn build_exit_phis<O: LoopFormOps>(
        &self,
        ops: &mut O,
        exit_id: BasicBlockId,
        branch_source_block: BasicBlockId,
        exit_snapshots: &[(BasicBlockId, BTreeMap<String, ValueId>)],
    ) -> Result<(), String> {
        ops.set_current_block(exit_id)?;

        let debug = std::env::var("NYASH_LOOPFORM_DEBUG").is_ok();
        if debug {
            crate::runtime::get_global_ring0()
                .log
                .debug("[DEBUG/exit_phi] ====== Exit PHI Generation ======");
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[DEBUG/exit_phi] exit_id = {:?}, header_id = {:?}, branch_source = {:?}",
                exit_id, self.header_id, branch_source_block
            ));
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[DEBUG/exit_phi] exit_snapshots.len() = {}",
                exit_snapshots.len()
            ));
            for (i, (bb, snap)) in exit_snapshots.iter().enumerate() {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[DEBUG/exit_phi]   snapshot[{}]: block = {:?}, num_vars = {}",
                    i,
                    bb,
                    snap.len()
                ));
            }
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[DEBUG/exit_phi] pinned.len() = {}, carriers.len() = {}",
                self.pinned.len(),
                self.carriers.len()
            ));
        }

        // Phase 25.2: LoopSnapshotMergeBox を使って exit PHI 統合

        // 1. header_vals を準備（pinned + carriers）
        let mut header_vals = BTreeMap::new();
        for pinned in &self.pinned {
            header_vals.insert(pinned.name.clone(), pinned.header_phi);
        }
        for carrier in &self.carriers {
            header_vals.insert(carrier.name.clone(), carrier.header_phi);
        }

        // 2. body_local_vars を収集（決定的順序のためBTreeSet使用）
        let mut body_local_names = Vec::new();
        let mut body_local_set: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        for (_block_id, snapshot) in exit_snapshots {
            // 決定的順序のため、keysをソートしてからイテレート
            let mut sorted_keys: Vec<_> = snapshot.keys().collect();
            sorted_keys.sort();
            for var_name in sorted_keys {
                // Step 5-5-D: Skip __pin$ temporary variables in exit PHI generation
                // These are BodyLocalInternal and should NOT get exit PHIs
                if var_name.starts_with("__pin$") && var_name.contains("$@") {
                    if debug {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[DEBUG/exit_phi] SKIP __pin$ variable: {}",
                            var_name
                        ));
                    }
                    continue;
                }

                let is_pinned = self.pinned.iter().any(|p| &p.name == var_name);
                let is_carrier = self.carriers.iter().any(|c| &c.name == var_name);
                if !is_pinned && !is_carrier && !body_local_set.contains(var_name) {
                    body_local_names.push(var_name.clone());
                    body_local_set.insert(var_name.clone());
                }
            }
        }

        if debug && !body_local_names.is_empty() {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[DEBUG/exit_phi] Found {} body-local variables",
                body_local_names.len()
            ));
        }

        // Option C: Body-local variables should NOT be added to header_vals!
        // They are defined inside the loop body, not in the header.
        // The inspector will correctly track which exit preds have them.

        // 📦 Hotfix 6: Filter exit_snapshots to only include valid CFG predecessors
        let exit_preds = ops.get_block_predecessors(exit_id);
        if debug {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[DEBUG/exit_phi] Exit block predecessors: {:?}",
                exit_preds
            ));
        }

        let mut filtered_snapshots = Vec::new();
        for (block_id, snapshot) in exit_snapshots {
            if !ops.block_exists(*block_id) {
                if debug {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[DEBUG/exit_phi] ⚠️ Skipping non-existent block {:?}",
                        block_id
                    ));
                }
                continue;
            }
            if !exit_preds.contains(block_id) {
                if debug {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[DEBUG/exit_phi] ⚠️ Skipping block {:?} (not in CFG predecessors)",
                        block_id
                    ));
                }
                continue;
            }
            filtered_snapshots.push((*block_id, snapshot.clone()));
        }

        // 3. Option C: Record snapshots in inspector for availability checking
        // Phase 69-2: LocalScopeInspectorBox 削除完了
        // variable_definitions は LoopScopeShape に移行済み（Phase 48-4）
        // inspector.record_*() 呼び出しは不要（LoopScopeShape 内部で処理）

        // 4. Option C: merge_exit_with_classification() でPHI pred mismatch防止
        // pinned/carrier名リストを準備
        let pinned_names: Vec<String> = self.pinned.iter().map(|p| p.name.clone()).collect();
        let carrier_names: Vec<String> = self.carriers.iter().map(|c| c.name.clone()).collect();

        // exit_preds を Vec に変換
        let exit_preds_vec: Vec<BasicBlockId> = exit_preds.iter().copied().collect();

        // Phase 69-2: inspector 引数を削除（LoopScopeShape に移行済み）
        let all_vars = LoopSnapshotMergeBox::merge_exit_with_classification(
            branch_source_block,
            &header_vals,
            &filtered_snapshots,
            &exit_preds_vec, // ← 実際のCFG predecessorsを渡す
            &pinned_names,
            &carrier_names,
        )?;

        // ========================================
        // Phase 59: PHI生成（PhiInputCollectorインライン化）
        // ========================================
        // 旧: PhiInputCollector::new() + add_snapshot + sanitize + optimize_same_value
        // 新: BTreeMapで直接処理（同等のロジックをインライン展開）
        for (var_name, inputs) in all_vars {
            // Step 1: sanitize - BTreeMapで重複削除＆ソート
            let mut sanitized: std::collections::BTreeMap<BasicBlockId, ValueId> =
                std::collections::BTreeMap::new();
            for (bb, val) in &inputs {
                sanitized.insert(*bb, *val);
            }
            let final_inputs: Vec<(BasicBlockId, ValueId)> = sanitized.into_iter().collect();

            // Dev-only (strict/dev + planner_required): fail-fast if an exit-phi input does not
            // match the value available at the predecessor block.
            //
            // This catches "Phi input ValueId exists in snapshot but is not actually defined/available
            // on that predecessor", which otherwise surfaces later as VM reg_load undefined.
            if crate::config::env::stageb_dev_verify_enabled()
                && crate::config::env::joinir_dev::strict_enabled()
                && crate::config::env::joinir_dev::planner_required_enabled()
            {
                for (pred_bb, incoming) in &final_inputs {
                    let observed = ops.get_variable_at_block(&var_name, *pred_bb);
                    if observed != Some(*incoming) {
                        if crate::config::env::joinir_dev::debug_enabled() {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[loopform/exit_phi:undefined_input] fn={} exit={:?} var={} pred={:?} incoming={:?} observed={:?}",
                                ops.mir_function().signature.name,
                                exit_id,
                                var_name,
                                pred_bb,
                                incoming,
                                observed
                            ));
                        }
                        return Err(format!(
                            "[freeze:contract][loopform/exit_phi:undefined_input] fn={} exit={:?} var={} pred={:?} incoming={:?} observed={:?}",
                            ops.mir_function().signature.name,
                            exit_id,
                            var_name,
                            pred_bb,
                            incoming,
                            observed
                        ));
                    }
                }
            }

            if debug {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[DEBUG/exit_phi] Variable '{}': {} inputs",
                    var_name,
                    final_inputs.len()
                ));
            }

            // Step 2: optimize_same_value - 全て同じ値ならPHI不要
            let same_value = if final_inputs.is_empty() {
                None
            } else if final_inputs.len() == 1 {
                Some(final_inputs[0].1)
            } else {
                let first_val = final_inputs[0].1;
                if final_inputs.iter().all(|(_, val)| *val == first_val) {
                    Some(first_val)
                } else {
                    None
                }
            };

            // Step 3: PHI生成 or 直接バインド
            if let Some(same_val) = same_value {
                // 全て同じ値 or 単一入力 → PHI 不要
                if debug {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[DEBUG/exit_phi] Variable '{}': single/same value, direct binding to {:?}",
                        var_name, same_val
                    ));
                }
                ops.update_var(var_name, same_val);
            } else {
                // 異なる値を持つ場合は PHI ノードを生成
                let phi_id = ops.new_value();
                if debug {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[DEBUG/exit_phi] Creating PHI {:?} for var '{}' with {} inputs",
                        phi_id,
                        var_name,
                        final_inputs.len()
                    ));
                    for (bb, val) in &final_inputs {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[DEBUG/exit_phi]   PHI input: pred={:?} val={:?}",
                            bb, val
                        ));
                    }
                }
                ops.emit_phi(phi_id, final_inputs)?;
                ops.update_var(var_name, phi_id);
            }
        }

        Ok(())
    }
}

/// Operations required by LoopFormBuilder
///
/// This trait abstracts the underlying MIR builder operations,
/// allowing LoopFormBuilder to work with both Rust MIR builder
/// and selfhost compiler's JSON-based approach.
/// Phase 26-E-3: LoopFormOps と PhiBuilderOps の関係（委譲設計）
///
/// **Design Rationale (ChatGPT + Claude consensus, 2025-11-22):**
/// - PhiBuilderOps = 低レベル「PHI命令発行」道具箱
/// - LoopFormOps = 高レベル「ループ構造構築」作業場
/// - 関係: **has-a（委譲）** ではなく is-a（継承）
///
/// **実装方針:**
/// - LoopFormOps trait: そのまま（継承なし）
/// - PhiBuilderOps 実装: 必要な型に個別実装
///   - `impl<'a> PhiBuilderOps for LoopBuilder<'a>` で委譲
///   - HashSet → Vec 変換 + ソートで決定性保証
///
/// **段階的統合:**
/// 1. If側: PhiBuilderBox 経由（既に完了）
/// 2. Loop側: LoopFormOps + ExitPhiBuilder/HeaderPhiBuilder（現状維持）
/// 3. 将来: 必要に応じて LoopBuilder に PhiBuilderOps 実装追加
pub trait LoopFormOps {
    /// Allocate a new ValueId
    fn new_value(&mut self) -> ValueId;

    /// Ensure MirFunction counter is after the given ValueId to prevent collisions
    /// CRITICAL: Must be called before allocating new ValueIds in LoopForm
    fn ensure_counter_after(&mut self, max_id: u32) -> Result<(), String>;

    /// 📦 Check if a block exists in the CFG (Hotfix 2: Exit PHI predecessor validation)
    /// Used to skip non-existent blocks when building exit PHIs.
    fn block_exists(&self, block: BasicBlockId) -> bool;

    /// 📦 Get actual CFG predecessors for a block (Hotfix 6: PHI input validation)
    /// Returns the set of blocks that actually branch to this block in the CFG.
    /// Used to validate exit PHI inputs against actual control flow.
    /// Phase 69-3: Changed to BTreeSet for determinism
    fn get_block_predecessors(
        &self,
        block: BasicBlockId,
    ) -> std::collections::BTreeSet<BasicBlockId>;

    /// Phase 26-A-4: ValueIdベースのパラメータ判定（型安全化）
    ///
    /// 旧実装（名前ベース）から新実装（ValueIdベース）に変更。
    /// MirValueKindによる型安全判定で、GUARDバグを根絶。
    fn is_parameter(&self, value_id: ValueId) -> bool;

    /// Set current block for instruction emission
    fn set_current_block(&mut self, block: BasicBlockId) -> Result<(), String>;

    /// Emit a copy instruction: dst = src
    fn emit_copy(&mut self, dst: ValueId, src: ValueId) -> Result<(), String>;

    /// Emit a jump instruction to target block
    fn emit_jump(&mut self, target: BasicBlockId) -> Result<(), String>;

    /// Emit a PHI node with given inputs
    fn emit_phi(
        &mut self,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String>;

    /// Update PHI node inputs (for sealing incomplete PHIs)
    fn update_phi_inputs(
        &mut self,
        block: BasicBlockId,
        phi_id: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String>;

    /// Update variable binding in current scope
    fn update_var(&mut self, name: String, value: ValueId);

    /// Get variable value at specific block
    fn get_variable_at_block(&self, name: &str, block: BasicBlockId) -> Option<ValueId>;

    /// Access underlying MirFunction (for liveness/MirQuery)
    fn mir_function(&self) -> &crate::mir::MirFunction;
}
