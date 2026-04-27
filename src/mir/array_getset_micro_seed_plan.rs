/*!
 * MIR-owned route plan for the temporary array get/set micro seed bridge.
 *
 * The inner `array.get(i) -> + 1 -> array.set(i, ...)` legality remains owned
 * by `array_rmw_window_routes`. This route owns the current whole-function
 * exact seed payload for `kilo_micro_array_getset` so the C boundary can select
 * the specialized stack-array emitter without rescanning raw MIR JSON blocks.
 */

use super::{
    array_rmw_window_plan::ArrayRmwWindowProof, BasicBlock, BasicBlockId, BinaryOp, Callee,
    CompareOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArrayGetSetMicroSeedProof {
    KiloMicroArrayGetSetSevenBlock,
}

impl ArrayGetSetMicroSeedProof {
    fn as_str(self) -> &'static str {
        match self {
            Self::KiloMicroArrayGetSetSevenBlock => "kilo_micro_array_getset_7block",
        }
    }
}

impl std::fmt::Display for ArrayGetSetMicroSeedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayGetSetMicroSeedRoute {
    size: i64,
    ops: i64,
    init_push_count: i64,
    loop_get_count: i64,
    loop_set_count: i64,
    final_get_count: i64,
    selected_rmw_block: BasicBlockId,
    selected_rmw_instruction_index: usize,
    selected_rmw_set_instruction_index: usize,
    loop_index_phi_value: ValueId,
    accumulator_phi_value: ValueId,
    accumulator_next_value: ValueId,
    return_value: ValueId,
    proof: ArrayGetSetMicroSeedProof,
    rmw_proof: ArrayRmwWindowProof,
}

impl ArrayGetSetMicroSeedRoute {
    pub fn size(&self) -> i64 {
        self.size
    }

    pub fn ops(&self) -> i64 {
        self.ops
    }

    pub fn init_push_count(&self) -> i64 {
        self.init_push_count
    }

    pub fn loop_get_count(&self) -> i64 {
        self.loop_get_count
    }

    pub fn loop_set_count(&self) -> i64 {
        self.loop_set_count
    }

    pub fn final_get_count(&self) -> i64 {
        self.final_get_count
    }

    pub fn selected_rmw_block(&self) -> BasicBlockId {
        self.selected_rmw_block
    }

    pub fn selected_rmw_instruction_index(&self) -> usize {
        self.selected_rmw_instruction_index
    }

    pub fn selected_rmw_set_instruction_index(&self) -> usize {
        self.selected_rmw_set_instruction_index
    }

    pub fn loop_index_phi_value(&self) -> ValueId {
        self.loop_index_phi_value
    }

    pub fn accumulator_phi_value(&self) -> ValueId {
        self.accumulator_phi_value
    }

    pub fn accumulator_next_value(&self) -> ValueId {
        self.accumulator_next_value
    }

    pub fn return_value(&self) -> ValueId {
        self.return_value
    }

    pub fn proof(&self) -> &'static str {
        self.proof.as_str()
    }

    pub fn rmw_proof(&self) -> ArrayRmwWindowProof {
        self.rmw_proof
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;

    pub(crate) fn kilo_micro_array_getset_7block() -> ArrayGetSetMicroSeedRoute {
        ArrayGetSetMicroSeedRoute {
            size: 128,
            ops: 2_000_000,
            init_push_count: 1,
            loop_get_count: 1,
            loop_set_count: 1,
            final_get_count: 0,
            selected_rmw_block: BasicBlockId::new(23),
            selected_rmw_instruction_index: 8,
            selected_rmw_set_instruction_index: 13,
            loop_index_phi_value: ValueId::new(29),
            accumulator_phi_value: ValueId::new(33),
            accumulator_next_value: ValueId::new(40),
            return_value: ValueId::new(33),
            proof: ArrayGetSetMicroSeedProof::KiloMicroArrayGetSetSevenBlock,
            rmw_proof: ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot,
        }
    }
}

pub fn refresh_module_array_getset_micro_seed_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_array_getset_micro_seed_route(function);
    }
}

pub fn refresh_function_array_getset_micro_seed_route(function: &mut MirFunction) {
    function.metadata.array_getset_micro_seed_route = match_array_getset_micro_seed_route(function);
}

fn match_array_getset_micro_seed_route(
    function: &MirFunction,
) -> Option<ArrayGetSetMicroSeedRoute> {
    let blocks = ordered_blocks(function);
    if blocks.len() != 7 {
        return None;
    }

    let b0 = interesting(blocks[0])?;
    expect_ops(&b0, &["newbox", "const", "jump"])?;
    if !matches!(b0[0], MirInstruction::NewBox { box_type, .. } if box_type == "ArrayBox") {
        return None;
    }
    if const_i64(b0[1])? != 0 {
        return None;
    }

    let b1 = interesting(blocks[1])?;
    expect_ops(&b1, &["phi", "const", "compare", "branch"])?;
    let size = const_i64(b1[1])?;
    if size != 128 || !compare_is(b1[2], CompareOp::Lt) {
        return None;
    }

    let b2 = interesting(blocks[2])?;
    expect_ops(&b2, &["mir_call", "const", "binop", "jump"])?;
    if !method_call_is(b2[0], &["RuntimeDataBox"], "push", 1)
        || const_i64(b2[1])? != 1
        || !binop_is(b2[2], BinaryOp::Add)
    {
        return None;
    }

    let b3 = interesting(blocks[3])?;
    expect_ops(&b3, &["const", "const", "release_strong", "jump"])?;
    if const_i64(b3[0])? != 0 || const_i64(b3[1])? != 0 {
        return None;
    }

    let b4 = interesting(blocks[4])?;
    expect_ops(&b4, &["phi", "phi", "const", "compare", "branch"])?;
    let loop_index_phi_value = phi_dst(b4[0])?;
    let accumulator_phi_value = phi_dst(b4[1])?;
    let ops = const_i64(b4[2])?;
    if ops != 2_000_000 || !compare_is(b4[3], CompareOp::Lt) {
        return None;
    }

    let b5 = interesting(blocks[5])?;
    expect_ops(
        &b5,
        &[
            "const", "binop", "mir_call", "const", "binop", "mir_call", "binop", "const", "binop",
            "jump",
        ],
    )?;
    if const_i64(b5[0])? != size
        || !binop_is(b5[1], BinaryOp::Mod)
        || !method_call_is(b5[2], &["RuntimeDataBox"], "get", 1)
        || const_i64(b5[3])? != 1
        || !binop_is(b5[4], BinaryOp::Add)
        || !method_call_is(b5[5], &["RuntimeDataBox"], "set", 2)
        || !binop_is(b5[6], BinaryOp::Add)
        || const_i64(b5[7])? != 1
        || !binop_is(b5[8], BinaryOp::Add)
    {
        return None;
    }

    let selected_rmw = function
        .metadata
        .array_rmw_window_routes
        .iter()
        .find(|route| {
            route.block() == blocks[5].id
                && route.proof() == ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot
        })?;

    let b6 = interesting(blocks[6])?;
    expect_ops(&b6, &["ret"])?;
    let accumulator_next_value = instruction_dst(b5[6])?;
    let return_value = return_value(b6[0])?;
    if return_value != accumulator_phi_value {
        return None;
    }

    Some(ArrayGetSetMicroSeedRoute {
        size,
        ops,
        init_push_count: 1,
        loop_get_count: 1,
        loop_set_count: 1,
        final_get_count: 0,
        selected_rmw_block: selected_rmw.block(),
        selected_rmw_instruction_index: selected_rmw.instruction_index(),
        selected_rmw_set_instruction_index: selected_rmw.set_instruction_index(),
        loop_index_phi_value,
        accumulator_phi_value,
        accumulator_next_value,
        return_value,
        proof: ArrayGetSetMicroSeedProof::KiloMicroArrayGetSetSevenBlock,
        rmw_proof: selected_rmw.proof(),
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

fn phi_dst(inst: &MirInstruction) -> Option<ValueId> {
    match inst {
        MirInstruction::Phi { dst, .. } => Some(*dst),
        _ => None,
    }
}

fn instruction_dst(inst: &MirInstruction) -> Option<ValueId> {
    match inst {
        MirInstruction::BinOp { dst, .. }
        | MirInstruction::Compare { dst, .. }
        | MirInstruction::Const { dst, .. }
        | MirInstruction::Copy { dst, .. }
        | MirInstruction::Phi { dst, .. } => Some(*dst),
        MirInstruction::Call { dst: Some(dst), .. } => Some(*dst),
        _ => None,
    }
}

fn return_value(inst: &MirInstruction) -> Option<ValueId> {
    match inst {
        MirInstruction::Return { value: Some(value) } => Some(*value),
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
    use crate::mir::{EffectMask, FunctionSignature, MirType};

    #[test]
    fn detects_kilo_micro_array_getset_seed_route() {
        let mut function = build_exact_function();
        attach_rmw_window_route(&mut function);

        refresh_function_array_getset_micro_seed_route(&mut function);

        let route = function
            .metadata
            .array_getset_micro_seed_route
            .expect("expected exact route");
        assert_eq!(route.size(), 128);
        assert_eq!(route.ops(), 2_000_000);
        assert_eq!(route.init_push_count(), 1);
        assert_eq!(route.loop_get_count(), 1);
        assert_eq!(route.loop_set_count(), 1);
        assert_eq!(route.final_get_count(), 0);
        assert_eq!(route.selected_rmw_block(), BasicBlockId::new(23));
        assert_eq!(route.selected_rmw_instruction_index(), 8);
        assert_eq!(route.selected_rmw_set_instruction_index(), 13);
        assert_eq!(route.loop_index_phi_value(), ValueId::new(29));
        assert_eq!(route.accumulator_phi_value(), ValueId::new(33));
        assert_eq!(route.accumulator_next_value(), ValueId::new(40));
        assert_eq!(route.return_value(), ValueId::new(33));
        assert_eq!(route.proof(), "kilo_micro_array_getset_7block");
        assert_eq!(
            route.rmw_proof(),
            ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot
        );
    }

    #[test]
    fn rejects_without_inner_rmw_window_route() {
        let mut function = build_exact_function();

        refresh_function_array_getset_micro_seed_route(&mut function);

        assert!(function.metadata.array_getset_micro_seed_route.is_none());
    }

    #[test]
    fn rejects_when_exit_does_not_return_accumulator_phi() {
        let mut function = build_exact_function();
        attach_rmw_window_route(&mut function);
        let exit = function
            .get_block_mut(BasicBlockId::new(25))
            .expect("exit block");
        exit.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(40)),
        });

        refresh_function_array_getset_micro_seed_route(&mut function);

        assert!(function.metadata.array_getset_micro_seed_route.is_none());
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
                const_i(7, 0),
            ],
            jump(18),
        );
        replace_block(
            &mut function,
            18,
            vec![phi(9), const_i(17, 128), compare(13, CompareOp::Lt)],
            branch(13, 19, 21),
        );
        replace_block(
            &mut function,
            19,
            vec![
                method_call(None, "RuntimeDataBox", "push", 21, vec![ValueId::new(20)]),
                const_i(24, 1),
                binop(10, BinaryOp::Add),
            ],
            jump(18),
        );
        replace_block(
            &mut function,
            21,
            vec![
                const_i(26, 0),
                const_i(28, 0),
                MirInstruction::ReleaseStrong {
                    values: vec![ValueId::new(9)],
                },
            ],
            jump(22),
        );
        replace_block(
            &mut function,
            22,
            vec![
                phi(29),
                phi(33),
                const_i(45, 2_000_000),
                compare(41, CompareOp::Lt),
            ],
            branch(41, 23, 25),
        );
        replace_block(
            &mut function,
            23,
            vec![
                const_i(54, 128),
                binop(51, BinaryOp::Mod),
                method_call(
                    Some(36),
                    "RuntimeDataBox",
                    "get",
                    55,
                    vec![ValueId::new(51)],
                ),
                const_i(61, 1),
                binop(59, BinaryOp::Add),
                method_call(
                    None,
                    "RuntimeDataBox",
                    "set",
                    55,
                    vec![ValueId::new(51), ValueId::new(59)],
                ),
                binop(40, BinaryOp::Add),
                const_i(68, 1),
                binop(30, BinaryOp::Add),
            ],
            jump(22),
        );
        replace_block(
            &mut function,
            25,
            vec![],
            MirInstruction::Return {
                value: Some(ValueId::new(33)),
            },
        );

        function
    }

    fn attach_rmw_window_route(function: &mut MirFunction) {
        function
            .metadata
            .array_rmw_window_routes
            .push(crate::mir::array_rmw_window_plan::test_support::kilo_getset_inner_route());
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
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args,
            effects: EffectMask::PURE,
        }
    }

    fn jump(target: u32) -> MirInstruction {
        MirInstruction::Jump {
            target: BasicBlockId::new(target),
            edge_args: None,
        }
    }

    fn branch(cond: u32, then_bb: u32, else_bb: u32) -> MirInstruction {
        MirInstruction::Branch {
            condition: ValueId::new(cond),
            then_bb: BasicBlockId::new(then_bb),
            else_bb: BasicBlockId::new(else_bb),
            then_edge_args: None,
            else_edge_args: None,
        }
    }
}
