use super::*;
use crate::mir::function::LocalContractWriteKind;
use crate::mir::type_contracts::local_slot::{
    refresh_function_local_identity_evidence, register_local_slot_contract,
};
use crate::mir::{BindingId, LocalSlotId};

fn local_function(
    declared_type_name: &str,
    write_kind: LocalContractWriteKind,
) -> (MirFunction, LocalSlotId, ValueId, ValueId) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "local_fact".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let slot = LocalSlotId::from(BindingId::new(0));
    register_local_slot_contract(&mut function, slot, "key", declared_type_name).unwrap();
    let src = ValueId::new(1);
    let dst = ValueId::new(2);
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::LocalContractWrite {
            dst,
            src,
            local_slot_id: slot,
            write_kind,
        });
    refresh_function_local_identity_evidence(&mut function);
    (function, slot, src, dst)
}

fn refresh(function: MirFunction) -> MirFunction {
    let mut module = MirModule::new("local_contract_fact_test".to_string());
    let name = function.signature.name.clone();
    module.add_function(function);
    refresh_module_exact_numeric_value_facts(&mut module);
    module.functions.remove(&name).unwrap()
}

#[test]
fn mapstore_i64_key_from_i64_local_init() {
    let (function, slot, src, dst) = local_function("i64", LocalContractWriteKind::Init);
    let function = refresh(function);
    assert!(!function
        .metadata
        .exact_numeric_value_facts
        .contains_key(&src));
    assert_eq!(
        function.metadata.exact_numeric_value_facts.get(&dst),
        Some(&ExactNumericValueFact {
            declared_type_name: "i64".to_string(),
            source: ExactNumericValueFactSource::LocalContractWrite {
                contract_id: "local-slot:0".to_string(),
                local_slot_id: slot,
                write_kind: LocalContractWriteKind::Init,
                src,
                block: BasicBlockId::new(0),
                instruction_index: 0,
            },
        })
    );
}

#[test]
fn mapstore_i64_key_from_i64_local_reassign() {
    let (function, _, _, dst) = local_function("i64", LocalContractWriteKind::Reassign);
    assert!(refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn mapstore_i64_key_from_dynamic_src_after_checked_local_write() {
    let (function, _, src, dst) = local_function("i64", LocalContractWriteKind::Init);
    let function = refresh(function);
    assert!(!function
        .metadata
        .exact_numeric_value_facts
        .contains_key(&src));
    assert!(function
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn mapstore_i64_key_from_local_write_through_copy() {
    let (mut function, _, _, dst) = local_function("i64", LocalContractWriteKind::Init);
    let copied = ValueId::new(3);
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: copied,
            src: dst,
        });
    refresh_function_local_identity_evidence(&mut function);
    assert_eq!(
        refresh(function)
            .metadata
            .exact_numeric_value_facts
            .get(&copied)
            .unwrap()
            .source,
        ExactNumericValueFactSource::Copy { src: dst }
    );
}

#[test]
fn mapstore_i64_key_from_two_checked_writes_through_phi() {
    let (mut function, slot, _, left) = local_function("i64", LocalContractWriteKind::Init);
    let right_src = ValueId::new(3);
    let right = ValueId::new(4);
    let merged = ValueId::new(5);
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::LocalContractWrite {
        dst: right,
        src: right_src,
        local_slot_id: slot,
        write_kind: LocalContractWriteKind::Reassign,
    });
    block.add_instruction(MirInstruction::Phi {
        dst: merged,
        inputs: vec![(BasicBlockId::new(0), left), (BasicBlockId::new(0), right)],
        type_hint: None,
    });
    refresh_function_local_identity_evidence(&mut function);
    assert!(refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&merged));
}

#[test]
fn mapstore_i64_local_root_attaches_after_local_identity_refresh() {
    let (mut function, _, _, dst) = local_function("i64", LocalContractWriteKind::Init);
    function
        .metadata
        .local_identity_evidence
        .push(crate::mir::function::LocalIdentityEvidence {
            local_slot_id: LocalSlotId::from(BindingId::new(0)),
            merge_value_id: ValueId::new(99),
            incoming_values: vec![],
        });
    assert!(!refresh(function.clone())
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
    refresh_function_local_identity_evidence(&mut function);
    assert!(refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn unannotated_local_has_no_exact_fact() {
    let mut function = local_function("i64", LocalContractWriteKind::Init).0;
    function.metadata.local_slot_contracts.clear();
    assert!(refresh(function)
        .metadata
        .exact_numeric_value_facts
        .is_empty());
}

#[test]
fn local_contract_without_write_does_not_create_fact() {
    let (mut function, _, _, _) = local_function("i64", LocalContractWriteKind::Init);
    function
        .blocks
        .values_mut()
        .for_each(|block| block.instructions.clear());
    refresh_function_local_identity_evidence(&mut function);
    assert!(refresh(function)
        .metadata
        .exact_numeric_value_facts
        .is_empty());
}

#[test]
fn write_without_contract_rejects_claim() {
    let (mut function, _, _, dst) = local_function("i64", LocalContractWriteKind::Init);
    function.metadata.local_slot_contracts.clear();
    assert!(!refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn integer_mirtype_local_has_no_exact_fact() {
    let mut function = local_function("i64", LocalContractWriteKind::Init).0;
    function.metadata.local_slot_contracts.clear();
    function
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Integer);
    assert!(refresh(function)
        .metadata
        .exact_numeric_value_facts
        .is_empty());
}

#[test]
fn u64_local_is_not_i64_key_fact() {
    let (function, _, _, dst) = local_function("u64", LocalContractWriteKind::Init);
    assert!(!refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn raw_store_does_not_create_local_exact_fact() {
    let (mut function, _, src, _) = local_function("i64", LocalContractWriteKind::Init);
    function
        .blocks
        .values_mut()
        .for_each(|block| block.instructions.clear());
    function
        .get_block_mut(function.entry_block)
        .unwrap()
        .add_instruction(MirInstruction::Store {
            value: src,
            ptr: ValueId::new(8),
        });
    refresh_function_local_identity_evidence(&mut function);
    assert!(refresh(function)
        .metadata
        .exact_numeric_value_facts
        .is_empty());
}

#[test]
fn stale_contract_id_rejects_claim() {
    let (mut function, _, _, dst) = local_function("i64", LocalContractWriteKind::Init);
    function.metadata.local_slot_contracts[0].contract_id = "stale".to_string();
    assert!(!refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn fact_attached_to_src_before_check_rejects() {
    let (function, _, src, dst) = local_function("i64", LocalContractWriteKind::Init);
    let function = refresh(function);
    assert!(!function
        .metadata
        .exact_numeric_value_facts
        .contains_key(&src));
    assert!(function
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn wrong_local_slot_id_rejects() {
    let (mut function, _, _, dst) = local_function("i64", LocalContractWriteKind::Init);
    if let MirInstruction::LocalContractWrite { local_slot_id, .. } = &mut function
        .get_block_mut(function.entry_block)
        .unwrap()
        .instructions[0]
    {
        *local_slot_id = LocalSlotId::from(BindingId::new(1));
    }
    refresh_function_local_identity_evidence(&mut function);
    assert!(!refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&dst));
}

#[test]
fn mixed_i64_dynamic_phi_has_no_hard_fact() {
    let (mut function, _, _, exact) = local_function("i64", LocalContractWriteKind::Init);
    let dynamic = ValueId::new(3);
    let merged = ValueId::new(4);
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: dynamic,
        value: ConstValue::Integer(7),
    });
    block.add_instruction(MirInstruction::Phi {
        dst: merged,
        inputs: vec![
            (BasicBlockId::new(0), exact),
            (BasicBlockId::new(0), dynamic),
        ],
        type_hint: None,
    });
    refresh_function_local_identity_evidence(&mut function);
    assert!(!refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&merged));
}

#[test]
fn different_i64_local_slots_phi_has_no_hard_fact() {
    let (mut function, _, _, left) = local_function("i64", LocalContractWriteKind::Init);
    let right_slot = LocalSlotId::from(BindingId::new(1));
    register_local_slot_contract(&mut function, right_slot, "other", "i64").unwrap();
    let right = ValueId::new(4);
    let merged = ValueId::new(5);
    let block = function.get_block_mut(function.entry_block).unwrap();
    block.add_instruction(MirInstruction::LocalContractWrite {
        dst: right,
        src: ValueId::new(3),
        local_slot_id: right_slot,
        write_kind: LocalContractWriteKind::Init,
    });
    block.add_instruction(MirInstruction::Phi {
        dst: merged,
        inputs: vec![(BasicBlockId::new(0), left), (BasicBlockId::new(0), right)],
        type_hint: None,
    });
    refresh_function_local_identity_evidence(&mut function);
    assert!(!refresh(function)
        .metadata
        .exact_numeric_value_facts
        .contains_key(&merged));
}
