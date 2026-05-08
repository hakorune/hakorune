//! MIR-owned static readonly data rows.
//!
//! Source declarations lower into `StaticDataPlan`; backend emitters consume the
//! rows and must not rediscover source syntax or table semantics.

use crate::ast::ASTNode;
use crate::mir::function::StaticDataPlan;

pub fn collect_static_data_plans_from_ast(ast: &ASTNode) -> Vec<StaticDataPlan> {
    let ASTNode::Program { statements, .. } = ast else {
        return Vec::new();
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
            Some(static_const_table_plan(name, element_type, values))
        })
        .collect()
}

pub fn static_const_table_plan(name: &str, element_type: &str, values: &[u64]) -> StaticDataPlan {
    StaticDataPlan {
        source_name: name.to_string(),
        symbol: format!(".hako.static.{}", name),
        element: element_type.to_string(),
        align: static_data_alignment(element_type),
        linkage: "private".to_string(),
        unnamed_addr: true,
        values: values.to_vec(),
    }
}

pub fn find_static_data_plan<'a>(
    plans: &'a [StaticDataPlan],
    source_name: &str,
) -> Option<&'a StaticDataPlan> {
    plans.iter().find(|plan| plan.source_name == source_name)
}

fn static_data_alignment(element_type: &str) -> u32 {
    match element_type {
        "u8" => 1,
        "u16" => 2,
        "u32" => 4,
        "u64" => 8,
        _ => 1,
    }
}
