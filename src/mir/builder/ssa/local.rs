use crate::mir::builder::ssa::phi_input_contract;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirInstruction, ValueId};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LocalKind {
    Recv,
    Arg,
    CompareOperand,
    Cond,
    FieldBase,
    Other(u8),
}

impl LocalKind {
    #[inline]
    fn tag(self) -> u8 {
        match self {
            LocalKind::Recv => 0,
            LocalKind::Arg => 1,
            LocalKind::CompareOperand => 2,
            LocalKind::Cond => 4,
            LocalKind::FieldBase => 0, // share recv slot for bases
            LocalKind::Other(k) => k,
        }
    }
}

#[inline]
fn strict_planner_required() -> bool {
    crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled()
}

fn format_value_ids(values: &[ValueId]) -> String {
    let mut out = String::from("[");
    for (idx, v) in values.iter().enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push('%');
        out.push_str(&v.0.to_string());
    }
    out.push(']');
    out
}

#[inline]
fn def_inst_kind(inst: &MirInstruction) -> &'static str {
    match inst {
        MirInstruction::Const { .. } => "Const",
        MirInstruction::BinOp { .. } => "BinOp",
        MirInstruction::UnaryOp { .. } => "UnaryOp",
        MirInstruction::Compare { .. } => "Compare",
        MirInstruction::Copy { .. } => "Copy",
        MirInstruction::FieldGet { .. } => "FieldGet",
        MirInstruction::FieldSet { .. } => "FieldSet",
        MirInstruction::VariantMake { .. } => "VariantMake",
        MirInstruction::VariantTag { .. } => "VariantTag",
        MirInstruction::VariantProject { .. } => "VariantProject",
        MirInstruction::Load { .. } => "Load",
        MirInstruction::Store { .. } => "Store",
        MirInstruction::Call { .. } => "Call",
        MirInstruction::NewClosure { .. } => "NewClosure",
        MirInstruction::Branch { .. } => "Branch",
        MirInstruction::Jump { .. } => "Jump",
        MirInstruction::Return { .. } => "Return",
        MirInstruction::Phi { .. } => "Phi",
        MirInstruction::NewBox { .. } => "NewBox",
        MirInstruction::TypeOp { .. } => "TypeOp",
        MirInstruction::Debug { .. } => "Debug",
        MirInstruction::KeepAlive { .. } => "KeepAlive",
        MirInstruction::ReleaseStrong { .. } => "ReleaseStrong",
        MirInstruction::Throw { .. } => "Throw",
        MirInstruction::Catch { .. } => "Catch",
        MirInstruction::Safepoint => "Safepoint",
        MirInstruction::RefNew { .. } => "RefNew",
        MirInstruction::WeakRef { .. } => "WeakRef",
        MirInstruction::Barrier { .. } => "Barrier",
        MirInstruction::FutureNew { .. } => "FutureNew",
        MirInstruction::FutureSet { .. } => "FutureSet",
        MirInstruction::Await { .. } => "Await",
        MirInstruction::Select { .. } => "Select",
    }
}

/// Ensure a value has an in-block definition and cache it per (bb, orig, kind).
/// Always emits a Copy in the current block when not cached.
pub fn ensure(builder: &mut MirBuilder, v: ValueId, kind: LocalKind) -> ValueId {
    ensure_inner(builder, v, kind, false).unwrap_or(v)
}

pub fn try_ensure(
    builder: &mut MirBuilder,
    v: ValueId,
    kind: LocalKind,
) -> Result<ValueId, String> {
    ensure_inner(
        builder,
        v,
        kind,
        crate::config::env::joinir_dev::strict_planner_required_debug_enabled()
            && kind == LocalKind::Arg,
    )
}

fn ensure_inner(
    builder: &mut MirBuilder,
    v: ValueId,
    kind: LocalKind,
    forbid_non_pure: bool,
) -> Result<ValueId, String> {
    let bb_opt = builder.current_block;

    // Get function name and entry block for logging (debug only)
    // Clone to avoid borrow checker issues with later mutable borrows
    let (fn_name, fn_entry) = if let Some(func) = builder.scope_ctx.current_function.as_ref() {
        (func.signature.name.clone(), func.entry_block)
    } else {
        ("<unknown>".to_string(), crate::mir::BasicBlockId(0))
    };

    if let Some(bb) = bb_opt {
        if crate::config::env::builder_local_ssa_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[local-ssa] ensure bb={:?} kind={:?} v=%{}",
                bb, kind, v.0
            ));
        }
        let key = (bb, v, kind.tag());
        if let Some(&loc) = builder.local_ssa_map.get(&key) {
            return Ok(loc);
        }

        // Ensure the current basic block exists in the function before emitting a Copy.
        // Stage-B 経路などでは current_block が割り当て済みでも、ブロック自体が
        // function にまだ追加されていない場合があり、そのまま emit_instruction すると
        // Copy が黙って落ちてしまう。ここで best-effort で作成しておく。
        // CRITICAL: Check for errors - if block creation fails, return original value.
        if let Err(e) = builder.ensure_block_exists(bb) {
            if crate::config::env::builder_local_ssa_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[local-ssa] ensure_block_exists FAILED bb={:?} kind={:?} v=%{} err={}",
                    bb, kind, v.0, e
                ));
            }
            return Ok(v);
        }

        // CRITICAL FIX: If `v` is from a pinned slot, check if there's a PHI value for that slot
        // in the current block's variable_ctx.variable_map. If so, use the PHI value directly instead of
        // emitting a Copy from the old value (which might not be defined in this block).
        // Try to detect pinned slots for this value and redirect to the latest slot value.
        // 1) First, look for "__pin$" entries in variable_ctx.variable_map that still point to v.
        // 2) If not found, consult builder.pin_slot_names to recover the slot name
        //    and then look up the current ValueId for that slot.
        let mut slot_name_opt: Option<String> = None;

        let names_for_v: Vec<String> = builder
            .variable_ctx
            .variable_map
            .iter()
            .filter(|(k, &vid)| vid == v && k.starts_with("__pin$"))
            .map(|(k, _)| k.clone())
            .collect();

        if let Some(first_pin_name) = names_for_v.first() {
            slot_name_opt = Some(first_pin_name.clone());
        } else if let Some(name) = builder.pin_slot_names.get(&v) {
            slot_name_opt = Some(name.clone());
        }

        if let Some(slot_name) = slot_name_opt {
            if let Some(&current_val) = builder.variable_ctx.variable_map.get(&slot_name) {
                if current_val != v {
                    // The slot has been updated (likely by a PHI or header rewrite).
                    // Use the updated value instead of the stale pinned ValueId.
                    if crate::config::env::builder_local_ssa_trace() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[local-ssa] phi-redirect bb={:?} kind={:?} slot={} %{} -> %{}",
                            bb, kind, slot_name, v.0, current_val.0
                        ));
                    }
                    if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!("[local-sa:ensure:phi_redirect] fn={} entry={:?} bb={:?} kind={:?} slot={} v=%{} current_val=%{}",
                            fn_name, fn_entry, bb, kind, slot_name, v.0, current_val.0));
                    }
                    builder.local_ssa_map.insert(key, current_val);
                    // Unconditional trace for phi-redirect cache hit
                    if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!("[local-sa:ensure:cache_hit] fn={} entry={:?} bb={:?} kind={:?} returning_cached_phi_val=%{}",
                            fn_name, fn_entry, bb, kind, current_val.0));
                    }
                    return Ok(current_val);
                }
            }
        }

        let loc = builder.next_value_id();

        // Unconditional trace to observe all ValueId allocations in ensure()
        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[local-sa:ensure:alloc] fn={} entry={:?} bb={:?} kind={:?} v=%{} loc=%{}",
                fn_name, fn_entry, bb, kind, v.0, loc.0
            ));
        }

        // Removed: [local-sa:ensure:GHOST_v36_input] observation (PHI issue resolved)

        // Find a definition for `v` so we can rematerialize it (pure) or validate dominance (non-pure).
        // Keep the scan local to avoid additional indexing and reuse the existing walk.
        let mut def_inst: Option<MirInstruction> = None;
        let mut def_block: Option<crate::mir::BasicBlockId> = None;
        let mut def_kind: &'static str = "NotFound";
        if let Some(func) = builder.scope_ctx.current_function.as_ref() {
            if func.params.iter().any(|pid| *pid == v) {
                def_kind = "Param";
                def_block = Some(func.entry_block);
            } else {
                'scan: for (bid, block) in func.blocks.iter() {
                    for inst in &block.instructions {
                        if inst.dst_value() == Some(v) {
                            def_kind = def_inst_kind(inst);
                            def_block = Some(*bid);
                            def_inst = Some(inst.clone());
                            break 'scan;
                        }
                    }
                    if let Some(term) = &block.terminator {
                        if term.dst_value() == Some(v) {
                            def_kind = def_inst_kind(term);
                            def_block = Some(*bid);
                            def_inst = Some(term.clone());
                            break 'scan;
                        }
                    }
                }
            }
        }

        let non_rematerializable = match &def_inst {
            Some(MirInstruction::Const { .. }) => false,
            Some(MirInstruction::BinOp { .. }) => false,
            Some(MirInstruction::Compare { .. }) => false,
            Some(MirInstruction::Copy { .. }) => false,
            Some(MirInstruction::Select { .. }) => false,
            _ => true,
        };

        if forbid_non_pure && non_rematerializable {
            let fn_name = builder
                .scope_ctx
                .current_function
                .as_ref()
                .map(|f| f.signature.name.as_str())
                .unwrap_or("<unknown>");
            let dominates = if let (Some(func), Some(def_block)) =
                (builder.scope_ctx.current_function.as_ref(), def_block)
            {
                let dominators = crate::mir::verification::utils::compute_dominators(func);
                dominators.dominates(def_block, bb)
            } else {
                false
            };
            if dominates {
                // Non-pure but dominating defs are SSA-safe; allow Copy fallback.
            } else {
                let def_block_label = def_block
                    .map(|b| format!("{:?}", b))
                    .unwrap_or_else(|| "None".to_string());

                // Diagnostic-only context: help pinpoint how an out-of-scope ValueId leaks into args.
                // Keep output stable + short: at most 3 variable_map hits.
                let mut varmap_hits: Vec<&str> = builder
                    .variable_ctx
                    .variable_map
                    .iter()
                    .filter_map(|(name, &vid)| if vid == v { Some(name.as_str()) } else { None })
                    .collect();
                varmap_hits.sort_unstable();
                if varmap_hits.len() > 3 {
                    varmap_hits.truncate(3);
                }
                let varmap_hits_str = if varmap_hits.is_empty() {
                    "[]".to_string()
                } else {
                    format!("[{}]", varmap_hits.join(","))
                };

                let pin = builder
                    .pin_slot_names
                    .get(&v)
                    .map(|s| s.as_str())
                    .unwrap_or("none");

                let alloc_context = if def_kind == "NotFound" && def_block.is_none() {
                    let has_type = builder.type_ctx.value_types.contains_key(&v);
                    let has_origin_newbox = builder.type_ctx.value_origin_newbox.contains_key(&v);
                    let reserved = builder.comp_ctx.reserved_value_ids.contains(&v);
                    let next_value_id_hint = builder
                        .scope_ctx
                        .current_function
                        .as_ref()
                        .map(|f| f.next_value_id)
                        .unwrap_or(0);
                    let (def_blocks_has, def_blocks_bb) = if let Some(func) =
                        builder.scope_ctx.current_function.as_ref()
                    {
                        let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
                        let bb = def_blocks.get(&v).copied();
                        (bb.is_some(), bb)
                    } else {
                        (false, None)
                    };
                    let def_blocks_bb = def_blocks_bb
                        .map(|b| format!("{:?}", b))
                        .unwrap_or_else(|| "none".to_string());
                    format!(
                        " has_type={} has_origin_newbox={} reserved={} next_value_id_hint={} def_blocks_has={} def_blocks_bb={}",
                        has_type,
                        has_origin_newbox,
                        reserved,
                        next_value_id_hint,
                        if def_blocks_has { "yes" } else { "no" },
                        def_blocks_bb
                    )
                } else {
                    String::new()
                };

                return Err(format!(
                    "[freeze:contract][local_ssa/non_rematerializable_arg] fn={} bb={:?} kind={:?} v=%{} def_kind={} def_block={} varmap_hits={} pin={}{}",
                    fn_name,
                    bb,
                    kind,
                    v.0,
                    def_kind,
                    def_block_label,
                    varmap_hits_str,
                    pin,
                    alloc_context
                ));
            }
        }

        // CRITICAL: Check emit_instruction result - if emission fails, return original value
        // to avoid returning undefined ValueId.
        let emit_res = match def_inst {
            Some(MirInstruction::Const { value, .. }) => {
                builder.emit_instruction(MirInstruction::Const { dst: loc, value })
            }
            Some(MirInstruction::BinOp { op, lhs, rhs, .. }) => {
                let lhs_local = arg(builder, lhs);
                let rhs_local = arg(builder, rhs);
                builder.emit_instruction(MirInstruction::BinOp {
                    dst: loc,
                    op,
                    lhs: lhs_local,
                    rhs: rhs_local,
                })
            }
            Some(MirInstruction::Compare { op, lhs, rhs, .. }) => {
                let mut lhs_local = lhs;
                let mut rhs_local = rhs;
                finalize_compare(builder, &mut lhs_local, &mut rhs_local)?;
                builder.emit_instruction(MirInstruction::Compare {
                    dst: loc,
                    op,
                    lhs: lhs_local,
                    rhs: rhs_local,
                })
            }
            Some(MirInstruction::Select {
                cond,
                then_val,
                else_val,
                ..
            }) => {
                let cond_local = ensure_inner(builder, cond, LocalKind::Cond, forbid_non_pure)?;
                let then_local = ensure_inner(builder, then_val, kind, forbid_non_pure)?;
                let else_local = ensure_inner(builder, else_val, kind, forbid_non_pure)?;
                builder.emit_instruction(MirInstruction::Select {
                    dst: loc,
                    cond: cond_local,
                    then_val: then_local,
                    else_val: else_local,
                })
            }
            Some(MirInstruction::Copy { src, .. }) => {
                let src_local = if src == v {
                    src
                } else {
                    ensure_inner(builder, src, kind, forbid_non_pure)?
                };
                builder.emit_instruction(MirInstruction::Copy {
                    dst: loc,
                    src: src_local,
                })
            }
            _ => {
                // Fail-fast: check dominance before fallback Copy (strict/dev+planner_required)
                // Only check for specific def_kind that are known to cause dominance issues
                // to avoid expensive dominance computation on every fallback Copy.
                if strict_planner_required() && def_kind == "Call" {
                    // Debug trace for Call fallback Copy path
                    if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!("[local-sa:ensure:call_fallback] fn={} entry={:?} bb={:?} kind={:?} v=%{} def_block={:?} def_kind={}",
                            fn_name, fn_entry, bb, kind, v.0, def_block, def_kind));
                    }
                    if let Some(def_b) = def_block {
                        let dominates =
                            if let Some(func) = builder.scope_ctx.current_function.as_ref() {
                                let dominators =
                                    crate::mir::verification::utils::compute_dominators(func);
                                dominators.dominates(def_b, bb)
                            } else {
                                false
                            };
                        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                            let ring0 = crate::runtime::get_global_ring0();
                            ring0.log.debug(&format!("[local-sa:ensure:call_dominance] fn={} bb={:?} def_block={:?} dominates={}",
                                fn_name, bb, def_b, dominates));
                        }
                        if !dominates {
                            return Err(format!(
                                "[freeze:contract][local_ssa/non_dominating_copy] fn={} bb={:?} src=%{} def_block={:?} def_kind={}",
                                fn_name, bb, v.0, def_b, def_kind
                            ));
                        }
                    }
                }
                builder.emit_instruction(MirInstruction::Copy { dst: loc, src: v })
            }
        };

        if let Err(e) = emit_res {
            if crate::config::env::builder_local_ssa_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[local-ssa] emit_instruction FAILED bb={:?} kind={:?} v=%{} dst=%{} err={}",
                    bb, kind, v.0, loc.0, e
                ));
            }
            // Debug trace for emission failure
            if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!("[local-sa:ensure:fail] fn={} entry={:?} bb={:?} kind={:?} v=%{} loc=%{} returning_v",
                    fn_name, fn_entry, bb, kind, v.0, loc.0));
            }
            // Failed to emit Copy - return original value instead of undefined dst
            return Ok(v);
        }
        if crate::config::env::builder_local_ssa_trace() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[local-ssa] copy  bb={:?} kind={:?} %{} -> %{}",
                bb, kind, v.0, loc.0
            ));
        }
        // Debug trace for emission success
        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[local-sa:ensure:success] fn={} entry={:?} bb={:?} kind={:?} v=%{} loc=%{} returning_loc",
                fn_name, fn_entry, bb, kind, v.0, loc.0));
        }
        // Success: register metadata and cache
        if let Some(t) = builder.type_ctx.value_types.get(&v).cloned() {
            builder.type_ctx.value_types.insert(loc, t);
        }
        if let Some(cls) = builder.type_ctx.value_origin_newbox.get(&v).cloned() {
            builder
                .type_ctx
                .value_origin_newbox
                .insert(loc, cls.clone());

            // CRITICAL FIX: For receiver kind, if type is missing but origin exists,
            // infer MirType::Box from origin
            if kind == LocalKind::Recv && builder.type_ctx.value_types.get(&loc).is_none() {
                builder
                    .type_ctx
                    .value_types
                    .insert(loc, crate::mir::MirType::Box(cls));
            }
        }
        if let Some(text) = builder.type_ctx.string_literals.get(&v).cloned() {
            builder.type_ctx.string_literals.insert(loc, text);
        }
        if let Some(map_value_type) = builder.type_ctx.map_value_types.get(&v).cloned() {
            builder.type_ctx.map_value_types.insert(loc, map_value_type);
        }
        let literal_facts: Vec<(String, crate::mir::MirType)> = builder
            .type_ctx
            .map_literal_value_types
            .iter()
            .filter(|((value_id, _), _)| *value_id == v)
            .map(|((_, key), ty)| (key.clone(), ty.clone()))
            .collect();
        for (key, ty) in literal_facts {
            builder
                .type_ctx
                .map_literal_value_types
                .insert((loc, key), ty);
        }
        builder.local_ssa_map.insert(key, loc);
        // Debug trace for newly created loc
        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[local-sa:ensure:new_loc] fn={} entry={:?} bb={:?} kind={:?} v=%{} returning_loc=%{}",
                fn_name, fn_entry, bb, kind, v.0, loc.0));
        }
        Ok(loc)
    } else {
        // bb is None - no current block, return original value
        if crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[local-sa:ensure:no_bb] fn={} entry={:?} kind={:?} v=%{} returning_v (no current block)",
                fn_name, fn_entry, kind, v.0));
        }
        Ok(v)
    }
}

#[inline]
pub fn recv(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::Recv)
}

#[inline]
pub fn arg(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::Arg)
}

#[inline]
pub fn cond(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::Cond)
}

#[inline]
pub fn field_base(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::FieldBase)
}

#[inline]
pub fn cmp_operand(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::CompareOperand)
}

/// Finalize only the args (legacy Call paths)
pub fn finalize_args(builder: &mut MirBuilder, args: &mut Vec<ValueId>) -> Result<(), String> {
    let args_list = if strict_planner_required() {
        Some(format_value_ids(args))
    } else {
        None
    };
    for a in args.iter_mut() {
        if strict_planner_required() {
            match try_ensure(builder, *a, LocalKind::Arg) {
                Ok(v) => *a = v,
                Err(e) => {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let (fn_name, params, entry) = builder
                            .scope_ctx
                            .current_function
                            .as_ref()
                            .map(|f| {
                                (
                                    f.signature.name.as_str(),
                                    f.params.as_slice(),
                                    Some(f.entry_block),
                                )
                            })
                            .unwrap_or(("<unknown>", &[][..], None));
                        let params_list = format_value_ids(params);
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[local-ssa/arg-context] fn={} bb={:?} kind=Arg v=%{} args={} params={} entry={:?}",
                            fn_name,
                            builder.current_block,
                            a.0,
                            args_list.as_deref().unwrap_or("[]"),
                            params_list,
                            entry
                        ));
                    }
                    return Err(e);
                }
            }
        } else {
            *a = arg(builder, *a);
        }
    }
    Ok(())
}

fn check_non_dominating_use(
    builder: &mut MirBuilder,
    v: ValueId,
    kind_label: &'static str,
) -> Result<(), String> {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return Ok(());
    }
    let bb = match builder.current_block {
        Some(bb) => bb,
        None => return Ok(()),
    };
    let (func_name, def_block_opt, def_kind, phi_inputs_opt) = {
        let Some(func) = builder.scope_ctx.current_function.as_ref() else {
            return Ok(());
        };
        let func_name = func.signature.name.clone();
        let mut def_kind: &'static str = "NotFound";
        let mut def_block_opt: Option<crate::mir::BasicBlockId> = None;
        let mut phi_inputs_opt: Option<Vec<(crate::mir::BasicBlockId, ValueId)>> = None;
        if func.params.iter().any(|pid| *pid == v) {
            def_kind = "Param";
            def_block_opt = Some(func.entry_block);
        } else {
            'scan: for (bid, block) in func.blocks.iter() {
                for inst in &block.instructions {
                    if inst.dst_value() == Some(v) {
                        def_kind = def_inst_kind(inst);
                        def_block_opt = Some(*bid);
                        if let crate::mir::MirInstruction::Phi { inputs, .. } = inst {
                            phi_inputs_opt = Some(inputs.clone());
                        }
                        break 'scan;
                    }
                }
                if let Some(term) = &block.terminator {
                    if term.dst_value() == Some(v) {
                        def_kind = def_inst_kind(term);
                        def_block_opt = Some(*bid);
                        break 'scan;
                    }
                }
            }
        }
        (func_name, def_block_opt, def_kind, phi_inputs_opt)
    };

    // Only check Phi-defined values to keep this path light.
    if def_kind != "Phi" {
        return Ok(());
    }

    let Some(func) = builder.scope_ctx.current_function.as_ref() else {
        return Ok(());
    };
    let Some(phi_def_bb) = def_block_opt else {
        return Ok(());
    };
    let phi_inputs = phi_inputs_opt.as_deref().unwrap_or(&[]);

    phi_input_contract::check_phi_input_contract(
        func, bb, kind_label, v, phi_def_bb, phi_inputs, &func_name,
    )
}

/// Finalize a single branch condition just before emitting a Branch.
/// Ensures the condition has a definition in the current block.
pub fn finalize_branch_cond(
    builder: &mut MirBuilder,
    condition_v: &mut ValueId,
) -> Result<(), String> {
    check_non_dominating_use(builder, *condition_v, "Cond")?;
    *condition_v = cond(builder, *condition_v);
    if crate::config::env::builder_local_ssa_trace() {
        if let Some(bb) = builder.current_block {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[local-ssa] finalize-branch bb={:?} cond=%{}",
                bb, condition_v.0
            ));
        }
    }
    Ok(())
}

/// Finalize compare operands just before emitting a Compare.
/// Applies in-block materialization to both lhs and rhs.
pub fn finalize_compare(
    builder: &mut MirBuilder,
    lhs: &mut ValueId,
    rhs: &mut ValueId,
) -> Result<(), String> {
    check_non_dominating_use(builder, *lhs, "CompareOperand")?;
    check_non_dominating_use(builder, *rhs, "CompareOperand")?;
    *lhs = cmp_operand(builder, *lhs);
    *rhs = cmp_operand(builder, *rhs);
    if crate::config::env::builder_local_ssa_trace() {
        if let Some(bb) = builder.current_block {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[local-ssa] finalize-compare bb={:?} lhs=%{} rhs=%{}",
                bb, lhs.0, rhs.0
            ));
        }
    }
    Ok(())
}
