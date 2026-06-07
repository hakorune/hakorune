//! FastMemory branch lowering.
//!
//! This module owns the narrow CFG shape allowed inside `fastmem` regions:
//! an `if/else` split whose condition is a region-local `OwnerEq` MemOp.

use super::lower_fastmem_stmt;
use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::loop_api::LoopBuilderApi;
use crate::mir::{MirInstruction, ValueId};

pub(super) fn lower_fastmem_if(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> Result<ValueId, String> {
    let Some(else_body) = else_body else {
        return Err("[freeze:contract][fastmem/branch_cfg_requires_else]".to_string());
    };
    let mut condition_value = lower_fastmem_branch_condition(builder, region, condition)?;
    condition_value = builder.local_cond(condition_value);
    crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut condition_value)?;

    let pre_branch_bb = builder.current_block()?;
    let pre_branch_var_map = builder.variable_ctx.variable_map.clone();
    let then_block = builder.next_block_id();
    let else_block = builder.next_block_id();
    let merge_block = builder.next_block_id();

    builder.start_new_block(then_block)?;
    builder.hint_scope_enter(0);
    lower_fastmem_branch_body(builder, region, then_body)?;
    let then_exit_block = builder.current_block()?;
    let then_reaches_merge = !builder.is_current_block_terminated();
    if then_reaches_merge {
        builder.hint_scope_leave(0);
    }

    builder.variable_ctx.variable_map = pre_branch_var_map.clone();
    builder.start_new_block(else_block)?;
    builder.hint_scope_enter(0);
    lower_fastmem_branch_body(builder, region, else_body)?;
    let else_exit_block = builder.current_block()?;
    let else_reaches_merge = !builder.is_current_block_terminated();
    if else_reaches_merge {
        builder.hint_scope_leave(0);
    }

    crate::mir::builder::emission::branch::emit_conditional_edgecfg(
        builder,
        pre_branch_bb,
        condition_value,
        then_block,
        then_exit_block,
        then_reaches_merge,
        else_block,
        else_exit_block,
        else_reaches_merge,
        merge_block,
    )?;
    builder.suppress_next_entry_pin_copy();
    builder.start_new_block(merge_block)?;
    builder.variable_ctx.variable_map = pre_branch_var_map;
    crate::mir::builder::emission::constant::emit_void(builder)
}

fn lower_fastmem_branch_condition(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    condition: ASTNode,
) -> Result<ValueId, String> {
    let ASTNode::Variable { name, .. } = condition else {
        return Err(
            "[freeze:contract][fastmem/branch_cfg_requires_owner_eq_condition]".to_string(),
        );
    };
    let condition_value = builder.build_variable_access(name)?;
    ensure_fastmem_owner_eq_condition(builder, region, condition_value)?;
    Ok(condition_value)
}

fn lower_fastmem_branch_body(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    body: Vec<ASTNode>,
) -> Result<Option<ValueId>, String> {
    let mut last_value = None;
    for stmt in body {
        last_value = Some(lower_fastmem_stmt(builder, region, stmt)?);
    }
    Ok(last_value)
}

fn ensure_fastmem_owner_eq_condition(
    builder: &MirBuilder,
    region: FastMemRegionId,
    condition_value: ValueId,
) -> Result<(), String> {
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
    let is_owner_eq = function.blocks.values().any(|block| {
        block.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::MemOp {
                    region: actual_region,
                    kind: MemOpKind::OwnerEq,
                    dst: Some(dst),
                    ..
                } if *actual_region == region && *dst == condition_value
            )
        })
    });
    if is_owner_eq {
        Ok(())
    } else {
        Err("[freeze:contract][fastmem/branch_cfg_requires_owner_eq_condition]".to_string())
    }
}
