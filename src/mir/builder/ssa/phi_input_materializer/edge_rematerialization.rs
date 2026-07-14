//! Per-edge PHI input rematerialization.
//!
//! A PHI input is an edge value: the incoming value must be valid at the
//! predecessor block attached to that input. If a pure value was defined on a
//! sibling path, this helper rematerializes an equivalent value in the
//! predecessor before PHI insertion.

use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CalleeBoxKind;
use std::collections::{HashMap, HashSet};

pub(super) struct PhiInputMaterializationAnalysis {
    def_blocks: HashMap<ValueId, BasicBlockId>,
    dominators: crate::mir::verification::utils::DominatorTree,
}

pub(super) struct PhiInputRematContext {
    pred: BasicBlockId,
    memo: HashMap<ValueId, ValueId>,
    visiting: HashSet<ValueId>,
}

impl PhiInputRematContext {
    pub(super) fn new(pred: BasicBlockId) -> Self {
        Self {
            pred,
            memo: HashMap::new(),
            visiting: HashSet::new(),
        }
    }

    fn remember(&mut self, original: ValueId, materialized: ValueId) {
        self.visiting.remove(&original);
        self.memo.insert(original, materialized);
    }
}

impl PhiInputMaterializationAnalysis {
    pub(super) fn new(func: &mut MirFunction) -> Self {
        func.update_cfg();
        Self {
            def_blocks: crate::mir::verification::utils::compute_def_blocks(func),
            dominators: crate::mir::verification::utils::compute_dominators(func),
        }
    }
}

fn find_def_inst(
    func: &MirFunction,
    value: ValueId,
) -> Option<(BasicBlockId, Option<MirInstruction>)> {
    if func.params.iter().any(|param| *param == value) {
        return Some((func.entry_block, None));
    }

    for (bb, block) in &func.blocks {
        for inst in &block.instructions {
            if inst.dst_value() == Some(value) {
                return Some((*bb, Some(inst.clone())));
            }
        }
        if let Some(term) = &block.terminator {
            if term.dst_value() == Some(value) {
                return Some((*bb, Some(term.clone())));
            }
        }
    }

    None
}

fn write_debug_function_dump(func: &MirFunction, reason: &str) -> String {
    if !crate::config::env::joinir_dev::debug_enabled()
        && !crate::config::env::builder_debug_enabled()
    {
        return "disabled".to_string();
    }

    let fn_name = func
        .signature
        .name
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => c,
            _ => '_',
        })
        .collect::<String>();
    let path = format!(
        "/tmp/mir_dump_phi_input_{}_{}_{}.txt",
        reason,
        fn_name,
        std::process::id()
    );
    match std::fs::write(&path, crate::mir::MirPrinter::new().print_function(func)) {
        Ok(_) => path,
        Err(_) => "write_failed".to_string(),
    }
}

fn format_defined_values(analysis: &PhiInputMaterializationAnalysis) -> String {
    let mut values = analysis
        .def_blocks
        .keys()
        .map(|value| value.0)
        .collect::<Vec<_>>();
    values.sort_unstable();
    let mut out = String::from("[");
    for (idx, value) in values.into_iter().take(24).enumerate() {
        if idx > 0 {
            out.push(',');
        }
        out.push('%');
        out.push_str(&value.to_string());
    }
    out.push(']');
    out
}

pub(super) fn rematerialize_for_pred(
    func: &mut MirFunction,
    analysis: &PhiInputMaterializationAnalysis,
    value: ValueId,
    context: &str,
    edge_kind: &str,
    remat_ctx: &mut PhiInputRematContext,
) -> Result<ValueId, String> {
    if let Some(cached) = remat_ctx.memo.get(&value).copied() {
        return Ok(cached);
    }

    let pred = remat_ctx.pred;
    let dominates_pred = analysis
        .def_blocks
        .get(&value)
        .copied()
        .map(|def_bb| analysis.dominators.dominates(def_bb, pred))
        .unwrap_or(false);

    if !remat_ctx.visiting.insert(value) {
        return Err(format!(
            "[freeze:contract][ssa/phi_input/remat_cycle] fn={} pred={:?} context={} edge={} value=%{}",
            func.signature.name, pred, context, edge_kind, value.0
        ));
    }

    let Some((def_bb, def_inst)) = find_def_inst(func, value) else {
        let mir_dump = write_debug_function_dump(func, "without_def");
        let defined_values = format_defined_values(analysis);
        return Err(format!(
            "[freeze:contract][ssa/phi_input/without_def] fn={} pred={:?} context={} edge={} value=%{} defined_values={} mir_dump={}",
            func.signature.name, pred, context, edge_kind, value.0, defined_values, mir_dump
        ));
    };

    let Some(def_inst) = def_inst else {
        if dominates_pred {
            remat_ctx.remember(value, value);
            return Ok(value);
        }
        return Err(format!(
            "[freeze:contract][ssa/phi_input/non_dominating_param] fn={} pred={:?} context={} edge={} value=%{} def_block={:?}",
            func.signature.name, pred, context, edge_kind, value.0, def_bb
        ));
    };

    if def_bb == pred {
        remat_ctx.remember(value, value);
        return Ok(value);
    }

    let remat_inst = match def_inst {
        MirInstruction::Const {
            value: const_value, ..
        } => {
            let dst = func.next_value_id();
            MirInstruction::Const {
                dst,
                value: const_value,
            }
        }
        MirInstruction::Copy { src, .. } => {
            let src = rematerialize_for_pred(func, analysis, src, context, edge_kind, remat_ctx)?;
            let dst = func.next_value_id();
            MirInstruction::Copy { dst, src }
        }
        MirInstruction::BinOp { op, lhs, rhs, .. } => {
            let lhs = rematerialize_for_pred(func, analysis, lhs, context, edge_kind, remat_ctx)?;
            let rhs = rematerialize_for_pred(func, analysis, rhs, context, edge_kind, remat_ctx)?;
            let dst = func.next_value_id();
            MirInstruction::BinOp { dst, op, lhs, rhs }
        }
        MirInstruction::Compare { op, lhs, rhs, .. } => {
            let lhs = rematerialize_for_pred(func, analysis, lhs, context, edge_kind, remat_ctx)?;
            let rhs = rematerialize_for_pred(func, analysis, rhs, context, edge_kind, remat_ctx)?;
            let dst = func.next_value_id();
            MirInstruction::Compare { dst, op, lhs, rhs }
        }
        MirInstruction::UnaryOp { op, operand, .. } => {
            let operand =
                rematerialize_for_pred(func, analysis, operand, context, edge_kind, remat_ctx)?;
            let dst = func.next_value_id();
            MirInstruction::UnaryOp { dst, op, operand }
        }
        MirInstruction::Select {
            cond,
            then_val,
            else_val,
            ..
        } => {
            let cond = rematerialize_for_pred(func, analysis, cond, context, edge_kind, remat_ctx)?;
            let then_val =
                rematerialize_for_pred(func, analysis, then_val, context, edge_kind, remat_ctx)?;
            let else_val =
                rematerialize_for_pred(func, analysis, else_val, context, edge_kind, remat_ctx)?;
            let dst = func.next_value_id();
            MirInstruction::Select {
                dst,
                cond,
                then_val,
                else_val,
            }
        }
        MirInstruction::Call {
            dst: Some(_),
            func: call_func,
            callee,
            args,
            effects,
        } if is_rematerializable_string_method_call(&callee) => {
            let args = args
                .into_iter()
                .map(|arg| {
                    rematerialize_for_pred(func, analysis, arg, context, edge_kind, remat_ctx)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let callee = rematerialize_callee_for_pred(
                func, analysis, callee, context, edge_kind, remat_ctx,
            )?;
            let dst = func.next_value_id();
            MirInstruction::Call {
                dst: Some(dst),
                func: call_func,
                callee,
                args,
                effects,
            }
        }
        other => {
            if dominates_pred {
                remat_ctx.remember(value, value);
                return Ok(value);
            }
            return Err(format!(
                "[freeze:contract][ssa/phi_input/non_rematerializable] fn={} pred={:?} context={} edge={} value=%{} def_block={:?} def_kind={:?}",
                func.signature.name, pred, context, edge_kind, value.0, def_bb, other
            ));
        }
    };

    let dst = remat_inst
        .dst_value()
        .ok_or_else(|| "[ssa/phi_input] rematerialized instruction missing dst".to_string())?;
    let fn_name = func.signature.name.clone();
    let block = func.get_block_mut(pred).ok_or_else(|| {
        format!(
            "[freeze:contract][ssa/phi_input/missing_pred_block] fn={} pred={:?} context={} edge={} value=%{}",
            fn_name, pred, context, edge_kind, value.0
        )
    })?;
    block.add_instruction_before_terminator(remat_inst);
    remat_ctx.remember(value, dst);
    Ok(dst)
}

fn is_rematerializable_string_method_call(callee: &Option<Callee>) -> bool {
    matches!(
        callee,
        Some(Callee::Method {
            box_name,
            method,
            box_kind: CalleeBoxKind::RuntimeData,
            ..
        }) if matches!(box_name.as_str(), "RuntimeDataBox" | "StringBox")
            && method == "substring"
    )
}

fn rematerialize_callee_for_pred(
    func: &mut MirFunction,
    analysis: &PhiInputMaterializationAnalysis,
    callee: Option<Callee>,
    context: &str,
    edge_kind: &str,
    remat_ctx: &mut PhiInputRematContext,
) -> Result<Option<Callee>, String> {
    match callee {
        Some(Callee::Method {
            box_name,
            method,
            receiver,
            certainty,
            box_kind,
        }) => {
            let receiver = receiver
                .map(|value| {
                    rematerialize_for_pred(func, analysis, value, context, edge_kind, remat_ctx)
                })
                .transpose()?;
            Ok(Some(Callee::Method {
                box_name,
                method,
                receiver,
                certainty,
                box_kind,
            }))
        }
        other => Ok(other),
    }
}

pub(in crate::mir::builder) fn for_pred(
    func: &mut MirFunction,
    pred: BasicBlockId,
    value: ValueId,
    context: &str,
    edge_kind: &str,
) -> Result<ValueId, String> {
    let analysis = PhiInputMaterializationAnalysis::new(func);
    let mut remat_ctx = PhiInputRematContext::new(pred);
    rematerialize_for_pred(func, &analysis, value, context, edge_kind, &mut remat_ctx)
}
