use super::*;
use crate::mir::function::{RecordValueBoundaryKind, UserBoxFieldDecl};
use crate::mir::{
    BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
};

fn record_decl() -> RecordDecl {
    RecordDecl {
        name: "Point".to_string(),
        type_parameters: Vec::new(),
        fields: vec![UserBoxFieldDecl {
            name: "x".to_string(),
            declared_type_name: Some("i64".to_string()),
            is_weak: false,
        }],
        default_field_names: Vec::new(),
    }
}

fn module_with_record_contract(include_check: bool) -> MirModule {
    let decl = record_decl();
    let fingerprint = record_schema_fingerprint(&decl);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.main/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let value = function.next_value_id();
    let dst = function.next_value_id();
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: value,
        value: ConstValue::Integer(7),
    });
    if include_check {
        block.add_instruction(MirInstruction::RecordFieldContractCheck {
            contract_id: "record-value:1".to_string(),
            schema_fingerprint: fingerprint.clone(),
            field_index: 0,
            value,
        });
    }
    block.add_instruction(MirInstruction::RecordValuePublish {
        dst,
        contract_id: "record-value:1".to_string(),
        boundary: RecordValueBoundaryKind::Construct,
        diagnostic_record_name: "Point".to_string(),
        schema_fingerprint: fingerprint,
        base: None,
        fields: vec![value],
    });
    block.add_instruction(MirInstruction::Return { value: Some(value) });

    let mut module = MirModule::new("record-contract".to_string());
    module
        .metadata
        .record_decls
        .insert("Point".to_string(), decl);
    module.add_function(function);
    module
}

#[test]
fn refresh_rebuilds_record_carrier_from_semantic_operations() {
    let mut module = module_with_record_contract(true);
    let bundle = crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
        &mut module,
        crate::mir::ContractRefreshBoundary::ToolDirectVerify,
    )
    .expect("record refresh");
    assert_eq!(bundle.carriers().record_values, 1);
    let contract = &bundle
        .module()
        .functions
        .get("Main.main/0")
        .unwrap()
        .metadata
        .record_value_contracts[0];
    assert_eq!(contract.fields.len(), 1);
    assert_eq!(contract.fields[0].diagnostic_field_name, "x");
}

#[test]
fn refresh_rejects_publish_without_active_field_check() {
    let mut module = module_with_record_contract(false);
    let error = match crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
        &mut module,
        crate::mir::ContractRefreshBoundary::ToolDirectVerify,
    ) {
        Ok(_) => panic!("missing field check must fail"),
        Err(error) => error,
    };
    assert!(error.contains(RECORD_CONTRACT_STALE_CARRIER_TAG), "{error}");
}

#[test]
fn refresh_rejects_source_schema_drift() {
    let mut module = module_with_record_contract(true);
    module
        .metadata
        .record_decls
        .get_mut("Point")
        .unwrap()
        .fields[0]
        .declared_type_name = Some("u8".to_string());
    let error = match crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
        &mut module,
        crate::mir::ContractRefreshBoundary::ToolDirectVerify,
    ) {
        Ok(_) => panic!("schema drift must fail"),
        Err(error) => error,
    };
    assert!(error.contains(RECORD_CONTRACT_SOURCE_DRIFT_TAG), "{error}");
}

#[test]
fn representation_facts_do_not_synthesize_record_contracts() {
    let decl = record_decl();
    let mut module = MirModule::new("record-representation-non-authority".to_string());
    module
        .metadata
        .record_decls
        .insert("Point".to_string(), decl);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.main/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.value_types.insert(
        crate::mir::ValueId::new(0),
        MirType::Box("Point".to_string()),
    );
    module.add_function(function);

    let bundle = crate::mir::semantic_refresh::refresh_and_validate_for_boundary(
        &mut module,
        crate::mir::ContractRefreshBoundary::ToolDirectVerify,
    )
    .expect("representation facts are ignored");
    assert_eq!(bundle.carriers().record_values, 0);
}
