//! Exact source-child inputs and PHI preparation for checked Loops.
//!
//! The port keeps the existing AST child-role vocabulary while carrying the
//! already-issued invocation source context beside every input.  It borrows
//! the callable ledger for one adapter callback and never owns a second
//! source or physical receipt.

use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::sealed::Sealed;
use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, LoopPlanExpressionPortErrorV1, LoopPlanExpressionPortV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::resolved_semantics::{
    BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1, SourceExprSiteV1,
};

#[derive(Debug, Clone)]
pub(super) struct CallableLoopSourceExprInputV1<'input> {
    node: &'input ASTNode,
    source: RawInvocationSourceContextV1,
}

#[derive(Debug, Clone)]
pub(super) struct CallableLoopSourceStmtInputV1<'input> {
    node: &'input ASTNode,
    source: RawInvocationSourceContextV1,
}

#[derive(Debug, Clone)]
pub(super) struct CallableLoopSourceBodyInputV1<'input> {
    statements: &'input [ASTNode],
    source: RawInvocationSourceContextV1,
}

/// One callback-scoped source/ledger capability.  The `Rc` is borrowed from
/// the invocation owner; this type never clones or retains it.
#[derive(Debug, Clone, Copy)]
pub(super) struct CallableLoopSourceExpressionPortV1<'ledger> {
    ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
}

impl<'ledger> CallableLoopSourceExpressionPortV1<'ledger> {
    pub(super) fn prepare_checked_phis_v2(
        &self,
        session: &super::normal_callable_loop_source_facts::SourceLoopPhiSessionV1,
        builder: &mut super::MirBuilder,
        product: Rc<crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2>,
        header: crate::mir::BasicBlockId,
        after: crate::mir::BasicBlockId,
    ) -> Result<(), String> {
        session.prepare_checked(builder, product, &self.ledger.borrow(), header, after)
    }

    pub(super) const fn new(ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>) -> Self {
        Self { ledger }
    }

    pub(super) fn expr<'input>(
        &self,
        node: &'input ASTNode,
        source: &RawInvocationSourceContextV1,
    ) -> Result<CallableLoopSourceExprInputV1<'input>, String> {
        require_located(source)?;
        Ok(CallableLoopSourceExprInputV1 {
            node,
            source: source.clone(),
        })
    }

    pub(super) fn body<'input>(
        &self,
        statements: &'input [ASTNode],
        source: &RawInvocationSourceContextV1,
    ) -> Result<CallableLoopSourceBodyInputV1<'input>, String> {
        require_located(source)?;
        Ok(CallableLoopSourceBodyInputV1 {
            statements,
            source: source.clone(),
        })
    }

    fn child_expr_input<'input>(
        &self,
        node: &'input ASTNode,
        source: &RawInvocationSourceContextV1,
        role: ExprChildRoleV1,
    ) -> Result<CallableLoopSourceExprInputV1<'input>, LoopPlanExpressionPortErrorV1> {
        let resolved = role
            .resolve(node)
            .ok_or(LoopPlanExpressionPortErrorV1::ExpressionRoleParentMismatch)?;
        let ExprChildSyntaxV1::Node(child) = resolved.syntax() else {
            return Err(LoopPlanExpressionPortErrorV1::ExpressionRoleHasNoSyntaxNode);
        };
        let child_source = source
            .child_expression(node, role)
            .map_err(LoopPlanExpressionPortErrorV1::Source)?;
        Ok(CallableLoopSourceExprInputV1 {
            node: child,
            source: child_source,
        })
    }

    fn child_body_input<'input>(
        &self,
        node: &'input ASTNode,
        source: &RawInvocationSourceContextV1,
        role: BodyChildRoleV1,
    ) -> Result<CallableLoopSourceBodyInputV1<'input>, LoopPlanExpressionPortErrorV1> {
        let resolved = role
            .resolve(node)
            .ok_or(LoopPlanExpressionPortErrorV1::BodyRoleParentMismatch)?;
        let statements = resolved
            .statements()
            .ok_or(LoopPlanExpressionPortErrorV1::BodyRoleParentMismatch)?;
        let child_source = source
            .child_body(node, role)
            .map_err(LoopPlanExpressionPortErrorV1::Source)?;
        Ok(CallableLoopSourceBodyInputV1 {
            statements,
            source: child_source,
        })
    }
}

impl Sealed for CallableLoopSourceExpressionPortV1<'_> {}

impl LoopPlanExpressionPortV1 for CallableLoopSourceExpressionPortV1<'_> {
    type ExprInput<'input>
        = CallableLoopSourceExprInputV1<'input>
    where
        Self: 'input;
    type StmtInput<'input>
        = CallableLoopSourceStmtInputV1<'input>
    where
        Self: 'input;
    type BodyInput<'input>
        = CallableLoopSourceBodyInputV1<'input>
    where
        Self: 'input;

    fn expr_syntax<'input>(&self, input: &Self::ExprInput<'input>) -> &'input ASTNode
    where
        Self: 'input,
    {
        input.node
    }

    fn body_statements<'input>(&self, input: &Self::BodyInput<'input>) -> &'input [ASTNode]
    where
        Self: 'input,
    {
        input.statements
    }

    fn body_stmt<'input>(
        &self,
        body: &Self::BodyInput<'input>,
        index: usize,
    ) -> Result<Self::StmtInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        let statements = self.body_statements(body);
        let node =
            statements
                .get(index)
                .ok_or(LoopPlanExpressionPortErrorV1::BodyIndexOutOfBounds {
                    index,
                    len: statements.len(),
                })?;
        let source = body
            .source
            .body_statement_context(node, index)
            .map_err(LoopPlanExpressionPortErrorV1::Source)?;
        Ok(CallableLoopSourceStmtInputV1 { node, source })
    }

    fn statement_expr<'input>(
        &self,
        statement: &Self::StmtInput<'input>,
    ) -> Result<Self::ExprInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        Ok(CallableLoopSourceExprInputV1 {
            node: statement.node,
            source: statement.source.clone(),
        })
    }

    fn child_expr<'input>(
        &self,
        parent: &Self::ExprInput<'input>,
        role: ExprChildRoleV1,
    ) -> Result<Self::ExprInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        self.child_expr_input(parent.node, &parent.source, role)
    }

    fn child_body<'input>(
        &self,
        parent: &Self::ExprInput<'input>,
        role: BodyChildRoleV1,
    ) -> Result<Self::BodyInput<'input>, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        self.child_body_input(parent.node, &parent.source, role)
    }

    fn call_source<'input>(
        &self,
        input: &Self::ExprInput<'input>,
    ) -> Result<CoreCallSourceV1, LoopPlanExpressionPortErrorV1>
    where
        Self: 'input,
    {
        if !matches!(self.expr_syntax(input), ASTNode::MethodCall { .. }) {
            return Ok(CoreCallSourceV1::Unlocated);
        }
        let site = input.source.site().cloned().ok_or_else(|| {
            LoopPlanExpressionPortErrorV1::Source(
                "[freeze:contract][callable-loop/source-site-missing]".to_owned(),
            )
        })?;
        Ok(CoreCallSourceV1::LocatedMethodCall(
            SourceExprSiteV1::from_node(site),
        ))
    }
}

fn require_located(source: &RawInvocationSourceContextV1) -> Result<(), String> {
    source
        .site()
        .is_some()
        .then_some(())
        .ok_or_else(|| "[freeze:contract][callable-loop/source-site-missing]".to_owned())
}
