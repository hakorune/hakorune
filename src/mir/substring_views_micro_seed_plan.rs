/*!
 * MIR-owned route plan for the temporary substring-views micro seed bridge.
 *
 * `StringKernelPlan` already owns the borrowed-slice window proofs. This module
 * only carries the exact micro bridge payload that the current temporary emitter
 * still needs: source literal/length and the loop bound guard.
 */

use super::{
    definitions::Callee, BasicBlock, BasicBlockId, BinaryOp, CompareOp, ConstValue, MirFunction,
    MirInstruction, MirModule,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubstringViewsMicroSeedProof {
    KiloMicroSubstringViewsOnly5Block,
}

impl SubstringViewsMicroSeedProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::KiloMicroSubstringViewsOnly5Block => "kilo_micro_substring_views_only_5block",
        }
    }
}

impl std::fmt::Display for SubstringViewsMicroSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstringViewsMicroSeedRoute {
    source: String,
    source_len: i64,
    loop_bound: i64,
    proof: SubstringViewsMicroSeedProof,
}

impl SubstringViewsMicroSeedRoute {
    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn source_len(&self) -> i64 {
        self.source_len
    }

    pub fn loop_bound(&self) -> i64 {
        self.loop_bound
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn kilo_micro_substring_views_only_5block() -> SubstringViewsMicroSeedRoute {
        SubstringViewsMicroSeedRoute {
            source: "line-seed-abcdef".to_string(),
            source_len: 16,
            loop_bound: 300000,
            proof: SubstringViewsMicroSeedProof::KiloMicroSubstringViewsOnly5Block,
        }
    }
}

pub fn refresh_module_substring_views_micro_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_substring_views_micro_seed_route(function);
    }
}

pub fn refresh_function_substring_views_micro_seed_route(function: &mut MirFunction) {
    function.metadata.substring_views_micro_seed_route =
        match_substring_views_micro_seed_route(function);
}

fn match_substring_views_micro_seed_route(
    function: &MirFunction,
) -> Option<SubstringViewsMicroSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 5 {
        return None;
    }

    let b0 = interesting(blocks[0]);
    let (source_inst, length_inst, zero_inst) = match_substring_views_entry_prefix(&b0)?;
    let source = const_string(source_inst)?;
    if source != "line-seed-abcdef" {
        return None;
    }
    let source_len = source.len() as i64;
    if source_len != 16
        || !method_call_is(length_inst, &["StringBox"], "length", 0)
        || const_i64(zero_inst)? != 0
    {
        return None;
    }

    let b1 = interesting(blocks[1]);
    expect_ops(&b1, &["phi", "phi", "phi", "const", "compare", "branch"])?;
    let loop_bound = const_i64(b1[3])?;
    if loop_bound != 300000 || !compare_is(b1[4], CompareOp::Lt) {
        return None;
    }

    let b2 = interesting(blocks[2]);
    let (zero_inst, split_const_inst, split_inst, first_substring_inst, second_substring_inst) =
        match_substring_views_body_prefix(&b2, source)?;
    if const_i64(zero_inst)? != 0
        || const_i64(split_const_inst)? != 2
        || !binop_is(split_inst, BinaryOp::Div)
        || !method_call_is(
            first_substring_inst,
            &["RuntimeDataBox", "StringBox"],
            "substring",
            2,
        )
        || !method_call_is(
            second_substring_inst,
            &["RuntimeDataBox", "StringBox"],
            "substring",
            2,
        )
    {
        return None;
    }

    let b3 = interesting(blocks[3]);
    expect_ops(&b3, &["phi", "phi", "const", "binop", "jump"])?;
    if const_i64(b3[2])? != 1 || !binop_is(b3[3], BinaryOp::Add) {
        return None;
    }

    let b4 = interesting(blocks[4]);
    expect_ops(&b4, &["mir_call", "mir_call", "binop", "binop", "ret"])?;
    if !method_call_is(b4[0], &["RuntimeDataBox", "StringBox"], "length", 0)
        || !method_call_is(b4[1], &["RuntimeDataBox", "StringBox"], "length", 0)
        || !binop_is(b4[2], BinaryOp::Add)
        || !binop_is(b4[3], BinaryOp::Add)
    {
        return None;
    }

    Some(SubstringViewsMicroSeedRoute {
        source: source.to_string(),
        source_len,
        loop_bound,
        proof: SubstringViewsMicroSeedProof::KiloMicroSubstringViewsOnly5Block,
    })
}

fn ordered_blocks(function: &MirFunction) -> Vec<&BasicBlock> {
    let mut ids: Vec<BasicBlockId> = function.blocks.keys().copied().collect();
    ids.sort();
    ids.into_iter()
        .filter_map(|id| function.blocks.get(&id))
        .collect()
}

fn interesting(block: &BasicBlock) -> Vec<&MirInstruction> {
    let mut insts: Vec<&MirInstruction> = block
        .instructions
        .iter()
        .filter(|inst| !matches!(inst, MirInstruction::Copy { .. }))
        .collect();
    if let Some(terminator) = block.terminator.as_ref() {
        insts.push(terminator);
    }
    insts
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

fn match_substring_views_entry_prefix<'a>(
    insts: &'a [&MirInstruction],
) -> Option<(&'a MirInstruction, &'a MirInstruction, &'a MirInstruction)> {
    match insts {
        [source, duplicate_source, length, zero, jump] => {
            if op_name(source) != "const"
                || op_name(duplicate_source) != "const"
                || op_name(length) != "mir_call"
                || op_name(zero) != "const"
                || op_name(jump) != "jump"
            {
                return None;
            }
            if const_string(duplicate_source)? != const_string(source)? {
                return None;
            }
            Some((source, length, zero))
        }
        [source, length, zero, jump] => {
            if op_name(source) != "const"
                || op_name(length) != "mir_call"
                || op_name(zero) != "const"
                || op_name(jump) != "jump"
            {
                return None;
            }
            Some((source, length, zero))
        }
        _ => None,
    }
}

fn match_substring_views_body_prefix<'a>(
    insts: &'a [&MirInstruction],
    source: &str,
) -> Option<(
    &'a MirInstruction,
    &'a MirInstruction,
    &'a MirInstruction,
    &'a MirInstruction,
    &'a MirInstruction,
)> {
    match insts {
        [zero, split_const, split, source_inst, first_substring, second_substring, jump] => {
            if op_name(zero) != "const"
                || op_name(split_const) != "const"
                || op_name(split) != "binop"
                || op_name(source_inst) != "const"
                || op_name(first_substring) != "mir_call"
                || op_name(second_substring) != "mir_call"
                || op_name(jump) != "jump"
            {
                return None;
            }
            if const_string(source_inst)? != source {
                return None;
            }
            Some((zero, split_const, split, first_substring, second_substring))
        }
        [zero, split_const, split, first_substring, second_substring, jump] => {
            if op_name(zero) != "const"
                || op_name(split_const) != "const"
                || op_name(split) != "binop"
                || op_name(first_substring) != "mir_call"
                || op_name(second_substring) != "mir_call"
                || op_name(jump) != "jump"
            {
                return None;
            }
            Some((zero, split_const, split, first_substring, second_substring))
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
    fn detects_exact_5_block_substring_views_route() {
        let mut function = build_exact_function();

        refresh_function_substring_views_micro_seed_route(&mut function);

        let route = function
            .metadata
            .substring_views_micro_seed_route
            .expect("expected exact route");
        assert_eq!(route.source(), "line-seed-abcdef");
        assert_eq!(route.source_len(), 16);
        assert_eq!(route.loop_bound(), 300000);
        assert_eq!(route.proof(), "kilo_micro_substring_views_only_5block");
    }

    #[test]
    fn rejects_when_loop_bound_changes() {
        let mut function = build_exact_function();
        let header = function
            .get_block_mut(BasicBlockId::new(18))
            .expect("header block");
        header.instructions[3] = const_i(36, 300001);

        refresh_function_substring_views_micro_seed_route(&mut function);

        assert!(function.metadata.substring_views_micro_seed_route.is_none());
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
            panic!("expected left length call");
        }
        if let Some(MirInstruction::Call { args, .. }) = exit.instructions.get_mut(1) {
            args.push(ValueId::new(101));
        } else {
            panic!("expected right length call");
        }

        refresh_function_substring_views_micro_seed_route(&mut function);

        assert!(function.metadata.substring_views_micro_seed_route.is_some());
    }

    #[test]
    fn detects_current_copy_elided_source_shapes() {
        let mut function = build_exact_function();
        let entry = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block");
        entry.instructions.remove(1);
        let body = function
            .get_block_mut(BasicBlockId::new(19))
            .expect("body block");
        body.instructions.remove(3);

        refresh_function_substring_views_micro_seed_route(&mut function);

        assert!(function.metadata.substring_views_micro_seed_route.is_some());
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
                const_i(17, 0),
            ],
            jump(18),
        );
        replace_block(
            &mut function,
            18,
            vec![
                phi(21),
                phi(25),
                phi(27),
                const_i(36, 300000),
                compare(32, CompareOp::Lt),
            ],
            branch(32, 19, 21),
        );
        replace_block(
            &mut function,
            19,
            vec![
                const_i(39, 0),
                const_i(45, 2),
                binop(41, BinaryOp::Div),
                const_s(47, "line-seed-abcdef"),
                method_call(
                    Some(30),
                    "RuntimeDataBox",
                    "substring",
                    46,
                    vec![ValueId::new(39), ValueId::new(40)],
                ),
                method_call(
                    Some(31),
                    "RuntimeDataBox",
                    "substring",
                    46,
                    vec![ValueId::new(40), ValueId::new(48)],
                ),
            ],
            jump(20),
        );
        replace_block(
            &mut function,
            20,
            vec![phi(26), phi(28), const_i(53, 1), binop(22, BinaryOp::Add)],
            jump(18),
        );
        replace_block(
            &mut function,
            21,
            vec![
                method_call(Some(55), "StringBox", "length", 57, vec![]),
                method_call(Some(58), "StringBox", "length", 60, vec![]),
                binop(64, BinaryOp::Add),
                binop(69, BinaryOp::Add),
            ],
            MirInstruction::Return {
                value: Some(ValueId::new(69)),
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
