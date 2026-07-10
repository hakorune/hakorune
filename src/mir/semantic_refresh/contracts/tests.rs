use super::*;
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
