use std::collections::HashSet;

use crate::mir::{
    BinaryOp, Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId,
};

use super::super::{CONCAT3_EXTERN, CONCAT_HH_EXTERN};

pub(super) fn infer_stringish_values(function: &MirFunction) -> HashSet<ValueId> {
    let mut out: HashSet<ValueId> = HashSet::new();

    for (vid, ty) in &function.metadata.value_types {
        if is_stringish_type(ty) {
            out.insert(*vid);
        }
    }

    let mut changed = true;
    while changed {
        changed = false;
        for block in function.blocks.values() {
            for inst in &block.instructions {
                let inserted = match inst {
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::String(_),
                    } => out.insert(*dst),
                    MirInstruction::Copy { dst, src } if out.contains(src) => out.insert(*dst),
                    MirInstruction::BinOp {
                        dst,
                        op: BinaryOp::Add,
                        lhs,
                        rhs,
                    } if out.contains(lhs) || out.contains(rhs) => out.insert(*dst),
                    MirInstruction::Call {
                        dst: Some(dst),
                        callee: Some(Callee::Extern(name) | Callee::Global(name)),
                        ..
                    } if is_string_concat_symbol(name) => out.insert(*dst),
                    _ => false,
                };
                if inserted {
                    changed = true;
                }
            }
        }
    }

    out
}

fn is_stringish_type(ty: &MirType) -> bool {
    match ty {
        MirType::String => true,
        MirType::Box(name) if name == "StringBox" => true,
        _ => false,
    }
}

fn is_string_concat_symbol(name: &str) -> bool {
    name == CONCAT_HH_EXTERN || name == CONCAT3_EXTERN
}
