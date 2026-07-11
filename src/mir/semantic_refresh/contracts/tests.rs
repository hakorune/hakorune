use super::*;
use crate::mir::function::MirParamDecl;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

fn function_with_exact_return() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.answer/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_return_type_name = Some("i64".to_string());
    function
}

#[test]
fn facade_rebuilds_and_validates_declared_return_carrier() {
    let mut module = MirModule::new("refresh-contracts".to_string());
    module.add_function(function_with_exact_return());

    let bundle =
        refresh_and_validate_for_boundary(&mut module, ContractRefreshBoundary::ToolDirectVerify)
            .expect("refresh");

    assert_eq!(bundle.boundary(), ContractRefreshBoundary::ToolDirectVerify);
    assert_eq!(bundle.carriers().return_exits, 1);
    assert_eq!(bundle.carriers().total(), 1);
    assert!(bundle
        .module()
        .get_function("Main.answer/0")
        .unwrap()
        .metadata
        .return_exit_contract
        .is_some());
}

#[test]
fn facade_accepts_module_without_active_contracts() {
    let mut module = MirModule::new("plain".to_string());
    let bundle =
        refresh_and_validate_for_boundary(&mut module, ContractRefreshBoundary::BackendPreflight)
            .expect("refresh");

    assert_eq!(bundle.carriers(), ContractCarrierSummary::default());
}

#[test]
fn facade_rebuilds_typed_array_parameter_from_source_metadata() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.take/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_param_decls = vec![MirParamDecl {
        name: "bytes".to_string(),
        declared_type_name: Some("Array<u8>".to_string()),
        implicit_receiver: false,
    }];
    let mut module = MirModule::new("typed-array-refresh".to_string());
    module.add_function(function);

    let bundle =
        refresh_and_validate_for_boundary(&mut module, ContractRefreshBoundary::ToolDirectVerify)
            .expect("refresh");

    assert_eq!(bundle.carriers().typed_arrays, 1);
    let function = bundle.module().get_function("Main.take/1").unwrap();
    let contract = &function.metadata.typed_array_element_contracts[0];
    assert_eq!(contract.element_spec.element.source_name(), "u8");
    assert!(contract.runtime_check_required);
    assert!(!contract.proof_elision_allowed);
}
