//! Step shape extraction for loop analysis

use super::scan_shapes::StepShape;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::planner::Freeze;

pub(in crate::mir::builder) fn try_extract_step_shape(
    body: &[ASTNode],
) -> Result<Option<StepShape>, Freeze> {
    let Some(last) = body.last() else {
        return Ok(None);
    };

    let ASTNode::Assignment { target, value, .. } = last else {
        return Ok(None);
    };
    let ASTNode::Variable { name: var, .. } = target.as_ref() else {
        return Ok(None);
    };

    let ASTNode::BinaryOp {
        operator,
        left,
        right,
        ..
    } = value.as_ref()
    else {
        return Ok(None);
    };
    let ASTNode::Variable { name: lhs, .. } = left.as_ref() else {
        return Ok(None);
    };
    if lhs != var {
        return Ok(None);
    }

    let ASTNode::Literal { value, .. } = right.as_ref() else {
        return Ok(None);
    };
    let LiteralValue::Integer(k) = value else {
        return Ok(None);
    };

    let k = match operator {
        BinaryOperator::Add => *k,
        BinaryOperator::Subtract => -*k,
        _ => return Ok(None),
    };

    if k != 1 && k != -1 {
        return Ok(None);
    }

    Ok(Some(StepShape::AssignAddConst {
        var: var.clone(),
        k,
    }))
}
