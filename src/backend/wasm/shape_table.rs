use crate::mir::{ConstValue, MirInstruction, MirModule};
use crate::mir::BinaryOp;

const P10_LOOP_EXTERN_CANDIDATE_ID: &str = "wsm.p10.main_loop_extern_call.v0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeShape {
    MainReturnI32Const,
    MainReturnI32ConstViaCopy,
    MainReturnI32ConstBinOp,
}

impl NativeShape {
    pub(crate) fn id(self) -> &'static str {
        match self {
            NativeShape::MainReturnI32Const => "wsm.p4.main_return_i32_const.v0",
            NativeShape::MainReturnI32ConstViaCopy => "wsm.p5.main_return_i32_const_via_copy.v0",
            NativeShape::MainReturnI32ConstBinOp => "wsm.p9.main_return_i32_const_binop.v0",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeMatch {
    pub(crate) shape: NativeShape,
    pub(crate) value: i32,
}

type ShapeMatcher = fn(&MirModule) -> Option<NativeMatch>;

const NATIVE_SHAPE_TABLE: &[ShapeMatcher] = &[
    match_main_return_i32_const,
    match_main_return_i32_const_via_copy,
    match_main_return_i32_const_binop,
];

pub(crate) fn match_native_shape(mir_module: &MirModule) -> Option<NativeMatch> {
    for matcher in NATIVE_SHAPE_TABLE {
        if let Some(found) = matcher(mir_module) {
            return Some(found);
        }
    }
    None
}

/// Analysis-only candidate detector for WSM-P10.
/// This does not alter route planning and remains bridge-only.
pub(crate) fn detect_p10_loop_extern_call_candidate(mir_module: &MirModule) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_extern_call = false;
    let mut has_branch = false;
    let mut has_jump = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            if matches!(
                inst,
                MirInstruction::Call {
                    callee: Some(crate::mir::Callee::Extern(_)),
                    ..
                }
            ) {
                has_extern_call = true;
            }
        }

        if let Some(term) = &block.terminator {
            match term {
                MirInstruction::Branch { .. } => has_branch = true,
                MirInstruction::Jump { .. } => has_jump = true,
                _ => {}
            }
        }
    }

    (has_extern_call && has_branch && has_jump).then_some(P10_LOOP_EXTERN_CANDIDATE_ID)
}

fn match_main_return_i32_const(mir_module: &MirModule) -> Option<NativeMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 1 {
        return None;
    }

    let MirInstruction::Const { dst, value } = &entry.instructions[0] else {
        return None;
    };
    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != dst {
        return None;
    }

    let ConstValue::Integer(n) = value else {
        return None;
    };
    let value = i32::try_from(*n).ok()?;
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32Const,
        value,
    })
}

fn match_main_return_i32_const_via_copy(mir_module: &MirModule) -> Option<NativeMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 2 {
        return None;
    }

    let MirInstruction::Const { dst, value } = &entry.instructions[0] else {
        return None;
    };
    let MirInstruction::Copy { dst: copy_dst, src } = &entry.instructions[1] else {
        return None;
    };
    if src != dst {
        return None;
    }

    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != copy_dst {
        return None;
    }

    let ConstValue::Integer(n) = value else {
        return None;
    };
    let value = i32::try_from(*n).ok()?;
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32ConstViaCopy,
        value,
    })
}

fn match_main_return_i32_const_binop(mir_module: &MirModule) -> Option<NativeMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 3 {
        return None;
    }

    let MirInstruction::Const {
        dst: lhs_dst,
        value: lhs_value,
    } = &entry.instructions[0]
    else {
        return None;
    };
    let MirInstruction::Const {
        dst: rhs_dst,
        value: rhs_value,
    } = &entry.instructions[1]
    else {
        return None;
    };
    let MirInstruction::BinOp {
        dst: binop_dst,
        op,
        lhs,
        rhs,
    } = &entry.instructions[2]
    else {
        return None;
    };
    if lhs != lhs_dst || rhs != rhs_dst {
        return None;
    }

    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != binop_dst {
        return None;
    }

    let ConstValue::Integer(lhs_n) = lhs_value else {
        return None;
    };
    let ConstValue::Integer(rhs_n) = rhs_value else {
        return None;
    };

    let folded = fold_i64_binop(*op, *lhs_n, *rhs_n)?;
    let value = i32::try_from(folded).ok()?;
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32ConstBinOp,
        value,
    })
}

fn fold_i64_binop(op: BinaryOp, lhs: i64, rhs: i64) -> Option<i64> {
    match op {
        BinaryOp::Add => lhs.checked_add(rhs),
        BinaryOp::Sub => lhs.checked_sub(rhs),
        BinaryOp::Mul => lhs.checked_mul(rhs),
        BinaryOp::Div => lhs.checked_div(rhs),
        BinaryOp::Mod => lhs.checked_rem(rhs),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, BinaryOp, Callee, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType, ValueId,
    };

    fn make_module_with_single_const_return(value: i64) -> MirModule {
        let mut module = MirModule::new("test".to_string());
        let entry = BasicBlockId(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry,
        );
        let dst = ValueId(1);
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        block.add_instruction(MirInstruction::Const {
            dst,
            value: ConstValue::Integer(value),
        });
        block.add_instruction(MirInstruction::Return { value: Some(dst) });
        module.add_function(function);
        module
    }

    #[test]
    fn wasm_shape_table_matches_min_const_return_contract() {
        let module = make_module_with_single_const_return(-1);
        let found = match_native_shape(&module).expect("shape table should match");
        assert_eq!(found.shape.id(), "wsm.p4.main_return_i32_const.v0");
        assert_eq!(found.value, -1);
    }

    #[test]
    fn wasm_shape_table_rejects_non_const_return_contract() {
        let mut module = make_module_with_single_const_return(7);
        let entry = module
            .get_function_mut("main")
            .expect("main should exist")
            .get_block_mut(BasicBlockId(0))
            .expect("entry block should exist");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(9),
        });

        assert!(
            match_native_shape(&module).is_none(),
            "shape table must fail-fast outside strict pilot shape"
        );
    }

    #[test]
    fn wasm_shape_table_matches_const_copy_return_contract() {
        let mut module = MirModule::new("test".to_string());
        let entry = BasicBlockId(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry,
        );
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let const_dst = ValueId(1);
        let copy_dst = ValueId(2);
        block.add_instruction(MirInstruction::Const {
            dst: const_dst,
            value: ConstValue::Integer(8),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: copy_dst,
            src: const_dst,
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(copy_dst),
        });
        module.add_function(function);

        let found = match_native_shape(&module).expect("const-copy-return shape should match");
        assert_eq!(found.shape.id(), "wsm.p5.main_return_i32_const_via_copy.v0");
        assert_eq!(found.value, 8);
    }

    #[test]
    fn wasm_shape_table_matches_const_binop_return_contract() {
        let mut module = MirModule::new("test".to_string());
        let entry = BasicBlockId(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry,
        );
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let lhs = ValueId(1);
        let rhs = ValueId(2);
        let out = ValueId(3);
        block.add_instruction(MirInstruction::Const {
            dst: lhs,
            value: ConstValue::Integer(40),
        });
        block.add_instruction(MirInstruction::Const {
            dst: rhs,
            value: ConstValue::Integer(2),
        });
        block.add_instruction(MirInstruction::BinOp {
            dst: out,
            op: BinaryOp::Add,
            lhs,
            rhs,
        });
        block.add_instruction(MirInstruction::Return { value: Some(out) });
        module.add_function(function);

        let found = match_native_shape(&module).expect("const-binop-return shape should match");
        assert_eq!(found.shape.id(), "wsm.p9.main_return_i32_const_binop.v0");
        assert_eq!(found.value, 42);
    }

    #[test]
    fn wasm_shape_table_rejects_const_binop_div_by_zero_contract() {
        let mut module = MirModule::new("test".to_string());
        let entry = BasicBlockId(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry,
        );
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let lhs = ValueId(1);
        let rhs = ValueId(2);
        let out = ValueId(3);
        block.add_instruction(MirInstruction::Const {
            dst: lhs,
            value: ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Const {
            dst: rhs,
            value: ConstValue::Integer(0),
        });
        block.add_instruction(MirInstruction::BinOp {
            dst: out,
            op: BinaryOp::Div,
            lhs,
            rhs,
        });
        block.add_instruction(MirInstruction::Return { value: Some(out) });
        module.add_function(function);

        assert!(
            match_native_shape(&module).is_none(),
            "const-binop-return must fail-fast on invalid arithmetic"
        );
    }

    #[test]
    fn wasm_shape_table_detects_p10_loop_extern_candidate_contract() {
        let mut module = MirModule::new("test".to_string());
        let entry = BasicBlockId(0);
        let loop_bb = BasicBlockId(1);
        let exit_bb = BasicBlockId(2);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::IO,
            },
            entry,
        );

        {
            let block = function
                .get_block_mut(entry)
                .expect("entry block must exist");
            let cond = ValueId(1);
            block.add_instruction(MirInstruction::Const {
                dst: cond,
                value: ConstValue::Integer(1),
            });
            block.add_instruction(MirInstruction::Branch {
                condition: cond,
                then_bb: loop_bb,
                else_bb: exit_bb,
                then_edge_args: None,
                else_edge_args: None,
            });
        }

        let mut loop_block = BasicBlock::new(loop_bb);
        loop_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId(99),
            callee: Some(Callee::Extern("env.console.log".to_string())),
            args: vec![ValueId(1)],
            effects: EffectMask::IO,
        });
        loop_block.add_instruction(MirInstruction::Jump {
            target: entry,
            edge_args: None,
        });
        function.add_block(loop_block);

        let mut exit_block = BasicBlock::new(exit_bb);
        let out = ValueId(2);
        exit_block.add_instruction(MirInstruction::Const {
            dst: out,
            value: ConstValue::Integer(0),
        });
        exit_block.add_instruction(MirInstruction::Return { value: Some(out) });
        function.add_block(exit_block);

        module.add_function(function);

        let found =
            detect_p10_loop_extern_call_candidate(&module).expect("p10 candidate should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.v0");
    }

    #[test]
    fn wasm_shape_table_rejects_p10_candidate_without_loop_contract() {
        let mut module = MirModule::new("test".to_string());
        let entry = BasicBlockId(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::IO,
            },
            entry,
        );

        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let v = ValueId(1);
        block.add_instruction(MirInstruction::Const {
            dst: v,
            value: ConstValue::Integer(1),
        });
        block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId(99),
            callee: Some(Callee::Extern("env.console.log".to_string())),
            args: vec![v],
            effects: EffectMask::IO,
        });
        block.add_instruction(MirInstruction::Return { value: Some(v) });
        module.add_function(function);

        assert!(
            detect_p10_loop_extern_call_candidate(&module).is_none(),
            "non-loop extern shape must stay outside p10 candidate contract"
        );
    }
}
