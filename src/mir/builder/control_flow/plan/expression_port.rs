//! One source-child demand port for CorePlan value normalization.
//!
//! The port delegates structural child lookup to the existing PATH0 role
//! vocabulary. It owns neither source paths nor callable-result claims.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1};

use super::CoreCallSourceV1;

mod sealed {
    pub trait Sealed {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopPlanExpressionPortErrorV1 {
    BodyIndexOutOfBounds { index: usize, len: usize },
    ExpressionRoleParentMismatch,
    ExpressionRoleHasNoSyntaxNode,
    BodyRoleParentMismatch,
    RootBodyRequestedAsChild,
    Located(crate::mir::callable_result_representation::CallableResultLegacyLocationErrorV1),
}

impl LoopPlanExpressionPortErrorV1 {
    pub(in crate::mir::builder) fn render(&self) -> String {
        format!("[loop-plan-expression-port] {self:?}")
    }
}

/// Closed, stack-scoped source input port used by the normalizer SSOT.
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
    fn stmt_syntax<'input>(&self, input: &Self::StmtInput<'input>) -> &'input ASTNode
    where
        Self: 'input;
    fn body_statements<'input>(&self, input: &Self::BodyInput<'input>) -> &'input [ASTNode]
    where
        Self: 'input;

    fn synthetic_expr<'input>(&self, node: &'input ASTNode) -> Self::ExprInput<'input>
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

    fn child_expr_from_stmt<'input>(
        &self,
        parent: &Self::StmtInput<'input>,
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

    fn child_body_from_stmt<'input>(
        &self,
        parent: &Self::StmtInput<'input>,
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

#[derive(Debug, Default)]
pub(in crate::mir::builder) struct RawLoopPlanExpressionPortV1;

impl RawLoopPlanExpressionPortV1 {
    pub(in crate::mir::builder) const fn new() -> Self {
        Self
    }

    pub(in crate::mir::builder) const fn expr<'input>(
        &self,
        node: &'input ASTNode,
    ) -> &'input ASTNode {
        node
    }
}

impl sealed::Sealed for RawLoopPlanExpressionPortV1 {}

impl LoopPlanExpressionPortV1 for RawLoopPlanExpressionPortV1 {
    type ExprInput<'input> = &'input ASTNode;
    type StmtInput<'input> = &'input ASTNode;
    type BodyInput<'input> = &'input [ASTNode];

    fn expr_syntax<'input>(&self, input: &&'input ASTNode) -> &'input ASTNode
    where
        Self: 'input,
    {
        input
    }

    fn stmt_syntax<'input>(&self, input: &&'input ASTNode) -> &'input ASTNode
    where
        Self: 'input,
    {
        input
    }

    fn body_statements<'input>(&self, input: &&'input [ASTNode]) -> &'input [ASTNode]
    where
        Self: 'input,
    {
        input
    }

    fn synthetic_expr<'input>(&self, node: &'input ASTNode) -> &'input ASTNode
    where
        Self: 'input,
    {
        node
    }

    fn body_stmt<'input>(
        &self,
        body: &&'input [ASTNode],
        index: usize,
    ) -> Result<&'input ASTNode, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        body.get(index)
            .ok_or(LoopPlanExpressionPortErrorV1::BodyIndexOutOfBounds {
                index,
                len: body.len(),
            })
    }

    fn statement_expr<'input>(
        &self,
        statement: &&'input ASTNode,
    ) -> Result<&'input ASTNode, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        // Raw parity: the pre-port normalizer consumed the sole branch node
        // directly and applied its own pure-value admission afterward.
        Ok(*statement)
    }

    fn child_expr<'input>(
        &self,
        parent: &&'input ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<&'input ASTNode, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        raw_child_expr(parent, role)
    }

    fn child_expr_from_stmt<'input>(
        &self,
        parent: &&'input ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<&'input ASTNode, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        raw_child_expr(parent, role)
    }

    fn child_body<'input>(
        &self,
        parent: &&'input ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<&'input [ASTNode], LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        raw_child_body(parent, role)
    }

    fn child_body_from_stmt<'input>(
        &self,
        parent: &&'input ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<&'input [ASTNode], LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        raw_child_body(parent, role)
    }

    fn call_source<'input>(
        &self,
        _input: &&'input ASTNode,
    ) -> Result<CoreCallSourceV1, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        Ok(CoreCallSourceV1::Unlocated)
    }
}

fn raw_child_expr(
    parent: &ASTNode,
    role: ExprChildRoleV1,
) -> Result<&ASTNode, LoopPlanExpressionPortErrorV1> {
    let resolved = role
        .resolve(parent)
        .ok_or(LoopPlanExpressionPortErrorV1::ExpressionRoleParentMismatch)?;
    match resolved.syntax() {
        ExprChildSyntaxV1::Node(node) => Ok(node),
        ExprChildSyntaxV1::SyntheticName | ExprChildSyntaxV1::Missing => {
            Err(LoopPlanExpressionPortErrorV1::ExpressionRoleHasNoSyntaxNode)
        }
    }
}

fn raw_child_body(
    parent: &ASTNode,
    role: BodyChildRoleV1,
) -> Result<&[ASTNode], LoopPlanExpressionPortErrorV1> {
    let resolved = role
        .resolve(parent)
        .ok_or(LoopPlanExpressionPortErrorV1::BodyRoleParentMismatch)?;
    resolved.statements().ok_or_else(|| {
        if matches!(role, BodyChildRoleV1::FunctionBody) {
            LoopPlanExpressionPortErrorV1::RootBodyRequestedAsChild
        } else {
            LoopPlanExpressionPortErrorV1::BodyRoleParentMismatch
        }
    })
}

mod located {
    use crate::mir::callable_result_representation::{
        LegacyBodyInputV1, LegacyExprInputV1, LegacyStmtInputV1,
        VerifiedCallableResultLegacySourceViewV1,
    };

    use super::*;

    pub(in crate::mir::builder) enum LocatedLoopPlanExprInputV1<'plan, 'syntax> {
        Located(LegacyExprInputV1<'plan>),
        Synthetic(&'syntax ASTNode),
    }

    pub(in crate::mir::builder) enum LocatedLoopPlanStmtInputV1<'plan, 'syntax> {
        Located(LegacyStmtInputV1<'plan>),
        Synthetic(&'syntax ASTNode),
    }

    pub(in crate::mir::builder) enum LocatedLoopPlanBodyInputV1<'plan, 'syntax> {
        Located(LegacyBodyInputV1<'plan>),
        Synthetic(&'syntax [ASTNode]),
    }

    #[derive(Debug)]
    pub(in crate::mir::builder) struct LocatedLoopPlanExpressionPortV1<'plan> {
        view: VerifiedCallableResultLegacySourceViewV1<'plan>,
    }

    impl<'plan> LocatedLoopPlanExpressionPortV1<'plan> {
        pub(in crate::mir::builder) const fn new(
            view: VerifiedCallableResultLegacySourceViewV1<'plan>,
        ) -> Self {
            Self { view }
        }

        pub(in crate::mir::builder) fn located_expr(
            &self,
            input: LegacyExprInputV1<'plan>,
        ) -> LocatedLoopPlanExprInputV1<'plan, 'plan> {
            LocatedLoopPlanExprInputV1::Located(input)
        }

        pub(in crate::mir::builder) fn located_stmt(
            &self,
            input: LegacyStmtInputV1<'plan>,
        ) -> LocatedLoopPlanStmtInputV1<'plan, 'plan> {
            LocatedLoopPlanStmtInputV1::Located(input)
        }

        pub(in crate::mir::builder) fn located_body(
            &self,
            input: LegacyBodyInputV1<'plan>,
        ) -> LocatedLoopPlanBodyInputV1<'plan, 'plan> {
            LocatedLoopPlanBodyInputV1::Located(input)
        }

        pub(in crate::mir::builder) fn require_exact_stmt(
            &self,
            input: &LegacyStmtInputV1<'plan>,
        ) -> Result<(), LoopPlanExpressionPortErrorV1> {
            self.view
                .require_located_stmt_carrier(input)
                .map_err(LoopPlanExpressionPortErrorV1::Located)
        }

        pub(in crate::mir::builder) fn require_exact_body(
            &self,
            input: &LegacyBodyInputV1<'plan>,
        ) -> Result<(), LoopPlanExpressionPortErrorV1> {
            self.view
                .require_located_body_carrier(input)
                .map_err(LoopPlanExpressionPortErrorV1::Located)
        }

        pub(in crate::mir::builder) fn exact_child_expr_from_stmt(
            &self,
            parent: &LegacyStmtInputV1<'plan>,
            role: ExprChildRoleV1,
        ) -> Result<LegacyExprInputV1<'plan>, LoopPlanExpressionPortErrorV1> {
            self.require_exact_stmt(parent)?;
            self.view
                .child_expr_from_stmt(parent, role)
                .map_err(LoopPlanExpressionPortErrorV1::Located)
        }

        pub(in crate::mir::builder) fn exact_child_body_from_stmt(
            &self,
            parent: &LegacyStmtInputV1<'plan>,
            role: BodyChildRoleV1,
        ) -> Result<LegacyBodyInputV1<'plan>, LoopPlanExpressionPortErrorV1> {
            self.require_exact_stmt(parent)?;
            self.view
                .child_body_from_stmt(parent, role)
                .map_err(LoopPlanExpressionPortErrorV1::Located)
        }

        pub(in crate::mir::builder) fn exact_body_stmt(
            &self,
            body: &LegacyBodyInputV1<'plan>,
            index: usize,
        ) -> Result<LegacyStmtInputV1<'plan>, LoopPlanExpressionPortErrorV1> {
            self.require_exact_body(body)?;
            self.view
                .body_stmt(body, index)
                .map_err(LoopPlanExpressionPortErrorV1::Located)
        }
    }

    impl sealed::Sealed for LocatedLoopPlanExpressionPortV1<'_> {}

    impl<'plan> LoopPlanExpressionPortV1 for LocatedLoopPlanExpressionPortV1<'plan> {
        type ExprInput<'input>
            = LocatedLoopPlanExprInputV1<'plan, 'input>
        where
            Self: 'input;
        type StmtInput<'input>
            = LocatedLoopPlanStmtInputV1<'plan, 'input>
        where
            Self: 'input;
        type BodyInput<'input>
            = LocatedLoopPlanBodyInputV1<'plan, 'input>
        where
            Self: 'input;

        fn expr_syntax<'input>(&self, input: &Self::ExprInput<'input>) -> &'input ASTNode
        where
            Self: 'input,
        {
            match input {
                LocatedLoopPlanExprInputV1::Located(input) => input.node(),
                LocatedLoopPlanExprInputV1::Synthetic(node) => node,
            }
        }

        fn stmt_syntax<'input>(&self, input: &Self::StmtInput<'input>) -> &'input ASTNode
        where
            Self: 'input,
        {
            match input {
                LocatedLoopPlanStmtInputV1::Located(input) => input.node(),
                LocatedLoopPlanStmtInputV1::Synthetic(node) => node,
            }
        }

        fn body_statements<'input>(&self, input: &Self::BodyInput<'input>) -> &'input [ASTNode]
        where
            Self: 'input,
        {
            match input {
                LocatedLoopPlanBodyInputV1::Located(input) => input.statements(),
                LocatedLoopPlanBodyInputV1::Synthetic(body) => body,
            }
        }

        fn synthetic_expr<'input>(&self, node: &'input ASTNode) -> Self::ExprInput<'input>
        where
            Self: 'input,
        {
            LocatedLoopPlanExprInputV1::Synthetic(node)
        }

        fn body_stmt<'input>(
            &self,
            body: &Self::BodyInput<'input>,
            index: usize,
        ) -> Result<Self::StmtInput<'input>, LoopPlanExpressionPortErrorV1>
        where
            Self: 'input,
        {
            match body {
                LocatedLoopPlanBodyInputV1::Located(body) => self
                    .view
                    .body_stmt(body, index)
                    .map(LocatedLoopPlanStmtInputV1::Located)
                    .map_err(LoopPlanExpressionPortErrorV1::Located),
                LocatedLoopPlanBodyInputV1::Synthetic(body) => body
                    .get(index)
                    .map(LocatedLoopPlanStmtInputV1::Synthetic)
                    .ok_or(LoopPlanExpressionPortErrorV1::BodyIndexOutOfBounds {
                        index,
                        len: body.len(),
                    }),
            }
        }

        fn statement_expr<'input>(
            &self,
            statement: &Self::StmtInput<'input>,
        ) -> Result<Self::ExprInput<'input>, LoopPlanExpressionPortErrorV1>
        where
            Self: 'input,
        {
            match statement {
                LocatedLoopPlanStmtInputV1::Located(statement) => self
                    .view
                    .statement_expression(statement)
                    .map(LocatedLoopPlanExprInputV1::Located)
                    .map_err(LoopPlanExpressionPortErrorV1::Located),
                LocatedLoopPlanStmtInputV1::Synthetic(statement) => {
                    Ok(LocatedLoopPlanExprInputV1::Synthetic(statement))
                }
            }
        }

        fn child_expr<'input>(
            &self,
            parent: &Self::ExprInput<'input>,
            role: ExprChildRoleV1,
        ) -> Result<Self::ExprInput<'input>, LoopPlanExpressionPortErrorV1>
        where
            Self: 'input,
        {
            match parent {
                LocatedLoopPlanExprInputV1::Located(parent) => self
                    .view
                    .child_expr(parent, role)
                    .map(LocatedLoopPlanExprInputV1::Located)
                    .map_err(LoopPlanExpressionPortErrorV1::Located),
                LocatedLoopPlanExprInputV1::Synthetic(parent) => {
                    raw_child_expr(parent, role).map(LocatedLoopPlanExprInputV1::Synthetic)
                }
            }
        }

        fn child_expr_from_stmt<'input>(
            &self,
            parent: &Self::StmtInput<'input>,
            role: ExprChildRoleV1,
        ) -> Result<Self::ExprInput<'input>, LoopPlanExpressionPortErrorV1>
        where
            Self: 'input,
        {
            match parent {
                LocatedLoopPlanStmtInputV1::Located(parent) => self
                    .view
                    .child_expr_from_stmt(parent, role)
                    .map(LocatedLoopPlanExprInputV1::Located)
                    .map_err(LoopPlanExpressionPortErrorV1::Located),
                LocatedLoopPlanStmtInputV1::Synthetic(parent) => {
                    raw_child_expr(parent, role).map(LocatedLoopPlanExprInputV1::Synthetic)
                }
            }
        }

        fn child_body<'input>(
            &self,
            parent: &Self::ExprInput<'input>,
            role: BodyChildRoleV1,
        ) -> Result<Self::BodyInput<'input>, LoopPlanExpressionPortErrorV1>
        where
            Self: 'input,
        {
            match parent {
                LocatedLoopPlanExprInputV1::Located(parent) => self
                    .view
                    .child_body(parent, role)
                    .map(LocatedLoopPlanBodyInputV1::Located)
                    .map_err(LoopPlanExpressionPortErrorV1::Located),
                LocatedLoopPlanExprInputV1::Synthetic(parent) => {
                    raw_child_body(parent, role).map(LocatedLoopPlanBodyInputV1::Synthetic)
                }
            }
        }

        fn child_body_from_stmt<'input>(
            &self,
            parent: &Self::StmtInput<'input>,
            role: BodyChildRoleV1,
        ) -> Result<Self::BodyInput<'input>, LoopPlanExpressionPortErrorV1>
        where
            Self: 'input,
        {
            match parent {
                LocatedLoopPlanStmtInputV1::Located(parent) => self
                    .view
                    .child_body_from_stmt(parent, role)
                    .map(LocatedLoopPlanBodyInputV1::Located)
                    .map_err(LoopPlanExpressionPortErrorV1::Located),
                LocatedLoopPlanStmtInputV1::Synthetic(parent) => {
                    raw_child_body(parent, role).map(LocatedLoopPlanBodyInputV1::Synthetic)
                }
            }
        }

        fn call_source<'input>(
            &self,
            input: &Self::ExprInput<'input>,
        ) -> Result<CoreCallSourceV1, LoopPlanExpressionPortErrorV1>
        where
            Self: 'input,
        {
            let LocatedLoopPlanExprInputV1::Located(input) = input else {
                return Ok(CoreCallSourceV1::Unlocated);
            };
            if !matches!(input.node(), ASTNode::MethodCall { .. }) {
                return Ok(CoreCallSourceV1::Unlocated);
            }
            self.view
                .require_expr_carrier(input)
                .map_err(LoopPlanExpressionPortErrorV1::Located)?;
            let (_, site) = input
                .activation_site()
                .map_err(LoopPlanExpressionPortErrorV1::Located)?;
            Ok(CoreCallSourceV1::LocatedMethodCall(site.clone()))
        }
    }
}

pub(in crate::mir::builder) use located::LocatedLoopPlanExpressionPortV1;
