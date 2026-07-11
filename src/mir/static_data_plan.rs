//! MIR-owned static readonly data rows.
//!
//! Source declarations lower into `StaticDataPlan`; backend emitters consume the
//! rows and must not rediscover source syntax or table semantics.

use crate::ast::ASTNode;
use crate::mir::function::{
    StaticDataPlan, StaticElementType, StaticTableContractSpec, StaticTableId,
};

pub fn collect_static_table_specs_from_ast(
    module_name: &str,
    ast: &ASTNode,
) -> Result<Vec<StaticTableContractSpec>, String> {
    let ASTNode::Program { statements, .. } = ast else {
        return Ok(Vec::new());
    };

    statements
        .iter()
        .filter_map(|statement| {
            let ASTNode::StaticConstTable {
                name,
                element_type,
                values,
                ..
            } = statement
            else {
                return None;
            };
            Some(static_table_contract_spec(
                module_name,
                name,
                element_type,
                values,
            ))
        })
        .collect()
}

fn static_table_contract_spec(
    module_name: &str,
    name: &str,
    element_type: &str,
    values: &[u64],
) -> Result<StaticTableContractSpec, String> {
    if element_type != "u16" {
        return Err(format!(
            "[static-const/unsupported-element] {} element={}",
            name, element_type
        ));
    }
    let values = values
        .iter()
        .map(|value| {
            u16::try_from(*value)
                .map_err(|_| format!("[static-const/value-out-of-range] {} value={}", name, value))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(StaticTableContractSpec {
        table_id: StaticTableId {
            module_name: module_name.to_string(),
            declaration_name: name.to_string(),
        },
        diagnostic_name: name.to_string(),
        element: StaticElementType::U16,
        values,
    })
}

pub fn static_data_plan_from_spec(spec: &StaticTableContractSpec) -> StaticDataPlan {
    StaticDataPlan {
        source_name: spec.table_id.declaration_name.clone(),
        symbol: format!(".hako.static.{}", spec.table_id.declaration_name),
        element: spec.element.as_str().to_string(),
        align: 2,
        linkage: "private".to_string(),
        unnamed_addr: true,
        values: spec.values.iter().map(|value| u64::from(*value)).collect(),
    }
}

pub fn static_data_plans_from_specs(specs: &[StaticTableContractSpec]) -> Vec<StaticDataPlan> {
    specs.iter().map(static_data_plan_from_spec).collect()
}

pub fn find_static_data_plan<'a>(
    plans: &'a [StaticDataPlan],
    source_name: &str,
) -> Option<&'a StaticDataPlan> {
    plans.iter().find(|plan| plan.source_name == source_name)
}
