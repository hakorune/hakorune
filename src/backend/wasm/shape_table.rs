use crate::mir::{ConstValue, MirInstruction, MirModule};
use crate::mir::BinaryOp;

const P10_LOOP_EXTERN_CANDIDATE_ID: &str = "wsm.p10.main_loop_extern_call.v0";
const P10_MIN4_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.fixed3.v0";
const P10_MIN6_WARN_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.warn.fixed4.v0";
const P10_MIN7_INFO_NATIVE_SHAPE_ID: &str = "wsm.p10.main_loop_extern_call.info.fixed4.v0";
const P10_MIN5_WARN_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.warn.fixed3.inventory.v0";
const P10_MIN5_INFO_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.info.fixed3.inventory.v0";
const P10_MIN5_ERROR_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.error.fixed3.inventory.v0";
const P10_MIN5_DEBUG_INVENTORY_ID: &str = "wsm.p10.main_loop_extern_call.debug.fixed3.inventory.v0";

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

/// WSM-P10-min4 native promotion matcher.
/// Keep this conservative so existing bridge contracts stay stable.
pub(crate) fn detect_p10_min4_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_branch = false;
    let mut has_jump = false;
    let mut extern_log_calls = 0usize;
    let mut has_other_call = false;
    let mut has_const_3 = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            match inst {
                MirInstruction::Const {
                    value: ConstValue::Integer(3),
                    ..
                } => has_const_3 = true,
                MirInstruction::Call { callee: Some(callee), .. } => match callee {
                    crate::mir::Callee::Extern(name) => {
                        if name == "env.console.log" {
                            extern_log_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    crate::mir::Callee::Method {
                        box_name, method, ..
                    } => {
                        if box_name == "console" && method == "log" {
                            extern_log_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    _ => has_other_call = true,
                },
                MirInstruction::Call { .. } => has_other_call = true,
                _ => {}
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

    if has_branch && has_jump && has_const_3 && extern_log_calls == 1 && !has_other_call {
        Some(P10_MIN4_NATIVE_SHAPE_ID)
    } else {
        None
    }
}

/// WSM-P10-min6 warn-family native promotion matcher.
/// Keep strict boundary so min5 inventory fixtures remain bridge-only.
pub(crate) fn detect_p10_min6_warn_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    detect_p10_fixed4_console_method_native_shape(
        mir_module,
        "warn",
        "env.console.warn",
        P10_MIN6_WARN_NATIVE_SHAPE_ID,
    )
}

/// WSM-P10-min7 info-family native promotion matcher.
/// Keep strict boundary so min5 inventory fixtures remain bridge-only.
pub(crate) fn detect_p10_min7_info_native_promotable_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    detect_p10_fixed4_console_method_native_shape(
        mir_module,
        "info",
        "env.console.info",
        P10_MIN7_INFO_NATIVE_SHAPE_ID,
    )
}

fn detect_p10_fixed4_console_method_native_shape(
    mir_module: &MirModule,
    method_name: &str,
    extern_name: &str,
    shape_id: &'static str,
) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_branch = false;
    let mut has_jump = false;
    let mut has_const_4 = false;
    let mut method_calls = 0usize;
    let mut has_other_call = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            match inst {
                MirInstruction::Const {
                    value: ConstValue::Integer(4),
                    ..
                } => has_const_4 = true,
                MirInstruction::Call { callee: Some(callee), .. } => match callee {
                    crate::mir::Callee::Extern(name) => {
                        if name == extern_name {
                            method_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    crate::mir::Callee::Method {
                        box_name, method, ..
                    } => {
                        if box_name == "console" && method == method_name {
                            method_calls += 1;
                        } else {
                            has_other_call = true;
                        }
                    }
                    _ => has_other_call = true,
                },
                MirInstruction::Call { .. } => has_other_call = true,
                _ => {}
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

    if has_branch && has_jump && has_const_4 && method_calls == 1 && !has_other_call {
        Some(shape_id)
    } else {
        None
    }
}

/// WSM-P10-min5 expansion inventory matcher.
/// Analysis-only: records adjacent loop/extern shapes that are still bridge-only.
pub(crate) fn detect_p10_min5_expansion_inventory_shape(
    mir_module: &MirModule,
) -> Option<&'static str> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() < 2 {
        return None;
    }

    let mut has_branch = false;
    let mut has_jump = false;
    let mut has_const_3 = false;
    let mut call_method: Option<&'static str> = None;
    let mut has_other_call = false;

    for block in main.blocks.values() {
        for inst in &block.instructions {
            match inst {
                MirInstruction::Const {
                    value: ConstValue::Integer(3),
                    ..
                } => has_const_3 = true,
                MirInstruction::Call { callee: Some(callee), .. } => {
                    let found = match callee {
                        crate::mir::Callee::Extern(name) => match name.as_str() {
                            "env.console.warn" => Some(P10_MIN5_WARN_INVENTORY_ID),
                            "env.console.info" => Some(P10_MIN5_INFO_INVENTORY_ID),
                            "env.console.error" => Some(P10_MIN5_ERROR_INVENTORY_ID),
                            "env.console.debug" => Some(P10_MIN5_DEBUG_INVENTORY_ID),
                            _ => None,
                        },
                        crate::mir::Callee::Method {
                            box_name, method, ..
                        } => {
                            if box_name != "console" {
                                None
                            } else {
                                match method.as_str() {
                                    "warn" => Some(P10_MIN5_WARN_INVENTORY_ID),
                                    "info" => Some(P10_MIN5_INFO_INVENTORY_ID),
                                    "error" => Some(P10_MIN5_ERROR_INVENTORY_ID),
                                    "debug" => Some(P10_MIN5_DEBUG_INVENTORY_ID),
                                    _ => None,
                                }
                            }
                        }
                        _ => None,
                    };
                    if let Some(shape_id) = found {
                        if call_method.replace(shape_id).is_some() {
                            has_other_call = true;
                        }
                    } else {
                        has_other_call = true;
                    }
                }
                MirInstruction::Call { .. } => has_other_call = true,
                _ => {}
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

    if has_branch && has_jump && has_const_3 && !has_other_call {
        call_method
    } else {
        None
    }
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

    #[test]
    fn wasm_shape_table_detects_p10_min4_native_promotable_contract() {
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
        let c3 = ValueId(3);
        loop_block.add_instruction(MirInstruction::Const {
            dst: c3,
            value: ConstValue::Integer(3),
        });
        loop_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId(99),
                callee: Some(Callee::Method {
                    box_name: "console".to_string(),
                    method: "log".to_string(),
                    receiver: Some(ValueId(98)),
                    certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                    box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
                }),
            args: vec![c3],
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
        let found = detect_p10_min4_native_promotable_shape(&module)
            .expect("p10 min4 native promotable should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.fixed3.v0");
    }

    #[test]
    fn wasm_shape_table_rejects_p10_min4_native_promotable_with_other_calls_contract() {
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
            value: ConstValue::Integer(3),
        });
        block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId(99),
            callee: Some(Callee::Extern("env.canvas.fillRect".to_string())),
            args: vec![v],
            effects: EffectMask::IO,
        });
        block.add_instruction(MirInstruction::Return { value: Some(v) });
        module.add_function(function);

        assert!(
            detect_p10_min4_native_promotable_shape(&module).is_none(),
            "shape with non-console extern must stay outside min4 native promotion"
        );
    }

    fn make_p10_loop_console_method_module(method: &str) -> MirModule {
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
        let c3 = ValueId(3);
        loop_block.add_instruction(MirInstruction::Const {
            dst: c3,
            value: ConstValue::Integer(3),
        });
        loop_block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId(99),
            callee: Some(Callee::Method {
                box_name: "console".to_string(),
                method: method.to_string(),
                receiver: Some(ValueId(98)),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![c3],
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
        module
    }

    #[test]
    fn wasm_shape_table_detects_p10_min5_expansion_warn_inventory_contract() {
        let module = make_p10_loop_console_method_module("warn");
        let found = detect_p10_min5_expansion_inventory_shape(&module)
            .expect("warn inventory shape should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.warn.fixed3.inventory.v0");
    }

    #[test]
    fn wasm_shape_table_detects_p10_min5_expansion_info_inventory_contract() {
        let module = make_p10_loop_console_method_module("info");
        let found = detect_p10_min5_expansion_inventory_shape(&module)
            .expect("info inventory shape should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.info.fixed3.inventory.v0");
    }

    #[test]
    fn wasm_shape_table_detects_p10_min5_expansion_error_inventory_contract() {
        let module = make_p10_loop_console_method_module("error");
        let found = detect_p10_min5_expansion_inventory_shape(&module)
            .expect("error inventory shape should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.error.fixed3.inventory.v0");
    }

    #[test]
    fn wasm_shape_table_detects_p10_min5_expansion_debug_inventory_contract() {
        let module = make_p10_loop_console_method_module("debug");
        let found = detect_p10_min5_expansion_inventory_shape(&module)
            .expect("debug inventory shape should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.debug.fixed3.inventory.v0");
    }

    #[test]
    fn wasm_shape_table_rejects_p10_min5_expansion_unknown_method_contract() {
        let module = make_p10_loop_console_method_module("log");
        assert!(
            detect_p10_min5_expansion_inventory_shape(&module).is_none(),
            "log shape belongs to min4 promotion and must stay outside min5 expansion inventory"
        );
    }

    #[test]
    fn wasm_shape_table_detects_p10_min6_warn_native_promotable_contract() {
        let mut module = make_p10_loop_console_method_module("warn");
        let main = module.get_function_mut("main").expect("main function exists");
        for block in main.blocks.values_mut() {
            for inst in &mut block.instructions {
                if let MirInstruction::Const { value, .. } = inst {
                    if *value == ConstValue::Integer(3) {
                        *value = ConstValue::Integer(4);
                    }
                }
            }
        }
        let found = detect_p10_min6_warn_native_promotable_shape(&module)
            .expect("p10 min6 warn native shape should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.warn.fixed4.v0");
    }

    #[test]
    fn wasm_shape_table_rejects_p10_min6_warn_native_promotable_for_fixed3_contract() {
        let module = make_p10_loop_console_method_module("warn");
        assert!(
            detect_p10_min6_warn_native_promotable_shape(&module).is_none(),
            "fixed3 warn shape must stay outside min6 promotion and remain min5 inventory"
        );
    }

    #[test]
    fn wasm_shape_table_detects_p10_min7_info_native_promotable_contract() {
        let mut module = make_p10_loop_console_method_module("info");
        let main = module.get_function_mut("main").expect("main function exists");
        for block in main.blocks.values_mut() {
            for inst in &mut block.instructions {
                if let MirInstruction::Const { value, .. } = inst {
                    if *value == ConstValue::Integer(3) {
                        *value = ConstValue::Integer(4);
                    }
                }
            }
        }
        let found = detect_p10_min7_info_native_promotable_shape(&module)
            .expect("p10 min7 info native shape should match");
        assert_eq!(found, "wsm.p10.main_loop_extern_call.info.fixed4.v0");
    }

    #[test]
    fn wasm_shape_table_rejects_p10_min7_info_native_promotable_for_fixed3_contract() {
        let module = make_p10_loop_console_method_module("info");
        assert!(
            detect_p10_min7_info_native_promotable_shape(&module).is_none(),
            "fixed3 info shape must stay outside min7 promotion and remain min5 inventory"
        );
    }
}
