use super::*;
use crate::mir::function::{StaticElementType, StaticTableContractSpec, StaticTableId};
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
};

fn module_with_table() -> MirModule {
    let mut module = MirModule::new("table-module".to_string());
    let spec = StaticTableContractSpec {
        table_id: StaticTableId {
            module_name: module.name.clone(),
            declaration_name: "DATA".to_string(),
        },
        diagnostic_name: "DATA".to_string(),
        element: StaticElementType::U16,
        values: vec![1, 65535],
    };
    module.metadata.static_data_plans =
        crate::mir::static_data_plan::static_data_plans_from_specs(std::slice::from_ref(&spec));
    module.metadata.static_table_contract_specs.push(spec);

    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.main/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::READ,
        },
        BasicBlockId::new(0),
    );
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::StaticDataLoad {
            dst: ValueId::new(1),
            source_name: "DATA".to_string(),
            symbol: ".hako.static.DATA".to_string(),
            element: "u16".to_string(),
            len: 2,
            align: 2,
            index: ValueId::new(0),
        });
    module.add_function(function);
    module
}

#[test]
fn refresh_rebuilds_static_table_carrier() {
    let mut module = module_with_table();
    let bundle = crate::mir::refresh_and_validate_for_boundary(
        &mut module,
        crate::mir::ContractRefreshBoundary::ToolDirectVerify,
    )
    .unwrap();
    assert_eq!(bundle.carriers().static_tables, 1);
    assert_eq!(
        bundle
            .module()
            .metadata
            .verified_static_table_contracts
            .len(),
        1
    );
}

#[test]
fn plan_without_source_spec_is_rejected() {
    let mut module = module_with_table();
    module.metadata.static_table_contract_specs.clear();
    let error = refresh_module_static_table_contracts(&mut module).unwrap_err();
    assert!(error.contains(SPEC_MISSING_TAG));
}

#[test]
fn source_plan_drift_is_rejected() {
    let mut module = module_with_table();
    module.metadata.static_data_plans[0].values[0] = 2;
    let error = refresh_module_static_table_contracts(&mut module).unwrap_err();
    assert!(error.contains(DRIFT_TAG));
}

#[test]
fn duplicate_source_identity_is_rejected() {
    let mut module = module_with_table();
    module
        .metadata
        .static_table_contract_specs
        .push(module.metadata.static_table_contract_specs[0].clone());
    let error = refresh_module_static_table_contracts(&mut module).unwrap_err();
    assert!(error.contains(DUPLICATE_ID_TAG));
}

#[test]
fn load_metadata_drift_is_rejected() {
    let mut module = module_with_table();
    let function = module.functions.get_mut("Main.main/0").unwrap();
    let instruction = &mut function
        .get_block_mut(function.entry_block)
        .unwrap()
        .instructions[0];
    let MirInstruction::StaticDataLoad { len, .. } = instruction else {
        panic!("expected StaticDataLoad");
    };
    *len = 3;
    let error = refresh_module_static_table_contracts(&mut module).unwrap_err();
    assert!(error.contains(DRIFT_TAG));
}
