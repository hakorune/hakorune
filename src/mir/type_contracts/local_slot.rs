use crate::mir::function::{LocalIdentityEvidence, LocalSlotContract};
use crate::mir::numeric_substrate::{exact_numeric_mir_type_from_declared_name, NumericTarget};
use crate::mir::type_contracts::guarantee_matrix::exact_numeric_local_slot_contract_is_active;
use crate::mir::{LocalSlotId, MirFunction, MirInstruction};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const LOCAL_SLOT_EXACT_NUMERIC_CAPABILITY: &str = "local_slot_exact_numeric";
pub(crate) const LOCAL_CONTRACT_CARRIER_MISSING_TAG: &str = "[type/local_contract_carrier_missing]";
pub(crate) const LOCAL_CONTRACT_CARRIER_DRIFT_TAG: &str = "[type/local_contract_carrier_drift]";
pub(crate) const LOCAL_CONTRACT_DUPLICATE_SLOT_TAG: &str = "[type/local_contract_duplicate_slot]";
pub(crate) const LOCAL_CONTRACT_WRITE_SITE_MISSING_TAG: &str =
    "[type/local_contract_write_site_missing]";

pub(crate) fn is_exact_numeric_local_type(declared_type_name: Option<&str>) -> bool {
    exact_numeric_mir_type_from_declared_name(declared_type_name, NumericTarget::host()).is_some()
}

pub(crate) fn register_local_slot_contract(
    function: &mut MirFunction,
    local_slot_id: LocalSlotId,
    source_name: &str,
    declared_type_name: &str,
) -> Result<(), String> {
    if !exact_numeric_local_slot_contract_is_active() {
        return Err("[type/local_contract_carrier_drift] activation=inactive".to_string());
    }
    if !is_exact_numeric_local_type(Some(declared_type_name)) {
        return Ok(());
    }
    if function
        .metadata
        .local_slot_contracts
        .iter()
        .any(|contract| contract.local_slot_id == local_slot_id)
    {
        return Err(format!(
            "{} function={} slot={:?}",
            LOCAL_CONTRACT_DUPLICATE_SLOT_TAG, function.signature.name, local_slot_id
        ));
    }
    function
        .metadata
        .local_slot_contracts
        .push(LocalSlotContract {
            contract_id: format!("local-slot:{}", local_slot_id.binding_id().raw()),
            local_slot_id,
            diagnostic_source_name: source_name.to_string(),
            declared_type_name: declared_type_name.to_string(),
            runtime_check_required: true,
            proof_elision_allowed: false,
            backend_capability_required: LOCAL_SLOT_EXACT_NUMERIC_CAPABILITY.to_string(),
        });
    function
        .metadata
        .local_slot_contracts
        .sort_by_key(|contract| contract.local_slot_id);
    Ok(())
}

pub(crate) fn validate_local_slot_contracts(function: &MirFunction) -> Result<(), String> {
    let mut slots = BTreeSet::new();
    let mut write_counts = BTreeMap::<LocalSlotId, usize>::new();
    for contract in &function.metadata.local_slot_contracts {
        if !slots.insert(contract.local_slot_id) {
            return Err(format!(
                "{} function={} slot={:?}",
                LOCAL_CONTRACT_DUPLICATE_SLOT_TAG, function.signature.name, contract.local_slot_id
            ));
        }
        if !is_exact_numeric_local_type(Some(&contract.declared_type_name))
            || !contract.runtime_check_required
            || contract.proof_elision_allowed
            || contract.backend_capability_required != LOCAL_SLOT_EXACT_NUMERIC_CAPABILITY
        {
            return Err(format!(
                "{} function={} slot={:?}",
                LOCAL_CONTRACT_CARRIER_DRIFT_TAG, function.signature.name, contract.local_slot_id
            ));
        }
    }
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            if let MirInstruction::LocalContractWrite { local_slot_id, .. } = instruction {
                *write_counts.entry(*local_slot_id).or_default() += 1;
            }
        }
    }
    for contract in &function.metadata.local_slot_contracts {
        if write_counts
            .get(&contract.local_slot_id)
            .copied()
            .unwrap_or(0)
            == 0
        {
            return Err(format!(
                "{} function={} slot={:?}",
                LOCAL_CONTRACT_WRITE_SITE_MISSING_TAG,
                function.signature.name,
                contract.local_slot_id
            ));
        }
    }
    for slot in write_counts.keys() {
        if !slots.contains(slot) {
            return Err(format!(
                "{} function={} slot={:?}",
                LOCAL_CONTRACT_CARRIER_MISSING_TAG, function.signature.name, slot
            ));
        }
    }
    let expected_evidence = build_local_identity_evidence(function);
    if function.metadata.local_identity_evidence != expected_evidence {
        return Err(format!(
            "{} function={} identity_evidence_expected={:?} actual={:?}",
            LOCAL_CONTRACT_CARRIER_DRIFT_TAG,
            function.signature.name,
            expected_evidence,
            function.metadata.local_identity_evidence
        ));
    }
    Ok(())
}

pub(crate) fn refresh_function_local_identity_evidence(function: &mut MirFunction) {
    function.metadata.local_identity_evidence = build_local_identity_evidence(function);
}

fn build_local_identity_evidence(function: &MirFunction) -> Vec<LocalIdentityEvidence> {
    let mut checked_slots = BTreeMap::<crate::mir::ValueId, LocalSlotId>::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            if let MirInstruction::LocalContractWrite {
                dst, local_slot_id, ..
            } = instruction
            {
                checked_slots.insert(*dst, *local_slot_id);
            }
        }
    }

    let mut evidence_by_merge = BTreeMap::<crate::mir::ValueId, LocalIdentityEvidence>::new();
    loop {
        let mut changed = false;
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                match instruction {
                    MirInstruction::Copy { dst, src } => {
                        if let Some(slot) = checked_slots.get(src).copied() {
                            changed |= checked_slots.insert(*dst, slot) != Some(slot);
                        }
                    }
                    MirInstruction::Phi { dst, inputs, .. } if !inputs.is_empty() => {
                        let incoming = inputs.iter().map(|(_, value)| *value).collect::<Vec<_>>();
                        let slots = incoming
                            .iter()
                            .filter_map(|value| checked_slots.get(value).copied())
                            .collect::<Vec<_>>();
                        if slots.len() == incoming.len()
                            && slots.iter().all(|slot| *slot == slots[0])
                        {
                            let slot = slots[0];
                            changed |= checked_slots.insert(*dst, slot) != Some(slot);
                            evidence_by_merge.insert(
                                *dst,
                                LocalIdentityEvidence {
                                    local_slot_id: slot,
                                    merge_value_id: *dst,
                                    incoming_values: incoming,
                                },
                            );
                        }
                    }
                    _ => {}
                }
            }
        }
        if !changed {
            break;
        }
    }
    evidence_by_merge.into_values().collect()
}

pub(crate) fn local_slot_contract(
    function: &MirFunction,
    local_slot_id: LocalSlotId,
) -> Option<&LocalSlotContract> {
    function
        .metadata
        .local_slot_contracts
        .iter()
        .find(|contract| contract.local_slot_id == local_slot_id)
}

#[cfg(test)]
mod tests;
