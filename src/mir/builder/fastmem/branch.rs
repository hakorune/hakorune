//! FastMemory branch lowering.
//!
//! This module owns the narrow CFG shape allowed inside `fastmem` regions:
//! an `if/else` split whose condition is a region-local `OwnerEq` MemOp.

use crate::ast::{ASTNode, Span};
use crate::mir::builder::MirBuilder;
use crate::mir::function::FastMemBranchConditionProofKind;
use crate::mir::instruction::{FastMemRegionId, MemOpKind};
use crate::mir::ValueId;

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
    let condition_for_if = condition.clone();
    let condition_value = lower_fastmem_branch_condition(builder, region, condition)?;
    builder.add_fastmem_branch_condition_fact(
        region,
        condition_value,
        FastMemBranchConditionProofKind::SourceAssumeOwnerEq,
        true,
    )?;

    let then_node = ASTNode::Program {
        statements: then_body,
        span: Span::unknown(),
    };
    let else_node = ASTNode::Program {
        statements: else_body,
        span: Span::unknown(),
    };
    builder.lower_if_form(condition_for_if, then_node, Some(else_node))
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
                crate::mir::MirInstruction::MemOp {
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
