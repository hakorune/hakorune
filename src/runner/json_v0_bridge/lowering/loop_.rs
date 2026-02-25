/*!
 * JSON v0 Loop Lowering Front
 *
 * Phase 25.1q 方針:
 * - ループ構造 / PHI / スナップショットの意味論は
 *   `phi_core::loopform_builder::LoopFormBuilder` +
 *   `phi_core::loop_snapshot_merge::LoopSnapshotMergeBox`
 *   側に SSOT として集約する。
 * - このファイルは「JSON v0 → LoopForm v2」への薄いフロントとし、
 *   ループ意味論や PHI 生成ロジックをここで新規実装しない。
 *
 * 設計ガード:
 * - ループのブロック構造・PHI・スナップショットのマージ方針を
 *   変更したい場合は、必ず `loopform_builder.rs` /
 *   `loop_snapshot_merge.rs` を修正すること。
 * - `loop_.rs` 側では:
 *     - ブロック ID 準備
 *     - 変数マップ / snapshot の受け渡し
 *     - LoopFormOps 実装を通じた呼び出し
 *   だけを行う。
 */

use super::super::ast::ExprV0;
use super::super::ast::StmtV0;
use super::{lower_stmt_list_with_vars, new_block, BridgeEnv, LoopContext};
use crate::ast::Span;
use crate::config::env;
use crate::mir::builder::copy_emitter::{self, CopyEmitReason};
use crate::mir::phi_core::loopform::{LoopFormBuilder, LoopFormOps};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use crate::runtime::get_global_ring0;
use std::collections::BTreeMap;

/// LoopForm v2 用の JSON bridge 実装。
///
/// - MirFunction 上で直接動作し、LoopFormBuilder が要求する最小限の
///   インターフェースだけを提供する。
/// - 「値の割り当て」「PHI の挿入」「変数マップの更新」「スナップショット参照」
///   だけを担当し、ループ意味論そのものは持たない。
struct LoopFormJsonOps<'a> {
    f: &'a mut MirFunction,
    vars: &'a mut BTreeMap<String, ValueId>,
    block_var_maps: &'a mut BTreeMap<BasicBlockId, BTreeMap<String, ValueId>>,
    current_block: BasicBlockId,
}

impl<'a> LoopFormJsonOps<'a> {
    fn new(
        f: &'a mut MirFunction,
        vars: &'a mut BTreeMap<String, ValueId>,
        block_var_maps: &'a mut BTreeMap<BasicBlockId, BTreeMap<String, ValueId>>,
        current_block: BasicBlockId,
    ) -> Self {
        Self {
            f,
            vars,
            block_var_maps,
            current_block,
        }
    }
}

impl LoopFormOps for LoopFormJsonOps<'_> {
    fn new_value(&mut self) -> ValueId {
        // MirFunction の next_value_id を直接使う（LoopBuilder と同じポリシー）
        self.f.next_value_id()
    }

    fn ensure_counter_after(&mut self, max_id: u32) -> Result<(), String> {
        // FunctionDefBuilder / main wrapper が params を signature に反映している前提で、
        // パラメータ数と既存 ValueId を両方考慮して next_value_id を前に進める。
        let param_count = self.f.signature.params.len() as u32;
        let min_counter = param_count.max(max_id + 1);
        if self.f.next_value_id < min_counter {
            self.f.next_value_id = min_counter;
        }
        Ok(())
    }

    fn block_exists(&self, block: BasicBlockId) -> bool {
        self.f.blocks.contains_key(&block)
    }

    fn get_block_predecessors(
        &self,
        block: BasicBlockId,
    ) -> std::collections::BTreeSet<BasicBlockId> {
        // Phase 69-3: Changed to BTreeSet for determinism
        self.f
            .blocks
            .get(&block)
            .map(|bb| bb.predecessors.clone())
            .unwrap_or_default()
    }

    /// Phase 26-A-4: ValueIdベースのパラメータ判定（JSON bridge版）
    ///
    /// JSON bridge は MirBuilder の value_kinds にアクセスできないため、
    /// 逆引きして変数名を取得し、既存のヒューリスティックを適用する。
    fn is_parameter(&self, value_id: ValueId) -> bool {
        // Phase 26-A-4: ValueId から変数名を逆引き
        // vars マップを逆引きして変数名を取得
        let name = self
            .vars
            .iter()
            .find(|(_, &v)| v == value_id)
            .map(|(n, _)| n.as_str());

        // 変数名が見つかった場合、既存のヒューリスティックを適用
        // JSON bridge ではパラメータ名の SSOT は FunctionDefBuilder 側にある。
        // ここでは「典型的な受け口」だけを pinned 候補とし、それ以外は
        // carrier として扱う（ループ意味論上は安全）。
        //
        // - CLI entry: args
        // - インスタンスメソッド receiver: me
        name.map(|n| matches!(n, "me" | "args")).unwrap_or(false)
    }

    fn set_current_block(&mut self, block: BasicBlockId) -> Result<(), String> {
        if !self.f.blocks.contains_key(&block) {
            return Err(format!("Block {:?} not found", block));
        }
        self.current_block = block;
        Ok(())
    }

    fn emit_copy(&mut self, dst: ValueId, src: ValueId) -> Result<(), String> {
        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
            let def_blocks = crate::mir::verification::utils::compute_def_blocks(self.f);
            let dominators = crate::mir::verification::utils::compute_dominators(self.f);
            let def_block = def_blocks.get(&src).copied();
            let dominates = def_block
                .map(|db| dominators.dominates(db, self.current_block))
                .unwrap_or(false);
            if !dominates {
                let def_block_label = def_block
                    .map(|b| format!("{:?}", b))
                    .unwrap_or_else(|| "None".to_string());
                return Err(format!(
                    "[freeze:contract][json_v0_bridge/non_dominating_copy] fn={} bb={:?} src=%{} def_block={} op=emit_copy",
                    self.f.signature.name,
                    self.current_block,
                    src.0,
                    def_block_label
                ));
            }
        }
        copy_emitter::emit_copy_in_block(
            self.f,
            self.current_block,
            dst,
            src,
            CopyEmitReason::JsonV0BridgeLoopformEmitCopy,
        )
    }

    fn emit_jump(&mut self, target: BasicBlockId) -> Result<(), String> {
        crate::mir::ssot::cf_common::set_jump(self.f, self.current_block, target);
        Ok(())
    }

    fn emit_phi(
        &mut self,
        dst: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
            self.f,
            self.current_block,
            dst,
            inputs,
            Span::unknown(),
        )?;
        Ok(())
    }

    fn update_phi_inputs(
        &mut self,
        block: BasicBlockId,
        phi_id: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        if let Some(bb) = self.f.get_block_mut(block) {
            for inst in &mut bb.instructions {
                if let MirInstruction::Phi {
                    dst,
                    inputs: phi_inputs,
                    .. // Phase 63-6: Ignore type_hint
                } = inst
                {
                    if *dst == phi_id {
                        *phi_inputs = inputs;
                        return Ok(());
                    }
                }
            }
            Err(format!(
                "PHI instruction {:?} not found in block {:?}",
                phi_id, block
            ))
        } else {
            Err(format!("Block {:?} not found while updating PHI", block))
        }
    }

    fn update_var(&mut self, name: String, value: ValueId) {
        self.vars.insert(name.clone(), value);
        // 現在ブロックのスナップショットも更新しておく（get_variable_at_block 用）
        self.block_var_maps
            .entry(self.current_block)
            .or_insert_with(BTreeMap::new)
            .insert(name, value);
    }

    fn get_variable_at_block(&self, name: &str, block: BasicBlockId) -> Option<ValueId> {
        if let Some(map) = self.block_var_maps.get(&block) {
            if let Some(v) = map.get(name) {
                return Some(*v);
            }
        }
        // In strict/dev + planner_required, do not fall back to "current vars".
        // `get_variable_at_block` is used as a contract observation point: returning a value
        // that is not actually available on the predecessor edge can silently inject invalid
        // PHI inputs (later surfacing as VM reg_load undefined).
        let strict_or_dev =
            crate::config::env::joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
        if strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled() {
            // Parameters dominate every block; it's safe to fall back for them even in strict mode.
            if matches!(name, "me" | "args") {
                return self.vars.get(name).copied();
            }
            return None;
        }
        self.vars.get(name).copied()
    }

    fn mir_function(&self) -> &crate::mir::MirFunction {
        self.f
    }
}

pub(super) fn lower_loop_stmt(
    f: &mut MirFunction,
    cur_bb: BasicBlockId,
    cond: &ExprV0,
    body: &[StmtV0],
    vars: &mut BTreeMap<String, ValueId>,
    loop_stack: &mut Vec<LoopContext>,
    env: &BridgeEnv,
) -> Result<BasicBlockId, String> {
    // DEBUG: Track loop lowering calls (Stage-B / JSON v0) — dev用トレース
    if env::env_bool("HAKO_LOOP_PHI_TRACE") {
        get_global_ring0().log.debug(&format!(
            "[loop-phi/json] lower_loop_stmt called, fn={}, base_vars={}",
            f.signature.name,
            vars.len()
        ));
    }

    // Unification toggle（legacy フラグ。実装は LoopForm v2 に一本化済み）
    let unify_on = env::env_bool_default("NYASH_MIR_UNIFY_LOOPFORM", true);
    if !unify_on {
        crate::cli_v!(
            "[loopform] NYASH_MIR_UNIFY_LOOPFORM=0 requested; JSON front still uses LoopForm v2 bridge"
        );
    }

    // Block layout（AST LoopBuilder に近い形に揃える）
    let preheader_bb = cur_bb;
    let header_bb = new_block(f);
    let body_bb = new_block(f);
    let latch_bb = new_block(f);
    let exit_bb = new_block(f);
    // JSON ルート用 canonical continue_merge ブロック
    let continue_merge_bb = new_block(f);

    // 1) preheader スナップショット（Env_in(loop)）
    let base_vars = vars.clone();

    // DEBUG: Log preheader snapshot
    if env::env_bool("NYASH_LOOPFORM_DEBUG") {
        get_global_ring0().log.debug(&format!(
            "[loop_/lower] === PREHEADER SNAPSHOT (bb={:?}) ===",
            preheader_bb
        ));
        get_global_ring0()
            .log
            .debug(&format!("[loop_/lower] Function: {}", f.signature.name));
        get_global_ring0().log.debug(&format!(
            "[loop_/lower] base_vars.len() = {}",
            base_vars.len()
        ));
        let mut sorted: Vec<_> = base_vars.iter().collect();
        sorted.sort_by_key(|(name, _)| name.as_str());
        for (name, value) in &sorted {
            get_global_ring0().log.debug(&format!(
                "[loop_/lower]   preheader var: {} = {:?}",
                name, value
            ));
        }
    }

    let mut block_var_maps: BTreeMap<BasicBlockId, BTreeMap<String, ValueId>> = BTreeMap::new();
    block_var_maps.insert(preheader_bb, base_vars.clone());

    // 2) LoopFormBuilder + LoopFormJsonOps を用いて preheader/header PHI を構築
    let mut loopform = LoopFormBuilder::new(preheader_bb, header_bb);
    let mut ops = LoopFormJsonOps::new(f, vars, &mut block_var_maps, preheader_bb);

    loopform.prepare_structure(&mut ops, &base_vars)?;
    loopform.emit_preheader(&mut ops)?;
    loopform.emit_header_phis(&mut ops)?;

    // 3) ループ条件を header ブロックで評価し、body/exit へ分岐
    let (cval, cend) = super::expr::lower_expr_with_vars(env, ops.f, header_bb, cond, ops.vars)?;
    // Record a snapshot at the actual condition-end block (cend).
    // Short-circuit lowering (&&/||) can create intermediate blocks; exit PHIs must be
    // able to observe variable availability at cend deterministically.
    ops.block_var_maps.insert(cend, ops.vars.clone());
    crate::mir::ssot::cf_common::set_branch(ops.f, cend, cval, body_bb, exit_bb);

    // 4) ループ本体を lowering（break/continue スナップショットは lowering.rs 側が管理）
    let mut body_vars = ops.vars.clone();
    super::loop_runtime::push_loop_snapshot_frames();
    loop_stack.push(LoopContext {
        cond_bb: header_bb,
        exit_bb,
        continue_merge_bb: Some(continue_merge_bb),
    });
    super::loop_runtime::detect_and_push_increment_hint(body);
    let bend_res = lower_stmt_list_with_vars(ops.f, body_bb, body, &mut body_vars, loop_stack, env);
    loop_stack.pop();
    let _ = super::loop_runtime::pop_increment_hint();
    let bend = bend_res?;

    // Step 5-1: Writes集合収集（選択肢2+3統合: Snapshot比較で再代入検出）
    // base_vars (preheader) と body_vars を比較し、ValueId が変わった変数を特定
    use std::collections::HashSet;
    let mut writes = HashSet::new();
    for (name, &body_value) in &body_vars {
        // Skip __pin$ temporary variables - they are always BodyLocalInternal
        // (Task先生の発見: これらをcarrier扱いすると未定義ValueIdエラーの原因になる)
        if name.starts_with("__pin$") && name.contains("$@") {
            continue;
        }

        if let Some(&base_value) = base_vars.get(name) {
            if body_value != base_value {
                writes.insert(name.clone());
            }
        }
        // else: body で新規定義された変数（body-local）、header PHI 不要
    }

    // DEBUG: Log writes collection
    if env::env_bool("NYASH_LOOPFORM_DEBUG") {
        let func_name = ops.f.signature.name.clone(); // Clone before borrowing
        get_global_ring0().log.debug("[loop_/lower] === WRITES COLLECTION (Step 5-1) ===");
        get_global_ring0()
            .log
            .debug(&format!("[loop_/lower] Function: {}", func_name));
        get_global_ring0().log.debug(&format!(
            "[loop_/lower] {} variables modified in loop body",
            writes.len()
        ));
        let mut sorted_writes: Vec<_> = writes.iter().collect();
        sorted_writes.sort();
        for name in &sorted_writes {
            get_global_ring0()
                .log
                .debug(&format!("[loop_/lower]   WRITE: {}", name));
        }
    }

    // スナップショット収集（Env_out(loop) 用）
    let continue_snaps = super::loop_runtime::pop_continue_snapshots();
    let exit_snaps = super::loop_runtime::pop_exit_snapshots();

    // Record per-edge snapshots into block_var_maps so LoopFormBuilder can observe
    // predecessor-availability deterministically (strict/planner_required).
    for (bb, snap) in &continue_snaps {
        ops.block_var_maps.insert(*bb, snap.clone());
    }
    for (bb, snap) in &exit_snaps {
        ops.block_var_maps.insert(*bb, snap.clone());
    }

    // 5) latch（body末尾）スナップショットを LoopForm 用のマップに登録
    ops.block_var_maps.insert(latch_bb, body_vars.clone());

    // body から latch へのフォールスルー経路（continue / break は既に terminator 済み）
    if let Some(bb) = ops.f.get_block_mut(bend) {
        if !bb.is_terminated() {
            crate::mir::ssot::cf_common::set_jump(ops.f, bend, latch_bb);
        }
    }

    // latch から header への canonical backedge
    crate::mir::ssot::cf_common::set_jump(ops.f, latch_bb, header_bb);

    // 6) continue 経路を canonical continue_merge に統合し、header PHI 用 snapshot を 1 本にまとめる
    let canonical_continue_snaps: Vec<(BasicBlockId, BTreeMap<String, ValueId>)> =
        if continue_snaps.is_empty() {
            Vec::new()
        } else {
            // 6-1) 各変数ごとに (pred_bb, value) の入力を集約
            let mut all_inputs: BTreeMap<String, Vec<(BasicBlockId, ValueId)>> = BTreeMap::new();
            for (bb, snap) in &continue_snaps {
                // Only merge variables that exist at loop entry (base_vars).
                //
                // Body-local variables can appear in `continue` snapshots after they are first
                // assigned, but they do NOT need to be carried through the canonical continue-merge
                // block for header PHIs. Including them here creates "partial PHI" inputs where
                // early-continue edges don't have a value yet, which later fails MIR verification
                // (`invalid_phi` / missing input from predecessor).
                //
                // Contract (Phase 29bq selfhost): header/continue merge handles Env_in(loop) only.
                for (name, _) in &base_vars {
                    let Some(&val) = snap.get(name) else {
                        let strict_planner_required =
                            crate::config::env::joinir_dev::strict_enabled()
                                && crate::config::env::joinir_dev::planner_required_enabled();
                        if strict_planner_required {
                            return Err(format!(
                                "[freeze:contract][json_v0_bridge/continue_snapshot_missing_base_var] fn={} pred_bb={:?} var={}",
                                ops.f.signature.name, bb, name
                            ));
                        }
                        // Non-strict fallback: treat missing as base value.
                        if let Some(&base_val) = base_vars.get(name) {
                            all_inputs
                                .entry(name.clone())
                                .or_default()
                                .push((*bb, base_val));
                        }
                        continue;
                    };
                    all_inputs.entry(name.clone()).or_default().push((*bb, val));
                }
            }

            // 6-2) continue_merge_bb に必要な PHI を生成しつつ、merged_snapshot を作る
            // Phase 62: PhiInputCollector インライン化
            let mut merged_snapshot: BTreeMap<String, ValueId> = BTreeMap::new();
            for (name, inputs) in all_inputs {
                // Inline PhiInputCollector logic
                // 1. Sanitize: remove duplicates and sort
                let mut seen: BTreeMap<BasicBlockId, ValueId> = BTreeMap::new();
                for (bb, val) in inputs.iter() {
                    seen.insert(*bb, *val);
                }
                let mut sanitized_inputs: Vec<(BasicBlockId, ValueId)> = seen.into_iter().collect();
                sanitized_inputs.sort_by_key(|(bb, _)| bb.0);

                // 2. Optimize: check if all inputs have the same value
                let value = if sanitized_inputs.is_empty() {
                    // Should not happen, but handle gracefully
                    continue;
                } else if sanitized_inputs.len() == 1 {
                    // Single input - no PHI needed
                    sanitized_inputs[0].1
                } else {
                    let first_val = sanitized_inputs[0].1;
                    if sanitized_inputs.iter().all(|(_, val)| *val == first_val) {
                        // All same value - no PHI needed
                        first_val
                    } else {
                        // Different values - PHI required
                        let phi_id = ops.f.next_value_id();
                        crate::mir::ssot::cf_common::insert_phi_at_head_spanned(
                            ops.f,
                            continue_merge_bb,
                            phi_id,
                            sanitized_inputs,
                            Span::unknown(),
                        )?;
                        phi_id
                    }
                };
                merged_snapshot.insert(name, value);
            }

            // continue_merge_bb から header への backedge を 1 本だけ張る
            crate::mir::ssot::cf_common::set_jump(ops.f, continue_merge_bb, header_bb);
            // LoopForm から get_variable_at_block されたときのために snapshot も登録
            ops.block_var_maps
                .insert(continue_merge_bb, merged_snapshot.clone());

            vec![(continue_merge_bb, merged_snapshot)]
        };

    // 7) header PHI seal（latch + canonical continue_merge スナップショット）
    // Step 5-1/5-2: Pass writes 集合 for PHI縮約
    // Phase 27.4C: JSON v0 bridge は常に header_bypass = false（本線経路）
    loopform.seal_phis(
        &mut ops,
        latch_bb,
        &canonical_continue_snaps,
        &writes,
        false, // header_bypass (JSON v0 bridge はレガシー経路なので false)
    )?;

    // 8) exit PHI（header fallthrough + break スナップショット）
    // Phase 69-2: inspector 引数削除（merge_exit_with_classification 内部で構築）
    loopform.build_exit_phis(&mut ops, exit_bb, cend, &exit_snaps)?;

    Ok(exit_bb)
}
