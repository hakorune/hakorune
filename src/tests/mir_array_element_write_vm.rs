use crate::backend::VM;
use crate::mir::{
    ArrayElementWriteKind, ArrayWriteSiteId, BasicBlockId, ConstValue, EffectMask,
    FunctionSignature, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    let _ = std::panic::catch_unwind(|| init_global_ring0(default_ring0()));
}

#[test]
fn vm_array_element_write_delegates_to_array_surface() {
    ensure_ring0_initialized();
    let mut module = MirModule::new("array-write-vm".to_string());
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.main/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Box("ArrayBox".to_string()),
            effects: EffectMask::MUT,
        },
        BasicBlockId::new(0),
    );
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(0),
        box_type: "ArrayBox".to_string(),
        args: Vec::new(),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(42),
    });
    block.add_instruction(
        crate::mir::array_element_write::instruction(
            ArrayWriteSiteId::new(0),
            None,
            ArrayElementWriteKind::Push,
            crate::mir::ArrayWriteProducerKind::MethodCall,
            ValueId::new(0),
            None,
            ValueId::new(1),
        )
        .unwrap(),
    );
    block.add_instruction(MirInstruction::Return {
        value: Some(ValueId::new(0)),
    });
    module.add_function(function);

    let mut vm = VM::new();
    let result = vm.execute_module(&module).unwrap();
    assert_eq!(result.to_string_box().value, "[42]");
}
