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
pub enum SubstringViewsMicroSeedProof {
    KiloMicroSubstringViewsOnly5Block,
}

impl std::fmt::Display for SubstringViewsMicroSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KiloMicroSubstringViewsOnly5Block => {
                f.write_str("kilo_micro_substring_views_only_5block")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubstringViewsMicroSeedRoute {
    pub source: String,
    pub source_len: i64,
    pub loop_bound: i64,
    pub proof: SubstringViewsMicroSeedProof,
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
    expect_ops(&b0, &["const", "const", "mir_call", "const", "jump"])?;
    let source = const_string(b0[0])?;
    if source != "line-seed-abcdef" || const_string(b0[1])? != source {
        return None;
    }
    let source_len = source.len() as i64;
    if source_len != 16
        || !method_call_is(b0[2], &["StringBox"], "length", 0)
        || const_i64(b0[3])? != 0
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
    expect_ops(
        &b2,
        &[
            "const", "const", "binop", "const", "mir_call", "mir_call", "jump",
        ],
    )?;
    if const_i64(b2[0])? != 0
        || const_i64(b2[1])? != 2
        || !binop_is(b2[2], BinaryOp::Div)
        || const_string(b2[3])? != source
        || !method_call_is(b2[4], &["RuntimeDataBox", "StringBox"], "substring", 2)
        || !method_call_is(b2[5], &["RuntimeDataBox", "StringBox"], "substring", 2)
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
    fn detects_exact_5_block_substring_views_route() {
        let mut function = build_exact_function();

        refresh_function_substring_views_micro_seed_route(&mut function);

        let route = function
            .metadata
            .substring_views_micro_seed_route
            .expect("expected exact route");
        assert_eq!(route.source, "line-seed-abcdef");
        assert_eq!(route.source_len, 16);
        assert_eq!(route.loop_bound, 300000);
        assert_eq!(
            route.proof,
            SubstringViewsMicroSeedProof::KiloMicroSubstringViewsOnly5Block
        );
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
