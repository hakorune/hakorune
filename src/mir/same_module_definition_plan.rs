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
    let mut seen = BTreeSet::<String>::new();
    let mut cursor = 0;
    if let Some(edges) = immediate.get(root) {
        for edge in edges {
            push_unique(edge.clone(), &mut seen, &mut out);
        }
    }
    while cursor < out.len() {
        let edge = out[cursor].clone();
        cursor += 1;
        if edge.definition_kind != SameModuleDefinitionKind::Function {
            continue;
        }
        if let Some(edges) = immediate.get(&edge.target_symbol) {
            for child in edges {
                push_unique(child.clone(), &mut seen, &mut out);
            }
        }
    }
    out
}

fn push_unique(
    edge: SameModuleDefinitionPlan,
    seen: &mut BTreeSet<String>,
    out: &mut Vec<SameModuleDefinitionPlan>,
) {
    let key = format!(
        "{}:{}",
        edge.definition_kind.as_json_name(),
        edge.target_symbol
    );
    if seen.insert(key) {
        out.push(edge);
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
