use super::super::build_mir_json_root;
use crate::mir::type_contracts::return_exit::refresh_function_return_exit_contract;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType};

fn return_function(refresh: bool) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.value/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.metadata.declared_return_type_name = Some("u8".to_string());
    if refresh {
        refresh_function_return_exit_contract(&mut function);
    }
    function
}

#[test]
fn mir_json_exports_typed_return_exit_contract() {
    let mut module = MirModule::new("return-contract-json".to_string());
    module.add_function(return_function(true));
    let root = build_mir_json_root(&module).expect("mir json root");
    let metadata = &root["functions"][0]["metadata"];
    assert_eq!(metadata["declared_return_type_name"], "u8");
    assert_eq!(
        metadata["return_exit_contract"]["contract_kind"],
        "exact_numeric"
    );
    assert_eq!(
        metadata["return_exit_contract"]["void_policy"],
        "reject_void"
    );
    assert_eq!(
        metadata["return_exit_contract"]["runtime_check_required"],
        true
    );
    assert_eq!(
        metadata["return_exit_contract"]["proof_elision_allowed"],
        false
    );
    assert_eq!(
        metadata["return_exit_contract"]["owner"],
        "function_return_contract"
    );
}

#[test]
fn mir_json_rejects_missing_return_contract_carrier() {
    let mut module = MirModule::new("missing-return-contract-json".to_string());
    module.add_function(return_function(false));
    let error = build_mir_json_root(&module).unwrap_err();
    assert!(
        error.contains("[type/return_contract_carrier_missing]"),
        "{error}"
    );
}
