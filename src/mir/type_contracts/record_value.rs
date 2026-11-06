use crate::mir::function::{
    RecordContractDisposition, RecordDecl, RecordFieldValueContract, RecordValueContract,
};
use crate::mir::numeric_substrate::{exact_numeric_mir_type_from_declared_name, NumericTarget};
use crate::mir::{MirFunction, MirInstruction};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const RECORD_VALUE_CONTRACT_CAPABILITY: &str = "record_value_contracts";
pub(crate) const RECORD_CONTRACT_STALE_CARRIER_TAG: &str = "[type/record_contract_stale_carrier]";
pub(crate) const RECORD_CONTRACT_SOURCE_DRIFT_TAG: &str = "[type/record_contract_source_drift]";
#[allow(dead_code)] // consumed by the vm-reference-only record consumer
pub(crate) const RECORD_CONTRACT_REFRESH_BYPASS_TAG: &str = "[type/record_contract_refresh_bypass]";

pub(crate) fn record_schema_fingerprint(decl: &RecordDecl) -> String {
    let mut source = format!("record-v1|{}:{}|", decl.name.len(), decl.name);
    source.push_str(&format!("{}|", decl.type_parameters.len()));
    for parameter in &decl.type_parameters {
        source.push_str(&format!("{}:{}|", parameter.len(), parameter));
    }
    source.push_str(&format!("{}|", decl.fields.len()));
    for (index, field) in decl.fields.iter().enumerate() {
        let declared = field.declared_type_name.as_deref().unwrap_or("");
        let has_default = decl.default_field_names.contains(&field.name);
        source.push_str(&format!(
            "{}:{}:{}:{}:{}:{}|",
            index,
            field.name.len(),
            field.name,
            declared.len(),
            declared,
            u8::from(has_default)
        ));
    }
    format!("{:x}", Sha256::digest(source.as_bytes()))
}

pub(crate) fn is_active_record_field_type(declared_type_name: Option<&str>) -> bool {
    exact_numeric_mir_type_from_declared_name(declared_type_name, NumericTarget::host()).is_some()
}

pub(crate) fn refresh_function_record_value_contracts(
    function: &mut MirFunction,
    record_decls: &BTreeMap<String, RecordDecl>,
) -> Result<(), String> {
    function.metadata.record_value_contracts =
        build_record_value_contracts(function, record_decls)?;
    Ok(())
}

pub(crate) fn validate_record_value_contracts(
    function: &MirFunction,
    record_decls: &BTreeMap<String, RecordDecl>,
) -> Result<(), String> {
    let expected = build_record_value_contracts(function, record_decls)?;
    if function.metadata.record_value_contracts != expected {
        return Err(format!(
            "{} function={} expected={} actual={}",
            RECORD_CONTRACT_STALE_CARRIER_TAG,
            function.signature.name,
            expected.len(),
            function.metadata.record_value_contracts.len()
        ));
    }
    Ok(())
}

fn build_record_value_contracts(
    function: &MirFunction,
    record_decls: &BTreeMap<String, RecordDecl>,
) -> Result<Vec<RecordValueContract>, String> {
    let declarations_by_fingerprint = record_decls
        .values()
        .map(|decl| (record_schema_fingerprint(decl), decl))
        .collect::<BTreeMap<_, _>>();
    let mut checks = BTreeMap::new();
    let mut publishes = Vec::new();
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::RecordFieldContractCheck {
                    contract_id,
                    schema_fingerprint,
                    field_index,
                    value,
                } => {
                    let key = (contract_id.clone(), *field_index as usize);
                    if checks
                        .insert(key, (schema_fingerprint.clone(), *value))
                        .is_some()
                    {
                        return Err(format!(
                            "{} function={} contract={} field_index={}",
                            RECORD_CONTRACT_SOURCE_DRIFT_TAG,
                            function.signature.name,
                            contract_id,
                            field_index
                        ));
                    }
                }
                MirInstruction::RecordValuePublish {
                    dst,
                    contract_id,
                    boundary,
                    diagnostic_record_name,
                    schema_fingerprint,
                    base,
                    fields,
                } => publishes.push((
                    *dst,
                    contract_id,
                    *boundary,
                    diagnostic_record_name,
                    schema_fingerprint,
                    *base,
                    fields,
                )),
                _ => {}
            }
        }
    }
    publishes.sort_by(|a, b| a.1.cmp(b.1));

    let mut seen_contracts = BTreeSet::new();
    let mut contracts = Vec::with_capacity(publishes.len());
    for (dst, contract_id, boundary, record_name, fingerprint, base, field_values) in publishes {
        if !seen_contracts.insert(contract_id.clone()) {
            return Err(format!(
                "{} function={} duplicate_contract={}",
                RECORD_CONTRACT_SOURCE_DRIFT_TAG, function.signature.name, contract_id
            ));
        }
        let decl = declarations_by_fingerprint
            .get(fingerprint)
            .ok_or_else(|| {
                format!(
                    "{} function={} record={} fingerprint={}",
                    RECORD_CONTRACT_SOURCE_DRIFT_TAG,
                    function.signature.name,
                    record_name,
                    fingerprint
                )
            })?;
        if decl.name != *record_name || decl.fields.len() != field_values.len() {
            return Err(format!(
                "{} function={} record={} field_count={}",
                RECORD_CONTRACT_SOURCE_DRIFT_TAG,
                function.signature.name,
                record_name,
                field_values.len()
            ));
        }

        let mut fields = Vec::new();
        for (field_index, (field, value)) in decl.fields.iter().zip(field_values.iter()).enumerate()
        {
            let Some(declared_type) = field.declared_type_name.as_deref() else {
                continue;
            };
            if !is_active_record_field_type(Some(declared_type)) {
                continue;
            }
            let Some((check_fingerprint, checked_value)) =
                checks.remove(&(contract_id.clone(), field_index))
            else {
                return Err(format!(
                    "{} function={} contract={} field={}",
                    RECORD_CONTRACT_STALE_CARRIER_TAG,
                    function.signature.name,
                    contract_id,
                    field.name
                ));
            };
            if check_fingerprint != *fingerprint || checked_value != *value {
                return Err(format!(
                    "{} function={} contract={} field={}",
                    RECORD_CONTRACT_SOURCE_DRIFT_TAG,
                    function.signature.name,
                    contract_id,
                    field.name
                ));
            }
            fields.push(RecordFieldValueContract {
                field_index,
                diagnostic_field_name: field.name.clone(),
                value_id: *value,
                declared_type_name: declared_type.to_string(),
                disposition: RecordContractDisposition::RuntimeCheckedContract,
            });
        }
        contracts.push(RecordValueContract {
            contract_id: contract_id.clone(),
            boundary,
            diagnostic_record_name: record_name.clone(),
            schema_fingerprint: fingerprint.clone(),
            dst_value_id: dst,
            base_value_id: base,
            fields,
            backend_capability_required: RECORD_VALUE_CONTRACT_CAPABILITY.to_string(),
        });
    }
    if let Some(((contract_id, field_index), _)) = checks.into_iter().next() {
        return Err(format!(
            "{} function={} orphan_check={} field_index={}",
            RECORD_CONTRACT_SOURCE_DRIFT_TAG, function.signature.name, contract_id, field_index
        ));
    }
    Ok(contracts)
}

#[cfg(test)]
mod tests;
