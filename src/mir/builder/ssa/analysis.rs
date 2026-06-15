use crate::mir::builder::MirBuilder;
use crate::mir::{MirInstruction, ValueId};

#[inline]
pub(crate) fn strict_planner_required() -> bool {
    crate::config::env::joinir_dev::strict_enabled()
        && crate::config::env::joinir_dev::planner_required_enabled()
}

pub(crate) fn value_defined_in_current_function(builder: &MirBuilder, v: ValueId) -> bool {
    let Some(func) = builder.scope_ctx.current_function.as_ref() else {
        return false;
    };
    if func.params.iter().any(|param| *param == v) {
        return true;
    }
    for block in func.blocks.values() {
        if block
            .instructions
            .iter()
            .any(|inst| inst.dst_value() == Some(v))
        {
            return true;
        }
        if block
            .terminator
            .as_ref()
            .is_some_and(|inst| inst.dst_value() == Some(v))
        {
            return true;
        }
    }
    false
}

pub(crate) fn format_value_ids(values: &[ValueId]) -> String {
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
pub(crate) fn def_inst_kind(inst: &MirInstruction) -> &'static str {
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

#[derive(Clone, Debug)]
pub(crate) struct FieldGetAliasRoot {
    pub(crate) value: ValueId,
    pub(crate) block: crate::mir::BasicBlockId,
    pub(crate) field: String,
}

pub(crate) fn find_value_def(
    builder: &MirBuilder,
    value: ValueId,
) -> Option<(crate::mir::BasicBlockId, MirInstruction)> {
    let func = builder.scope_ctx.current_function.as_ref()?;
    for (bid, block) in func.blocks.iter() {
        for inst in &block.instructions {
            if inst.dst_value() == Some(value) {
                return Some((*bid, inst.clone()));
            }
        }
        if let Some(term) = &block.terminator {
            if term.dst_value() == Some(value) {
                return Some((*bid, term.clone()));
            }
        }
    }
    None
}

pub(crate) fn field_get_alias_root(
    builder: &MirBuilder,
    seed: ValueId,
) -> Option<FieldGetAliasRoot> {
    let mut current = seed;
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..8 {
        if !seen.insert(current) {
            return None;
        }
        let (block, inst) = find_value_def(builder, current)?;
        match inst {
            MirInstruction::Copy { src, .. } => current = src,
            MirInstruction::FieldGet { dst, field, .. } => {
                return Some(FieldGetAliasRoot {
                    value: dst,
                    block,
                    field,
                });
            }
            _ => return None,
        }
    }
    None
}

pub(crate) fn dominated_call_result_root(
    builder: &MirBuilder,
    seed: ValueId,
    current_block: crate::mir::BasicBlockId,
) -> Option<ValueId> {
    let mut current = seed;
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..8 {
        if !seen.insert(current) {
            return None;
        }
        let (block, inst) = find_value_def(builder, current)?;
        match inst {
            MirInstruction::Copy { src, .. } => current = src,
            MirInstruction::Call { .. } => {
                if block == current_block {
                    return Some(current);
                }
                let func = builder.scope_ctx.current_function.as_ref()?;
                let dominators = crate::mir::verification::utils::compute_dominators(func);
                return dominators
                    .dominates(block, current_block)
                    .then_some(current);
            }
            _ => return None,
        }
    }
    None
}

pub(crate) fn same_block_copy_root(
    builder: &MirBuilder,
    seed: ValueId,
    current_block: crate::mir::BasicBlockId,
) -> Option<ValueId> {
    let mut current = seed;
    let mut seen = std::collections::BTreeSet::new();
    let mut saw_copy = false;
    for _ in 0..8 {
        if !seen.insert(current) {
            return None;
        }
        let (block, inst) = find_value_def(builder, current)?;
        if block != current_block {
            return None;
        }
        match inst {
            MirInstruction::Copy { src, .. } => {
                saw_copy = true;
                current = src;
            }
            _ if saw_copy => return Some(current),
            _ => return None,
        }
    }
    None
}

pub(crate) fn has_dominated_same_field_set_after_root(
    builder: &MirBuilder,
    root_block: crate::mir::BasicBlockId,
    current_block: crate::mir::BasicBlockId,
    field: &str,
) -> bool {
    let Some(func) = builder.scope_ctx.current_function.as_ref() else {
        return true;
    };
    let dominators = crate::mir::verification::utils::compute_dominators(func);
    if !dominators.dominates(root_block, current_block) {
        return true;
    }
    for (bid, block) in func.blocks.iter() {
        if !dominators.dominates(root_block, *bid) {
            continue;
        }
        for inst in block.instructions.iter().chain(block.terminator.iter()) {
            if matches!(inst, MirInstruction::FieldSet { field: f, .. } if f == field) {
                return true;
            }
        }
    }
    false
}
