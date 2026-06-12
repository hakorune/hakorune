use super::CarrierSets;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::features::body_view::BodyView;
use crate::mir::builder::control_flow::recipes::loop_cond_continue_only::{
    ContinueOnlyRecipe, ContinueOnlyStmtRecipe,
};
use std::collections::BTreeSet;

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
        collect_local_vars_from_item_co(&body_view, stmt, &mut locals);
    }

    let mut carriers = BTreeSet::new();
    for stmt in &recipe.items {
        collect_carrier_vars_from_item_co(&body_view, stmt, &locals, &mut carriers);
    }
    carriers.into_iter().collect()
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

fn collect_local_vars_from_items_co(
    body: &BodyView<'_>,
    items: &[ContinueOnlyStmtRecipe],
    locals: &mut BTreeSet<String>,
) {
    for stmt in items {
        collect_local_vars_from_item_co(body, stmt, locals);
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
