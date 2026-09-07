use super::*;
use crate::mir::instruction::InvokeOperation;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction};

#[test]
fn site_allocator_counts_invoke_terminators_and_standalone_writes_together() {
    let mut builder = MirBuilder::new();
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array_site_allocator".into(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(
        crate::mir::array_element_write::instruction(
            ArrayWriteSiteId::new(3),
            None,
            ArrayElementWriteKind::LiteralAppend,
            ArrayWriteProducerKind::Literal,
            ValueId::new(0),
            None,
            ValueId::new(1),
        )
        .unwrap(),
    );
    block.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::ArrayElementWrite {
            site_id: ArrayWriteSiteId::new(7),
            kind: ArrayElementWriteKind::LiteralAppend,
            producer: ArrayWriteProducerKind::Literal,
            receiver: ValueId::new(0),
            index: None,
            value: ValueId::new(1),
        },
        fault_frame: ValueId::new(2),
        normal_landing: BasicBlockId::new(1),
        fault_landing: BasicBlockId::new(2),
    });
    builder.function_state.current_function = Some(function);
    assert_eq!(builder.next_array_write_site_id(), ArrayWriteSiteId::new(8));
    builder
        .function_state
        .current_function
        .as_mut()
        .unwrap()
        .get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .terminator = None;
    assert_eq!(builder.next_array_write_site_id(), ArrayWriteSiteId::new(4));
}
