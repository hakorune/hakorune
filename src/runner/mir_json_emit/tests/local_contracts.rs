use super::super::build_mir_json_root;
use crate::mir::function::{LocalContractWriteKind, LocalSlotContract};
use crate::mir::{
    BasicBlockId, BindingId, EffectMask, FunctionSignature, LocalSlotId, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};

#[test]
fn mir_json_exports_local_contract_carrier_and_write() {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.local/0".to_string(),
            params: Vec::new(),
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let slot = LocalSlotId::from(BindingId::new(4));
    function
        .metadata
        .local_slot_contracts
        .push(LocalSlotContract {
            contract_id: "local-slot:4".to_string(),
            local_slot_id: slot,
            diagnostic_source_name: "count".to_string(),
            declared_type_name: "u8".to_string(),
            runtime_check_required: true,
            proof_elision_allowed: false,
            backend_capability_required: "local_slot_exact_numeric".to_string(),
        });
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::LocalContractWrite {
            dst: ValueId::new(2),
            src: ValueId::new(1),
            local_slot_id: slot,
            write_kind: LocalContractWriteKind::Init,
        });
    let mut module = MirModule::new("local-contract-json".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).unwrap();
    let function = &root["functions"][0];
    assert_eq!(
        function["metadata"]["local_slot_contracts"][0]["local_slot_id"],
        4
    );
    assert_eq!(
        function["metadata"]["local_slot_contracts"][0]["declared_type_name"],
        "u8"
    );
    assert_eq!(
        function["blocks"][0]["instructions"][0]["op"],
        "local_contract_write"
    );
    assert_eq!(
        function["blocks"][0]["instructions"][0]["write_kind"],
        "init"
    );
    assert!(function["metadata"]["local_identity_evidence"].is_array());
}
