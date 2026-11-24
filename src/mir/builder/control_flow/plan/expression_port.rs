//! Source-child lookup for checked Loop preflight.
//!
//! The port delegates structural child lookup to the existing PATH0 role
//! vocabulary. It owns neither source paths nor callable-result claims.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};

use super::CoreCallSourceV1;

pub(in crate::mir::builder) mod sealed {
    pub(in crate::mir::builder) trait Sealed {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopPlanExpressionPortErrorV1 {
    Source(String),
    BodyIndexOutOfBounds { index: usize, len: usize },
    ExpressionRoleParentMismatch,
    ExpressionRoleHasNoSyntaxNode,
    BodyRoleParentMismatch,
}

impl LoopPlanExpressionPortErrorV1 {
    pub(in crate::mir::builder) fn render(&self) -> String {
        format!("[loop-plan-expression-port] {self:?}")
    }
}

/// Closed, stack-scoped source input port used by checked Loop preflight.
pub(in crate::mir::builder) trait LoopPlanExpressionPortV1:
    sealed::Sealed
{
    type ExprInput<'input>
    where
        Self: 'input;
    type StmtInput<'input>
    where
        Self: 'input;
    type BodyInput<'input>
    where
        Self: 'input;

    fn expr_syntax<'input>(&self, input: &Self::ExprInput<'input>) -> &'input ASTNode
    where
        Self: 'input;
    fn body_statements<'input>(&self, input: &Self::BodyInput<'input>) -> &'input [ASTNode]
    where
        Self: 'input;

    fn body_stmt<'input>(
        &self,
        body: &Self::BodyInput<'input>,
        index: usize,
    ) -> Result<Self::StmtInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input;

    fn statement_expr<'input>(
        &self,
        statement: &Self::StmtInput<'input>,
    ) -> Result<Self::ExprInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input;

    fn child_expr<'input>(
        &self,
        parent: &Self::ExprInput<'input>,
        role: ExprChildRoleV1,
    ) -> Result<Self::ExprInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input;

    fn child_body<'input>(
        &self,
        parent: &Self::ExprInput<'input>,
        role: BodyChildRoleV1,
    ) -> Result<Self::BodyInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input;

    fn call_source<'input>(
        &self,
        input: &Self::ExprInput<'input>,
    ) -> Result<CoreCallSourceV1, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input;
}
