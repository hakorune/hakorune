//! Common Subexpression Elimination (CSE) for pure MIR instructions.
//!
//! CSE is intentionally conservative: only an exact supported duplicate in
//! the same basic block is rewritten, and statistics count physical rewrites.

use crate::mir::{MirFunction, MirInstruction, MirModule, MirType, ValueId};
use std::collections::HashMap;

/// Run CSE across the module. Returns the number of eliminated expressions.
pub fn eliminate_common_subexpressions(module: &mut MirModule) -> usize {
    let mut eliminated = 0usize;
    let fast_int = std::env::var("NYASH_LLVM_FAST_INT").ok().as_deref() == Some("1");
    for (_name, func) in module.functions.iter_mut() {
        eliminated += cse_in_function(func, fast_int);
    }
    eliminated
}

fn cse_in_function(function: &mut MirFunction, fast_int: bool) -> usize {
    let mut eliminated = 0usize;

    // Helper: check if both operands are numeric (Integer/Float) via value type hints
    let is_numeric = |vid: ValueId| -> bool {
        match function.metadata.value_types.get(&vid) {
            Some(MirType::Integer) | Some(MirType::Float) => true,
            _ => false,
        }
    };

    for (_bid, block) in &mut function.blocks {
        let mut expression_map: HashMap<String, ValueId> = HashMap::new();
        let mut field_get_map: HashMap<(ValueId, String), ValueId> = HashMap::new();
        for inst in &mut block.instructions {
            if !inst.effects().is_pure() {
                // Conservative invalidation: side effects may change object fields.
                field_get_map.clear();
            }

            if let MirInstruction::FieldGet {
                dst, base, field, ..
            } = inst
            {
                let key = (*base, field.clone());
                if let Some(&existing) = field_get_map.get(&key) {
                    let dst = *dst;
                    *inst = MirInstruction::Copy { dst, src: existing };
                    eliminated += 1;
                    continue;
                }
                field_get_map.insert(key, *dst);
                continue;
            }

            if inst.effects().is_pure() {
                let key = instruction_key(inst);
                if let Some(&existing) = expression_map.get(&key) {
                    if let Some(dst) = inst.dst_value() {
                        // Prefer existing SSA value in the same block when FAST_INT is enabled.
                        let mut rewritten = false;
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
                                        rewritten = true;
                                    }
                                }
                                MirInstruction::Compare { .. }
                                | MirInstruction::UnaryOp { .. }
                                | MirInstruction::TypeOp { .. } => {
                                    *inst = MirInstruction::Copy { dst, src: existing };
                                    rewritten = true;
                                }
                                _ => {}
                            }
                        }
                        if rewritten {
                            eliminated += 1;
                        }
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
        MirInstruction::LegacyCallV0 {
            callee, func, args, ..
        } => {
            let args_str = args
                .iter()
                .map(|v| v.as_u32().to_string())
                .collect::<Vec<_>>()
                .join(",");

            // The typed Callee is the target key. Legacy None remains an
            // explicit compatibility key until the core field cutover.
            callee.as_ref().map_or_else(
                || format!("call_legacy_{}_{}", func.as_u32(), args_str),
                |callee| format!("call_callee_{:?}_{}", callee, args_str),
            )
        }
        other => format!("other_{:?}", other),
    }
}

#[cfg(test)]
mod tests {
    use super::{cse_in_function, instruction_key};
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask,
        FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
    };

    fn function(return_type: MirType) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "cse_test".to_string(),
                params: vec![],
                return_type,
                effects: EffectMask::PURE,
            },
            BasicBlockId(0),
        )
    }

    fn add_numeric_compare(block: &mut BasicBlock) {
        block.add_instruction(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(2),
        });
        block.add_instruction(MirInstruction::Compare {
            dst: ValueId(3),
            op: CompareOp::Eq,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
    }

    #[test]
    fn cse_rewrites_only_duplicate_pure_instructions_in_same_block() {
        let mut function = function(MirType::Bool);
        {
            let block = function.get_block_mut(BasicBlockId(0)).expect("entry");
            add_numeric_compare(block);
            block.add_instruction(MirInstruction::Compare {
                dst: ValueId(4),
                op: CompareOp::Eq,
                lhs: ValueId(1),
                rhs: ValueId(2),
            });
        }
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Bool);
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Bool);

        let eliminated = cse_in_function(&mut function, true);
        assert_eq!(eliminated, 1);
        assert!(matches!(
            function.entry_block().instructions.last(),
            Some(MirInstruction::Copy {
                dst: ValueId(4),
                src: ValueId(3),
            })
        ));
    }

    #[test]
    fn cse_does_not_reuse_duplicate_pure_instructions_across_sibling_blocks() {
        let mut function = function(MirType::Bool);
        {
            let entry = function.get_block_mut(BasicBlockId(0)).expect("entry");
            add_numeric_compare(entry);
        }
        let mut sibling = BasicBlock::new(BasicBlockId(1));
        add_numeric_compare(&mut sibling);
        function.add_block(sibling);
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Bool);

        let eliminated = cse_in_function(&mut function, true);
        assert_eq!(eliminated, 0);
        assert!(function.blocks.values().all(|block| {
            block
                .instructions
                .iter()
                .any(|inst| matches!(inst, MirInstruction::Compare { .. }))
        }));
    }

    #[test]
    fn cse_counts_only_actual_copy_rewrites() {
        let mut function = function(MirType::String);
        let block = function.get_block_mut(BasicBlockId(0)).expect("entry");
        block.add_instruction(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::String("left".to_string()),
        });
        block.add_instruction(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::String("right".to_string()),
        });
        block.add_instruction(MirInstruction::BinOp {
            dst: ValueId(3),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.add_instruction(MirInstruction::BinOp {
            dst: ValueId(4),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::String);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::String);
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::String);
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::String);

        let eliminated = cse_in_function(&mut function, true);
        assert_eq!(eliminated, 0);
        assert!(matches!(
            function.entry_block().instructions.last(),
            Some(MirInstruction::BinOp {
                dst: ValueId(4),
                op: BinaryOp::Add,
                ..
            })
        ));
    }

    fn call(callee: Option<Callee>, func: ValueId) -> MirInstruction {
        MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(1)),
            func,
            callee,
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn cse_call_key_uses_typed_callee_and_ignores_stale_func() {
        let typed_a = call(
            Some(Callee::Global(crate::mir::test_global_target(
                "target/1".to_string(),
            ))),
            ValueId::new(10),
        );
        let typed_b = call(
            Some(Callee::Global(crate::mir::test_global_target(
                "target/1".to_string(),
            ))),
            ValueId::new(11),
        );
        assert_eq!(instruction_key(&typed_a), instruction_key(&typed_b));
    }

    #[test]
    fn cse_closure_key_does_not_use_legacy_func() {
        let make = |func| {
            call(
                Some(Callee::Closure {
                    params: vec!["x".to_string()],
                    captures: vec![("x".to_string(), ValueId::new(7))],
                    me_capture: None,
                }),
                func,
            )
        };
        assert_eq!(
            instruction_key(&make(ValueId::new(10))),
            instruction_key(&make(ValueId::new(11)))
        );
    }

    #[test]
    fn cse_call_key_keeps_legacy_func_compatibility_distinct() {
        let legacy_a = call(None, ValueId::new(10));
        let legacy_b = call(None, ValueId::new(11));
        assert_ne!(instruction_key(&legacy_a), instruction_key(&legacy_b));
    }
}
