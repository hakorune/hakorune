use super::super::emit_mir_json_string_for_harness_bin;
use crate::mir::function::{StaticElementType, StaticTableContractSpec, StaticTableId};
use crate::mir::MirModule;

#[test]
fn export_contains_source_plan_and_refreshed_static_table_carrier() {
    let mut module = MirModule::new("static-json".to_string());
    let spec = StaticTableContractSpec {
        table_id: StaticTableId {
            module_name: module.name.clone(),
            declaration_name: "DATA".to_string(),
        },
        diagnostic_name: "DATA".to_string(),
        element: StaticElementType::U16,
        values: vec![3, 5, 8],
    };
    module.metadata.static_data_plans =
        crate::mir::static_data_plan::static_data_plans_from_specs(std::slice::from_ref(&spec));
    module.metadata.static_table_contract_specs.push(spec);

    let output = emit_mir_json_string_for_harness_bin(&module).unwrap();
    let root: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(root["static_table_contract_specs"][0]["element"], "u16");
    assert_eq!(root["static_data_plans"][0]["align"], 2);
    assert_eq!(
        root["verified_static_table_contracts"][0]["proof"],
        "source_spec_and_plan_structurally_match"
    );
}
