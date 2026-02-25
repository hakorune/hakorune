use crate::mir::builder::MirBuilder;
use crate::mir::{MirFunction, MirInstruction, ValueId};

/// BlockScheduleBox — manage physical insertion points within a block.
/// Contract: PHI group → materialize group (Copy/Id) → body (Call etc.)
pub struct BlockScheduleBox;

impl BlockScheduleBox {
    #[inline]
    fn strict_planner_required() -> bool {
        crate::config::env::joinir_dev::strict_planner_required_debug_enabled()
    }

    fn find_def_inst(function: &MirFunction, target: ValueId) -> Option<MirInstruction> {
        for bb in function.blocks.values() {
            for inst in &bb.instructions {
                let matches = match inst {
                    MirInstruction::Const { dst, .. }
                    | MirInstruction::BinOp { dst, .. }
                    | MirInstruction::Compare { dst, .. }
                    | MirInstruction::Copy { dst, .. }
                    | MirInstruction::Phi { dst, .. } => *dst == target,
                    _ => false,
                };
                if matches {
                    return Some(inst.clone());
                }
            }
            if let Some(term) = &bb.terminator {
                let matches = match term {
                    MirInstruction::Const { dst, .. }
                    | MirInstruction::BinOp { dst, .. }
                    | MirInstruction::Compare { dst, .. }
                    | MirInstruction::Copy { dst, .. }
                    | MirInstruction::Phi { dst, .. } => *dst == target,
                    _ => false,
                };
                if matches {
                    return Some(term.clone());
                }
            }
        }
        None
    }

    fn resolve_pure_def(builder: &MirBuilder, src: ValueId) -> Option<MirInstruction> {
        let func = builder.scope_ctx.current_function.as_ref()?;
        let def = Self::find_def_inst(func, src)?;
        match def {
            MirInstruction::Const { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. } => Some(def),
            MirInstruction::Copy { src: inner, .. } => {
                let inner_def = Self::find_def_inst(func, inner)?;
                match inner_def {
                    MirInstruction::Const { .. }
                    | MirInstruction::BinOp { .. }
                    | MirInstruction::Compare { .. } => Some(inner_def),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn insert_after_phis_inst(
        builder: &mut MirBuilder,
        inst: MirInstruction,
        src_for_meta: ValueId,
    ) -> Result<(), String> {
        if let (Some(ref mut function), Some(bb)) =
            (&mut builder.scope_ctx.current_function, builder.current_block)
        {
            if let Some(block) = function.get_block_mut(bb) {
                let dst = inst.dst_value();
                block.insert_spanned_after_phis(crate::mir::SpannedInstruction {
                    inst,
                    span: builder.metadata_ctx.current_span(),
                });
                if let Some(dst) = dst {
                    crate::mir::builder::metadata::propagate::propagate(
                        builder, src_for_meta, dst,
                    );
                }
                return Ok(());
            }
        }
        Err("No current function/block to insert after-phis instruction".into())
    }

    /// Insert a Copy immediately after PHI nodes. Returns the local value id.
    #[allow(dead_code)]
    pub fn ensure_after_phis_copy(
        builder: &mut MirBuilder,
        src: ValueId,
    ) -> Result<ValueId, String> {
        if let Some(bb) = builder.current_block {
            if let Some(&cached) = builder.schedule_mat_map.get(&(bb, src)) {
                if crate::config::env::builder_schedule_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[schedule/after-phis] bb={:?} src=%{} cached dst=%{}",
                        bb, src.0, cached.0
                    ));
                }
                return Ok(cached);
            }
            if Self::strict_planner_required() {
                if let Some(func) = builder.scope_ctx.current_function.as_ref() {
                    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                    let dominators = crate::mir::verification::utils::compute_dominators(func);
                    let def_block = def_blocks.get(&src).copied();
                    let dominates = def_block
                        .map(|def| dominators.dominates(def, bb))
                        .unwrap_or(false);
                    if !dominates {
                        return Err(format!(
                            "[freeze:contract][schedule/non_dominating_copy] fn={} bb={:?} src=%{} def_block={:?}",
                            func.signature.name, bb, src.0, def_block
                        ));
                    }
                }
            }

            let dst = builder.next_value_id();
            if let Some(def_inst) = Self::resolve_pure_def(builder, src) {
                if crate::config::env::builder_schedule_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[schedule/after-phis] bb={:?} src=%{} new dst=%{} (rematerialize)",
                        bb, src.0, dst.0
                    ));
                }
                let remat_inst = match def_inst {
                    MirInstruction::Const { value, .. } => MirInstruction::Const { dst, value },
                    MirInstruction::BinOp { op, lhs, rhs, .. } => {
                        MirInstruction::BinOp { dst, op, lhs, rhs }
                    }
                    MirInstruction::Compare { op, lhs, rhs, .. } => {
                        MirInstruction::Compare { dst, op, lhs, rhs }
                    }
                    _ => MirInstruction::Copy { dst, src },
                };
                Self::insert_after_phis_inst(builder, remat_inst, src)?;
            } else {
                if crate::config::env::builder_schedule_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[schedule/after-phis] bb={:?} src=%{} new dst=%{} (inserting Copy) builder.current_block={:?}",
                        bb, src.0, dst.0, builder.current_block
                    ));
                }
                builder.insert_copy_after_phis(dst, src)?;
            }
            builder.schedule_mat_map.insert((bb, src), dst);
            return Ok(dst);
        }
        Err("No current block".into())
    }

    /// Emit a Copy right before the next emitted instruction (best-effort):
    /// place it at the tail of the current block. Returns the local value id.
    #[allow(dead_code)]
    pub fn emit_before_call_copy(
        builder: &mut MirBuilder,
        src: ValueId,
    ) -> Result<ValueId, String> {
        if let Some(def_inst) = Self::resolve_pure_def(builder, src) {
            let dst = builder.next_value_id();
            if crate::config::env::builder_schedule_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[schedule/before-call] bb={:?} src=%{} dst=%{} (rematerialize)",
                    builder.current_block, src.0, dst.0
                ));
            }
            let remat_inst = match def_inst {
                MirInstruction::Const { value, .. } => MirInstruction::Const { dst, value },
                MirInstruction::BinOp { op, lhs, rhs, .. } => {
                    let lhs_local = crate::mir::builder::ssa::local::arg(builder, lhs);
                    let rhs_local = crate::mir::builder::ssa::local::arg(builder, rhs);
                    MirInstruction::BinOp {
                        dst,
                        op,
                        lhs: lhs_local,
                        rhs: rhs_local,
                    }
                }
                MirInstruction::Compare { op, lhs, rhs, .. } => {
                    let mut lhs_local = lhs;
                    let mut rhs_local = rhs;
                    crate::mir::builder::ssa::local::finalize_compare(
                        builder,
                        &mut lhs_local,
                        &mut rhs_local,
                    )?;
                    MirInstruction::Compare {
                        dst,
                        op,
                        lhs: lhs_local,
                        rhs: rhs_local,
                    }
                }
                _ => MirInstruction::Copy { dst, src },
            };
            builder.emit_instruction(remat_inst)?;
            crate::mir::builder::metadata::propagate::propagate(builder, src, dst);
            return Ok(dst);
        }

        if Self::strict_planner_required() {
            return Err(format!(
                "[freeze:contract][schedule/non_dominating_src] bb={:?} src={:?}",
                builder.current_block, src
            ));
        }

        // Prefer to reuse the after-phis materialized id for this src in this block
        let base = Self::ensure_after_phis_copy(builder, src)?;
        let dst = builder.next_value_id();
        if crate::config::env::builder_schedule_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[schedule/before-call] bb={:?} src=%{} base=%{} dst=%{} (emitting Copy)",
                builder.current_block, src.0, base.0, dst.0
            ));
        }
        builder.emit_instruction(MirInstruction::Copy { dst, src: base })?;
        // Propagate metadata to keep dst consistent with base
        crate::mir::builder::metadata::propagate::propagate(builder, base, dst);
        Ok(dst)
    }

    /// Dev-only: verify simple block order invariants.
    /// - PHI group must be at the block head (no PHI after first non-PHI)
    /// - If a Copy immediately precedes a Call-like instruction, prefer that Copy's src
    ///   to be the previously materialized after-PHIs id (best-effort warning only).
    pub fn verify_order(builder: &mut MirBuilder) {
        if !crate::config::env::builder_block_schedule_verify() {
            return;
        }
        let (f_opt, bb_opt) = (
            builder.scope_ctx.current_function.as_ref(),
            builder.current_block,
        );
        let (Some(fun), Some(bb_id)) = (f_opt, bb_opt) else {
            return;
        };
        let Some(bb) = fun.get_block(bb_id) else {
            return;
        };

        // 1) PHI group must be at head
        let mut seen_non_phi = false;
        for (idx, inst) in bb.instructions.iter().enumerate() {
            match inst {
                MirInstruction::Phi { .. } => {
                    if seen_non_phi {
                        if crate::config::env::builder_schedule_trace() {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.warn(&format!("[block-schedule][verify] WARN: PHI found after non-PHI at bb={:?} idx={}", bb_id, idx));
                        }
                    }
                }
                _ => {
                    seen_non_phi = true;
                }
            }
        }

        // 2) If a Copy is immediately before a Call-like, prefer it to be derived from after-PHIs copy
        let is_call_like = |mi: &MirInstruction| -> bool {
            matches!(mi, MirInstruction::Call { .. })
        };
        for w in bb.instructions.windows(2) {
            if let [MirInstruction::Copy { dst: _, src }, call] = w {
                if is_call_like(call) {
                    // best-effort: src should be one of the after-PHIs materialized ids for this bb
                    let derived_ok = builder.schedule_mat_map.values().any(|&v| v == *src);
                    if !derived_ok {
                        if crate::config::env::builder_schedule_trace() {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.warn(&format!("[block-schedule][verify] WARN: tail Copy src=%{} is not from after-PHIs in bb={:?}", src.0, bb_id));
                        }
                    }
                }
            }
        }
    }
}
