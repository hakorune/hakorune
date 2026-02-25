//! Shared helper for general-if lowering with branch-specific maps.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::parts;
use crate::mir::builder::control_flow::plan::steps::lower_stmt_block;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub(in crate::mir::builder) fn lower_if_with_branch_lowerers_and_updates<LowerStmt>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    carrier_phis: &BTreeMap<String, ValueId>,
    carrier_updates: &mut BTreeMap<String, ValueId>,
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&[ASTNode]>,
    error_prefix: &str,
    lower_stmt: LowerStmt,
) -> Result<Vec<LoweredRecipe>, String>
where
    LowerStmt: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, ValueId>,
        &mut BTreeMap<String, ValueId>,
        &ASTNode,
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let cond_view = CondBlockView::from_expr(condition);

    // Allow both then/else blocks to record carrier updates without borrowing conflicts.
    let carrier_updates_cell = RefCell::new(std::mem::take(carrier_updates));
    let lower_stmt_cell = RefCell::new(lower_stmt);

    let mut lower_then = |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, ValueId>| {
        lower_stmt_block(then_body, |stmt| {
            let mut carrier_updates = carrier_updates_cell.borrow_mut();
            let mut lower_stmt = lower_stmt_cell.borrow_mut();
            lower_stmt(builder, bindings, &mut *carrier_updates, stmt)
        })
    };

    let mut lower_else_closure =
        |builder: &mut MirBuilder, bindings: &mut BTreeMap<String, ValueId>| {
            let Some(body) = else_body else {
                return Err(format!("{error_prefix}: else_body missing in lower_else"));
            };
            lower_stmt_block(body, |stmt| {
                let mut carrier_updates = carrier_updates_cell.borrow_mut();
                let mut lower_stmt = lower_stmt_cell.borrow_mut();
                lower_stmt(builder, bindings, &mut *carrier_updates, stmt)
            })
        };

    let lower_else = else_body.map(|_| {
        &mut lower_else_closure
            as &mut dyn FnMut(
                &mut MirBuilder,
                &mut BTreeMap<String, ValueId>,
            ) -> Result<Vec<LoweredRecipe>, String>
    });

    let should_update_binding = |name: &str, bindings: &BTreeMap<String, ValueId>| {
        carrier_phis.contains_key(name) || bindings.contains_key(name)
    };

    let plans = parts::entry::lower_if_join_with_branch_lowerers(
        builder,
        current_bindings,
        &cond_view,
        error_prefix,
        &mut lower_then,
        lower_else,
        &should_update_binding,
    )?;

    *carrier_updates = carrier_updates_cell.into_inner();
    Ok(plans)
}
