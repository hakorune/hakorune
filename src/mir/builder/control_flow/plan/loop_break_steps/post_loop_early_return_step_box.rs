//! PostLoopEarlyReturnStepBox (Phase 107)
//!
//! Responsibility: emulate `return i` inside the loop by emitting a post-loop
//! early-return guard. This keeps JoinIR lowering "break-only" while preserving
//! source semantics for the recognized family.

use crate::ast::{ASTNode, Span};
use crate::mir::builder::MirBuilder;

pub(crate) struct PostLoopEarlyReturnStepBox;

impl PostLoopEarlyReturnStepBox {
    pub(crate) fn maybe_emit(
        builder: &mut MirBuilder,
        plan: Option<
            &crate::mir::builder::control_flow::plan::policies::post_loop_early_return_plan::PostLoopEarlyReturnPlan,
        >,
    ) -> Result<(), String> {
        let Some(plan) = plan else { return Ok(()); };

        let ret_stmt = ASTNode::Return {
            value: Some(Box::new(plan.ret_expr.clone())),
            span: Span::unknown(),
        };

        builder.build_statement(ASTNode::If {
            condition: Box::new(plan.cond.clone()),
            then_body: vec![ret_stmt],
            else_body: None,
            span: Span::unknown(),
        })?;

        Ok(())
    }
}
