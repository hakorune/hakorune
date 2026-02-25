use super::{observe, origin, utils};
use super::{BasicBlockId, MirBuilder, MirInstruction, ValueId};
use crate::mir::diagnostics::{caller_string, mir_dump_value, FreezeContract};
use crate::mir::BasicBlock;

impl MirBuilder {
    /// Emit an instruction to the current basic block
    #[track_caller]
    pub(in crate::mir::builder) fn emit_instruction(
        &mut self,
        instruction: MirInstruction,
    ) -> Result<(), String> {
        // Capture caller location at function entry (before any closures) for accurate reporting
        let caller = std::panic::Location::caller();

        let block_id = self.current_block.ok_or("No current basic block")?;

        // Make instruction mutable for potential receiver materialization
        let mut instruction = instruction;

        // Precompute debug metadata to avoid borrow conflicts later
        let _dbg_fn_name = self
            .scope_ctx
            .current_function
            .as_ref()
            .map(|f| f.signature.name.clone());
        let _dbg_region_id = self.debug_current_region_id();

        // Fail-fast: emitting into a non-existent block is a hard bug.
        // Previously this could silently drop instructions via `get_block_mut(None)` paths.
        if let Err(e) = self.ensure_block_exists(block_id) {
            return Err(FreezeContract::new("builder/emit_missing_block")
                .field("bb", format!("{:?}", block_id))
                .field("err", e)
                .build());
        }
        // P0: PHI の軽量補強と観測は、関数ブロック取得前に実施して借用競合を避ける
        if let MirInstruction::Phi { dst, inputs, .. } = &instruction {
            origin::phi::propagate_phi_meta(self, *dst, inputs);
            observe::ssa::emit_phi(self, *dst, inputs);
        }

        // CRITICAL: Final receiver materialization for MethodCall
        // This ensures the receiver has an in-block definition in the same block as the Call.
        // Must happen BEFORE function mutable borrow to avoid borrowck conflicts.
        if let MirInstruction::Call {
            callee: Some(callee),
            dst,
            args,
            effects,
            ..
        } = &instruction
        {
            use crate::mir::definitions::call_unified::Callee;
            if let Callee::Method {
                box_name,
                method,
                receiver: Some(r),
                certainty,
                box_kind,
            } = callee.clone()
            {
                // LocalSSA: ensure receiver has a Copy in current_block
                let r_local = crate::mir::builder::ssa::local::recv(self, r);

                // Update instruction with materialized receiver
                let new_callee = Callee::Method {
                    box_name: box_name.clone(),
                    method: method.clone(),
                    receiver: Some(r_local),
                    certainty,
                    box_kind,
                };
                instruction = MirInstruction::Call {
                    dst: *dst,
                    func: crate::mir::ValueId::INVALID, // Legacy dummy (not a real SSA use)
                    callee: Some(new_callee),
                    args: args.clone(),
                    effects: *effects,
                };
            }
        }

        // Record caller only when emission succeeds.
        // If we record before validation/emit and the emit path returns Err, diagnostics may
        // retain non-existent dst ValueIds and trigger false typed-without-def freezes.
        let dst_value_for_metadata = if crate::config::env::joinir_dev::debug_enabled() {
            match &instruction {
                MirInstruction::Const { dst, .. } => Some(*dst),
                MirInstruction::BinOp { dst, .. } => Some(*dst),
                MirInstruction::UnaryOp { dst, .. } => Some(*dst),
                MirInstruction::Compare { dst, .. } => Some(*dst),
                MirInstruction::Load { dst, .. } => Some(*dst),
                MirInstruction::Call { dst, .. } => *dst,
                MirInstruction::Phi { dst, .. } => Some(*dst),
                MirInstruction::NewClosure { dst, .. } => Some(*dst),
                MirInstruction::NewBox { dst, .. } => Some(*dst),
                MirInstruction::TypeOp { dst, .. } => Some(*dst),
                MirInstruction::Copy { dst, .. } => Some(*dst),
                _ => None,
            }
        } else {
            None
        };

        if let Some(ref mut function) = self.scope_ctx.current_function {
            // Fail-fast: non-dominating Copy should not be emitted (strict/dev+planner_required only).
            if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                if let MirInstruction::Copy { src, .. } = &instruction {
                    let def_blocks = crate::mir::verification::utils::compute_def_blocks(function);
                    let dominators = crate::mir::verification::utils::compute_dominators(function);
                    let def_block = def_blocks.get(src).copied();
                    let dominates = def_block
                        .map(|def| dominators.dominates(def, block_id))
                        .unwrap_or(false);
                    if !dominates {
                        return Err(FreezeContract::new("builder/non_dominating_copy")
                            .field("fn", &function.signature.name)
                            .field("bb", format!("{:?}", block_id))
                            .field("src", format!("%{}", src.0))
                            .field("def_block", format!("{:?}", def_block))
                            .build());
                    }
                }
            }
            // Fail-fast: BinOp operands must be defined (strict/dev+planner_required only)
            if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                if let MirInstruction::BinOp { op, lhs, rhs, .. } = &instruction {
                    let def_blocks = crate::mir::verification::utils::compute_def_blocks(function);

                    let check_operand = |name: &str, v: ValueId| -> Result<(), String> {
                        // Treat function params as defined (false positive prevention)
                        if def_blocks.contains_key(&v) || function.params.contains(&v) {
                            Ok(())
                        } else {
                            let span = self.metadata_ctx.current_span();
                            let file = self
                                .metadata_ctx
                                .current_source_file()
                                .unwrap_or_else(|| "unknown".to_string());

                            // Debug-only: dump MIR to help identify where undefined operand is used (written once per error)
                            let mir_dump_path = if crate::config::env::joinir_dev::debug_enabled() {
                                let fn_name_sanitized = sanitize_for_path(&function.signature.name);
                                let pid = std::process::id();
                                let path = format!(
                                    "/tmp/mir_dump_binop_{}_{}.txt",
                                    fn_name_sanitized, pid
                                );
                                if let Ok(mut f) = std::fs::File::create(&path) {
                                    let _ = std::io::Write::write_all(
                                        &mut f,
                                        crate::mir::MirPrinter::new()
                                            .print_function(function)
                                            .as_bytes(),
                                    );
                                    Some(path)
                                } else {
                                    Some("write_failed".to_string())
                                }
                            } else {
                                None
                            };

                            Err(
                                FreezeContract::new("builder/binop_operand_out_of_function_scope")
                                    .field("fn", &function.signature.name)
                                    .field("bb", format!("{:?}", block_id))
                                    .field("op", format!("{:?}", op))
                                    .field("operand", name)
                                    .field("v", format!("%{}", v.0))
                                    .field("span", span.location_string())
                                    .field("span_start", span.start)
                                    .field("span_end", span.end)
                                    .field("file", file)
                                    .field("mir_dump", mir_dump_value(mir_dump_path))
                                    .field("caller", caller_string(caller))
                                    .build(),
                            )
                        }
                    };

                    check_operand("lhs", *lhs)?;
                    check_operand("rhs", *rhs)?;
                }
            }
            // Pre-capture branch/jump targets for predecessor update after we finish
            // mutably borrowing the current block.
            let (then_t, else_t, jump_t) = match &instruction {
                MirInstruction::Branch {
                    then_bb, else_bb, ..
                } => (Some(*then_bb), Some(*else_bb), None),
                MirInstruction::Jump { target, .. } => (None, None, Some(*target)),
                _ => (None, None, None),
            };

            // Extract function name before mutable borrow to avoid borrowck error
            let current_fn_name = function.signature.name.clone();

            if let Some(block) = function.get_block_mut(block_id) {
                // CRITICAL: Copy専用トレース（LocalSSA調査用）
                if let MirInstruction::Copy { dst, src } = &instruction {
                    if crate::config::env::builder_local_ssa_trace() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[emit-inst] fn={} bb={:?} COPY %{} <- %{}",
                            current_fn_name,
                            self.current_block.map(|b| b.0).unwrap_or(0),
                            dst.0,
                            src.0
                        ));
                    }
                }

                // Invariant: Call must always carry a Callee (unified path).
                if let MirInstruction::Call { callee, .. } = &instruction {
                    if callee.is_none() {
                        return Err("builder invariant violated: MirInstruction::Call.callee must be Some (unified call)".into());
                    } else if crate::config::env::builder_local_ssa_trace() {
                        use crate::mir::definitions::call_unified::Callee;
                        if let Some(Callee::Method {
                            box_name,
                            method,
                            receiver: Some(r),
                            ..
                        }) = callee
                        {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[emit-inst] fn={} bb={:?} Call {}.{} recv=%{}",
                                current_fn_name,
                                self.current_block.map(|b| b.0).unwrap_or(0),
                                box_name,
                                method,
                                r.0
                            ));
                        }
                    } else if crate::config::env::builder_trace_recv() {
                        use crate::mir::definitions::call_unified::Callee;
                        if let Some(Callee::Method {
                            box_name,
                            method,
                            receiver: Some(r),
                            ..
                        }) = callee
                        {
                            let names: Vec<String> = self
                                .variable_ctx
                                .variable_map
                                .iter()
                                .filter(|(_, &vid)| vid == *r)
                                .map(|(k, _)| k.clone())
                                .collect();
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!(
                                "[builder/recv-trace] fn={} bb={:?} method={}.{} recv=%{} aliases={:?}",
                                current_fn_name,
                                self.current_block,
                                box_name,
                                method,
                                r.0,
                                names
                            ));
                        }
                    }
                }
                if utils::builder_debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!(
                        "[BUILDER] emit @bb{} -> {}",
                        block_id,
                        match &instruction {
                            MirInstruction::TypeOp { dst, op, value, ty } =>
                                format!("typeop {:?} {} {:?} -> {}", op, value, ty, dst),
                            MirInstruction::Call {
                                func,
                                callee,
                                args,
                                dst,
                                ..
                            } => format!("call {:?} {}({:?}) -> {:?}", callee, func, args, dst),
                            MirInstruction::NewBox {
                                dst,
                                box_type,
                                args,
                            } => format!("new {}({:?}) -> {}", box_type, args, dst),
                            MirInstruction::Const { dst, value } =>
                                format!("const {:?} -> {}", value, dst),
                            MirInstruction::Branch {
                                condition,
                                then_bb,
                                else_bb,
                                ..
                            } => format!("br {}, {}, {}", condition, then_bb, else_bb),
                            MirInstruction::Jump { target, .. } => format!("br {}", target),
                            _ => format!("{:?}", instruction),
                        }
                    ));
                }
                // Phase 136 Step 6/7: Use metadata_ctx for span
                block.add_instruction_with_span(
                    instruction.clone(),
                    self.metadata_ctx.current_span(),
                );
                // Drop the mutable borrow of `block` before updating other blocks
            } else {
                return Err(FreezeContract::new("builder/emit_missing_block")
                    .field("fn", current_fn_name)
                    .field("bb", format!("{:?}", block_id))
                    .field("inst", format!("{:?}", instruction))
                    .build());
            }
            // Update predecessor sets for branch/jump immediately so that
            // debug_verify_phi_inputs can observe a consistent CFG without
            // requiring a full function.update_cfg() pass.
            if let Some(t) = then_t {
                if !function.blocks.contains_key(&t) {
                    function.add_block(BasicBlock::new(t));
                }
                if let Some(succ) = function.get_block_mut(t) {
                    succ.add_predecessor(block_id);
                }
            }
            if let Some(t) = else_t {
                if !function.blocks.contains_key(&t) {
                    function.add_block(BasicBlock::new(t));
                }
                if let Some(succ) = function.get_block_mut(t) {
                    succ.add_predecessor(block_id);
                }
            }
            if let Some(t) = jump_t {
                if !function.blocks.contains_key(&t) {
                    function.add_block(BasicBlock::new(t));
                }
                if let Some(succ) = function.get_block_mut(t) {
                    succ.add_predecessor(block_id);
                }
            }
            if let Some(dst) = dst_value_for_metadata {
                self.metadata_ctx.record_value_caller(dst, caller);
                if let Some(loc) = self.metadata_ctx.value_caller(dst) {
                    function
                        .metadata
                        .value_origin_callers
                        .insert(dst, loc.to_string());
                }
            }
            Ok(())
        } else {
            Err(format!("Basic block {} does not exist", block_id))
        }
    }

    /// Emit an ExternCall instruction with custom effects
    pub(in crate::mir::builder) fn emit_extern_call_with_effects(
        &mut self,
        iface_name: &str,
        method_name: &str,
        args: Vec<ValueId>,
        dst: Option<ValueId>,
        effects: crate::mir::EffectMask,
    ) -> Result<(), String> {
        self.emit_instruction(crate::mir::ssot::extern_call::extern_call(
            dst,
            iface_name.to_string(),
            method_name.to_string(),
            args,
            effects,
        ))
    }

    /// Emit an ExternCall instruction
    /// Standardizes ExternCall creation with consistent effect masking (defaulting to IMPURE).
    pub(in crate::mir::builder) fn emit_extern_call(
        &mut self,
        iface_name: &str,
        method_name: &str,
        args: Vec<ValueId>,
        dst: Option<ValueId>,
    ) -> Result<(), String> {
        // Determine effects based on iface/method (default to IMPURE)
        // Most extern calls are IO/Effectful.
        self.emit_extern_call_with_effects(
            iface_name,
            method_name,
            args,
            dst,
            crate::mir::EffectMask::IO,
        )
    }

    /// Update an existing PHI instruction's inputs (for loop sealing)
    /// Used by loop_lowering.rs Step 4 to patch provisional PHIs with actual inputs
    pub(super) fn update_phi_instruction(
        &mut self,
        block: BasicBlockId,
        phi_id: ValueId,
        new_inputs: Vec<(BasicBlockId, ValueId)>,
    ) -> Result<(), String> {
        if let Some(ref mut function) = self.scope_ctx.current_function {
            if let Some(block_data) = function.get_block_mut(block) {
                // Find PHI instruction with matching dst
                for inst in &mut block_data.instructions {
                    if let MirInstruction::Phi { dst, inputs, .. } = inst {
                        if *dst == phi_id {
                            *inputs = new_inputs;
                            return Ok(());
                        }
                    }
                }
                Err(format!(
                    "PHI instruction {} not found in block {}",
                    phi_id, block
                ))
            } else {
                Err(format!("Block {} not found", block))
            }
        } else {
            Err("No current function".to_string())
        }
    }
}

fn sanitize_for_path(s: &str) -> String {
    // Keep paths ASCII and shell-friendly.
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => c,
            _ => '_',
        })
        .collect()
}
