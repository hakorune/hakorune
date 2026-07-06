//! MIR-owned same-module definition list for C AOT lowering.
//!
//! The C shim may emit selected helper functions, but it must not discover the
//! transitive definition set by recursively scanning lowering plans. This module
//! publishes that closure as explicit metadata rows.

use crate::mir::{MirFunction, MirModule};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SameModuleDefinitionKind {
    Function,
    LeafI64,
}

impl SameModuleDefinitionKind {
    pub fn as_json_name(&self) -> &'static str {
        match self {
            Self::Function => "same_module_function",
            Self::LeafI64 => "leaf_i64_function",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SameModuleDefinitionPlan {
    pub target_symbol: String,
    pub definition_kind: SameModuleDefinitionKind,
    pub definition_owner: String,
    pub source: String,
}

pub fn refresh_module_same_module_definition_plans(module: &mut MirModule) {
    let function_names = module
        .function_names()
        .into_iter()
        .cloned()
        .collect::<Vec<_>>();
    let mut immediate = BTreeMap::<String, Vec<SameModuleDefinitionPlan>>::new();
    for name in &function_names {
        let Some(function) = module.get_function(name) else {
            continue;
        };
        immediate.insert(name.clone(), immediate_definition_edges(module, function));
    }

    for name in function_names {
        let plans = definition_closure(&name, &immediate);
        if let Some(function) = module.get_function_mut(&name) {
            function.metadata.same_module_definition_plans = plans;
        }
    }
}

fn definition_closure(
    root: &str,
    immediate: &BTreeMap<String, Vec<SameModuleDefinitionPlan>>,
) -> Vec<SameModuleDefinitionPlan> {
    let mut out = Vec::new();
    let mut visiting = BTreeSet::<String>::new();
    let mut emitted = BTreeSet::<String>::new();
    if let Some(edges) = immediate.get(root) {
        for edge in edges {
            visit_definition_edge(edge, immediate, &mut visiting, &mut emitted, &mut out);
        }
    }
    out
}

fn visit_definition_edge(
    edge: &SameModuleDefinitionPlan,
    immediate: &BTreeMap<String, Vec<SameModuleDefinitionPlan>>,
    visiting: &mut BTreeSet<String>,
    emitted: &mut BTreeSet<String>,
    out: &mut Vec<SameModuleDefinitionPlan>,
) {
    let key = format!(
        "{}:{}",
        edge.definition_kind.as_json_name(),
        edge.target_symbol
    );
    if emitted.contains(&key) {
        return;
    }
    if visiting.contains(&key) {
        return;
    }
    if edge.definition_kind == SameModuleDefinitionKind::Function {
        visiting.insert(key.clone());
        if let Some(children) = immediate.get(&edge.target_symbol) {
            for child in children {
                visit_definition_edge(child, immediate, visiting, emitted, out);
            }
        }
        visiting.remove(&key);
    }
    if emitted.insert(key) {
        out.push(edge.clone());
    }
}

fn immediate_definition_edges(
    module: &MirModule,
    function: &MirFunction,
) -> Vec<SameModuleDefinitionPlan> {
    let mut out = Vec::new();
    for route in &function.metadata.global_call_routes {
        let Some(target_symbol) = route.target_symbol() else {
            continue;
        };
        match route.definition_owner() {
            "leaf_i64" => out.push(definition_row(
                target_symbol,
                SameModuleDefinitionKind::LeafI64,
                route.definition_owner(),
                "global_call_routes",
            )),
            "generic_i64_or_leaf" => {
                let kind = if module
                    .get_function(target_symbol)
                    .and_then(|target| target.metadata.global_call_routes.first())
                    .is_none()
                    && route.target_shape() == Some("numeric_i64_leaf")
                {
                    SameModuleDefinitionKind::LeafI64
                } else {
                    SameModuleDefinitionKind::Function
                };
                out.push(definition_row(
                    target_symbol,
                    kind,
                    route.definition_owner(),
                    "global_call_routes",
                ));
            }
            "module_generic" | "uniform_mir" => out.push(definition_row(
                target_symbol,
                SameModuleDefinitionKind::Function,
                route.definition_owner(),
                "global_call_routes",
            )),
            _ => {}
        }
    }
    for route in &function.metadata.user_box_method_routes {
        if route.definition_owner() == "typed_object_method" {
            out.push(definition_row(
                route.target_symbol(),
                SameModuleDefinitionKind::Function,
                route.definition_owner(),
                "user_box_method_routes",
            ));
        }
    }
    out
}

fn definition_row(
    target_symbol: &str,
    definition_kind: SameModuleDefinitionKind,
    definition_owner: &str,
    source: &str,
) -> SameModuleDefinitionPlan {
    SameModuleDefinitionPlan {
        target_symbol: target_symbol.to_string(),
        definition_kind,
        definition_owner: definition_owner.to_string(),
        source: source.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(symbol: &str, owner: &str) -> SameModuleDefinitionPlan {
        definition_row(
            symbol,
            SameModuleDefinitionKind::Function,
            owner,
            "global_call_routes",
        )
    }

    #[test]
    fn closure_orders_dependencies_before_dependents() {
        let mut immediate = BTreeMap::<String, Vec<SameModuleDefinitionPlan>>::new();
        immediate.insert("main".to_string(), vec![row("Summary.build/1", "module_generic")]);
        immediate.insert(
            "Summary.build/1".to_string(),
            vec![row("PhaseState.parse/2", "uniform_mir")],
        );
        immediate.insert(
            "PhaseState.parse/2".to_string(),
            vec![row("Consumer.consume/4", "uniform_mir")],
        );

        let plans = definition_closure("main", &immediate);
        let symbols = plans
            .iter()
            .map(|plan| plan.target_symbol.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            symbols,
            vec![
                "Consumer.consume/4",
                "PhaseState.parse/2",
                "Summary.build/1"
            ]
        );
    }

    #[test]
    fn closure_keeps_cycles_finite() {
        let mut immediate = BTreeMap::<String, Vec<SameModuleDefinitionPlan>>::new();
        immediate.insert("main".to_string(), vec![row("A/0", "uniform_mir")]);
        immediate.insert("A/0".to_string(), vec![row("B/0", "uniform_mir")]);
        immediate.insert("B/0".to_string(), vec![row("A/0", "uniform_mir")]);

        let plans = definition_closure("main", &immediate);
        let symbols = plans
            .iter()
            .map(|plan| plan.target_symbol.as_str())
            .collect::<Vec<_>>();

        assert_eq!(symbols, vec!["B/0", "A/0"]);
    }
}
