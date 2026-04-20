/*!
 * MIR-owned route plan for the temporary array/string-store micro seed bridge.
 *
 * This keeps the current exact `kilo_micro_array_string_store` route-shape
 * recognition in MIR metadata. The C backend may consume the plan to select the
 * existing specialized emitter, but it must not rescan raw MIR JSON for this
 * semantic shape.
 */

use super::{
    definitions::Callee, BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction,
    MirInstruction, MirModule,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayStringStoreMicroSeedProof {
    KiloMicroArrayStringStore8Block,
}

impl std::fmt::Display for ArrayStringStoreMicroSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KiloMicroArrayStringStore8Block => {
                f.write_str("kilo_micro_array_string_store_8block")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayStringStoreMicroSeedRoute {
    pub seed: String,
    pub seed_len: i64,
    pub size: i64,
    pub ops: i64,
    pub suffix: String,
    pub store_len: i64,
    pub next_text_window_start: i64,
    pub next_text_window_len: i64,
    pub proof: ArrayStringStoreMicroSeedProof,
}

pub fn refresh_module_array_string_store_micro_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_string_store_micro_seed_route(function);
    }
}

pub fn refresh_function_array_string_store_micro_seed_route(function: &mut MirFunction) {
    function.metadata.array_string_store_micro_seed_route =
        match_array_string_store_micro_seed_route(function);
}

fn match_array_string_store_micro_seed_route(
    function: &MirFunction,
) -> Option<ArrayStringStoreMicroSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 8 {
        return None;
    }

    let b0 = interesting(blocks[0])?;
    expect_ops(&b0, &["newbox", "const", "const", "jump"])?;
    if !matches!(b0[0], MirInstruction::NewBox { box_type, .. } if box_type == "ArrayBox") {
        return None;
    }
    let seed = const_string(b0[1])?;
    if seed != "line-seed-abcdef" {
        return None;
    }
    let seed_len = seed.len() as i64;
    if seed_len != 16 || const_i64(b0[2])? != 0 {
        return None;
    }

    let b1 = interesting(blocks[1])?;
    expect_ops(&b1, &["phi", "const", "compare", "branch"])?;
    let size = const_i64(b1[1])?;
    if size != 128 || !compare_is(b1[2], CompareOp::Lt) {
        return None;
    }

    let b2 = interesting(blocks[2])?;
    expect_ops(&b2, &["const", "mir_call", "const", "binop", "jump"])?;
    if const_string(b2[0])? != seed {
        return None;
    }
    if !method_call_is(b2[1], &["RuntimeDataBox"], "push", 1) {
        return None;
    }
    if const_i64(b2[2])? != 1 || !binop_is(b2[3], BinaryOp::Add) {
        return None;
    }

    let b3 = interesting(blocks[3])?;
    if !b3
        .iter()
        .any(|inst| matches!(inst, MirInstruction::ReleaseStrong { .. }))
    {
        return None;
    }

    let b4 = interesting(blocks[4])?;
    expect_ops(&b4, &["phi", "phi", "phi", "const", "compare", "branch"])?;
    let ops = const_i64(b4[3])?;
    if ops != 800000 || !compare_is(b4[4], CompareOp::Lt) {
        return None;
    }

    let b5 = interesting(blocks[5])?;
    expect_ops(
        &b5,
        &[
            "const", "binop", "const", "binop", "mir_call", "mir_call", "binop", "const",
            "mir_call", "const", "const", "binop", "mir_call", "jump",
        ],
    )?;
    if const_i64(b5[0])? != size || !binop_is(b5[1], BinaryOp::Mod) {
        return None;
    }
    let suffix = const_string(b5[2])?;
    if suffix != "xy" || !binop_is(b5[3], BinaryOp::Add) {
        return None;
    }
    if !method_call_is(b5[4], &["RuntimeDataBox", "StringBox"], "set", 2) {
        return None;
    }
    if !method_call_is(b5[5], &["RuntimeDataBox", "StringBox"], "length", 0) {
        return None;
    }
    if !binop_is(b5[6], BinaryOp::Add) || const_string(b5[7])? != seed {
        return None;
    }
    if !method_call_is(b5[8], &["RuntimeDataBox", "StringBox"], "length", 0) {
        return None;
    }
    let next_text_window_start = const_i64(b5[9])?;
    if next_text_window_start != 2
        || const_i64(b5[10])? != next_text_window_start
        || !binop_is(b5[11], BinaryOp::Add)
    {
        return None;
    }
    if !method_call_is(b5[12], &["RuntimeDataBox", "StringBox"], "substring", 2) {
        return None;
    }

    let b6 = interesting(blocks[6])?;
    expect_ops(&b6, &["phi", "phi", "const", "binop", "jump"])?;
    if const_i64(b6[2])? != 1 || !binop_is(b6[3], BinaryOp::Add) {
        return None;
    }

    let b7 = interesting(blocks[7])?;
    expect_ops(&b7, &["mir_call", "binop", "ret"])?;
    if !method_call_is(b7[0], &["RuntimeDataBox", "StringBox"], "length", 0)
        || !binop_is(b7[1], BinaryOp::Add)
    {
        return None;
    }

    Some(ArrayStringStoreMicroSeedRoute {
        seed: seed.to_string(),
        seed_len,
        size,
        ops,
        suffix: suffix.to_string(),
        store_len: seed_len + suffix.len() as i64,
        next_text_window_start,
        next_text_window_len: seed_len,
        proof: ArrayStringStoreMicroSeedProof::KiloMicroArrayStringStore8Block,
    })
}

fn ordered_blocks(function: &MirFunction) -> Vec<&BasicBlock> {
    let mut ids: Vec<BasicBlockId> = function.blocks.keys().copied().collect();
    ids.sort();
    ids.into_iter()
        .filter_map(|id| function.blocks.get(&id))
        .collect()
}

fn interesting(block: &BasicBlock) -> Option<Vec<&MirInstruction>> {
    let mut insts: Vec<&MirInstruction> = block
        .instructions
        .iter()
        .filter(|inst| !matches!(inst, MirInstruction::Copy { .. }))
        .collect();
    insts.push(block.terminator.as_ref()?);
    Some(insts)
}

fn expect_ops(insts: &[&MirInstruction], expected: &[&str]) -> Option<()> {
    if insts.len() != expected.len() {
        return None;
    }
    for (inst, expected) in insts.iter().zip(expected.iter().copied()) {
        if op_name(inst) != expected {
            return None;
        }
    }
    Some(())
}

fn op_name(inst: &MirInstruction) -> &'static str {
    match inst {
        MirInstruction::Const { .. } => "const",
        MirInstruction::BinOp { .. } => "binop",
        MirInstruction::Compare { .. } => "compare",
        MirInstruction::Call { .. } => "mir_call",
        MirInstruction::Branch { .. } => "branch",
        MirInstruction::Jump { .. } => "jump",
        MirInstruction::Return { .. } => "ret",
        MirInstruction::Phi { .. } => "phi",
        MirInstruction::NewBox { .. } => "newbox",
        MirInstruction::ReleaseStrong { .. } => "release_strong",
        MirInstruction::Copy { .. } => "copy",
        _ => "other",
    }
}

fn const_i64(inst: &MirInstruction) -> Option<i64> {
    match inst {
        MirInstruction::Const {
            value: ConstValue::Integer(value),
            ..
        } => Some(*value),
        _ => None,
    }
}

fn const_string(inst: &MirInstruction) -> Option<&str> {
    match inst {
        MirInstruction::Const {
            value: ConstValue::String(value),
            ..
        } => Some(value.as_str()),
        _ => None,
    }
}

fn binop_is(inst: &MirInstruction, expected: BinaryOp) -> bool {
    matches!(inst, MirInstruction::BinOp { op, .. } if *op == expected)
}

fn compare_is(inst: &MirInstruction, expected: CompareOp) -> bool {
    matches!(inst, MirInstruction::Compare { op, .. } if *op == expected)
}

fn method_call_is(
    inst: &MirInstruction,
    allowed_box_names: &[&str],
    expected_method: &str,
    expected_arg_count: usize,
) -> bool {
    match inst {
        MirInstruction::Call {
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            args,
            ..
        } => {
            method == expected_method
                && args.len() == expected_arg_count
                && allowed_box_names.iter().any(|allowed| *allowed == box_name)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{EffectMask, FunctionSignature, MirType, ValueId};

    #[test]
    fn detects_exact_8_block_micro_seed_route() {
        let mut function = build_exact_function();

        refresh_function_array_string_store_micro_seed_route(&mut function);

        let route = function
            .metadata
            .array_string_store_micro_seed_route
            .expect("expected exact route");
        assert_eq!(route.seed, "line-seed-abcdef");
        assert_eq!(route.seed_len, 16);
        assert_eq!(route.size, 128);
        assert_eq!(route.ops, 800000);
        assert_eq!(route.suffix, "xy");
        assert_eq!(route.store_len, 18);
        assert_eq!(route.next_text_window_start, 2);
        assert_eq!(route.next_text_window_len, 16);
        assert_eq!(
            route.proof,
            ArrayStringStoreMicroSeedProof::KiloMicroArrayStringStore8Block
        );
    }

    #[test]
    fn rejects_when_loop_increment_is_not_split_out() {
        let mut function = build_exact_function();
        let body = function
            .get_block_mut(BasicBlockId::new(23))
            .expect("body block");
        body.instructions.push(const_i(97, 1));
        body.instructions.push(binop(35, BinaryOp::Add));
        function.blocks.remove(&BasicBlockId::new(24));

        refresh_function_array_string_store_micro_seed_route(&mut function);

        assert!(function
            .metadata
            .array_string_store_micro_seed_route
            .is_none());
    }

    fn build_exact_function() -> MirFunction {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));

        replace_block(
            &mut function,
            0,
            vec![
                MirInstruction::NewBox {
                    dst: ValueId::new(5),
                    box_type: "ArrayBox".to_string(),
                    args: vec![],
                },
                const_s(7, "line-seed-abcdef"),
                const_i(9, 0),
            ],
            jump(18),
        );
        replace_block(
            &mut function,
            18,
            vec![phi(11), const_i(19, 128), compare(15, CompareOp::Lt)],
            branch(15, 19, 21),
        );
        replace_block(
            &mut function,
            19,
            vec![
                const_s(23, "line-seed-abcdef"),
                method_call(None, "RuntimeDataBox", "push", 24, vec![ValueId::new(22)]),
                const_i(28, 1),
                binop(12, BinaryOp::Add),
            ],
            jump(18),
        );
        replace_block(
            &mut function,
            21,
            vec![
                const_i(30, 0),
                const_i(33, 0),
                MirInstruction::ReleaseStrong {
                    values: vec![ValueId::new(30)],
                },
            ],
            jump(22),
        );
        replace_block(
            &mut function,
            22,
            vec![
                phi(34),
                phi(38),
                phi(40),
                const_i(57, 800000),
                compare(53, CompareOp::Lt),
            ],
            branch(53, 23, 25),
        );
        replace_block(
            &mut function,
            23,
            vec![
                const_i(68, 128),
                binop(65, BinaryOp::Mod),
                const_s(71, "xy"),
                binop(69, BinaryOp::Add),
                method_call(
                    None,
                    "RuntimeDataBox",
                    "set",
                    72,
                    vec![ValueId::new(65), ValueId::new(69)],
                ),
                method_call(Some(45), "RuntimeDataBox", "length", 75, vec![]),
                binop(47, BinaryOp::Add),
                const_s(81, "line-seed-abcdef"),
                method_call(Some(49), "RuntimeDataBox", "length", 80, vec![]),
                const_i(85, 2),
                const_i(88, 2),
                binop(86, BinaryOp::Add),
                method_call(
                    Some(52),
                    "RuntimeDataBox",
                    "substring",
                    69,
                    vec![ValueId::new(85), ValueId::new(86)],
                ),
            ],
            jump(24),
        );
        replace_block(
            &mut function,
            24,
            vec![phi(39), phi(41), const_i(97, 1), binop(35, BinaryOp::Add)],
            jump(22),
        );
        replace_block(
            &mut function,
            25,
            vec![
                method_call(Some(99), "StringBox", "length", 101, vec![]),
                binop(104, BinaryOp::Add),
            ],
            MirInstruction::Return {
                value: Some(ValueId::new(104)),
            },
        );

        function
    }

    fn replace_block(
        function: &mut MirFunction,
        id: u32,
        instructions: Vec<MirInstruction>,
        terminator: MirInstruction,
    ) {
        let mut block = BasicBlock::new(BasicBlockId::new(id));
        for instruction in instructions {
            block.add_instruction(instruction);
        }
        block.set_terminator(terminator);
        function.add_block(block);
    }

    fn const_i(dst: u32, value: i64) -> MirInstruction {
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value: ConstValue::Integer(value),
        }
    }

    fn const_s(dst: u32, value: &str) -> MirInstruction {
        MirInstruction::Const {
            dst: ValueId::new(dst),
            value: ConstValue::String(value.to_string()),
        }
    }

    fn binop(dst: u32, op: BinaryOp) -> MirInstruction {
        MirInstruction::BinOp {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        }
    }

    fn compare(dst: u32, op: CompareOp) -> MirInstruction {
        MirInstruction::Compare {
            dst: ValueId::new(dst),
            op,
            lhs: ValueId::new(1),
            rhs: ValueId::new(2),
        }
    }

    fn phi(dst: u32) -> MirInstruction {
        MirInstruction::Phi {
            dst: ValueId::new(dst),
            inputs: vec![],
            type_hint: None,
        }
    }

    fn method_call(
        dst: Option<u32>,
        box_name: &str,
        method: &str,
        receiver: u32,
        args: Vec<ValueId>,
    ) -> MirInstruction {
        MirInstruction::Call {
            dst: dst.map(ValueId::new),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: box_name.to_string(),
                method: method.to_string(),
                receiver: Some(ValueId::new(receiver)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::IO,
        }
    }

    fn jump(target: u32) -> MirInstruction {
        MirInstruction::Jump {
            target: BasicBlockId::new(target),
            edge_args: None,
        }
    }

    fn branch(condition: u32, then_bb: u32, else_bb: u32) -> MirInstruction {
        MirInstruction::Branch {
            condition: ValueId::new(condition),
            then_bb: BasicBlockId::new(then_bb),
            else_bb: BasicBlockId::new(else_bb),
            then_edge_args: None,
            else_edge_args: None,
        }
    }
}
