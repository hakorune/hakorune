use crate::ast::Span;
use crate::mir::optimizer::MirOptimizer;
use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::ssot::extern_call::extern_call as build_extern_call;
use crate::mir::{
    BinaryOp, CompareOp, EffectMask, MirInstruction as I, MirModule, SpannedInstruction, ValueId,
};

/// Core-13 "pure" normalization: rewrite a few non-13 ops to allowed forms.
/// - Load(dst, ptr)  => ExternCall(Some dst, env.local.get, [ptr])
/// - Store(val, ptr) => ExternCall(None, env.local.set, [ptr, val])
/// - NewBox(dst, T, args...) => ExternCall(Some dst, env.box.new, [Const String(T), args...])
/// - UnaryOp:
///     Neg x    => BinOp(Sub, Const 0, x)
///     Not x    => Compare(Eq, x, Const false)
///     BitNot x => BinOp(BitXor, x, Const(-1))
pub fn normalize_pure_core13(_opt: &mut MirOptimizer, module: &mut MirModule) -> OptimizationStats {
    use crate::mir::types::ConstValue;
    let mut stats = OptimizationStats::new();
    for (_fname, function) in &mut module.functions {
        for (_bb, block) in &mut function.blocks {
            let old = block.drain_spanned_instructions();
            let mut out: Vec<SpannedInstruction> = Vec::with_capacity(old.len() + 8);
            for SpannedInstruction { inst, span } in old.into_iter() {
                match inst {
                    I::Load { dst, ptr } => {
                        out.push(SpannedInstruction {
                            inst: build_extern_call(
                                Some(dst),
                                "env.local".to_string(),
                                "get".to_string(),
                                vec![ptr],
                                EffectMask::READ,
                            ),
                            span,
                        });
                        stats.intrinsic_optimizations += 1;
                    }
                    I::Store { value, ptr } => {
                        out.push(SpannedInstruction {
                            inst: build_extern_call(
                                None,
                                "env.local".to_string(),
                                "set".to_string(),
                                vec![ptr, value],
                                EffectMask::WRITE,
                            ),
                            span,
                        });
                        stats.intrinsic_optimizations += 1;
                    }
                    I::NewBox {
                        dst,
                        box_type,
                        mut args,
                    } => {
                        // prepend type name as Const String
                        let ty_id = ValueId::new(function.next_value_id);
                        function.next_value_id += 1;
                        out.push(SpannedInstruction {
                            inst: I::Const {
                                dst: ty_id,
                                value: crate::mir::ConstValue::String(box_type),
                            },
                            span,
                        });
                        let mut call_args = Vec::with_capacity(1 + args.len());
                        call_args.push(ty_id);
                        call_args.append(&mut args);
                        out.push(SpannedInstruction {
                            inst: build_extern_call(
                                Some(dst),
                                "env.box".to_string(),
                                "new".to_string(),
                                call_args,
                                EffectMask::PURE,
                            ),
                            span,
                        });
                        stats.intrinsic_optimizations += 1;
                    }
                    I::UnaryOp { dst, op, operand } => {
                        match op {
                            crate::mir::UnaryOp::Neg => {
                                let zero = ValueId::new(function.next_value_id);
                                function.next_value_id += 1;
                                out.push(SpannedInstruction {
                                    inst: I::Const {
                                        dst: zero,
                                        value: crate::mir::ConstValue::Integer(0),
                                    },
                                    span,
                                });
                                out.push(SpannedInstruction {
                                    inst: I::BinOp {
                                        dst,
                                        op: BinaryOp::Sub,
                                        lhs: zero,
                                        rhs: operand,
                                    },
                                    span,
                                });
                            }
                            crate::mir::UnaryOp::Not => {
                                let f = ValueId::new(function.next_value_id);
                                function.next_value_id += 1;
                                out.push(SpannedInstruction {
                                    inst: I::Const {
                                        dst: f,
                                        value: crate::mir::ConstValue::Bool(false),
                                    },
                                    span,
                                });
                                out.push(SpannedInstruction {
                                    inst: I::Compare {
                                        dst,
                                        op: CompareOp::Eq,
                                        lhs: operand,
                                        rhs: f,
                                    },
                                    span,
                                });
                            }
                            crate::mir::UnaryOp::BitNot => {
                                let all1 = ValueId::new(function.next_value_id);
                                function.next_value_id += 1;
                                out.push(SpannedInstruction {
                                    inst: I::Const {
                                        dst: all1,
                                        value: crate::mir::ConstValue::Integer(-1),
                                    },
                                    span,
                                });
                                out.push(SpannedInstruction {
                                    inst: I::BinOp {
                                        dst,
                                        op: BinaryOp::BitXor,
                                        lhs: operand,
                                        rhs: all1,
                                    },
                                    span,
                                });
                            }
                        }
                        stats.intrinsic_optimizations += 1;
                    }
                    other => out.push(SpannedInstruction { inst: other, span }),
                }
            }
            block.instructions = out.iter().map(|s| s.inst.clone()).collect();
            block.instruction_spans = out.iter().map(|s| s.span).collect();

            if let Some(term) = block.terminator.take() {
                let term_span = block.terminator_span.take().unwrap_or_else(Span::unknown);
                let rewritten = match term {
                    I::Load { dst, ptr } => build_extern_call(
                        Some(dst),
                        "env.local".to_string(),
                        "get".to_string(),
                        vec![ptr],
                        EffectMask::READ,
                    ),
                    I::Store { value, ptr } => build_extern_call(
                        None,
                        "env.local".to_string(),
                        "set".to_string(),
                        vec![ptr, value],
                        EffectMask::WRITE,
                    ),
                    I::NewBox {
                        dst,
                        box_type,
                        mut args,
                    } => {
                        let ty_id = ValueId::new(function.next_value_id);
                        function.next_value_id += 1;
                        block.instructions.push(I::Const {
                            dst: ty_id,
                            value: ConstValue::String(box_type),
                        });
                        block.instruction_spans.push(term_span);
                        let mut call_args = Vec::with_capacity(1 + args.len());
                        call_args.push(ty_id);
                        call_args.append(&mut args);
                        build_extern_call(
                            Some(dst),
                            "env.box".to_string(),
                            "new".to_string(),
                            call_args,
                            EffectMask::PURE,
                        )
                    }
                    I::UnaryOp { dst, op, operand } => match op {
                        crate::mir::UnaryOp::Neg => {
                            let zero = ValueId::new(function.next_value_id);
                            function.next_value_id += 1;
                            block.instructions.push(I::Const {
                                dst: zero,
                                value: ConstValue::Integer(0),
                            });
                            block.instruction_spans.push(term_span);
                            I::BinOp {
                                dst,
                                op: BinaryOp::Sub,
                                lhs: zero,
                                rhs: operand,
                            }
                        }
                        crate::mir::UnaryOp::Not => {
                            let f = ValueId::new(function.next_value_id);
                            function.next_value_id += 1;
                            block.instructions.push(I::Const {
                                dst: f,
                                value: ConstValue::Bool(false),
                            });
                            block.instruction_spans.push(term_span);
                            I::Compare {
                                dst,
                                op: CompareOp::Eq,
                                lhs: operand,
                                rhs: f,
                            }
                        }
                        crate::mir::UnaryOp::BitNot => {
                            let all1 = ValueId::new(function.next_value_id);
                            function.next_value_id += 1;
                            block.instructions.push(I::Const {
                                dst: all1,
                                value: ConstValue::Integer(-1),
                            });
                            block.instruction_spans.push(term_span);
                            I::BinOp {
                                dst,
                                op: BinaryOp::BitXor,
                                lhs: operand,
                                rhs: all1,
                            }
                        }
                    },
                    other => other,
                };
                block.set_terminator_with_span(rewritten, term_span);
            }
        }
    }
    stats
}
