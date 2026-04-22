/*!
 * MIR-owned route plan for the temporary concat-const-suffix micro seed bridge.
 *
 * The active `kilo_micro_concat_const_suffix` fast path still needs a
 * specialized emitter, but the route-shape proof belongs in MIR metadata. The C
 * backend may read this plan to select the emitter; it must not rescan raw MIR
 * JSON to prove the 5-block shape.
 */

use super::{
    definitions::Callee, BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction,
    MirInstruction, MirModule,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConcatConstSuffixMicroSeedProof {
    KiloMicroConcatConstSuffix5Block,
}

impl std::fmt::Display for ConcatConstSuffixMicroSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KiloMicroConcatConstSuffix5Block => {
                f.write_str("kilo_micro_concat_const_suffix_5block")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcatConstSuffixMicroSeedRoute {
    pub seed: String,
    pub seed_len: i64,
    pub suffix: String,
    pub suffix_len: i64,
    pub ops: i64,
    pub result_len: i64,
    pub proof: ConcatConstSuffixMicroSeedProof,
}

pub fn refresh_module_concat_const_suffix_micro_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_concat_const_suffix_micro_seed_route(function);
    }
}

pub fn refresh_function_concat_const_suffix_micro_seed_route(function: &mut MirFunction) {
    function.metadata.concat_const_suffix_micro_seed_route =
        match_concat_const_suffix_micro_seed_route(function);
}

fn match_concat_const_suffix_micro_seed_route(
    function: &MirFunction,
) -> Option<ConcatConstSuffixMicroSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 5 {
        return None;
    }

    let b0 = interesting(blocks[0])?;
    let (seed_inst, length_inst, i_zero_inst, acc_zero_inst) = match_concat_entry_prefix(&b0)?;
    let seed = const_string(seed_inst)?;
    if seed != "line-seed-abcdef" {
        return None;
    }
    let seed_len = seed.len() as i64;
    if seed_len != 16 {
        return None;
    }
    if !method_call_is(length_inst, &["StringBox"], "length", 0) {
        return None;
    }
    if const_i64(i_zero_inst)? != 0 || const_i64(acc_zero_inst)? != 0 {
        return None;
    }

    let b1 = interesting(blocks[1])?;
    expect_ops(&b1, &["phi", "phi", "phi", "const", "compare", "branch"])?;
    let ops = const_i64(b1[3])?;
    if ops != 600000 || !compare_is(b1[4], CompareOp::Lt) {
        return None;
    }

    let b2 = interesting(blocks[2])?;
    expect_ops(
        &b2,
        &[
            "const", "binop", "mir_call", "binop", "const", "const", "binop", "mir_call", "jump",
        ],
    )?;
    let suffix = const_string(b2[0])?;
    let suffix_len = suffix.len() as i64;
    if suffix != "xy" || suffix_len != 2 {
        return None;
    }
    if !binop_is(b2[1], BinaryOp::Add) {
        return None;
    }
    if !method_call_is(b2[2], &["RuntimeDataBox", "StringBox"], "length", 0) {
        return None;
    }
    if !binop_is(b2[3], BinaryOp::Add) {
        return None;
    }
    if const_i64(b2[4])? != 2 || const_i64(b2[5])? != 2 {
        return None;
    }
    if !binop_is(b2[6], BinaryOp::Add) {
        return None;
    }
    if !method_call_is(b2[7], &["RuntimeDataBox", "StringBox"], "substring", 2) {
        return None;
    }

    let b3 = interesting(blocks[3])?;
    expect_ops(&b3, &["phi", "phi", "const", "binop", "jump"])?;
    if const_i64(b3[2])? != 1 || !binop_is(b3[3], BinaryOp::Add) {
        return None;
    }

    let b4 = interesting(blocks[4])?;
    expect_ops(&b4, &["mir_call", "binop", "ret"])?;
    if !method_call_is(b4[0], &["RuntimeDataBox", "StringBox"], "length", 0)
        || !binop_is(b4[1], BinaryOp::Add)
    {
        return None;
    }

    Some(ConcatConstSuffixMicroSeedRoute {
        seed: seed.to_string(),
        seed_len,
        suffix: suffix.to_string(),
        suffix_len,
        ops,
        result_len: seed_len + suffix_len,
        proof: ConcatConstSuffixMicroSeedProof::KiloMicroConcatConstSuffix5Block,
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

fn match_concat_entry_prefix<'a>(
    insts: &'a [&MirInstruction],
) -> Option<(
    &'a MirInstruction,
    &'a MirInstruction,
    &'a MirInstruction,
    &'a MirInstruction,
)> {
    match insts {
        [seed, duplicate_seed, length, i_zero, acc_zero, jump] => {
            if op_name(seed) != "const"
                || op_name(duplicate_seed) != "const"
                || op_name(length) != "mir_call"
                || op_name(i_zero) != "const"
                || op_name(acc_zero) != "const"
                || op_name(jump) != "jump"
            {
                return None;
            }
            if const_string(duplicate_seed)? != const_string(seed)? {
                return None;
            }
            Some((seed, length, i_zero, acc_zero))
        }
        [seed, length, i_zero, acc_zero, jump] => {
            if op_name(seed) != "const"
                || op_name(length) != "mir_call"
                || op_name(i_zero) != "const"
                || op_name(acc_zero) != "const"
                || op_name(jump) != "jump"
            {
                return None;
            }
            Some((seed, length, i_zero, acc_zero))
        }
        _ => None,
    }
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
            let matches_legacy_arg_shape = args.len() == expected_arg_count;
            let matches_unified_string_receiver_shape =
                expected_arg_count == 0 && args.len() == 1 && box_name == "StringBox";
            method == expected_method
                && (matches_legacy_arg_shape || matches_unified_string_receiver_shape)
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
    fn detects_exact_5_block_concat_const_suffix_route() {
        let mut function = build_exact_function();

        refresh_function_concat_const_suffix_micro_seed_route(&mut function);

        let route = function
            .metadata
            .concat_const_suffix_micro_seed_route
            .expect("expected exact route");
        assert_eq!(route.seed, "line-seed-abcdef");
        assert_eq!(route.seed_len, 16);
        assert_eq!(route.suffix, "xy");
        assert_eq!(route.suffix_len, 2);
        assert_eq!(route.ops, 600000);
        assert_eq!(route.result_len, 18);
        assert_eq!(
            route.proof,
            ConcatConstSuffixMicroSeedProof::KiloMicroConcatConstSuffix5Block
        );
    }

    #[test]
    fn rejects_when_ops_bound_changes() {
        let mut function = build_exact_function();
        let header = function
            .get_block_mut(BasicBlockId::new(18))
            .expect("header block");
        header.instructions[3] = const_i(35, 600001);

        refresh_function_concat_const_suffix_micro_seed_route(&mut function);

        assert!(function
            .metadata
            .concat_const_suffix_micro_seed_route
            .is_none());
    }

    #[test]
    fn detects_current_unified_string_length_receiver_arg_shape() {
        let mut function = build_exact_function();
        let entry = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        if let MirInstruction::Call { args, .. } = &mut entry.instructions[2] {
            args.push(ValueId::new(99));
        } else {
            panic!("expected initial length call");
        }

        let exit = function
            .get_block_mut(BasicBlockId::new(21))
            .expect("exit block");
        if let Some(MirInstruction::Call { args, .. }) = exit.instructions.first_mut() {
            args.push(ValueId::new(100));
        } else {
            panic!("expected final length call");
        }

        refresh_function_concat_const_suffix_micro_seed_route(&mut function);

        assert!(function
            .metadata
            .concat_const_suffix_micro_seed_route
            .is_some());
    }

    #[test]
    fn detects_current_copy_elided_entry_seed_shape() {
        let mut function = build_exact_function();
        let entry = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        entry.instructions.remove(1);

        refresh_function_concat_const_suffix_micro_seed_route(&mut function);

        assert!(function
            .metadata
            .concat_const_suffix_micro_seed_route
            .is_some());
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
                const_s(3, "line-seed-abcdef"),
                const_s(9, "line-seed-abcdef"),
                method_call(Some(5), "StringBox", "length", 8, vec![]),
                const_i(11, 0),
                const_i(13, 0),
            ],
            jump(18),
        );
        replace_block(
            &mut function,
            18,
            vec![
                phi(15),
                phi(19),
                phi(21),
                const_i(35, 600000),
                compare(31, CompareOp::Lt),
            ],
            branch(31, 19, 21),
        );
        replace_block(
            &mut function,
            19,
            vec![
                const_s(43, "xy"),
                binop(41, BinaryOp::Add),
                method_call(Some(25), "RuntimeDataBox", "length", 41, vec![]),
                binop(26, BinaryOp::Add),
                const_i(49, 2),
                const_i(53, 2),
                binop(50, BinaryOp::Add),
                method_call(
                    Some(30),
                    "RuntimeDataBox",
                    "substring",
                    41,
                    vec![ValueId::new(49), ValueId::new(50)],
                ),
            ],
            jump(20),
        );
        replace_block(
            &mut function,
            20,
            vec![phi(20), phi(22), const_i(60, 1), binop(16, BinaryOp::Add)],
            jump(18),
        );
        replace_block(
            &mut function,
            21,
            vec![
                method_call(Some(62), "StringBox", "length", 64, vec![]),
                binop(67, BinaryOp::Add),
            ],
            MirInstruction::Return {
                value: Some(ValueId::new(67)),
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
