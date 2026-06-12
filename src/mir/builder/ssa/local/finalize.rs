use super::{arg, cmp_operand, strict_planner_required, try_ensure, LocalKind};
use crate::mir::builder::ssa::phi_input_contract;
use crate::mir::builder::MirBuilder;
use crate::mir::{MirInstruction, ValueId};

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
        MirInstruction::StaticDataLoad { .. } => "StaticDataLoad",
        MirInstruction::Store { .. } => "Store",
        MirInstruction::MemOp { .. } => "MemOp",
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

/// Finalize only the args (legacy Call paths)
pub fn finalize_args(builder: &mut MirBuilder, args: &mut Vec<ValueId>) -> Result<(), String> {
    let args_list = if strict_planner_required() {
        Some(super::format_value_ids(args))
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
                        let params_list = super::format_value_ids(params);
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

/// Finalize a single branch condition just before emitting a Branch.
/// Ensures the condition has a definition in the current block.
pub fn finalize_branch_cond(
    builder: &mut MirBuilder,
    condition_v: &mut ValueId,
) -> Result<(), String> {
    check_non_dominating_use(builder, *condition_v, "Cond")?;
    *condition_v = super::cond(builder, *condition_v);
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
