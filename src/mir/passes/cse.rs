//! Common Subexpression Elimination (CSE) for pure MIR instructions.
//!
//! Note: Current implementation mirrors the prior monolithic behavior and
//! counts eliminations without rewriting uses (SSA update is TODO). This keeps
//! behavior identical while modularizing the pass for future enhancement.

use crate::mir::{MirFunction, MirInstruction, MirModule, MirType, ValueId};
use std::collections::HashMap;

/// Run CSE across the module. Returns the number of eliminated expressions.
pub fn eliminate_common_subexpressions(module: &mut MirModule) -> usize {
    let mut eliminated = 0usize;
    for (_name, func) in module.functions.iter_mut() {
        eliminated += cse_in_function(func);
    }
    eliminated
}

fn cse_in_function(function: &mut MirFunction) -> usize {
    let mut expression_map: HashMap<String, ValueId> = HashMap::new();
    let mut eliminated = 0usize;
    let fast_int = std::env::var("NYASH_LLVM_FAST_INT").ok().as_deref() == Some("1");

    // Helper: check if both operands are numeric (Integer/Float) via value type hints
    let is_numeric = |vid: ValueId| -> bool {
        match function.metadata.value_types.get(&vid) {
            Some(MirType::Integer) | Some(MirType::Float) => true,
            _ => false,
        }
    };

    for (_bid, block) in &mut function.blocks {
        for inst in &mut block.instructions {
            if inst.effects().is_pure() {
                let key = instruction_key(inst);
                if let Some(&existing) = expression_map.get(&key) {
                    if let Some(dst) = inst.dst_value() {
                        // Prefer existing SSA value in the same block when FAST_INT is enabled.
                        if fast_int {
                            match inst {
                                MirInstruction::BinOp { op, lhs, rhs, .. } => {
                                    // Only rewrite Add when both operands are numeric (avoid String + String)
                                    let allow = match op {
                                        crate::mir::BinaryOp::Add => {
                                            is_numeric(*lhs) && is_numeric(*rhs)
                                        }
                                        _ => true,
                                    };
                                    if allow {
                                        *inst = MirInstruction::Copy { dst, src: existing };
                                    }
                                }
                                MirInstruction::Compare { .. }
                                | MirInstruction::UnaryOp { .. }
                                | MirInstruction::TypeOp { .. } => {
                                    *inst = MirInstruction::Copy { dst, src: existing };
                                }
                                _ => {}
                            }
                        }
                        eliminated += 1;
                    }
                } else if let Some(dst) = inst.dst_value() {
                    expression_map.insert(key, dst);
                }
            }
        }
    }
    eliminated
}

fn instruction_key(i: &MirInstruction) -> String {
    match i {
        MirInstruction::Const { value, .. } => format!("const_{:?}", value),
        MirInstruction::BinOp { op, lhs, rhs, .. } => {
            format!("binop_{:?}_{}_{}", op, lhs.as_u32(), rhs.as_u32())
        }
        MirInstruction::Compare { op, lhs, rhs, .. } => {
            format!("cmp_{:?}_{}_{}", op, lhs.as_u32(), rhs.as_u32())
        }
        MirInstruction::Call {
            callee, func, args, ..
        } => {
            let args_str = args
                .iter()
                .map(|v| v.as_u32().to_string())
                .collect::<Vec<_>>()
                .join(",");

            // Include callee information to distinguish different call targets
            if let Some(c) = callee {
                use crate::mir::Callee;
                match c {
                    Callee::Global(name) => {
                        format!("call_global_{}_{}", name, args_str)
                    }
                    Callee::Method {
                        box_name,
                        method,
                        receiver,
                        ..
                    } => {
                        let recv_str = receiver
                            .map(|r| r.as_u32().to_string())
                            .unwrap_or_else(|| "static".to_string());
                        format!(
                            "call_method_{}.{}_{}_{}",
                            box_name, method, recv_str, args_str
                        )
                    }
                    Callee::Value(v) => {
                        format!("call_value_{}_{}", v.as_u32(), args_str)
                    }
                    Callee::Extern(name) => {
                        format!("call_extern_{}_{}", name, args_str)
                    }
                    Callee::Constructor { box_type } => {
                        format!("call_ctor_{}_{}", box_type, args_str)
                    }
                    Callee::Closure { .. } => {
                        // Closures are unique by definition (captures, params may differ)
                        // Use func as distinguisher
                        format!("call_closure_{}_{}", func.as_u32(), args_str)
                    }
                }
            } else {
                // Legacy path: no callee information, use func
                format!("call_legacy_{}_{}", func.as_u32(), args_str)
            }
        }
        other => format!("other_{:?}", other),
    }
}
