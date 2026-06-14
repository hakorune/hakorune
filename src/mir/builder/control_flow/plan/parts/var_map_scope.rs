use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::{BTreeMap, BTreeSet};

/// Run `f` with a saved variable_map snapshot and always restore afterward.
///
/// This keeps branch-local lowering failures from leaking partially-mutated
/// bindings into outer lowering paths.
pub(super) fn with_saved_variable_map<T, F>(builder: &mut MirBuilder, f: F) -> Result<T, String>
where
    F: FnOnce(&mut MirBuilder) -> Result<T, String>,
{
    let saved = builder.variable_ctx.variable_map.clone();
    let result = f(builder);
    builder.variable_ctx.variable_map = saved;
    result
}

/// Run `f` inside a lexical ScopeBox boundary.
///
/// Plan lowering updates both `branch_bindings` and `builder.variable_map`
/// directly, bypassing the normal Rust `LexicalScopeGuard`. This helper keeps
/// that route honest: names declared in the ScopeBox do not escape, while
/// assignments to preexisting outer names may escape.
pub(super) fn with_scopebox_binding_boundary<T, F>(
    builder: &mut MirBuilder,
    branch_bindings: &mut BTreeMap<String, ValueId>,
    body: &[ASTNode],
    f: F,
) -> Result<T, String>
where
    F: FnOnce(&mut MirBuilder, &mut BTreeMap<String, ValueId>) -> Result<T, String>,
{
    let pre_builder_map = builder.variable_ctx.variable_map.clone();
    let pre_bindings = branch_bindings.clone();
    let scope_locals = collect_scope_local_vars(body);

    let mut scoped_bindings = branch_bindings.clone();
    let result = f(builder, &mut scoped_bindings);

    builder.variable_ctx.variable_map = pre_builder_map.clone();
    *branch_bindings = pre_bindings.clone();

    if result.is_ok() {
        merge_scopebox_outer_updates(
            builder,
            branch_bindings,
            &pre_builder_map,
            &pre_bindings,
            &scope_locals,
            scoped_bindings,
        );
    }

    result
}

/// Publish a defined value into the MIR emission cache and branch bindings.
///
/// `branch_bindings` remains the logical path state for CorePlan lowering.
/// `builder.variable_map` is only re-sealed so map-first value lookup observes
/// the same already-defined value while emitting the current path.
pub(in crate::mir::builder::control_flow::plan::parts) fn publish_emission_cache(
    builder: &mut MirBuilder,
    name: String,
    value_id: ValueId,
) {
    builder.variable_ctx.variable_map.insert(name, value_id);
}

pub(in crate::mir::builder::control_flow::plan::parts) fn publish_defined_binding(
    builder: &mut MirBuilder,
    branch_bindings: &mut BTreeMap<String, ValueId>,
    name: String,
    value_id: ValueId,
) {
    branch_bindings.insert(name.clone(), value_id);
    publish_emission_cache(builder, name, value_id);
}

/// Re-seal the builder emission cache from the logical branch bindings.
pub(in crate::mir::builder::control_flow::plan::parts) fn reseal_branch_bindings(
    builder: &mut MirBuilder,
    branch_bindings: &BTreeMap<String, ValueId>,
) {
    for (name, value_id) in branch_bindings {
        publish_emission_cache(builder, name.clone(), *value_id);
    }
}

fn merge_scopebox_outer_updates(
    builder: &mut MirBuilder,
    branch_bindings: &mut BTreeMap<String, ValueId>,
    pre_builder_map: &BTreeMap<String, ValueId>,
    pre_bindings: &BTreeMap<String, ValueId>,
    scope_locals: &BTreeSet<String>,
    scoped_bindings: BTreeMap<String, ValueId>,
) {
    for (name, value_id) in scoped_bindings {
        if scope_locals.contains(&name) {
            continue;
        }
        if pre_bindings.contains_key(&name) || pre_builder_map.contains_key(&name) {
            publish_defined_binding(builder, branch_bindings, name, value_id);
        }
    }
}

fn collect_scope_local_vars(body: &[ASTNode]) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    for stmt in body {
        collect_scope_local_vars_from_stmt(stmt, &mut locals);
    }
    locals
}

fn collect_scope_local_vars_from_stmt(stmt: &ASTNode, locals: &mut BTreeSet<String>) {
    match stmt {
        ASTNode::Local { variables, .. } => {
            for name in variables {
                locals.insert(name.clone());
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_scope_local_vars_from_stmt(stmt, locals);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_scope_local_vars_from_stmt(stmt, locals);
                }
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::LoopRange { body, .. }
        | ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_scope_local_vars_from_stmt(stmt, locals);
            }
        }
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_scope_local_vars_from_stmt(stmt, locals);
            }
        }
        _ => {}
    }
}
