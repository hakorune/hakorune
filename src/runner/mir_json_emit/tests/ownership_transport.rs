use super::super::emit_mir_json_string_for_harness_bin;
use crate::mir::ownership_ssa::{
    verify_ownership_ssa_v1, FunctionResultOwnershipV1, MirOwnershipKindV1, OwnershipFunctionAbiV1,
    OwnershipFunctionOwnerV1,
};
use crate::mir::{
    storage_class::StorageClass, BasicBlockId, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};

#[test]
fn ownership_transport_round_trips_with_exact_boxref_witness() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Box("WidgetBox".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        BasicBlockId::new(0),
    );
    let src = ValueId::new(0);
    let dst = ValueId::new(1);
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
    entry.set_terminator(MirInstruction::Return { value: None });

    let owner = OwnershipFunctionOwnerV1::new(41);
    let abi = OwnershipFunctionAbiV1::new(
        owner,
        vec![MirOwnershipKindV1::Borrowed],
        FunctionResultOwnershipV1::None,
    );
    function.metadata.ownership_ssa_v1 =
        Some(verify_ownership_ssa_v1(&function, &abi).expect("seal ownership witness"));

    let mut module = MirModule::new("ownership_transport".to_string());
    module.add_function(function);
    let json = emit_mir_json_string_for_harness_bin(&module).expect("emit passive ownership MIR");
    let emitted: serde_json::Value = serde_json::from_str(&json).expect("parse emitted JSON");
    let witness = &emitted["functions"][0]["metadata"]["ownership_ssa_v1"];
    assert_eq!(witness["schema"], "VerifiedOwnershipSsaV1");
    assert_eq!(witness["producer"], "rust_ownership_ssa_verifier_v1");
    assert_eq!(witness["owner"], 41);
    assert_eq!(witness["backend"], "llvm_py");
    assert_eq!(witness["provider"], "nyash_kernel");
    assert_eq!(witness["value_kinds"]["0"], "borrowed");
    assert_eq!(witness["value_kinds"]["1"], "owned");
    assert_eq!(witness["operations"].as_array().unwrap().len(), 2);
    assert_eq!(witness["operations"][0]["op"], "copy_owned");
    assert_eq!(witness["operations"][1]["op"], "destroy_owned");

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
    assert!(matches!(
        main.get_block(BasicBlockId::new(0)).unwrap().terminator,
        Some(MirInstruction::Return { value: None })
    ));
}
