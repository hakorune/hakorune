#[cfg(feature = "rc-insertion-minimal")]
use crate::ast::Span;
use crate::mir::MirModule;
#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
#[cfg(feature = "rc-insertion-minimal")]
use std::collections::{HashMap, HashSet};
#[cfg(feature = "rc-insertion-minimal")]
#[path = "rc_insertion_helpers/apply.rs"]
mod apply;
#[cfg(feature = "rc-insertion-minimal")]
#[path = "rc_insertion_helpers/cleanup.rs"]
mod cleanup;
#[cfg(feature = "rc-insertion-minimal")]
#[path = "rc_insertion_helpers/contracts.rs"]
mod contracts;
#[cfg(feature = "rc-insertion-minimal")]
#[path = "rc_insertion_helpers/cycles.rs"]
mod cycles;
#[cfg(feature = "rc-insertion-minimal")]
#[path = "rc_insertion_helpers/plan.rs"]
mod plan;
#[cfg(feature = "rc-insertion-minimal")]
#[path = "rc_insertion_helpers/types.rs"]
mod types;
#[cfg(feature = "rc-insertion-minimal")]
#[path = "rc_insertion_helpers/util.rs"]
mod util;
#[cfg(feature = "rc-insertion-minimal")]
use types::{DropPoint, DropReason, DropSite};

/// Statistics from RC insertion pass
#[derive(Debug, Default, Clone)]
pub struct RcInsertionStats {
    /// Number of KeepAlive instructions inserted
    pub keepalive_inserted: usize,
    /// Number of Release instructions inserted
    pub release_inserted: usize,
    /// Number of functions processed
    pub functions_processed: usize,
    /// Number of blocks visited
    pub blocks_visited: usize,
}

/// Phase 29z P0: RC insertion pass - Minimal overwrite release
///
/// This pass is called after MIR optimization and verification.
/// Implements minimal case: overwrite release (x = <new> releases old value).
///
/// **CRITICAL SAFETY NOTES**:
/// 1. ReleaseStrong does SSA alias cleanup (releases all SSA values sharing same Arc)
///    - MUST NOT release values still in use
///    - Safety guard: Skip release if `old_value == value` (same Arc, no-op overwrite)
/// 2. Span mismatch: `instruction_spans` may not match `instructions` length
///    - Fill missing spans with `Span::unknown()` to prevent panic
///
/// **Scope** (Phase 29z P0 minimal):
/// - ✅ Single-block overwrite detection
/// - ✅ Safety guards (SSA alias cleanup protection)
/// - ✅ Minimal break-edge cleanup (empty Jump -> immediate Return)
/// - ✅ Minimal continue-edge cleanup (empty Jump -> loop-header-like Branch)
/// - ❌ PHI/complex loop cleanup (out of scope)
/// - ❌ Cross-block tracking (out of scope)
///
/// **Opt-in**: Default OFF, enabled only with Cargo feature `rc-insertion-minimal`
pub fn insert_rc_instructions(module: &mut MirModule) -> RcInsertionStats {
    let mut stats = RcInsertionStats::default();

    // Phase 29z P0: Default OFF unless explicitly enabled.
    // No new environment variables (per Phase 29z P0 constraints).
    #[cfg(not(feature = "rc-insertion-minimal"))]
    {
        // No-op pass (just count structures)
        for (_name, func) in &module.functions {
            stats.functions_processed += 1;
            stats.blocks_visited += func.blocks.len();
        }
        return stats;
    }

    #[cfg(feature = "rc-insertion-minimal")]
    {
        // Implement minimal overwrite release
        for (_name, func) in &mut module.functions {
            stats.functions_processed += 1;

            let mut predecessors: HashMap<BasicBlockId, Vec<BasicBlockId>> = HashMap::new();
            for (bid, b) in &func.blocks {
                let Some(term) = b.terminator.as_ref() else {
                    continue;
                };
                match term {
                    MirInstruction::Jump { target, .. } => {
                        predecessors.entry(*target).or_default().push(*bid);
                    }
                    MirInstruction::Branch {
                        then_bb, else_bb, ..
                    } => {
                        predecessors.entry(*then_bb).or_default().push(*bid);
                        predecessors.entry(*else_bb).or_default().push(*bid);
                    }
                    _ => {}
                }
            }

            let mut jump_chain_next: HashMap<BasicBlockId, BasicBlockId> = HashMap::new();
            for (bid, b) in &func.blocks {
                let Some(MirInstruction::Jump { target, .. }) = b.terminator.as_ref() else {
                    continue;
                };
                if !func.blocks.contains_key(target) {
                    continue;
                }
                let preds = predecessors
                    .get(target)
                    .map(|p| p.as_slice())
                    .unwrap_or(&[]);
                if preds.len() == 1 {
                    debug_assert!(
                        preds[0] == *bid,
                        "rc_insertion: predecessor map mismatch for jump chain"
                    );
                    jump_chain_next.insert(*bid, *target);
                }
            }

            let jump_chain_cycles = cycles::detect_jump_chain_cycles(&jump_chain_next);
            if !jump_chain_cycles.is_empty() {
                debug_assert!(
                    false,
                    "rc_insertion: jump-chain cycle detected; propagation disabled for cycle nodes"
                );
            }

            let empty_state: HashMap<ValueId, ValueId> = HashMap::new();
            let empty_null: HashSet<ValueId> = HashSet::new(); // P8: null 伝播用
            let mut initial_state: HashMap<BasicBlockId, HashMap<ValueId, ValueId>> =
                HashMap::new();
            let mut initial_null_values: HashMap<BasicBlockId, HashSet<ValueId>> = HashMap::new(); // P8
                                                                                                   // P5: end_states を保持して multi-pred join に使う
            let mut end_states: HashMap<BasicBlockId, HashMap<ValueId, ValueId>> = HashMap::new();
            let mut end_null_states: HashMap<BasicBlockId, HashSet<ValueId>> = HashMap::new(); // P8
            let max_iters = func.blocks.len().max(1);
            for iter in 0..max_iters {
                let mut changed = false;
                for (bid, block) in &func.blocks {
                    let state_in = initial_state.get(bid).unwrap_or(&empty_state);
                    let null_in = initial_null_values.get(bid).unwrap_or(&empty_null); // P8
                    let (_plan, end_state, end_null) = plan::plan_rc_insertion_for_block(
                        &block.instructions,
                        block.terminator.as_ref(),
                        state_in,
                        null_in, // P8
                    );

                    // P5: end_state を保存（multi-pred join で使う）
                    end_states.insert(*bid, end_state.clone());
                    end_null_states.insert(*bid, end_null); // P8

                    let Some(target) = jump_chain_next.get(bid).copied() else {
                        continue;
                    };
                    if jump_chain_cycles.contains(bid) || jump_chain_cycles.contains(&target) {
                        continue;
                    }

                    // P8: null_values 伝播（ptr_to_value が空でも null は伝播する）
                    if let Some(end_null) = end_null_states.get(bid) {
                        if !end_null.is_empty() {
                            let needs_null_update = match initial_null_values.get(&target) {
                                Some(existing) => existing != end_null,
                                None => true,
                            };
                            if needs_null_update {
                                initial_null_values.insert(target, end_null.clone());
                                changed = true;
                            }
                        } else if initial_null_values.remove(&target).is_some() {
                            changed = true;
                        }
                    }

                    if end_state.is_empty() {
                        if initial_state.remove(&target).is_some() {
                            changed = true;
                        }
                        continue;
                    }

                    let needs_update = match initial_state.get(&target) {
                        Some(existing) => existing != &end_state,
                        None => true,
                    };
                    if needs_update {
                        initial_state.insert(target, end_state);
                        changed = true;
                    }
                }

                // P6: multi-predecessor Return join 判定（intersection）
                // 全経路で必ず保持される ptr→value のみ ReturnCleanup で release
                for (bid, block) in &func.blocks {
                    // Return block のみ対象
                    let Some(MirInstruction::Return { .. }) = block.terminator.as_ref() else {
                        continue;
                    };

                    let preds = predecessors.get(bid).cloned().unwrap_or_default();
                    if preds.len() < 2 {
                        // 単一predecessor は既存ロジック（jump_chain_next）で処理済み
                        continue;
                    }

                    // P8: multi-pred Return では null_values を合流しない（保守的に空集合）
                    // 古い状態が残らないよう明示的に remove（P5/P6 の initial_state.remove と同じ残留バグ対策）
                    if initial_null_values.remove(bid).is_some() {
                        changed = true;
                    }

                    // 全predecessorのend_stateを収集
                    let mut pred_end_states: Vec<&HashMap<ValueId, ValueId>> = Vec::new();
                    for pred_bid in &preds {
                        if let Some(end_st) = end_states.get(pred_bid) {
                            pred_end_states.push(end_st);
                        }
                    }

                    // P6: intersection 計算
                    // 全 predecessor の end_state が揃っている場合のみ intersection 計算
                    if pred_end_states.len() != preds.len() || pred_end_states.is_empty() {
                        // predecessor の end_state が不完全 → join しない
                        if initial_state.remove(bid).is_some() {
                            changed = true;
                        }
                        continue;
                    }

                    // intersection 計算: 最初の state から所有 HashMap を作成（retain のため）
                    // ⚠️ pred_end_states[0] は &HashMap なので .clone() は参照複製になる
                    // → iter().map() で所有 HashMap を作る
                    let mut join_state: HashMap<ValueId, ValueId> =
                        pred_end_states[0].iter().map(|(k, v)| (*k, *v)).collect();

                    for other_state in pred_end_states.iter().skip(1) {
                        // join_state から、other_state に無い or value が違う ptr を削除
                        join_state.retain(|ptr, val| other_state.get(ptr) == Some(val));
                    }

                    if join_state.is_empty() {
                        // intersection が空 → cleanup しない
                        if initial_state.remove(bid).is_some() {
                            changed = true;
                        }
                    } else {
                        // 非 empty intersection → initial_state にセット
                        debug_assert!(
                            matches!(
                                block.terminator.as_ref(),
                                Some(MirInstruction::Return { .. })
                            ),
                            "rc_insertion: multi-pred join only for Return blocks"
                        );
                        if initial_state.get(bid) != Some(&join_state) {
                            initial_state.insert(*bid, join_state);
                            changed = true;
                        }
                    }
                }

                if !changed {
                    break;
                }
                if iter + 1 == max_iters {
                    debug_assert!(
                        false,
                        "rc_insertion: jump-chain propagation did not converge; possible cycle"
                    );
                    break;
                }
            }

            let break_cleanup_values_by_block = cleanup::collect_break_cleanup_values(
                func,
                &predecessors,
                &end_states,
                &initial_state,
            );
            let continue_cleanup_values_by_block =
                cleanup::collect_continue_cleanup_values(func, &predecessors, &end_states);
            if let Err(err) = contracts::verify_rc_phi_edge_contracts(
                &func.signature.name,
                func,
                &break_cleanup_values_by_block,
                &continue_cleanup_values_by_block,
            ) {
                contracts::fail_fast_rc_phi_edge_mismatch(err);
            }
            let return_blocks: HashSet<BasicBlockId> = func
                .blocks
                .iter()
                .filter_map(|(bid, b)| {
                    if matches!(b.terminator.as_ref(), Some(MirInstruction::Return { .. })) {
                        Some(*bid)
                    } else {
                        None
                    }
                })
                .collect();

            for (bid, block) in &mut func.blocks {
                stats.blocks_visited += 1;

                // Take ownership of instructions to rebuild with inserted releases
                let insts = std::mem::take(&mut block.instructions);
                let mut spans = std::mem::take(&mut block.instruction_spans);
                let terminator = block.terminator.take();
                let terminator_span = block.terminator_span.take();

                // SAFETY: Ensure spans match instructions length (fill with Span::unknown() if needed)
                // instruction_spans and instructions may not always match in length
                while spans.len() < insts.len() {
                    spans.push(Span::unknown());
                }

                let initial_state_for_block = initial_state.get(bid);
                if let Some(state) = initial_state_for_block {
                    let pred_count = predecessors.get(bid).map(|p| p.len()).unwrap_or(0);
                    // P5: multi-pred Return も許可（pred_count >= 2 で Return の場合）
                    let is_multi_pred_return = pred_count >= 2
                        && matches!(terminator.as_ref(), Some(MirInstruction::Return { .. }));
                    debug_assert!(
                        pred_count == 1 || is_multi_pred_return,
                        "rc_insertion: initial state requires single predecessor or multi-pred Return"
                    );
                    debug_assert!(
                        matches!(terminator.as_ref(), Some(MirInstruction::Return { .. }))
                            || matches!(terminator.as_ref(), Some(MirInstruction::Jump { .. }))
                            || matches!(terminator.as_ref(), Some(MirInstruction::Branch { .. })),
                        "rc_insertion: initial state only allowed for Jump/Branch/Return blocks"
                    );
                    debug_assert!(
                        !state.is_empty(),
                        "rc_insertion: initial state must be non-empty"
                    );
                    if let Some(MirInstruction::Jump { target, .. }) = terminator.as_ref() {
                        let target_pred_count =
                            predecessors.get(target).map(|p| p.len()).unwrap_or(0);
                        let is_jump_to_multi_pred_return =
                            target_pred_count >= 2 && return_blocks.contains(target);
                        let has_early_exit_cleanup = break_cleanup_values_by_block
                            .contains_key(bid)
                            || continue_cleanup_values_by_block.contains_key(bid);
                        debug_assert!(
                            jump_chain_next.contains_key(bid)
                                || is_jump_to_multi_pred_return
                                || has_early_exit_cleanup,
                            "rc_insertion: jump-state requires jump-chain or jump-to-multi-pred-return"
                        );
                    }
                }

                let null_in = initial_null_values.get(bid).unwrap_or(&empty_null); // P8
                let (plan, _end_state, _end_null) = plan::plan_rc_insertion_for_block(
                    &insts,
                    terminator.as_ref(),
                    initial_state_for_block.unwrap_or(&empty_state),
                    null_in, // P8
                );
                let mut plan = plan;
                if let Some(values) = break_cleanup_values_by_block.get(bid) {
                    if !values.is_empty() {
                        debug_assert!(
                            matches!(terminator.as_ref(), Some(MirInstruction::Jump { .. })),
                            "rc_insertion: BreakCleanup planned for non-Jump terminator"
                        );
                        plan.drops.push(DropSite {
                            at: DropPoint::BeforeTerminator,
                            values: values.clone(),
                            reason: DropReason::BreakCleanup,
                        });
                    }
                }
                if let Some(values) = continue_cleanup_values_by_block.get(bid) {
                    if !values.is_empty() {
                        debug_assert!(
                            matches!(terminator.as_ref(), Some(MirInstruction::Jump { .. })),
                            "rc_insertion: ContinueCleanup planned for non-Jump terminator"
                        );
                        plan.drops.push(DropSite {
                            at: DropPoint::BeforeTerminator,
                            values: values.clone(),
                            reason: DropReason::ContinueCleanup,
                        });
                    }
                }
                let (new_insts, new_spans, new_terminator, new_terminator_span) =
                    apply::apply_rc_plan(
                        insts,
                        spans,
                        terminator,
                        terminator_span,
                        plan,
                        &mut stats,
                    );

                block.instructions = new_insts;
                block.instruction_spans = new_spans;
                block.terminator = new_terminator;
                block.terminator_span = new_terminator_span;
            }
        }

        stats
    }
}
