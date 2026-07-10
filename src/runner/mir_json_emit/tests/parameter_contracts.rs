use super::super::build_mir_json_root;
use crate::mir::function::MirParamDecl;
use crate::mir::type_contracts::parameter_entry::refresh_function_parameter_entry_contracts;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

#[test]
fn mir_json_exports_typed_parameter_entry_contracts() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.take/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_param_decls = vec![MirParamDecl {
        name: "count".to_string(),
        declared_type_name: Some("u8".to_string()),
        implicit_receiver: false,
    }];
    refresh_function_parameter_entry_contracts(&mut function);

    let mut module = MirModule::new("parameter-contract-json".to_string());
    module.add_function(function);
    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];

    assert_eq!(metadata["declared_param_decls"][0]["name"], "count");
    assert_eq!(
        metadata["parameter_entry_contracts"][0]["contract_kind"],
        "exact_numeric"
    );
    assert_eq!(
        metadata["parameter_entry_contracts"][0]["formal_parameter_index"],
        0
    );
    assert_eq!(
        metadata["parameter_entry_contracts"][0]["parameter_value_id"],
        0
    );
    assert_eq!(
        metadata["parameter_entry_contracts"][0]["runtime_check_required"],
        true
    );
    assert_eq!(
        metadata["parameter_entry_contracts"][0]["proof_elision_allowed"],
        false
    );
}

#[test]
fn mir_json_rejects_missing_parameter_contract_carrier() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.take/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_param_decls = vec![MirParamDecl {
        name: "count".to_string(),
        declared_type_name: Some("u8".to_string()),
        implicit_receiver: false,
    }];
    let mut module = MirModule::new("missing-parameter-contract-json".to_string());
    module.add_function(function);

    let error = build_mir_json_root(&module).unwrap_err();
    assert!(
        error.contains("[type/parameter_contract_carrier_missing]"),
        "{error}"
    );
}
