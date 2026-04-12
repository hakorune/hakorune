use crate::mir::join_ir::lowering::try_lower_if_to_joinir;
use crate::mir::join_ir::JoinInst;
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};
use crate::tests::helpers::joinir_env;
use std::collections::BTreeMap;
use std::env;

fn ensure_ring0_initialized() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
}

pub(super) fn strict_if_env_guard() -> impl Drop {
    ensure_ring0_initialized();
    env::set_var("NYASH_JOINIR_CORE", "1");
    env::set_var("NYASH_JOINIR_STRICT", "1");
    struct Guard;
    impl Drop for Guard {
        fn drop(&mut self) {
            let _ = env::remove_var("NYASH_JOINIR_CORE");
            let _ = env::remove_var("NYASH_JOINIR_STRICT");
        }
    }
    Guard
}

/// Helper to create a simple if/else function matching the "simple" pattern
pub(super) fn create_simple_pattern_mir() -> MirFunction {
    let mut blocks = BTreeMap::new();

    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId(0),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    blocks.insert(BasicBlockId::new(0), entry);

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(1)),
    });
    blocks.insert(BasicBlockId::new(1), then_block);

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(2)),
    });
    blocks.insert(BasicBlockId::new(2), else_block);

    use crate::mir::function::FunctionMetadata;
    use crate::mir::{EffectMask, MirType};

    MirFunction {
        signature: crate::mir::FunctionSignature {
            name: "IfSelectTest.test/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry_block: BasicBlockId::new(0),
        blocks: blocks.into_iter().collect(),
        locals: vec![],
        params: vec![ValueId(0)],
        next_value_id: 3,
        metadata: FunctionMetadata::default(),
    }
}

/// Helper to create a local pattern function
pub(super) fn create_local_pattern_mir() -> MirFunction {
    let mut blocks = BTreeMap::new();

    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId(0),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    blocks.insert(BasicBlockId::new(0), entry);

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(3),
        src: ValueId(10),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    blocks.insert(BasicBlockId::new(1), then_block);

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Copy {
        dst: ValueId(3),
        src: ValueId(20),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });
    blocks.insert(BasicBlockId::new(2), else_block);

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    blocks.insert(BasicBlockId::new(3), merge_block);

    use crate::mir::function::FunctionMetadata;
    use crate::mir::{EffectMask, MirType};

    MirFunction {
        signature: crate::mir::FunctionSignature {
            name: "IfSelectTest.main/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry_block: BasicBlockId::new(0),
        blocks: blocks.into_iter().collect(),
        locals: vec![],
        params: vec![],
        next_value_id: 21,
        metadata: FunctionMetadata::default(),
    }
}

/// Helper to create a JoinFunction with a valid Select instruction
pub(super) fn create_select_joinir() -> crate::mir::join_ir::JoinFunction {
    use crate::mir::join_ir::{ConstValue, JoinFuncId, JoinFunction, JoinInst, MirLikeInst};

    let func_id = JoinFuncId::new(0);
    let mut join_func = JoinFunction::new(
        func_id,
        "IfSelectTest.test/1".to_string(),
        vec![ValueId(0)],
    );

    join_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(1),
        value: ConstValue::Integer(10),
    }));
    join_func.body.push(JoinInst::Compute(MirLikeInst::Const {
        dst: ValueId(2),
        value: ConstValue::Integer(20),
    }));

    join_func.body.push(JoinInst::Select {
        dst: ValueId(3),
        cond: ValueId(0),
        then_val: ValueId(1),
        else_val: ValueId(2),
        type_hint: None,
    });

    join_func.body.push(JoinInst::Ret {
        value: Some(ValueId(3)),
    });

    join_func
}

/// Helper to create a JoinFunction with multiple Select instructions (invalid)
pub(super) fn create_double_select_joinir() -> crate::mir::join_ir::JoinFunction {
    use crate::mir::join_ir::{JoinFuncId, JoinFunction, JoinInst};

    let func_id = JoinFuncId::new(0);
    let mut join_func =
        JoinFunction::new(func_id, "IfSelectTest.test/1".to_string(), vec![ValueId(0)]);

    join_func.body.push(JoinInst::Select {
        dst: ValueId(1),
        cond: ValueId(0),
        then_val: ValueId(10),
        else_val: ValueId(20),
        type_hint: None,
    });

    join_func.body.push(JoinInst::Select {
        dst: ValueId(2),
        cond: ValueId(0),
        then_val: ValueId(30),
        else_val: ValueId(40),
        type_hint: None,
    });

    join_func.body.push(JoinInst::Ret {
        value: Some(ValueId(1)),
    });

    join_func
}

/// Helper to create a 2-variable IfMerge pattern MIR
pub(super) fn create_if_merge_simple_pattern_mir() -> MirFunction {
    let mut blocks = BTreeMap::new();

    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId(0),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    blocks.insert(BasicBlockId::new(0), entry);

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(1),
    });
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: crate::mir::ConstValue::Integer(2),
    });
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(10)),
    });
    blocks.insert(BasicBlockId::new(1), then_block);

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(3),
    });
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: crate::mir::ConstValue::Integer(4),
    });
    else_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(20)),
    });
    blocks.insert(BasicBlockId::new(2), else_block);

    use crate::mir::function::FunctionMetadata;
    use crate::mir::{EffectMask, MirType};

    MirFunction {
        signature: crate::mir::FunctionSignature {
            name: "IfMergeTest.simple_true/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry_block: BasicBlockId::new(0),
        blocks: blocks.into_iter().collect(),
        locals: vec![],
        params: vec![ValueId(0)],
        next_value_id: 23,
        metadata: FunctionMetadata::default(),
    }
}

/// Helper to create a 3-variable IfMerge pattern MIR
pub(super) fn create_if_merge_multiple_pattern_mir() -> MirFunction {
    let mut blocks = BTreeMap::new();

    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId(0),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    blocks.insert(BasicBlockId::new(0), entry);

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(10),
    });
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: crate::mir::ConstValue::Integer(20),
    });
    then_block.instructions.push(MirInstruction::Const {
        dst: ValueId(5),
        value: crate::mir::ConstValue::Integer(30),
    });
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(10)),
    });
    blocks.insert(BasicBlockId::new(1), then_block);

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(3),
        value: crate::mir::ConstValue::Integer(40),
    });
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(4),
        value: crate::mir::ConstValue::Integer(50),
    });
    else_block.instructions.push(MirInstruction::Const {
        dst: ValueId(5),
        value: crate::mir::ConstValue::Integer(60),
    });
    else_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(20)),
    });
    blocks.insert(BasicBlockId::new(2), else_block);

    use crate::mir::function::FunctionMetadata;
    use crate::mir::{EffectMask, MirType};

    MirFunction {
        signature: crate::mir::FunctionSignature {
            name: "IfMergeTest.multiple_true/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry_block: BasicBlockId::new(0),
        blocks: blocks.into_iter().collect(),
        locals: vec![],
        params: vec![ValueId(0)],
        next_value_id: 24,
        metadata: FunctionMetadata::default(),
    }
}

/// Phase 63-2: Helper to create a simple pattern MIR with Const instructions
pub(super) fn create_simple_pattern_mir_with_const() -> MirFunction {
    let mut blocks = BTreeMap::new();

    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::Integer(10),
    });
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(20),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId(0),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    blocks.insert(BasicBlockId::new(0), entry);

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(1)),
    });
    blocks.insert(BasicBlockId::new(1), then_block);

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(2)),
    });
    blocks.insert(BasicBlockId::new(2), else_block);

    use crate::mir::function::FunctionMetadata;
    use crate::mir::{EffectMask, MirType};

    MirFunction {
        signature: crate::mir::FunctionSignature {
            name: "IfSelectTest.test/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        entry_block: BasicBlockId::new(0),
        blocks: blocks.into_iter().collect(),
        locals: vec![],
        params: vec![ValueId(0)],
        next_value_id: 3,
        metadata: FunctionMetadata::default(),
    }
}
