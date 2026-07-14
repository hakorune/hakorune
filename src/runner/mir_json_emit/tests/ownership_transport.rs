use super::super::emit_mir_json_string_for_harness_bin;
use crate::mir::{
    storage_class::StorageClass, BasicBlockId, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};

#[test]
fn ownership_transport_round_trips_with_exact_boxref_witness() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let src = ValueId::new(1);
    let dst = ValueId::new(2);
    function
        .metadata
        .value_types
        .insert(src, MirType::Box("WidgetBox".to_string()));
    function
        .metadata
        .value_types
        .insert(dst, MirType::Box("WidgetBox".to_string()));
    function
        .metadata
        .value_storage_classes
        .insert(src, StorageClass::BoxRef);
    function
        .metadata
        .value_storage_classes
        .insert(dst, StorageClass::BoxRef);
    let entry = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    entry
        .instructions
        .push(MirInstruction::CopyOwned { dst, src });
    entry
        .instructions
        .push(MirInstruction::DestroyOwned { value: dst });

    let mut module = MirModule::new("ownership_transport".to_string());
    module.add_function(function);
    let json = emit_mir_json_string_for_harness_bin(&module).expect("emit passive ownership MIR");
    let reparsed = crate::runner::mir_json_v0::parse_mir_v0_to_module(&json)
        .expect("reparse exact ownership witness");
    let main = reparsed.get_function("main").expect("main");
    let instructions = &main.get_block(BasicBlockId::new(0)).unwrap().instructions;
    assert!(matches!(
        instructions.as_slice(),
        [
            MirInstruction::CopyOwned { dst: actual_dst, src: actual_src },
            MirInstruction::DestroyOwned { value }
        ] if *actual_dst == dst && *actual_src == src && *value == dst
    ));
}
