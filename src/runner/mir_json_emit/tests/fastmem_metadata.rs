use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::function::{FastMemTableLengthFact, FastMemTableLengthPolicyKind};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{MirModule, ValueId};

#[test]
fn build_mir_json_root_emits_fastmem_table_length_facts() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("Main.fastmem/0", false);
    function
        .metadata
        .fastmem_table_length_facts
        .push(FastMemTableLengthFact {
            fact_id: 0,
            region: FastMemRegionId::new(0),
            table_id: "page_table".to_string(),
            table_value: ValueId::new(1),
            length_value: ValueId::new(50),
            resolved_length: Some(64),
            policy: FastMemTableLengthPolicyKind::ExplicitConstLen,
        });
    module
        .functions
        .insert("Main.fastmem/0".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["fastmem_table_length_facts"]
        .as_array()
        .expect("metadata.fastmem_table_length_facts array");

    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0]["fact_id"], 0);
    assert_eq!(facts[0]["region"], 0);
    assert_eq!(facts[0]["table_id"], "page_table");
    assert_eq!(facts[0]["table_value"], 1);
    assert_eq!(facts[0]["length_value"], 50);
    assert_eq!(facts[0]["resolved_length"], 64);
    assert_eq!(facts[0]["policy"], "explicit_const_len");
}
