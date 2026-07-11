//! Source-owned Weak field declaration contracts.

use crate::mir::function::{
    UserBoxFieldDecl, WeakFieldContractSpec, WeakFieldId, WeakFieldWriteContract,
};
use crate::mir::{MirFunction, MirInstruction, MirModule, ValueId, WeakFieldWriteSiteId};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const DUPLICATE_SPEC_TAG: &str = "[type/weak_field_contract_duplicate_spec]";
pub(crate) const SOURCE_DRIFT_TAG: &str = "[type/weak_field_contract_source_drift]";
pub(crate) const CARRIER_MISSING_TAG: &str = "[type/weak_field_contract_carrier_missing]";
pub(crate) const STALE_CARRIER_TAG: &str = "[type/weak_field_contract_stale_carrier]";
pub(crate) const RESIDUAL_FIELDSET_TAG: &str = "[type/weak_field_contract_residual_fieldset]";
pub(crate) const WEAK_FIELD_CAPABILITY: &str = "weak_field_runtime_guard_v1";

pub fn box_schema_fingerprint(box_name: &str, fields: &[UserBoxFieldDecl]) -> String {
    let mut source = format!("box-v1|{}:{}|{}|", box_name.len(), box_name, fields.len());
    for (index, field) in fields.iter().enumerate() {
        let declared = field.declared_type_name.as_deref().unwrap_or("");
        source.push_str(&format!(
            "{}:{}:{}:{}:{}:{}|",
            index,
            field.name.len(),
            field.name,
            declared.len(),
            declared,
            u8::from(field.is_weak)
        ));
    }
    format!("{:x}", Sha256::digest(source.as_bytes()))
}

pub(crate) fn refresh_module_specs(module: &mut MirModule) -> Result<(), String> {
    module.metadata.weak_field_contract_specs = build_specs(module)?;
    Ok(())
}

pub(crate) fn validate_module_specs(module: &MirModule) -> Result<(), String> {
    let expected = build_specs(module)?;
    if expected != module.metadata.weak_field_contract_specs {
        return Err(format!(
            "{} expected={} actual={}",
            SOURCE_DRIFT_TAG,
            expected.len(),
            module.metadata.weak_field_contract_specs.len()
        ));
    }
    Ok(())
}

fn build_specs(module: &MirModule) -> Result<Vec<WeakFieldContractSpec>, String> {
    let mut declarations = module
        .metadata
        .user_box_field_decls
        .iter()
        .collect::<Vec<_>>();
    declarations.sort_by(|left, right| left.0.cmp(right.0));
    let mut specs = Vec::new();
    for (box_name, fields) in declarations {
        let fingerprint = box_schema_fingerprint(box_name, fields);
        for (field_index, field) in fields.iter().enumerate() {
            if !field.is_weak {
                continue;
            }
            specs.push(WeakFieldContractSpec {
                contract_id: format!("weak-field:{fingerprint}:{field_index}"),
                weak_field_id: WeakFieldId {
                    box_schema_fingerprint: fingerprint.clone(),
                    field_index: field_index as u32,
                },
                diagnostic_box_name: box_name.clone(),
                diagnostic_field_name: field.name.clone(),
            });
        }
    }
    specs.sort();
    for pair in specs.windows(2) {
        if pair[0].contract_id == pair[1].contract_id {
            return Err(format!(
                "{} contract={}",
                DUPLICATE_SPEC_TAG, pair[0].contract_id
            ));
        }
    }
    Ok(specs)
}

pub(crate) fn canonicalize_function(
    function: &mut MirFunction,
    specs: &[WeakFieldContractSpec],
) -> Result<(), String> {
    let specs_by_field = specs
        .iter()
        .map(|spec| {
            (
                (
                    spec.diagnostic_box_name.as_str(),
                    spec.diagnostic_field_name.as_str(),
                ),
                spec,
            )
        })
        .collect::<BTreeMap<_, _>>();
    let origins = infer_box_origins(function);
    let mut next_site = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            MirInstruction::WeakFieldWrite { site_id, .. } => Some(site_id.0),
            _ => None,
        })
        .max()
        .map_or(0, |site| site.saturating_add(1));

    for instruction in function
        .blocks
        .values_mut()
        .flat_map(|block| block.instructions.iter_mut())
    {
        let MirInstruction::FieldSet {
            base, field, value, ..
        } = instruction
        else {
            continue;
        };
        let Some(box_name) = origins.get(base) else {
            continue;
        };
        let Some(spec) = specs_by_field.get(&(box_name.as_str(), field.as_str())) else {
            continue;
        };
        *instruction = MirInstruction::WeakFieldWrite {
            site_id: WeakFieldWriteSiteId::new(next_site),
            contract_id: spec.contract_id.clone(),
            base: *base,
            field_index: spec.weak_field_id.field_index,
            value: *value,
        };
        next_site = next_site.saturating_add(1);
    }
    Ok(())
}

pub(crate) fn refresh_function(
    function: &mut MirFunction,
    specs: &[WeakFieldContractSpec],
) -> Result<(), String> {
    function.metadata.weak_field_write_contracts = build_write_contracts(function, specs)?;
    Ok(())
}

pub(crate) fn validate_function(
    function: &MirFunction,
    specs: &[WeakFieldContractSpec],
) -> Result<(), String> {
    let expected = build_write_contracts(function, specs)?;
    if expected != function.metadata.weak_field_write_contracts {
        return Err(format!(
            "{} function={} expected={} actual={}",
            STALE_CARRIER_TAG,
            function.signature.name,
            expected.len(),
            function.metadata.weak_field_write_contracts.len()
        ));
    }
    reject_residual_known_fieldsets(function, specs)
}

fn build_write_contracts(
    function: &MirFunction,
    specs: &[WeakFieldContractSpec],
) -> Result<Vec<WeakFieldWriteContract>, String> {
    let specs_by_id = specs
        .iter()
        .map(|spec| (spec.contract_id.as_str(), spec))
        .collect::<BTreeMap<_, _>>();
    let mut seen_sites = BTreeSet::new();
    let mut contracts = Vec::new();
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
    {
        let MirInstruction::WeakFieldWrite {
            site_id,
            contract_id,
            base,
            field_index,
            value,
        } = instruction
        else {
            continue;
        };
        if !seen_sites.insert(*site_id) {
            return Err(format!(
                "{} function={} duplicate_site={}",
                SOURCE_DRIFT_TAG, function.signature.name, site_id.0
            ));
        }
        let spec = specs_by_id.get(contract_id.as_str()).ok_or_else(|| {
            format!(
                "{} function={} contract={}",
                CARRIER_MISSING_TAG, function.signature.name, contract_id
            )
        })?;
        if spec.weak_field_id.field_index != *field_index {
            return Err(format!(
                "{} function={} contract={} field_index={}",
                SOURCE_DRIFT_TAG, function.signature.name, contract_id, field_index
            ));
        }
        contracts.push(WeakFieldWriteContract {
            site_id: *site_id,
            contract_id: contract_id.clone(),
            base_value_id: *base,
            value_id: *value,
            box_schema_fingerprint: spec.weak_field_id.box_schema_fingerprint.clone(),
            field_index: *field_index,
            runtime_check_required: true,
            proof_elision_allowed: false,
            backend_capability_required: WEAK_FIELD_CAPABILITY.to_string(),
        });
    }
    contracts.sort_by_key(|contract| contract.site_id);
    Ok(contracts)
}

fn infer_box_origins(function: &MirFunction) -> BTreeMap<ValueId, String> {
    let mut origins = BTreeMap::new();
    let instructions = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .collect::<Vec<_>>();
    for instruction in &instructions {
        if let MirInstruction::NewBox { dst, box_type, .. } = instruction {
            origins.insert(*dst, box_type.clone());
        }
    }
    loop {
        let mut changed = false;
        for instruction in &instructions {
            let derived = match instruction {
                MirInstruction::Copy { dst, src } => {
                    origins.get(src).cloned().map(|origin| (*dst, origin))
                }
                MirInstruction::Phi { dst, inputs, .. } => {
                    let incoming = inputs
                        .iter()
                        .filter_map(|(_, value)| origins.get(value))
                        .collect::<Vec<_>>();
                    (!incoming.is_empty()
                        && incoming.len() == inputs.len()
                        && incoming.windows(2).all(|pair| pair[0] == pair[1]))
                    .then(|| (*dst, incoming[0].clone()))
                }
                _ => None,
            };
            if let Some((value, origin)) = derived {
                changed |= origins.insert(value, origin.as_str().to_string()).is_none();
            }
        }
        if !changed {
            return origins;
        }
    }
}

fn reject_residual_known_fieldsets(
    function: &MirFunction,
    specs: &[WeakFieldContractSpec],
) -> Result<(), String> {
    let origins = infer_box_origins(function);
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
    {
        let MirInstruction::FieldSet { base, field, .. } = instruction else {
            continue;
        };
        let Some(box_name) = origins.get(base) else {
            continue;
        };
        if specs.iter().any(|spec| {
            spec.diagnostic_box_name == *box_name && spec.diagnostic_field_name == *field
        }) {
            return Err(format!(
                "{} function={} box={} field={}",
                RESIDUAL_FIELDSET_TAG, function.signature.name, box_name, field
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specs_use_declaration_order_identity() {
        let mut module = MirModule::new("weak-spec".to_string());
        module.metadata.user_box_field_decls.insert(
            "Node".to_string(),
            vec![
                UserBoxFieldDecl {
                    name: "value".to_string(),
                    declared_type_name: None,
                    is_weak: false,
                },
                UserBoxFieldDecl {
                    name: "parent".to_string(),
                    declared_type_name: None,
                    is_weak: true,
                },
            ],
        );

        refresh_module_specs(&mut module).expect("spec refresh should succeed");

        let spec = &module.metadata.weak_field_contract_specs[0];
        assert_eq!(spec.weak_field_id.field_index, 1);
        assert_eq!(spec.diagnostic_field_name, "parent");
        assert!(spec.contract_id.starts_with("weak-field:"));
    }
}
