//! Join-scope helpers for branch-local variables.
//!
//! Centralizes branch-local collection and map filtering so join obligations
//! treat branch-scoped locals consistently across parts.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::recipe_tree::{RecipeBodies, RecipeBlock, RecipeItem};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn filter_branch_locals_from_maps(
    pre_if_map: &BTreeMap<String, crate::mir::ValueId>,
    then_map: &BTreeMap<String, crate::mir::ValueId>,
    else_map: &BTreeMap<String, crate::mir::ValueId>,
    branch_locals: &BTreeSet<String>,
) -> (
    BTreeMap<String, crate::mir::ValueId>,
    BTreeMap<String, crate::mir::ValueId>,
) {
    let mut then_map = then_map.clone();
    let mut else_map = else_map.clone();
    for name in branch_locals {
        if let Some(pre_val) = pre_if_map.get(name).copied() {
            then_map.insert(name.clone(), pre_val);
            else_map.insert(name.clone(), pre_val);
        } else {
            then_map.remove(name);
            else_map.remove(name);
        }
    }
    (then_map, else_map)
}

pub(super) fn collect_branch_local_vars_from_maps(
    pre_if_map: &BTreeMap<String, crate::mir::ValueId>,
    then_map: &BTreeMap<String, crate::mir::ValueId>,
    else_map: &BTreeMap<String, crate::mir::ValueId>,
) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    for name in then_map.keys() {
        if !pre_if_map.contains_key(name) && !else_map.contains_key(name) {
            locals.insert(name.clone());
        }
    }
    for name in else_map.keys() {
        if !pre_if_map.contains_key(name) && !then_map.contains_key(name) {
            locals.insert(name.clone());
        }
    }
    locals
}

pub(super) fn collect_branch_local_vars_from_body(
    then_body: &[ASTNode],
    else_body: Option<&[ASTNode]>,
) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    collect_local_vars_from_body(then_body, &mut locals);
    if let Some(body) = else_body {
        collect_local_vars_from_body(body, &mut locals);
    }
    locals
}

pub(super) fn collect_branch_local_vars_from_block_recursive(
    arena: &RecipeBodies,
    block: &RecipeBlock,
) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    let Some(body) = arena.get(block.body_id) else {
        return locals;
    };
    for item in &block.items {
        match item {
            RecipeItem::Stmt(stmt_ref) => {
                if let Some(ASTNode::Local { variables, .. }) = body.get_ref(*stmt_ref) {
                    for name in variables {
                        locals.insert(name.clone());
                    }
                }
            }
            RecipeItem::IfV2 {
                then_block,
                else_block,
                ..
            } => {
                locals.extend(collect_branch_local_vars_from_block_recursive(
                    arena, then_block,
                ));
                if let Some(else_block) = else_block.as_deref() {
                    locals.extend(collect_branch_local_vars_from_block_recursive(
                        arena, else_block,
                    ));
                }
            }
            RecipeItem::LoopV0 { body_block, .. } => {
                locals.extend(collect_branch_local_vars_from_block_recursive(
                    arena, body_block,
                ));
            }
            _ => {}
        }
    }
    locals
}

fn collect_local_vars_from_body(body: &[ASTNode], locals: &mut BTreeSet<String>) {
    for stmt in body {
        collect_local_vars_from_stmt(stmt, locals);
    }
}

fn collect_local_vars_from_stmt(stmt: &ASTNode, locals: &mut BTreeSet<String>) {
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
            collect_local_vars_from_body(then_body, locals);
            if let Some(else_body) = else_body.as_ref() {
                collect_local_vars_from_body(else_body, locals);
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. }
        | ASTNode::ScopeBox { body, .. } => {
            collect_local_vars_from_body(body, locals);
        }
        ASTNode::Program { statements, .. } => {
            collect_local_vars_from_body(statements, locals);
        }
        _ => {}
    }
}
