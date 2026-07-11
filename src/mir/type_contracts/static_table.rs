//! Readonly U16 Static Table contract refresh owner.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::function::{
    StaticElementType, StaticTableContractProof, StaticTableContractSpec, StaticTableId,
    VerifiedStaticTableContract,
};
use crate::mir::{MirInstruction, MirModule};

pub(crate) const DUPLICATE_ID_TAG: &str = "[type/static_table_contract_duplicate_id]";
pub(crate) const SPEC_MISSING_TAG: &str = "[type/static_table_contract_spec_missing]";
pub(crate) const PLAN_MISSING_TAG: &str = "[type/static_table_contract_plan_missing]";
pub(crate) const CARRIER_MISSING_TAG: &str = "[type/static_table_contract_carrier_missing]";
pub(crate) const DRIFT_TAG: &str = "[type/static_table_contract_drift]";

pub(crate) fn refresh_module_static_table_contracts(module: &mut MirModule) -> Result<(), String> {
    let carriers = rebuild(module)?;
    module.metadata.verified_static_table_contracts = carriers;
    Ok(())
}

pub(crate) fn validate_static_table_contracts(module: &MirModule) -> Result<(), String> {
    let rebuilt = rebuild(module)?;
    if rebuilt != module.metadata.verified_static_table_contracts {
        return Err(format!(
            "{} rebuilt={} carried={}",
            CARRIER_MISSING_TAG,
            rebuilt.len(),
            module.metadata.verified_static_table_contracts.len()
        ));
    }
    Ok(())
}

fn rebuild(module: &MirModule) -> Result<Vec<VerifiedStaticTableContract>, String> {
    let mut specs = BTreeMap::<StaticTableId, &StaticTableContractSpec>::new();
    for spec in &module.metadata.static_table_contract_specs {
        if spec.table_id.module_name != module.name
            || spec.table_id.declaration_name != spec.diagnostic_name
        {
            return Err(format!("{} table={}", DRIFT_TAG, spec.diagnostic_name));
        }
        if specs.insert(spec.table_id.clone(), spec).is_some() {
            return Err(format!(
                "{} table={}",
                DUPLICATE_ID_TAG, spec.diagnostic_name
            ));
        }
    }

    if specs.is_empty() && !module.metadata.static_data_plans.is_empty() {
        return Err(format!("{} plans_without_specs", SPEC_MISSING_TAG));
    }

    let mut used_plans = BTreeSet::new();
    let mut carriers = Vec::with_capacity(specs.len());
    for (id, spec) in specs {
        if spec.element != StaticElementType::U16 {
            return Err(format!(
                "{} table={} element",
                DRIFT_TAG, spec.diagnostic_name
            ));
        }
        let plan = module
            .metadata
            .static_data_plans
            .iter()
            .find(|plan| plan.source_name == id.declaration_name)
            .ok_or_else(|| format!("{} table={}", PLAN_MISSING_TAG, id.declaration_name))?;
        if !used_plans.insert(plan.source_name.clone()) {
            return Err(format!("{} plan={}", DUPLICATE_ID_TAG, plan.source_name));
        }
        validate_spec_plan(spec, plan)?;
        validate_load_sites(module, spec, plan)?;
        carriers.push(VerifiedStaticTableContract {
            table_id: id,
            element: StaticElementType::U16,
            len: spec.values.len() as u32,
            plan_symbol: plan.symbol.clone(),
            proof: StaticTableContractProof::SourceSpecAndPlanStructurallyMatch,
        });
    }
    if used_plans.len() != module.metadata.static_data_plans.len() {
        return Err(format!("{} orphan_plan", SPEC_MISSING_TAG));
    }
    validate_all_loads_have_specs(module)?;
    carriers.sort_by(|left, right| left.table_id.cmp(&right.table_id));
    Ok(carriers)
}

fn validate_all_loads_have_specs(module: &MirModule) -> Result<(), String> {
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                let MirInstruction::StaticDataLoad { source_name, .. } = instruction else {
                    continue;
                };
                if !module
                    .metadata
                    .static_table_contract_specs
                    .iter()
                    .any(|spec| {
                        spec.table_id.module_name == module.name
                            && spec.table_id.declaration_name == *source_name
                    })
                {
                    return Err(format!("{} table={}", SPEC_MISSING_TAG, source_name));
                }
            }
        }
    }
    Ok(())
}

fn validate_spec_plan(
    spec: &StaticTableContractSpec,
    plan: &crate::mir::function::StaticDataPlan,
) -> Result<(), String> {
    let values = spec
        .values
        .iter()
        .map(|value| u64::from(*value))
        .collect::<Vec<_>>();
    let expected_symbol = format!(".hako.static.{}", spec.table_id.declaration_name);
    if plan.symbol != expected_symbol
        || plan.element != "u16"
        || plan.align != 2
        || plan.linkage != "private"
        || !plan.unnamed_addr
        || plan.values != values
    {
        return Err(format!("{} table={}", DRIFT_TAG, spec.diagnostic_name));
    }
    Ok(())
}

fn validate_load_sites(
    module: &MirModule,
    spec: &StaticTableContractSpec,
    plan: &crate::mir::function::StaticDataPlan,
) -> Result<(), String> {
    for function in module.functions.values() {
        for block in function.blocks.values() {
            for instruction in &block.instructions {
                let MirInstruction::StaticDataLoad {
                    source_name,
                    symbol,
                    element,
                    len,
                    align,
                    ..
                } = instruction
                else {
                    continue;
                };
                if source_name == &spec.table_id.declaration_name
                    && (symbol != &plan.symbol
                        || element != "u16"
                        || *len != spec.values.len() as u32
                        || *align != 2)
                {
                    return Err(format!("{} table={} load", DRIFT_TAG, source_name));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
