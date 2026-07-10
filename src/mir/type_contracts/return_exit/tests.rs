use super::*;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

fn function_with_return(declared: Option<&str>) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.value/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_return_type_name = declared.map(str::to_string);
    function
}

#[test]
fn refresh_builds_only_explicit_exact_numeric_return_contracts() {
    let mut exact = function_with_return(Some("u8"));
    refresh_function_return_exit_contract(&mut exact);
    let contract = exact.metadata.return_exit_contract.as_ref().unwrap();
    assert_eq!(contract.declared_type_name, "u8");
    assert!(contract.runtime_check_required);
    assert!(!contract.proof_elision_allowed);
    assert!(validate_return_exit_contract(&exact).is_ok());

    let mut dynamic = function_with_return(Some("String"));
    refresh_function_return_exit_contract(&mut dynamic);
    assert!(dynamic.metadata.return_exit_contract.is_none());
}

#[test]
fn validation_rejects_missing_extra_and_drifted_carriers() {
    let mut function = function_with_return(Some("u8"));
    refresh_function_return_exit_contract(&mut function);
    let expected = function.metadata.return_exit_contract.clone().unwrap();

    function.metadata.return_exit_contract = None;
    assert!(validate_return_exit_contract(&function)
        .unwrap_err()
        .contains(RETURN_CONTRACT_CARRIER_MISSING_TAG));

    function.metadata.return_exit_contract = Some(expected.clone());
    function.metadata.declared_return_type_name = None;
    assert!(validate_return_exit_contract(&function)
        .unwrap_err()
        .contains(RETURN_CONTRACT_CARRIER_DRIFT_TAG));

    function.metadata.declared_return_type_name = Some("u8".to_string());
    let mut drifted = expected;
    drifted.declared_type_name = "i8".to_string();
    function.metadata.return_exit_contract = Some(drifted);
    assert!(validate_return_exit_contract(&function)
        .unwrap_err()
        .contains(RETURN_CONTRACT_CARRIER_DRIFT_TAG));
}
