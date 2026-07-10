use super::*;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType, ValueId};

fn function_with_declarations(declarations: Vec<MirParamDecl>) -> MirFunction {
    let signature = FunctionSignature {
        name: "Main.f/2".to_string(),
        params: vec![MirType::Unknown; declarations.len()],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function.metadata.declared_param_decls = declarations;
    function
}

fn explicit(name: &str, declared_type_name: Option<&str>) -> MirParamDecl {
    MirParamDecl {
        name: name.to_string(),
        declared_type_name: declared_type_name.map(str::to_string),
        implicit_receiver: false,
    }
}

#[test]
fn refresh_builds_exact_numeric_rows_only() {
    let mut function = function_with_declarations(vec![
        explicit("count", Some("u8")),
        explicit("label", Some("String")),
    ]);
    refresh_function_parameter_entry_contracts(&mut function);

    assert_eq!(function.metadata.parameter_entry_contracts.len(), 1);
    let contract = &function.metadata.parameter_entry_contracts[0];
    assert_eq!(contract.formal_parameter_index, 0);
    assert_eq!(contract.source_parameter_index, 0);
    assert_eq!(contract.parameter_value_id, ValueId::new(0));
    assert_eq!(contract.declared_type_name, "u8");
    assert!(validate_parameter_entry_contracts(&function).is_ok());
}

#[test]
fn function_semantic_refresh_rebuilds_parameter_contract_rows() {
    let mut function = function_with_declarations(vec![explicit("count", Some("u8"))]);
    crate::mir::semantic_refresh::refresh_function_semantic_metadata(
        &mut function,
        &crate::mir::function::ModuleMetadata::default(),
    );
    assert_eq!(function.metadata.parameter_entry_contracts.len(), 1);

    function.metadata.declared_param_decls[0].declared_type_name = Some("String".to_string());
    crate::mir::semantic_refresh::refresh_function_semantic_metadata(
        &mut function,
        &crate::mir::function::ModuleMetadata::default(),
    );
    assert!(function.metadata.parameter_entry_contracts.is_empty());
}

#[test]
fn implicit_receiver_is_excluded_without_name_inference() {
    let mut function = function_with_declarations(vec![
        MirParamDecl {
            name: "me".to_string(),
            declared_type_name: None,
            implicit_receiver: true,
        },
        explicit("count", Some("i64")),
    ]);
    refresh_function_parameter_entry_contracts(&mut function);

    let contract = &function.metadata.parameter_entry_contracts[0];
    assert_eq!(contract.formal_parameter_index, 1);
    assert_eq!(contract.source_parameter_index, 0);
}

#[test]
fn validation_rejects_missing_duplicate_and_drifted_rows() {
    let mut function = function_with_declarations(vec![explicit("count", Some("u8"))]);
    refresh_function_parameter_entry_contracts(&mut function);
    let expected = function.metadata.parameter_entry_contracts[0].clone();

    function.metadata.parameter_entry_contracts.clear();
    assert!(validate_parameter_entry_contracts(&function)
        .unwrap_err()
        .contains(PARAMETER_CONTRACT_CARRIER_MISSING_TAG));

    function.metadata.parameter_entry_contracts = vec![expected.clone(), expected.clone()];
    assert!(validate_parameter_entry_contracts(&function)
        .unwrap_err()
        .contains(PARAMETER_CONTRACT_DUPLICATE_INDEX_TAG));

    function.metadata.parameter_entry_contracts = vec![ParameterEntryContract {
        source_parameter_name: "other".to_string(),
        ..expected
    }];
    assert!(validate_parameter_entry_contracts(&function)
        .unwrap_err()
        .contains(PARAMETER_CONTRACT_ROW_DRIFT_TAG));
}
