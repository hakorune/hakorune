//! Carrier variable collection helpers (analysis-only).

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::recipes::loop_cond_continue_only::{
    ContinueOnlyRecipe, ContinueOnlyStmtRecipe,
};
use crate::mir::builder::control_flow::recipes::loop_cond_continue_with_return::{
    ContinueWithReturnItem, ContinueWithReturnRecipe,
};
use crate::mir::builder::MirBuilder;
use std::collections::{BTreeMap, BTreeSet};

/// Carrier variable sets (SSOT entry point).
///
/// vars は collect_carrier_vars の返却順序を保持する（順序不変）。
/// 後続で BTreeSet が必要なら呼び出し側で変換する。
pub struct CarrierSets {
    pub vars: Vec<String>,
}

/// Collect carrier vars from AST body (wrapper for SSOT entry).
pub(in crate::mir::builder) fn collect_from_body(body: &[ASTNode]) -> CarrierSets {
    CarrierSets {
        vars: collect_carrier_vars(body),
    }
}

/// Collect outer carrier vars from AST body (SSOT entry).
///
/// "outer" = builder.variable_ctx.variable_map に既に存在する変数のみ収集。
pub(in crate::mir::builder) fn collect_outer_from_body(
    builder: &MirBuilder,
    body: &[ASTNode],
) -> CarrierSets {
    CarrierSets {
        vars: collect_outer_carrier_vars_impl(builder, body),
    }
}

fn collect_outer_carrier_vars_impl(builder: &MirBuilder, body: &[ASTNode]) -> Vec<String> {
    let mut carriers = BTreeMap::<String, ()>::new();

    fn scan_stmt(builder: &MirBuilder, carriers: &mut BTreeMap<String, ()>, stmt: &ASTNode) {
        match stmt {
            ASTNode::Program { statements, .. } => {
                for stmt in statements {
                    scan_stmt(builder, carriers, stmt);
                }
            }
            ASTNode::Assignment { target, .. } => {
                let ASTNode::Variable { name, .. } = target.as_ref() else {
                    return;
                };
                if builder.variable_ctx.variable_map.contains_key(name) {
                    carriers.insert(name.clone(), ());
                }
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                for stmt in then_body {
                    scan_stmt(builder, carriers, stmt);
                }
                if let Some(else_body) = else_body {
                    for stmt in else_body {
                        scan_stmt(builder, carriers, stmt);
                    }
                }
            }
            ASTNode::Loop { body, .. }
            | ASTNode::While { body, .. }
            | ASTNode::ForRange { body, .. } => {
                for stmt in body {
                    scan_stmt(builder, carriers, stmt);
                }
            }
            ASTNode::ScopeBox { body, .. } => {
                for stmt in body {
                    scan_stmt(builder, carriers, stmt);
                }
            }
            _ => {}
        }
    }

    for stmt in body {
        scan_stmt(builder, &mut carriers, stmt);
    }
    carriers.keys().cloned().collect()
}

pub(in crate::mir::builder) fn collect_carrier_vars(body: &[ASTNode]) -> Vec<String> {
    let mut loop_locals = BTreeMap::<String, ()>::new();
    for stmt in body {
        collect_local_vars_from_stmt(stmt, &mut loop_locals);
    }

    let mut carriers = BTreeMap::<String, ()>::new();
    for stmt in body {
        collect_carrier_vars_from_stmt(stmt, &loop_locals, &mut carriers);
    }
    carriers.keys().cloned().collect()
}

fn collect_carrier_vars_from_stmt(
    stmt: &ASTNode,
    loop_locals: &BTreeMap<String, ()>,
    carriers: &mut BTreeMap<String, ()>,
) {
    match stmt {
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
        }
        ASTNode::Assignment { target, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return;
            };
            if !loop_locals.contains_key(name) {
                carriers.insert(name.clone(), ());
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. } => {
            for stmt in body {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
                }
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_carrier_vars_from_stmt(stmt, loop_locals, carriers);
            }
        }
        _ => {}
    }
}

fn collect_local_vars_from_stmt(stmt: &ASTNode, locals: &mut BTreeMap<String, ()>) {
    match stmt {
        ASTNode::Program { statements, .. } => {
            for stmt in statements {
                collect_local_vars_from_stmt(stmt, locals);
            }
        }
        ASTNode::Local { variables, .. } => {
            for name in variables {
                locals.insert(name.clone(), ());
            }
        }
        ASTNode::Loop { body, .. }
        | ASTNode::While { body, .. }
        | ASTNode::ForRange { body, .. } => {
            for stmt in body {
                collect_local_vars_from_stmt(stmt, locals);
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_local_vars_from_stmt(stmt, locals);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_local_vars_from_stmt(stmt, locals);
                }
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_local_vars_from_stmt(stmt, locals);
            }
        }
        _ => {}
    }
}

// ============================================================================
// ContinueWithReturn recipe carrier collection (SSOT entry)
// ============================================================================

/// Collect carrier vars from ContinueWithReturn recipe (SSOT entry).
pub(in crate::mir::builder) fn collect_from_recipe_continue_with_return(
    recipe: &ContinueWithReturnRecipe,
) -> CarrierSets {
    let vars_set = collect_carrier_vars_from_recipe_cwr(recipe);
    CarrierSets {
        vars: vars_set.into_iter().collect(),
    }
}

fn collect_carrier_vars_from_recipe_cwr(recipe: &ContinueWithReturnRecipe) -> BTreeSet<String> {
    let mut locals = BTreeSet::new();
    let body_view = BodyView::Recipe(&recipe.body);
    collect_local_vars_from_items_cwr(&body_view, &recipe.items, &mut locals);

    let mut carriers = BTreeSet::new();
    collect_carrier_vars_from_items_cwr(&body_view, &recipe.items, &locals, &mut carriers);
    carriers
}

fn collect_local_vars_from_items_cwr(
    body: &BodyView<'_>,
    items: &[ContinueWithReturnItem],
    locals: &mut BTreeSet<String>,
) {
    for item in items {
        collect_local_vars_from_item_cwr(body, item, locals);
    }
}

fn collect_local_vars_from_item_cwr(
    body: &BodyView<'_>,
    stmt: &ContinueWithReturnItem,
    locals: &mut BTreeSet<String>,
) {
    match stmt {
        ContinueWithReturnItem::Stmt(node) | ContinueWithReturnItem::IfAny(node) => {
            let Some(stmt) = body.get_stmt(*node) else {
                return;
            };
            collect_local_vars_from_ast_cwr(stmt, locals);
        }
        ContinueWithReturnItem::ContinueIf {
            if_stmt,
            prelude_span,
            prelude_items,
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*prelude_span) else {
                return;
            };
            let prelude_view = BodyView::Slice(prelude_body);
            collect_local_vars_from_items_cwr(&prelude_view, prelude_items, locals);
        }
        ContinueWithReturnItem::HeteroReturnIf { if_stmt } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If {
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return;
            };
            if let Some(node) = then_body.first() {
                collect_local_vars_from_ast_cwr(node, locals);
            }
            if let Some(else_body) = else_body {
                for node in else_body {
                    collect_local_vars_from_ast_cwr(node, locals);
                }
            }
        }
    }
}

fn collect_local_vars_from_ast_cwr(stmt: &ASTNode, locals: &mut BTreeSet<String>) {
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
                collect_local_vars_from_ast_cwr(stmt, locals);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_local_vars_from_ast_cwr(stmt, locals);
                }
            }
        }
        _ => {}
    }
}

fn collect_carrier_vars_from_items_cwr(
    body: &BodyView<'_>,
    items: &[ContinueWithReturnItem],
    locals: &BTreeSet<String>,
    carriers: &mut BTreeSet<String>,
) {
    for item in items {
        collect_carrier_vars_from_item_cwr(body, item, locals, carriers);
    }
}

fn collect_carrier_vars_from_item_cwr(
    body: &BodyView<'_>,
    stmt: &ContinueWithReturnItem,
    locals: &BTreeSet<String>,
    carriers: &mut BTreeSet<String>,
) {
    match stmt {
        ContinueWithReturnItem::Stmt(node) | ContinueWithReturnItem::IfAny(node) => {
            let Some(stmt) = body.get_stmt(*node) else {
                return;
            };
            collect_carrier_vars_from_ast_cwr(stmt, locals, carriers);
        }
        ContinueWithReturnItem::ContinueIf {
            if_stmt,
            prelude_span,
            prelude_items,
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*prelude_span) else {
                return;
            };
            let prelude_view = BodyView::Slice(prelude_body);
            collect_carrier_vars_from_items_cwr(&prelude_view, prelude_items, locals, carriers);
        }
        ContinueWithReturnItem::HeteroReturnIf { if_stmt } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If {
                then_body,
                else_body,
                ..
            } = stmt
            else {
                return;
            };
            if let Some(node) = then_body.first() {
                collect_carrier_vars_from_ast_cwr(node, locals, carriers);
            }
            if let Some(else_body) = else_body {
                for node in else_body {
                    collect_carrier_vars_from_ast_cwr(node, locals, carriers);
                }
            }
        }
    }
}

fn collect_carrier_vars_from_ast_cwr(
    stmt: &ASTNode,
    locals: &BTreeSet<String>,
    carriers: &mut BTreeSet<String>,
) {
    match stmt {
        ASTNode::Assignment { target, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return;
            };
            if !locals.contains(name) {
                carriers.insert(name.clone());
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_carrier_vars_from_ast_cwr(stmt, locals, carriers);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_carrier_vars_from_ast_cwr(stmt, locals, carriers);
                }
            }
        }
        _ => {}
    }
}

// ============================================================================
// ContinueOnlyStmtRecipe carrier collection (SSOT entry, T28 実装移設)
// ============================================================================

/// Collect carrier vars from ContinueOnly recipe (SSOT entry).
pub(in crate::mir::builder) fn collect_from_recipe_continue_only(
    recipe: &ContinueOnlyRecipe,
) -> CarrierSets {
    CarrierSets {
        vars: collect_carrier_vars_from_recipe_co(recipe),
    }
}

fn collect_carrier_vars_from_recipe_co(recipe: &ContinueOnlyRecipe) -> Vec<String> {
    let mut locals = BTreeSet::new();
    let body_view = BodyView::Recipe(&recipe.body);
    for stmt in &recipe.items {
        collect_local_vars_from_recipe_co(&body_view, stmt, &mut locals);
    }

    let mut carriers = BTreeSet::new();
    for stmt in &recipe.items {
        collect_carrier_vars_from_recipe_stmt_co(&body_view, stmt, &locals, &mut carriers);
    }
    carriers.into_iter().collect()
}

fn collect_local_vars_from_recipe_co(
    body: &BodyView<'_>,
    stmt: &ContinueOnlyStmtRecipe,
    locals: &mut BTreeSet<String>,
) {
    collect_local_vars_from_item_co(body, stmt, locals);
}

fn collect_local_vars_from_items_co(
    body: &BodyView<'_>,
    items: &[ContinueOnlyStmtRecipe],
    locals: &mut BTreeSet<String>,
) {
    for stmt in items {
        collect_local_vars_from_item_co(body, stmt, locals);
    }
}

fn collect_local_vars_from_item_co(
    body: &BodyView<'_>,
    stmt: &ContinueOnlyStmtRecipe,
    locals: &mut BTreeSet<String>,
) {
    match stmt {
        ContinueOnlyStmtRecipe::Stmt(node) => {
            let Some(stmt) = body.get_stmt(*node) else {
                return;
            };
            collect_local_vars_from_stmt_co(stmt, locals);
        }
        ContinueOnlyStmtRecipe::ContinueIf {
            if_stmt,
            prelude_span,
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*prelude_span) else {
                return;
            };
            for stmt in prelude_body {
                collect_local_vars_from_stmt_co(stmt, locals);
            }
        }
        ContinueOnlyStmtRecipe::ContinueIfGroupPrelude {
            if_stmt,
            prelude_span,
            prelude_items,
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*prelude_span) else {
                return;
            };
            let prelude_view = BodyView::Slice(prelude_body);
            collect_local_vars_from_items_co(&prelude_view, prelude_items, locals);
        }
        ContinueOnlyStmtRecipe::GroupIf {
            then_body,
            else_body,
            ..
        } => {
            let then_view = BodyView::Recipe(&then_body.body);
            collect_local_vars_from_items_co(&then_view, &then_body.items, locals);
            if let Some(else_body) = else_body {
                let else_view = BodyView::Recipe(&else_body.body);
                collect_local_vars_from_items_co(&else_view, &else_body.items, locals);
            }
        }
        ContinueOnlyStmtRecipe::ContinueIfNestedLoop {
            inner_loop_prelude_span,
            inner_loop_prelude_items,
            inner_loop_body,
            inner_loop_stmt,
            inner_loop_postlude_span,
            inner_loop_postlude_items,
            if_stmt,
            ..
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*inner_loop_prelude_span) else {
                return;
            };
            let prelude_view = BodyView::Slice(prelude_body);
            collect_local_vars_from_items_co(&prelude_view, inner_loop_prelude_items, locals);
            let inner_view = BodyView::Recipe(inner_loop_body);
            let Some(loop_stmt) = inner_view.get_stmt(*inner_loop_stmt) else {
                return;
            };
            collect_local_vars_from_stmt_co(loop_stmt, locals);
            let Some(postlude_body) = then_view.get_span(*inner_loop_postlude_span) else {
                return;
            };
            let postlude_view = BodyView::Slice(postlude_body);
            collect_local_vars_from_items_co(&postlude_view, inner_loop_postlude_items, locals);
        }
    }
}

fn collect_carrier_vars_from_recipe_stmt_co(
    body: &BodyView<'_>,
    stmt: &ContinueOnlyStmtRecipe,
    locals: &BTreeSet<String>,
    carriers: &mut BTreeSet<String>,
) {
    collect_carrier_vars_from_item_co(body, stmt, locals, carriers);
}

fn collect_carrier_vars_from_items_co(
    body: &BodyView<'_>,
    items: &[ContinueOnlyStmtRecipe],
    locals: &BTreeSet<String>,
    carriers: &mut BTreeSet<String>,
) {
    for stmt in items {
        collect_carrier_vars_from_item_co(body, stmt, locals, carriers);
    }
}

fn collect_carrier_vars_from_item_co(
    body: &BodyView<'_>,
    stmt: &ContinueOnlyStmtRecipe,
    locals: &BTreeSet<String>,
    carriers: &mut BTreeSet<String>,
) {
    match stmt {
        ContinueOnlyStmtRecipe::Stmt(node) => {
            let Some(stmt) = body.get_stmt(*node) else {
                return;
            };
            collect_carrier_vars_from_stmt_co(stmt, locals, carriers);
        }
        ContinueOnlyStmtRecipe::ContinueIf {
            if_stmt,
            prelude_span,
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*prelude_span) else {
                return;
            };
            for stmt in prelude_body {
                collect_carrier_vars_from_stmt_co(stmt, locals, carriers);
            }
        }
        ContinueOnlyStmtRecipe::ContinueIfGroupPrelude {
            if_stmt,
            prelude_span,
            prelude_items,
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*prelude_span) else {
                return;
            };
            let prelude_view = BodyView::Slice(prelude_body);
            collect_carrier_vars_from_items_co(&prelude_view, prelude_items, locals, carriers);
        }
        ContinueOnlyStmtRecipe::GroupIf {
            then_body,
            else_body,
            ..
        } => {
            let then_view = BodyView::Recipe(&then_body.body);
            collect_carrier_vars_from_items_co(&then_view, &then_body.items, locals, carriers);
            if let Some(else_body) = else_body {
                let else_view = BodyView::Recipe(&else_body.body);
                collect_carrier_vars_from_items_co(&else_view, &else_body.items, locals, carriers);
            }
        }
        ContinueOnlyStmtRecipe::ContinueIfNestedLoop {
            inner_loop_prelude_span,
            inner_loop_prelude_items,
            inner_loop_body,
            inner_loop_stmt,
            inner_loop_postlude_span,
            inner_loop_postlude_items,
            if_stmt,
            ..
        } => {
            let Some(stmt) = body.get_stmt(*if_stmt) else {
                return;
            };
            let ASTNode::If { then_body, .. } = stmt else {
                return;
            };
            let then_view = BodyView::Slice(then_body);
            let Some(prelude_body) = then_view.get_span(*inner_loop_prelude_span) else {
                return;
            };
            let prelude_view = BodyView::Slice(prelude_body);
            collect_carrier_vars_from_items_co(
                &prelude_view,
                inner_loop_prelude_items,
                locals,
                carriers,
            );
            let inner_view = BodyView::Recipe(inner_loop_body);
            let Some(loop_stmt) = inner_view.get_stmt(*inner_loop_stmt) else {
                return;
            };
            collect_carrier_vars_from_stmt_co(loop_stmt, locals, carriers);
            let Some(postlude_body) = then_view.get_span(*inner_loop_postlude_span) else {
                return;
            };
            let postlude_view = BodyView::Slice(postlude_body);
            collect_carrier_vars_from_items_co(
                &postlude_view,
                inner_loop_postlude_items,
                locals,
                carriers,
            );
        }
    }
}

fn collect_local_vars_from_stmt_co(stmt: &ASTNode, locals: &mut BTreeSet<String>) {
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
                collect_local_vars_from_stmt_co(stmt, locals);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_local_vars_from_stmt_co(stmt, locals);
                }
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_local_vars_from_stmt_co(stmt, locals);
            }
        }
        _ => {}
    }
}

fn collect_carrier_vars_from_stmt_co(
    stmt: &ASTNode,
    locals: &BTreeSet<String>,
    carriers: &mut BTreeSet<String>,
) {
    match stmt {
        ASTNode::Assignment { target, .. } => {
            let ASTNode::Variable { name, .. } = target.as_ref() else {
                return;
            };
            if !locals.contains(name) {
                carriers.insert(name.clone());
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for stmt in then_body {
                collect_carrier_vars_from_stmt_co(stmt, locals, carriers);
            }
            if let Some(else_body) = else_body {
                for stmt in else_body {
                    collect_carrier_vars_from_stmt_co(stmt, locals, carriers);
                }
            }
        }
        ASTNode::ScopeBox { body, .. } => {
            for stmt in body {
                collect_carrier_vars_from_stmt_co(stmt, locals, carriers);
            }
        }
        _ => {}
    }
}
