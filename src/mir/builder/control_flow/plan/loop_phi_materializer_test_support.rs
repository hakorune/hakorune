use crate::mir::builder::MirBuilder;
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirType, ValueId,
};

pub(in crate::mir::builder) fn bb(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

pub(in crate::mir::builder) fn seed_builder(builder: &mut MirBuilder) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "m6b/accum/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        bb(0),
    );
    for id in 1..8 {
        function.add_block(BasicBlock::new(bb(id)));
    }
    {
        let entry = function.get_block_mut(bb(0)).unwrap();
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(30),
            value: ConstValue::Bool(true),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::Integer(0),
        });
        entry.add_instruction(MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(0),
        });
        entry.set_terminator(MirInstruction::Jump {
            target: bb(1),
            edge_args: None,
        });
    }
    function
        .get_block_mut(bb(1))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: ValueId::new(30),
            then_bb: bb(2),
            else_bb: bb(3),
            then_edge_args: None,
            else_edge_args: None,
        });
    {
        let body = function.get_block_mut(bb(2)).unwrap();
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Integer(1),
        });
        body.add_instruction(MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::Integer(1),
        });
        body.set_terminator(MirInstruction::Jump {
            target: bb(1),
            edge_args: None,
        });
    }
    function
        .get_block_mut(bb(3))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_predecessor(bb(0));
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_predecessor(bb(2));
    function
        .get_block_mut(bb(2))
        .unwrap()
        .add_predecessor(bb(1));
    function
        .get_block_mut(bb(3))
        .unwrap()
        .add_predecessor(bb(1));
    for (value, ty) in [
        (30, MirType::Bool),
        (10, MirType::Integer),
        (11, MirType::Integer),
        (12, MirType::Integer),
        (13, MirType::Integer),
    ] {
        function
            .metadata
            .value_types
            .insert(ValueId::new(value), ty);
    }
    let value_types = function.metadata.value_types.clone();
    builder.function_state.current_function = Some(function);
    builder.function_state.type_ctx.value_types = value_types;
}

pub(in crate::mir::builder) fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    seed_builder(&mut builder);
    builder
}

pub(in crate::mir::builder) fn standard5_builder() -> MirBuilder {
    let mut builder = seeded_builder();
    let function = builder.function_state.current_function.as_mut().unwrap();
    let body = function.get_block_mut(bb(2)).unwrap();
    body.instructions.clear();
    body.instruction_spans.clear();
    function
        .get_block_mut(bb(1))
        .unwrap()
        .predecessors
        .remove(&bb(2));
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_predecessor(bb(3));
    function
        .get_block_mut(bb(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: bb(3),
            edge_args: None,
        });
    function
        .get_block_mut(bb(3))
        .unwrap()
        .predecessors
        .remove(&bb(1));
    function
        .get_block_mut(bb(3))
        .unwrap()
        .add_predecessor(bb(2));
    let step = function.get_block_mut(bb(3)).unwrap();
    step.add_instruction(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(1),
    });
    step.add_instruction(MirInstruction::Const {
        dst: ValueId::new(13),
        value: ConstValue::Integer(1),
    });
    step.set_terminator(MirInstruction::Jump {
        target: bb(1),
        edge_args: None,
    });
    function
        .get_block_mut(bb(4))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    function
        .get_block_mut(bb(4))
        .unwrap()
        .add_predecessor(bb(1));
    builder
}

pub(in crate::mir::builder) fn nested_resume_builder() -> MirBuilder {
    let mut builder = seeded_builder();
    let function = builder.function_state.current_function.as_mut().unwrap();
    function.add_block(BasicBlock::new(bb(8)));
    function.add_block(BasicBlock::new(bb(9)));

    let body = function.get_block_mut(bb(2)).unwrap();
    body.instructions.clear();
    body.instruction_spans.clear();
    body.set_terminator(MirInstruction::Jump {
        target: bb(7),
        edge_args: None,
    });
    function
        .get_block_mut(bb(7))
        .unwrap()
        .add_predecessor(bb(2));
    function
        .get_block_mut(bb(7))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: bb(8),
            edge_args: None,
        });
    function
        .get_block_mut(bb(8))
        .unwrap()
        .add_predecessor(bb(7));
    let step = function.get_block_mut(bb(8)).unwrap();
    step.add_instruction(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(1),
    });
    step.add_instruction(MirInstruction::Const {
        dst: ValueId::new(13),
        value: ConstValue::Integer(1),
    });
    step.set_terminator(MirInstruction::Jump {
        target: bb(1),
        edge_args: None,
    });
    function
        .get_block_mut(bb(1))
        .unwrap()
        .predecessors
        .remove(&bb(2));
    function
        .get_block_mut(bb(1))
        .unwrap()
        .add_predecessor(bb(8));
    function
        .get_block_mut(bb(9))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    builder
}
