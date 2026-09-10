//! Source-aware expression port for the first callable GenericLoop cohort.
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
use crate::mir::ValueId;

#[derive(Debug)]
pub(super) enum CallableLoopSourceExprInputV1<'input> {
    Located {
        node: &'input ASTNode,
        source: RawInvocationSourceContextV1,
    },
    Synthetic(&'input ASTNode),
}

#[derive(Debug)]
pub(super) enum CallableLoopSourceStmtInputV1<'input> {
    Located {
        node: &'input ASTNode,
        source: RawInvocationSourceContextV1,
    },
    Synthetic(&'input ASTNode),
}

#[derive(Debug)]
pub(super) enum CallableLoopSourceBodyInputV1<'input> {
    Located {
        statements: &'input [ASTNode],
        source: RawInvocationSourceContextV1,
    },
    Synthetic(&'input [ASTNode]),
}

/// One callback-scoped source/ledger capability.  The `Rc` is borrowed from
/// the invocation owner; this type never clones or retains it.
#[derive(Debug, Clone, Copy)]
pub(super) struct CallableLoopSourceExpressionPortV1<'ledger> {
    ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
}

impl<'ledger> CallableLoopSourceExpressionPortV1<'ledger> {
    pub(super) const fn new(ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>) -> Self {
        Self { ledger }
    }

    pub(super) fn expr<'input>(
        &self,
        node: &'input ASTNode,
        source: &RawInvocationSourceContextV1,
    ) -> Result<CallableLoopSourceExprInputV1<'input>, String> {
        require_located(source)?;
        Ok(CallableLoopSourceExprInputV1::Located {
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
        Ok(CallableLoopSourceBodyInputV1::Located {
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
        Ok(CallableLoopSourceExprInputV1::Located {
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
        Ok(CallableLoopSourceBodyInputV1::Located {
            statements,
            source: child_source,
        })
    }

    fn source_of_expr<'input>(
        input: &'input CallableLoopSourceExprInputV1<'input>,
    ) -> Option<&'input RawInvocationSourceContextV1> {
        match input {
            CallableLoopSourceExprInputV1::Located { source, .. } => Some(source),
            CallableLoopSourceExprInputV1::Synthetic(_) => None,
        }
    }

    fn exact_site(
        source: Option<&RawInvocationSourceContextV1>,
    ) -> Result<crate::mir::resolved_semantics::SourceNodeSiteV1, String> {
        source
            .and_then(RawInvocationSourceContextV1::site)
            .cloned()
            .ok_or_else(|| "[freeze:contract][callable-loop/source-site-missing]".to_owned())
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
        match input {
            CallableLoopSourceExprInputV1::Located { node, .. }
            | CallableLoopSourceExprInputV1::Synthetic(node) => node,
        }
    }

    fn stmt_syntax<'input>(&self, input: &Self::StmtInput<'input>) -> &'input ASTNode
    where
        Self: 'input,
    {
        match input {
            CallableLoopSourceStmtInputV1::Located { node, .. }
            | CallableLoopSourceStmtInputV1::Synthetic(node) => node,
        }
    }

    fn body_statements<'input>(&self, input: &Self::BodyInput<'input>) -> &'input [ASTNode]
    where
        Self: 'input,
    {
        match input {
            CallableLoopSourceBodyInputV1::Located { statements, .. }
            | CallableLoopSourceBodyInputV1::Synthetic(statements) => statements,
        }
    }

    fn synthetic_expr<'input>(&self, node: &'input ASTNode) -> Self::ExprInput<'input>
    where
        Self: 'input,
    {
        CallableLoopSourceExprInputV1::Synthetic(node)
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
        match body {
            CallableLoopSourceBodyInputV1::Located { source, .. } => {
                let child_source = source
                    .body_statement_context(node, index)
                    .map_err(LoopPlanExpressionPortErrorV1::Source)?;
                Ok(CallableLoopSourceStmtInputV1::Located {
                    node,
                    source: child_source,
                })
            }
            CallableLoopSourceBodyInputV1::Synthetic(_) => {
                Ok(CallableLoopSourceStmtInputV1::Synthetic(node))
            }
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
            CallableLoopSourceStmtInputV1::Located { node, source } => {
                Ok(CallableLoopSourceExprInputV1::Located {
                    node,
                    source: source.clone(),
                })
            }
            CallableLoopSourceStmtInputV1::Synthetic(node) => {
                Ok(CallableLoopSourceExprInputV1::Synthetic(node))
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
            CallableLoopSourceExprInputV1::Located { node, source } => {
                self.child_expr_input(node, source, role)
            }
            CallableLoopSourceExprInputV1::Synthetic(node) => {
                super::control_flow::plan::expression_port::raw_child_expr(node, role)
                    .map(CallableLoopSourceExprInputV1::Synthetic)
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
            CallableLoopSourceStmtInputV1::Located { node, source } => {
                self.child_expr_input(node, source, role)
            }
            CallableLoopSourceStmtInputV1::Synthetic(node) => {
                super::control_flow::plan::expression_port::raw_child_expr(node, role)
                    .map(CallableLoopSourceExprInputV1::Synthetic)
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
            CallableLoopSourceExprInputV1::Located { node, source } => {
                self.child_body_input(node, source, role)
            }
            CallableLoopSourceExprInputV1::Synthetic(node) => {
                super::control_flow::plan::expression_port::raw_child_body(node, role)
                    .map(CallableLoopSourceBodyInputV1::Synthetic)
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
            CallableLoopSourceStmtInputV1::Located { node, source } => {
                self.child_body_input(node, source, role)
            }
            CallableLoopSourceStmtInputV1::Synthetic(node) => {
                super::control_flow::plan::expression_port::raw_child_body(node, role)
                    .map(CallableLoopSourceBodyInputV1::Synthetic)
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
        if !matches!(self.expr_syntax(input), ASTNode::MethodCall { .. }) {
            return Ok(CoreCallSourceV1::Unlocated);
        }
        let site = Self::exact_site(Self::source_of_expr(input))
            .map_err(LoopPlanExpressionPortErrorV1::Source)?;
        Ok(CoreCallSourceV1::LocatedMethodCall(
            SourceExprSiteV1::from_node(site),
        ))
    }

    fn exact_source_variable_value<'input>(
        &self,
        input: &Self::ExprInput<'input>,
    ) -> Result<Option<ValueId>, String>
    where
        Self: 'input,
    {
        if !matches!(
            self.expr_syntax(input),
            ASTNode::Variable { .. } | ASTNode::Me { .. } | ASTNode::This { .. }
        ) {
            return Ok(None);
        }
        let site = Self::exact_site(Self::source_of_expr(input))?;
        self.ledger.borrow_mut().read_variable(&site).map(Some)
    }

    fn exact_source_assignment_rebind<'input>(
        &self,
        target: &Self::ExprInput<'input>,
        value: ValueId,
    ) -> Result<bool, String>
    where
        Self: 'input,
    {
        if !matches!(self.expr_syntax(target), ASTNode::Variable { .. }) {
            return Ok(false);
        }
        let site = Self::exact_site(Self::source_of_expr(target))?;
        self.ledger.borrow_mut().rebind(&site, value)?;
        Ok(true)
    }
}

fn require_located(source: &RawInvocationSourceContextV1) -> Result<(), String> {
    source
        .site()
        .is_some()
        .then_some(())
        .ok_or_else(|| "[freeze:contract][callable-loop/source-site-missing]".to_owned())
}
