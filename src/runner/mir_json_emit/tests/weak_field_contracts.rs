use super::super::emit_mir_json_string_for_harness_bin;
use crate::mir::{
    BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, UserBoxFieldDecl, ValueId,
};

#[test]
fn export_contains_refreshed_weak_field_spec_carrier_and_operation() {
    let mut module = MirModule::new("weak-json".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Node".to_string(), vec!["parent".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Node".to_string(),
        vec![UserBoxFieldDecl {
            name: "parent".to_string(),
            declared_type_name: Some("Node".to_string()),
            is_weak: true,
        }],
    );

    let entry = BasicBlockId::new(0);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.write/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::WRITE,
        },
        entry,
    );
    let mut block = BasicBlock::new(entry);
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(0),
            box_type: "Node".to_string(),
            args: Vec::new(),
        },
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: crate::mir::ConstValue::Void,
        },
        MirInstruction::FieldSet {
            base: ValueId::new(0),
            field: "parent".to_string(),
            value: ValueId::new(1),
            declared_type: None,
        },
        MirInstruction::Return { value: None },
    ]);
    function.add_block(block);
    module.add_function(function);

    let output = emit_mir_json_string_for_harness_bin(&module).expect("weak field MIR JSON");
    let root: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        root["weak_field_contract_specs"].as_array().unwrap().len(),
        1
    );
    let function = &root["functions"][0];
    assert_eq!(
        function["metadata"]["weak_field_write_contracts"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let operations = function["blocks"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|block| block["instructions"].as_array().unwrap())
        .filter_map(|instruction| instruction["op"].as_str())
        .collect::<Vec<_>>();
    assert!(operations.contains(&"weak_field_write"));
    assert!(!output.contains("BoxBase"));
    assert!(!output.contains("runtime_pointer"));
}
