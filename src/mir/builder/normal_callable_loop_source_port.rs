//! Source-aware expression port for the first callable GenericLoop cohort.
//!
//! The port keeps the existing AST child-role vocabulary while carrying the
//! already-issued invocation source context beside every input.  It borrows
//! the callable ledger for one adapter callback and never owns a second
//! source or physical receipt.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::expression_port::sealed::Sealed;
use crate::mir::builder::control_flow::plan::expression_port::{
    ExactSourceDeclaredInstanceCallV1, ExactSourceMethodCallV1,
};
use crate::mir::builder::control_flow::plan::{
    CoreCallSourceV1, LoopPlanExpressionPortErrorV1, LoopPlanExpressionPortV1,
};
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::builder::stmts::{CompletedLocalBindingV1, CompletedLocalStatementV1};
use crate::mir::normal_callable_semantic_package::DeclaredInstanceCallLocatorScopeV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1, OwnedExprSiteV1,
    SourceExprSiteV1,
};
use crate::mir::ValueId;

#[derive(Debug, Clone)]
pub(super) enum CallableLoopSourceExprInputV1<'input> {
    Located {
        node: &'input ASTNode,
        source: RawInvocationSourceContextV1,
    },
    Synthetic(&'input ASTNode),
}

#[derive(Debug, Clone)]
pub(super) enum CallableLoopSourceStmtInputV1<'input> {
    Located {
        node: &'input ASTNode,
        source: RawInvocationSourceContextV1,
    },
    Synthetic(&'input ASTNode),
}

#[derive(Debug, Clone)]
pub(super) enum CallableLoopSourceBodyInputV1<'input> {
    Located {
        statements: &'input [ASTNode],
        source: RawInvocationSourceContextV1,
    },
    Synthetic(&'input [ASTNode]),
}

/// One callback-scoped source/ledger capability.  The `Rc` is borrowed from
/// the invocation owner; this type never clones or retains it.  The optional
/// declared-instance locator is the same package-owned scope the body lane
/// uses — the port only re-lends it, never owns a second locator.
#[derive(Debug, Clone, Copy)]
pub(super) struct CallableLoopSourceExpressionPortV1<'ledger> {
    ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
    declared_instance_locator: Option<DeclaredInstanceCallLocatorScopeV1<'ledger>>,
}

impl<'ledger> CallableLoopSourceExpressionPortV1<'ledger> {
    pub(super) const fn new(
        ledger: &'ledger Rc<RefCell<CallableSemanticLoweringState>>,
        declared_instance_locator: Option<DeclaredInstanceCallLocatorScopeV1<'ledger>>,
    ) -> Self {
        Self {
            ledger,
            declared_instance_locator,
        }
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

    /// Publishes a loop-carrier physical value into the callable ledger so
    /// source reads observe the same reaching definition the phi spine emits.
    /// `name` is the carrier name the AST carrier collection already produced;
    /// the ledger resolves it through resolver-owned binding records instead
    /// of a second name authority.
    pub(super) fn publish_loop_carrier_value(
        &self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        self.ledger
            .borrow_mut()
            .publish_source_loop_final_value_named(name, value)
    }

    /// Captures the ledger `values` projection for the speculative
    /// branch-then-condition transaction the shared `if` state cores run.
    /// The port forwards to the ledger so `values` stays the sole physical
    /// authority; no second store is snapshot here.
    pub(super) fn source_values_snapshot(&self) -> BTreeMap<BindingRefV1, ValueId> {
        self.ledger.borrow().source_values_snapshot()
    }

    /// Restores the `values` projection captured by `source_values_snapshot`.
    /// Branch-side rebinds roll back; consumption receipts stay monotone.
    pub(super) fn restore_source_values(&self, snapshot: BTreeMap<BindingRefV1, ValueId>) {
        self.ledger.borrow_mut().restore_source_values(snapshot);
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
        _physical_bindings: &BTreeMap<String, ValueId>,
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
        let source_value = self.ledger.borrow_mut().read_variable(&site)?;
        Ok(Some(source_value))
    }

    fn exact_source_statement_call<'input>(
        &self,
        input: &Self::ExprInput<'input>,
        method: &str,
        arity: u32,
    ) -> Result<
        Option<
            crate::mir::builder::control_flow::plan::expression_port::ExactSourceStatementCallV1,
        >,
        String,
    >
    where
        Self: 'input,
    {
        let site = Self::exact_site(Self::source_of_expr(input))?;
        Ok(self.ledger.borrow_mut().take_source_array_push(&SourceExprSiteV1::from_node(site), method, arity)?
            .map(|(receiver, emission)| crate::mir::builder::control_flow::plan::expression_port::ExactSourceStatementCallV1::ArrayPush { receiver, emission }))
    }

    fn exact_source_method_call<'input>(
        &self,
        input: &Self::ExprInput<'input>,
        method: &str,
        arity: u32,
    ) -> Result<Option<ExactSourceMethodCallV1>, String>
    where
        Self: 'input,
    {
        if !matches!(self.expr_syntax(input), ASTNode::MethodCall { .. }) {
            return Ok(None);
        }
        let site = Self::exact_site(Self::source_of_expr(input))?;
        self.ledger.borrow_mut().take_source_core_method_call(
            &SourceExprSiteV1::from_node(site),
            method,
            arity,
        )
    }

    fn exact_source_declared_instance_call_v1<'input>(
        &self,
        input: &Self::ExprInput<'input>,
        method: &str,
        arity: u32,
    ) -> Result<Option<ExactSourceDeclaredInstanceCallV1>, String>
    where
        Self: 'input,
    {
        let Some(locator) = self.declared_instance_locator else {
            return Ok(None);
        };
        if !matches!(self.expr_syntax(input), ASTNode::MethodCall { .. }) {
            return Ok(None);
        }
        let site = Self::exact_site(Self::source_of_expr(input))?;
        let expected_site = OwnedExprSiteV1::new(
            self.ledger.borrow().owner(),
            SourceExprSiteV1::from_node(site),
        );
        locator
            .take_exact_relation(&expected_site, |relation| {
                if relation.target_key().name() != method || relation.target_key().arity() != arity
                {
                    return Err(
                        "[freeze:contract][declared-instance/locator/key-mismatch]".to_owned()
                    );
                }
                self.ledger
                    .borrow_mut()
                    .take_exact_receiver_value(
                        expected_site.owner(),
                        relation.receiver_site().node(),
                        relation.receiver_binding(),
                    )
                    .map_err(|error| error.to_string())
                    .map(|receiver| {
                        ExactSourceDeclaredInstanceCallV1::new(
                            relation.target_key().clone(),
                            receiver,
                        )
                    })
            })
            .map_err(|error| format!("[freeze:contract][declared-instance/locator/{error:?}]"))
            .map(Some)
    }

    fn exact_source_receiver_value<'input>(
        &self,
        input: &Self::ExprInput<'input>,
    ) -> Result<Option<ValueId>, String>
    where
        Self: 'input,
    {
        if !matches!(
            self.expr_syntax(input),
            ASTNode::Me { .. } | ASTNode::This { .. }
        ) {
            return Ok(None);
        }
        let site = Self::exact_site(Self::source_of_expr(input))?;
        if self.ledger.borrow().source_read_binding(&site).is_err() {
            // Unregistered receivers (e.g. `this` inside a static box)
            // keep their existing resolution path; there is no site to
            // consume.
            return Ok(None);
        }
        let receiver_value = self.ledger.borrow_mut().read_variable(&site)?;
        Ok(Some(receiver_value))
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

    fn exact_source_local_completion<'input>(
        &self,
        statement: &Self::StmtInput<'input>,
        values: &[ValueId],
    ) -> Result<bool, String>
    where
        Self: 'input,
    {
        let ASTNode::Local { variables, .. } = self.stmt_syntax(statement) else {
            return Ok(false);
        };
        if variables.len() != values.len() {
            return Err("[freeze:contract][callable-loop/local-completion-shape]".to_owned());
        }
        let source = match statement {
            CallableLoopSourceStmtInputV1::Located { source, .. } => Some(source),
            CallableLoopSourceStmtInputV1::Synthetic(_) => None,
        };
        let site = Self::exact_site(source)?;
        let result = values
            .last()
            .copied()
            .ok_or_else(|| "[freeze:contract][callable-loop/local-completion-empty]".to_owned())?;
        let bindings = values
            .iter()
            .enumerate()
            .map(|(ordinal, value)| {
                let ordinal = u32::try_from(ordinal).map_err(|_| {
                    "[freeze:contract][callable-loop/local-completion-ordinal-overflow]".to_owned()
                })?;
                Ok(CompletedLocalBindingV1::new(ordinal, *value, *value))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let completed = CompletedLocalStatementV1::from_parts(result, bindings);
        self.ledger
            .borrow_mut()
            .record_completed_local(&site, &completed)?;
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
