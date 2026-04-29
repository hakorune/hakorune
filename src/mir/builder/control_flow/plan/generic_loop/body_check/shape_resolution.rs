//! Compatibility entry for generic loop body validation and shape resolution.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::GenericLoopV1ShapeId;

pub(in crate::mir::builder) fn check_body_generic_v0(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
) -> Option<&'static str> {
    super::validation_v0::check_body_generic_v0(body, loop_var, loop_increment)
}

pub(in crate::mir::builder) fn check_body_generic_v1(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
    require_shape: bool,
) -> Result<Option<&'static str>, Freeze> {
    super::validation_v1::check_body_generic_v1(
        body,
        loop_var,
        loop_increment,
        condition,
        require_shape,
    )
}

pub(in crate::mir::builder) fn detect_generic_loop_v1_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> Result<Option<GenericLoopV1ShapeId>, Freeze> {
    super::shape_detection::detect_generic_loop_v1_shape(body, loop_var, loop_increment, condition)
}
