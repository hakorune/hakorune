use crate::mir::builder::MirBuilder;
use crate::mir::definitions::call_unified::Callee;
use crate::mir::ValueId;

fn strict_planner_required() -> bool {
    crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled()
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

fn callee_label(callee: &Callee) -> String {
    match callee {
        Callee::Global(name) => format!("Global({})", name),
        Callee::Method {
            box_name, method, ..
        } => format!("Method({}.{})", box_name, method),
        Callee::Constructor { box_type } => format!("Constructor({})", box_type),
        Callee::Closure { .. } => "Closure".to_string(),
        Callee::Value(v) => format!("Value(%{})", v.0),
        Callee::Extern(name) => format!("Extern({})", name),
    }
}

/// Get short instruction name for 1-line log output
fn short_inst_name(inst: &crate::mir::MirInstruction) -> &'static str {
    match inst {
        crate::mir::MirInstruction::Const { .. } => "Const",
        crate::mir::MirInstruction::Copy { .. } => "Copy",
        crate::mir::MirInstruction::BinOp { .. } => "BinOp",
        crate::mir::MirInstruction::Compare { .. } => "Compare",
        crate::mir::MirInstruction::Call { .. } => "Call",
        crate::mir::MirInstruction::Phi { .. } => "Phi",
        crate::mir::MirInstruction::TypeOp { .. } => "TypeOp",
        crate::mir::MirInstruction::Return { .. } => "Return",
        crate::mir::MirInstruction::Branch { .. } => "Branch",
        crate::mir::MirInstruction::Jump { .. } => "Jump",
        _ => "Other",
    }
}

fn check_call_arg_scope(
    builder: &MirBuilder,
    callee: &Callee,
    args: &[ValueId],
) -> Result<(), String> {
    if !strict_planner_required() {
        return Ok(());
    }
    let func = match builder.scope_ctx.current_function.as_ref() {
        Some(f) => f,
        None => return Ok(()),
    };
    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
    let args_list = format_value_ids(args);
    let params_list = format_value_ids(&func.params);
    let fn_name = func.signature.name.as_str();
    let bb = builder.current_block;
    let entry = func.entry_block;
    let call_label = callee_label(callee);

    let check_value = |role: &str, v: ValueId| -> Result<(), String> {
        if def_blocks.contains_key(&v) {
            return Ok(());
        }

        // Debug-only observation: provenance of undefined ValueId
        if crate::config::env::joinir_dev::debug_enabled() {
            // Reverse lookup variable_map to find variable names pointing to this ValueId
            let varmap_hits: Vec<&str> = builder
                .variable_ctx
                .variable_map
                .iter()
                .filter(|(_, &vid)| vid == v)
                .map(|(name, _)| name.as_str())
                .collect();

            // Check if this is a pin slot (raw value, no prefix added)
            let pin_slot_name = builder
                .pin_slot_names
                .get(&v)
                .map(|name| name.as_str())
                .unwrap_or("none");

            // Current function's next_value_id (field reference only, no consumption)
            let fn_next_value_id = func.next_value_id;

            crate::runtime::get_global_ring0().log.debug(&format!(
                "[call/arg_scope:provenance] fn={} bb={:?} role={} v=%{} varmap_hits=[{:?}] pin={} next={}",
                fn_name,
                bb,
                role,
                v.0,
                varmap_hits,
                pin_slot_name,
                fn_next_value_id
            ));
        }

        // Collect undefined uses within the function for quick diagnosis
        let (undef_in_func, first_undef_use, undef0_varmap_hits, undef0_pin) = {
            use std::collections::BTreeSet;

            let mut used_values: Vec<ValueId> = Vec::new();
            for block in func.blocks.values() {
                used_values.extend(block.used_values()); // Includes both instructions AND terminator
            }

            // Defined set = def_blocks keys + params (explicitly add params for safety)
            let mut defined_set: BTreeSet<ValueId> = def_blocks.keys().copied().collect();
            defined_set.extend(func.params.iter().copied());

            // Undefined = used but not defined
            let undef_values: BTreeSet<ValueId> = used_values
                .into_iter()
                .filter(|v| !defined_set.contains(v))
                .collect();

            // Format undef_in_func (first 3, ascending by ValueId.0 for stable order)
            let undef_list: Vec<ValueId> = undef_values.iter().copied().take(3).collect();
            let undef_str = format_value_ids(&undef_list);

            // Find first undefined use location (by block order, then instruction order)
            let first_use_str = if let Some(&first_undef) = undef_values.iter().min_by_key(|v| v.0)
            {
                let mut found = None;
                'outer: for (block_id, block) in &func.blocks {
                    // Check instructions first
                    for sp in block.iter_spanned() {
                        if sp.inst.used_values().contains(&first_undef) {
                            let inst_name = short_inst_name(&sp.inst);
                            found = Some(format!(
                                "bb{:?} inst={} used=%{}",
                                block_id, inst_name, first_undef.0
                            ));
                            break 'outer;
                        }
                    }
                    // Then check terminator
                    if let Some(term) = &block.terminator {
                        if term.used_values().contains(&first_undef) {
                            let term_name = short_inst_name(term);
                            found = Some(format!(
                                "bb{:?} term={} used=%{}",
                                block_id, term_name, first_undef.0
                            ));
                            break 'outer;
                        }
                    }
                }
                found.unwrap_or_else(|| format!("bb? inst=? used=%{}", first_undef.0))
            } else {
                "none".to_string()
            };

            // Collect provenance for first_undef (only in fail-fast path)
            let (varmap_hits_str, pin_str) =
                if let Some(&first_undef) = undef_values.iter().min_by_key(|v| v.0) {
                    // Reverse lookup variable_map to find variable names pointing to first_undef
                    let varmap_hits: Vec<&str> = builder
                        .variable_ctx
                        .variable_map
                        .iter()
                        .filter(|(_, &vid)| vid == first_undef)
                        .map(|(name, _)| name.as_str())
                        .collect();

                    // Check if first_undef is a pin slot
                    let pin_slot_name = builder
                        .pin_slot_names
                        .get(&first_undef)
                        .map(|name| name.as_str())
                        .unwrap_or("none");

                    // Format as stable 1-line output
                    let varmap_str = if varmap_hits.is_empty() {
                        "[]".to_string()
                    } else {
                        format!("[{}]", varmap_hits.join(","))
                    };

                    (varmap_str, pin_slot_name.to_string())
                } else {
                    ("[]".to_string(), "none".to_string())
                };

            (undef_str, first_use_str, varmap_hits_str, pin_str)
        };

        let span = builder.metadata_ctx.current_span();
        let file = builder
            .metadata_ctx
            .current_source_file()
            .unwrap_or_else(|| "unknown".to_string());

        let mir_dump_path = if crate::config::env::joinir_dev::debug_enabled() {
            let fn_name_sanitized = sanitize_for_path(fn_name);
            let pid = std::process::id();
            let path = format!("/tmp/mir_dump_callarg_{}_{}.txt", fn_name_sanitized, pid);

            if let Ok(mut f) = std::fs::File::create(&path) {
                let p = crate::mir::MirPrinter::new();
                let _ = std::io::Write::write_all(&mut f, p.print_function(func).as_bytes());
                Some(path)
            } else {
                Some("write_failed".to_string())
            }
        } else {
            None
        };

        Err(format!(
            "[freeze:contract][call/arg_out_of_function_scope] fn={} call={} bb={:?} role={} v=%{} args={} params={} entry={:?} span={} span_start={} span_end={} file={} mir_dump={} undef_in_func={} first_undef_use={} undef0_varmap_hits={} undef0_pin={}",
            fn_name,
            call_label,
            bb,
            role,
            v.0,
            args_list,
            params_list,
            entry,
            span.location_string(),
            span.start,
            span.end,
            file,
            mir_dump_path.unwrap_or_else(|| "disabled".to_string()),
            undef_in_func,
            first_undef_use,
            undef0_varmap_hits,
            undef0_pin
        ))
    };

    if let Callee::Method {
        receiver: Some(r), ..
    } = callee
    {
        check_value("recv", *r)?;
    } else if let Callee::Value(v) = callee {
        check_value("callee", *v)?;
    }

    for (idx, arg) in args.iter().enumerate() {
        let role = format!("arg[{}]", idx);
        check_value(&role, *arg)?;
    }

    Ok(())
}

/// Finalize call operands (receiver/args) using LocalSSA; thin wrapper to centralize usage.
pub fn finalize_call_operands(
    builder: &mut MirBuilder,
    callee: &mut Callee,
    args: &mut Vec<ValueId>,
) -> Result<(), String> {
    check_call_arg_scope(builder, callee, args)?;

    // Step 1: Receiver materialization (pin slot + LocalSSA) in a dedicated box
    crate::mir::builder::receiver::finalize_method_receiver(builder, callee);

    // Step 2: LocalSSA materialization for args only
    crate::mir::builder::ssa::local::finalize_args(builder, args)?;

    // Step 3: Disabled - BlockScheduleBox insert-after-phis doesn't work correctly
    // The Copy instructions are being inserted but then lost when blocks are finalized.
    // Instead, rely solely on LocalSSA which uses emit_instruction (the normal path).
    //
    // TODO: Fix BlockScheduleBox or remove it entirely if LocalSSA is sufficient.

    /* DISABLED - causes ValueId(22) undefined error
    if let Callee::Method { box_name, method, receiver: Some(r_local), certainty } = callee.clone() {
        if let Ok(r_after) = crate::mir::builder::schedule::block::BlockScheduleBox::ensure_after_phis_copy(builder, r_local) {
            if let Ok(r_tail) = crate::mir::builder::schedule::block::BlockScheduleBox::emit_before_call_copy(builder, r_after) {
                *callee = Callee::Method { box_name, method, receiver: Some(r_tail), certainty };
            } else {
                *callee = Callee::Method { box_name, method, receiver: Some(r_after), certainty };
            }
        }
    }
    */

    Ok(())
}

/// Verify block schedule invariants after emitting a call (dev-only WARNs inside).
pub fn verify_after_call(builder: &mut MirBuilder) {
    crate::mir::builder::schedule::block::BlockScheduleBox::verify_order(builder);
}
