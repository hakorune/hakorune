/*!
 * MIR Verification - SSA form and semantic verification
 *
 * Implements dominance checking, SSA verification, and semantic analysis
 */

use super::{MirFunction, MirModule};
use crate::debug::log as dlog;
use crate::mir::verification_types::VerificationError;
mod awaits;
mod barrier;
mod cfg;
mod dom;
mod legacy;
mod ssa;
pub(crate) mod utils; // Phase 257 P1-2: Made public for loop_header_phi_builder

/// MIR verifier for SSA form and semantic correctness
pub struct MirVerifier {
    /// Current verification errors
    errors: Vec<VerificationError>,
}

impl MirVerifier {
    /// Create a new MIR verifier
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Verify an entire MIR module
    pub fn verify_module(&mut self, module: &MirModule) -> Result<(), Vec<VerificationError>> {
        self.errors.clear();

        // Stage‑B/selfhost 専用: dev verify を一時緩和するためのトグル
        if !crate::config::env::stageb_dev_verify_enabled() {
            return Ok(());
        }

        for (_name, function) in &module.functions {
            if let Err(mut func_errors) = self.verify_function(function) {
                // Dev-only trace: BreakFinderBox / LoopSSA 周辺のSSAバグを詳細に観測する。
                //
                // NYASH_BREAKFINDER_SSA_TRACE=1 のときだけ有効になり、
                // compiler_stageb.hako など大規模モジュール内での
                // BreakFinderBox.* に対する UndefinedValue を詳細に出力する。
                //
                // 併せて、同じトグルで「任意の関数」に対する UndefinedValue も
                // 簡易ログとして出力し、どの関数で支配関係が崩れているかを
                // 追いやすくしている（箱理論の観測レイヤー強化）。
                if std::env::var("NYASH_BREAKFINDER_SSA_TRACE").ok().as_deref() == Some("1") {
                    let log = crate::runtime::get_global_ring0().log.clone();
                    // 1) BreakFinderBox / LoopSSA 向けの詳細ログ
                    if function.signature.name.starts_with("BreakFinderBox.")
                        || function.signature.name.starts_with("LoopSSA.")
                    {
                        for e in &func_errors {
                            if let VerificationError::UndefinedValue {
                                value,
                                block,
                                instruction_index,
                            } = e
                            {
                                if let Some(bb) = function.blocks.get(block) {
                                    let inst = bb.instructions.get(*instruction_index);
                                    log.debug(&format!(
                                        "[breakfinder/ssa] UndefinedValue {:?} in fn {} at bb={:?}, inst={} => {:?}",
                                        value,
                                        function.signature.name,
                                        block,
                                        instruction_index,
                                        inst,
                                    ));
                                } else {
                                    log.debug(&format!(
                                        "[breakfinder/ssa] UndefinedValue {:?} in fn {} at bb={:?}, inst={}",
                                        value,
                                        function.signature.name,
                                        block,
                                        instruction_index
                                    ));
                                }
                            }
                        }
                    }

                    // 2) 任意の関数向けの簡易 UndefinedValue ログ
                    for e in &func_errors {
                        match e {
                            VerificationError::UndefinedValue {
                                value,
                                block,
                                instruction_index,
                            } => {
                                log.debug(&format!(
                                    "[mir-ssa-debug] UndefinedValue {:?} in fn {} at bb={:?}, inst={}",
                                    value,
                                    function.signature.name,
                                    block,
                                    instruction_index
                                ));
                                if let Some(bb) = function.blocks.get(block) {
                                    let inst_opt = bb
                                        .all_spanned_instructions_enumerated()
                                        .nth(*instruction_index);
                                    if let Some((_idx, sp)) = inst_opt {
                                        log.debug(&format!(
                                            "[mir-ssa-debug-inst]   inst={:?}",
                                            sp.inst
                                        ));
                                    }
                                }
                            }
                            VerificationError::DominatorViolation {
                                value,
                                use_block,
                                def_block,
                            } => {
                                log.debug(&format!(
                                    "[mir-ssa-debug] DominatorViolation {:?} in fn {}: use_block={:?}, def_block={:?}",
                                    value,
                                    function.signature.name,
                                    use_block,
                                    def_block
                                ));
                            }
                            _ => {}
                        }
                    }
                }

                // Add function context to errors
                for _error in &mut func_errors {
                    // Could add function name to error context here
                }
                self.errors.extend(func_errors);
            }
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Verify a single MIR function
    pub fn verify_function(
        &mut self,
        function: &MirFunction,
    ) -> Result<(), Vec<VerificationError>> {
        let mut local_errors = Vec::new();

        // Dominator computation is expensive; compute once per function and reuse.
        let (preds, def_block, dominators) = if crate::config::env::verify_allow_no_phi() {
            (None, None, None)
        } else {
            (
                Some(utils::compute_predecessors(function)),
                Some(utils::compute_def_blocks(function)),
                Some(utils::compute_dominators(function)),
            )
        };

        // 1. Check SSA form
        if let Err(mut ssa_errors) = self.verify_ssa_form(function) {
            local_errors.append(&mut ssa_errors);
        }

        // 2. Check dominance relations
        if let Some((def_block, dominators)) = def_block.as_ref().zip(dominators.as_ref()) {
            if let Err(mut dom_errors) = dom::check_dominance_with(function, def_block, dominators)
            {
                local_errors.append(&mut dom_errors);
            }
        } else if let Err(mut dom_errors) = self.verify_dominance(function) {
            local_errors.append(&mut dom_errors);
        }

        // 3. Check control flow integrity
        if let Err(mut cfg_errors) = self.verify_control_flow(function) {
            local_errors.append(&mut cfg_errors);
        }

        // Phase 257 P1-1: PHI predecessor validation
        if let Err(mut phi_errors) = cfg::check_phi_predecessors(function) {
            local_errors.append(&mut phi_errors);
        }

        // 4. Check merge-block value usage (ensure Phi is used)
        if let Some((preds, def_block, dominators)) = preds
            .as_ref()
            .zip(def_block.as_ref())
            .zip(dominators.as_ref())
            .map(|((p, d), doms)| (p, d, doms))
        {
            if let Err(mut merge_errors) =
                cfg::check_merge_uses_with(function, preds, def_block, dominators)
            {
                local_errors.append(&mut merge_errors);
            }
        } else if let Err(mut merge_errors) = self.verify_merge_uses(function) {
            local_errors.append(&mut merge_errors);
        }
        // 5. Minimal checks for WeakRef/Barrier
        if let Err(mut weak_barrier_errors) = self.verify_weakref_and_barrier(function) {
            local_errors.append(&mut weak_barrier_errors);
        }
        // 6. Light barrier-context diagnostic (strict mode only)
        if let Err(mut barrier_ctx) = self.verify_barrier_context(function) {
            local_errors.append(&mut barrier_ctx);
        }
        // 7. Forbid legacy instructions (must be rewritten to Core-15)
        if let Err(mut legacy_errors) = self.verify_no_legacy_ops(function) {
            local_errors.append(&mut legacy_errors);
        }
        // 8. Async semantics: ensure checkpoints around await
        if let Err(mut await_cp) = self.verify_await_checkpoints(function) {
            local_errors.append(&mut await_cp);
        }

        // 9. PHI-off strict edge-copy policy (optional)
        if crate::config::env::mir_no_phi() && crate::config::env::verify_edge_copy_strict() {
            if let Err(mut ecs) = self.verify_edge_copy_strict(function) {
                local_errors.append(&mut ecs);
            }
        }

        // 10. Ret-block purity (optional, dev-only)
        if crate::config::env::verify_ret_purity() {
            if let Err(mut rpe) = self.verify_ret_block_purity(function) {
                local_errors.append(&mut rpe);
            }
        }

        if local_errors.is_empty() {
            Ok(())
        } else {
            if dlog::on("NYASH_DEBUG_VERIFIER") {
                let log = crate::runtime::get_global_ring0().log.clone();
                log.debug(&format!(
                    "[VERIFY] {} errors in function {}",
                    local_errors.len(),
                    function.signature.name
                ));
                for e in &local_errors {
                    match e {
                        VerificationError::MergeUsesPredecessorValue {
                            value,
                            merge_block,
                            pred_block,
                        } => {
                            log.debug(&format!(
                                "  • MergeUsesPredecessorValue: value=%{:?} merge_bb={:?} pred_bb={:?} -- hint: insert/use Phi in merge block for values from predecessors",
                                value, merge_block, pred_block
                            ));
                        }
                        VerificationError::DominatorViolation {
                            value,
                            use_block,
                            def_block,
                        } => {
                            log.debug(&format!(
                                "  • DominatorViolation: value=%{:?} use_bb={:?} def_bb={:?} -- hint: ensure definition dominates use, or route via Phi",
                                value, use_block, def_block
                            ));
                        }
                        VerificationError::InvalidPhi {
                            phi_value,
                            block,
                            reason,
                        } => {
                            log.debug(&format!(
                                "  • InvalidPhi: phi_dst=%{:?} in bb={:?} reason={} -- hint: check inputs cover all predecessors and placed at block start",
                                phi_value, block, reason
                            ));
                        }
                        VerificationError::InvalidWeakRefSource {
                            weak_ref,
                            block,
                            instruction_index,
                            reason,
                        } => {
                            log.debug(&format!(
                                "  • InvalidWeakRefSource: weak=%{:?} at {}:{} reason='{}' -- hint: source must be WeakRef(new); ensure creation precedes load and value flows correctly",
                                weak_ref, block, instruction_index, reason
                            ));
                        }
                        VerificationError::InvalidBarrierPointer {
                            ptr,
                            block,
                            instruction_index,
                            reason,
                        } => {
                            log.debug(&format!(
                                "  • InvalidBarrierPointer: ptr=%{:?} at {}:{} reason='{}' -- hint: barrier pointer must be a valid ref (not void/null); ensure it is defined and non-void",
                                ptr, block, instruction_index, reason
                            ));
                        }
                        VerificationError::SuspiciousBarrierContext {
                            block,
                            instruction_index,
                            note,
                        } => {
                            log.debug(&format!(
                                "  • SuspiciousBarrierContext: at {}:{} note='{}' -- hint: place barrier within ±2 of load/store/ref ops in same block or disable strict check",
                                block, instruction_index, note
                            ));
                        }
                        other => {
                            log.debug(&format!("  • {:?}", other));
                        }
                    }
                }
            }
            Err(local_errors)
        }
    }

    /// When PHI-off strict mode is enabled, enforce that merge blocks do not
    /// introduce self-copies and that each predecessor provides a Copy into the
    /// merged destination for values used in the merge block that do not dominate it.
    fn verify_edge_copy_strict(
        &self,
        function: &MirFunction,
    ) -> Result<(), Vec<VerificationError>> {
        let mut errors = Vec::new();
        let preds = utils::compute_predecessors(function);
        let def_block = utils::compute_def_blocks(function);
        let dominators = utils::compute_dominators(function);

        for (merge_bid, merge_bb) in &function.blocks {
            let p = preds.get(merge_bid).cloned().unwrap_or_default();
            if p.len() < 2 {
                continue; // Only enforce on real merges (>=2 predecessors)
            }

            // Collect values used in merge block
            let mut used_values = std::collections::HashSet::new();
            for sp in merge_bb.all_spanned_instructions() {
                for v in sp.inst.used_values() {
                    used_values.insert(v);
                }
            }

            // For each used value that doesn't dominate the merge block, enforce edge-copy policy
            for &u in &used_values {
                if let Some(&def_bb) = def_block.get(&u) {
                    // If the def dominates the merge block, edge copies are not required
                    let dominated = dominators.dominates(def_bb, *merge_bid);
                    if dominated && def_bb != *merge_bid {
                        continue;
                    }
                }

                // Merge block itself must not contain a Copy to the merged value
                let has_self_copy_in_merge = merge_bb.instructions.iter().any(
                    |inst| matches!(inst, super::MirInstruction::Copy { dst, .. } if *dst == u),
                );
                if has_self_copy_in_merge {
                    errors.push(VerificationError::EdgeCopyStrictViolation {
                        block: *merge_bid,
                        value: u,
                        pred_block: None,
                        reason:
                            "merge block contains Copy to merged value; use predecessor copies only"
                                .to_string(),
                    });
                }

                // Each predecessor must provide a Copy into the merged destination
                for pred in &p {
                    let Some(pbb) = function.blocks.get(pred) else {
                        continue;
                    };
                    let has_copy = pbb.instructions.iter().any(|inst| {
                        matches!(
                            inst,
                            super::MirInstruction::Copy { dst, .. } if *dst == u
                        )
                    });
                    if !has_copy {
                        errors.push(VerificationError::MergeUsesPredecessorValue {
                            value: u,
                            merge_block: *merge_bid,
                            pred_block: *pred,
                        });
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Verify that any block ending with Return contains no side-effecting instructions before it.
    /// Allowed before Return: Const, Copy, Phi only. Others are considered side-effecting for this policy.
    fn verify_ret_block_purity(
        &self,
        function: &MirFunction,
    ) -> Result<(), Vec<VerificationError>> {
        use super::MirInstruction as I;
        let mut errors = Vec::new();
        for (bid, bb) in &function.blocks {
            if let Some(I::Return { .. }) = bb.terminator {
                for (idx, inst) in bb.instructions.iter().enumerate() {
                    let allowed = matches!(inst, I::Const { .. } | I::Copy { .. } | I::Phi { .. });
                    if !allowed {
                        let name = format!("{:?}", inst);
                        errors.push(VerificationError::RetBlockSideEffect {
                            block: *bid,
                            instruction_index: idx,
                            name,
                        });
                    }
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Reject legacy instructions that should be rewritten to Core-15 equivalents
    /// Skips check when NYASH_VERIFY_ALLOW_LEGACY=1
    fn verify_no_legacy_ops(&self, function: &MirFunction) -> Result<(), Vec<VerificationError>> {
        legacy::check_no_legacy_ops(function)
    }

    /// Ensure that each Await instruction (or ExternCall(env.future.await)) is immediately
    /// preceded and followed by a checkpoint.
    /// A checkpoint is either MirInstruction::Safepoint or ExternCall("env.runtime", "checkpoint").
    fn verify_await_checkpoints(
        &self,
        function: &MirFunction,
    ) -> Result<(), Vec<VerificationError>> {
        awaits::check_await_checkpoints(function)
    }

    /// Verify WeakRef/Barrier minimal semantics
    fn verify_weakref_and_barrier(
        &self,
        function: &MirFunction,
    ) -> Result<(), Vec<VerificationError>> {
        barrier::check_weakref_and_barrier(function)
    }

    /// Light diagnostic: Barrier should be near memory ops in the same block (best-effort)
    /// Enabled only when NYASH_VERIFY_BARRIER_STRICT=1
    fn verify_barrier_context(&self, function: &MirFunction) -> Result<(), Vec<VerificationError>> {
        barrier::check_barrier_context(function)
    }

    /// Verify SSA form properties
    fn verify_ssa_form(&self, function: &MirFunction) -> Result<(), Vec<VerificationError>> {
        ssa::check_ssa_form(function)
    }

    /// Verify dominance relations (def must dominate use across blocks)
    fn verify_dominance(&self, function: &MirFunction) -> Result<(), Vec<VerificationError>> {
        dom::check_dominance(function)
    }

    /// Verify control flow graph integrity
    fn verify_control_flow(&self, function: &MirFunction) -> Result<(), Vec<VerificationError>> {
        cfg::check_control_flow(function)
    }

    /// Verify that blocks with multiple predecessors do not use predecessor-defined values directly.
    /// In merge blocks, values coming from predecessors must be routed through Phi.
    fn verify_merge_uses(&self, function: &MirFunction) -> Result<(), Vec<VerificationError>> {
        cfg::check_merge_uses(function)
    }

    /// Get all verification errors from the last run
    pub fn get_errors(&self) -> &[VerificationError] {
        &self.errors
    }

    /// Clear verification errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
}

impl Default for MirVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
