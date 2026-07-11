use std::collections::{BTreeMap, BTreeSet};

use crate::mir::function::{
    TypedArrayBoundaryValue, TypedArrayContractBoundary, TypedArrayContractDisposition,
    TypedArrayContractSource, TypedArrayElementContract,
};
use crate::mir::{MirFunction, MirInstruction, ValueId};
use crate::typed_array_contract_spec::parse_annotation;

pub(crate) const CARRIER_MISSING_TAG: &str = "[type/typed_array_contract_carrier_missing]";
pub(crate) const STALE_CARRIER_TAG: &str = "[type/typed_array_contract_stale_carrier]";
pub(crate) const SOURCE_DRIFT_TAG: &str = "[type/typed_array_contract_source_drift]";
pub(crate) const STATE_CONFLICT_TAG: &str = "[type/typed_array_contract_state_conflict]";

pub(crate) fn register_instruction_source(
    function: &mut MirFunction,
    boundary: TypedArrayContractBoundary,
    source_identity: crate::mir::function::TypedArrayContractSourceIdentity,
    value: ValueId,
    declared_type: Option<&str>,
    discriminator: &str,
) -> Result<Option<String>, String> {
    let Some(declared_type) = declared_type else {
        return Ok(None);
    };
    let Some(element_spec) = parse_annotation(declared_type)? else {
        return Ok(None);
    };
    let contract_id = format!("typed-array:{discriminator}");
    function
        .metadata
        .typed_array_contract_sources
        .push(TypedArrayContractSource {
            contract_id: contract_id.clone(),
            boundary,
            source_identity,
            boundary_value: TypedArrayBoundaryValue::Value(value),
            element_spec,
        });
    Ok(Some(contract_id))
}

pub(crate) fn register_source_with_id(
    function: &mut MirFunction,
    contract_id: String,
    boundary: TypedArrayContractBoundary,
    source_identity: crate::mir::function::TypedArrayContractSourceIdentity,
    value: ValueId,
    element_spec: crate::typed_array_contract_spec::ArrayElementContractSpec,
) {
    function
        .metadata
        .typed_array_contract_sources
        .push(TypedArrayContractSource {
            contract_id,
            boundary,
            source_identity,
            boundary_value: TypedArrayBoundaryValue::Value(value),
            element_spec,
        });
}

pub(crate) fn local_slot_spec(
    function: &MirFunction,
    slot: crate::mir::LocalSlotId,
) -> Option<crate::typed_array_contract_spec::ArrayElementContractSpec> {
    function
        .metadata
        .typed_array_contract_sources
        .iter()
        .find_map(|source| match source.source_identity {
            crate::mir::function::TypedArrayContractSourceIdentity::LocalSlot(source_slot)
                if source_slot == slot =>
            {
                Some(source.element_spec)
            }
            _ => None,
        })
}

pub(crate) fn refresh_source_rows(function: &mut MirFunction) -> Result<(), String> {
    function
        .metadata
        .typed_array_contract_sources
        .retain(|source| {
            !matches!(
                source.boundary,
                TypedArrayContractBoundary::ParameterEntry | TypedArrayContractBoundary::ReturnExit
            )
        });
    for (formal_index, declaration) in function.metadata.declared_param_decls.iter().enumerate() {
        if declaration.implicit_receiver {
            continue;
        }
        let Some(name) = declaration.declared_type_name.as_deref() else {
            continue;
        };
        let Some(element_spec) = parse_annotation(name)? else {
            continue;
        };
        let Some(value) = function.params.get(formal_index).copied() else {
            return Err(format!(
                "{} function={} parameter={}",
                SOURCE_DRIFT_TAG, function.signature.name, formal_index
            ));
        };
        function
            .metadata
            .typed_array_contract_sources
            .push(TypedArrayContractSource {
                contract_id: format!("typed-array:parameter:{formal_index}"),
                boundary: TypedArrayContractBoundary::ParameterEntry,
                source_identity:
                    crate::mir::function::TypedArrayContractSourceIdentity::Parameter {
                        formal_index,
                    },
                boundary_value: TypedArrayBoundaryValue::Value(value),
                element_spec,
            });
    }
    if let Some(name) = function.metadata.declared_return_type_name.as_deref() {
        if let Some(element_spec) = parse_annotation(name)? {
            function
                .metadata
                .typed_array_contract_sources
                .push(TypedArrayContractSource {
                    contract_id: "typed-array:return".to_string(),
                    boundary: TypedArrayContractBoundary::ReturnExit,
                    source_identity: crate::mir::function::TypedArrayContractSourceIdentity::Return,
                    boundary_value: TypedArrayBoundaryValue::FinalReturn,
                    element_spec,
                });
        }
    }
    Ok(())
}

pub(crate) fn refresh_function(function: &mut MirFunction) -> Result<(), String> {
    function.metadata.typed_array_element_contracts = build(function)?;
    Ok(())
}

pub(crate) fn validate_function(function: &MirFunction) -> Result<(), String> {
    let expected = build(function)?;
    if expected != function.metadata.typed_array_element_contracts {
        return Err(format!(
            "{} function={} expected={} actual={}",
            STALE_CARRIER_TAG,
            function.signature.name,
            expected.len(),
            function.metadata.typed_array_element_contracts.len()
        ));
    }
    Ok(())
}

fn build(function: &MirFunction) -> Result<Vec<TypedArrayElementContract>, String> {
    let terms = function
        .metadata
        .array_state_terms
        .iter()
        .map(|term| (term.value, term.term_id))
        .collect::<BTreeMap<_, _>>();
    let claims = collect_claims(function)?;
    let mut seen_ids = BTreeSet::new();
    let mut state_specs = BTreeMap::new();
    let mut carriers = Vec::new();
    for source in sorted_sources(&function.metadata.typed_array_contract_sources) {
        if !seen_ids.insert(source.contract_id.as_str()) {
            return Err(format!(
                "{} duplicate_contract={}",
                SOURCE_DRIFT_TAG, source.contract_id
            ));
        }
        validate_claim_presence(source, &claims)?;
        let state_term = match source.boundary_value {
            TypedArrayBoundaryValue::Value(value) => Some(*terms.get(&value).ok_or_else(|| {
                format!(
                    "{} contract={} value={}",
                    CARRIER_MISSING_TAG, source.contract_id, value.0
                )
            })?),
            TypedArrayBoundaryValue::FinalReturn => None,
        };
        if let Some(term) = state_term {
            if let Some(previous) = state_specs.insert(term, source.element_spec) {
                if previous != source.element_spec {
                    return Err(format!(
                        "{} contract={}",
                        STATE_CONFLICT_TAG, source.contract_id
                    ));
                }
            }
        }
        carriers.push(TypedArrayElementContract {
            contract_id: source.contract_id.clone(),
            boundary: source.boundary,
            source_identity: source.source_identity.clone(),
            boundary_value: source.boundary_value,
            state_term,
            element_spec: source.element_spec,
            disposition: TypedArrayContractDisposition::RuntimeCheckedContract,
            runtime_check_required: true,
            proof_elision_allowed: false,
            backend_capability_required: crate::mir::function::TYPED_ARRAY_EXACT_NUMERIC_CAPABILITY
                .to_string(),
        });
    }
    Ok(carriers)
}

fn sorted_sources(sources: &[TypedArrayContractSource]) -> Vec<&TypedArrayContractSource> {
    let mut sorted = sources.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| a.contract_id.cmp(&b.contract_id));
    sorted
}

fn collect_claims(function: &MirFunction) -> Result<BTreeMap<&str, ValueId>, String> {
    let mut claims = BTreeMap::new();
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
    {
        let MirInstruction::ArrayStateContractClaim { contract_id, array } = instruction else {
            continue;
        };
        if claims.insert(contract_id.as_str(), *array).is_some() {
            return Err(format!(
                "{} duplicate_claim={}",
                SOURCE_DRIFT_TAG, contract_id
            ));
        }
    }
    Ok(claims)
}

fn validate_claim_presence(
    source: &TypedArrayContractSource,
    claims: &BTreeMap<&str, ValueId>,
) -> Result<(), String> {
    let requires_instruction = matches!(
        source.boundary,
        TypedArrayContractBoundary::LocalInit
            | TypedArrayContractBoundary::LocalReassign
            | TypedArrayContractBoundary::BoxFieldWrite
            | TypedArrayContractBoundary::RecordConstruct
            | TypedArrayContractBoundary::RecordWithUpdate
    );
    if !requires_instruction {
        return Ok(());
    }
    let TypedArrayBoundaryValue::Value(expected) = source.boundary_value else {
        return Err(format!(
            "{} contract={}",
            SOURCE_DRIFT_TAG, source.contract_id
        ));
    };
    match claims.get(source.contract_id.as_str()) {
        Some(actual) if *actual == expected => Ok(()),
        _ => Err(format!(
            "{} contract={}",
            CARRIER_MISSING_TAG, source.contract_id
        )),
    }
}
