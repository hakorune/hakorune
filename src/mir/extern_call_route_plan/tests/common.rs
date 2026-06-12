use super::*;
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType, ValueId};

pub(crate) fn make_function_with_call(
    callee: &str,
    args: Vec<ValueId>,
    dst: Option<ValueId>,
) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("STAGE1_SOURCE_TEXT".to_string()),
    });
    block.instructions.push(MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Extern(callee.to_string())),
        args,
        effects: EffectMask::PURE,
    });
    function.blocks.insert(BasicBlockId::new(0), block);
    function
}
