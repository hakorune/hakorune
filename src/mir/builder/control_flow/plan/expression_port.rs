//! One source-child demand port for CorePlan value normalization.
//!
//! The port delegates structural child lookup to the existing PATH0 role
//! vocabulary. It owns neither source paths nor callable-result claims.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, ExprChildSyntaxV1};
use crate::mir::ValueId;

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
    RootBodyRequestedAsChild,
}

mod source_method;
pub(in crate::mir::builder) use source_method::{
    ExactSourceDeclaredInstanceCallV1, ExactSourceMethodCallV1, ExactSourceStatementCallV1,
};

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

    /// Located source identity, independently of the eventual value shape.
    /// Synthetic/raw inputs carry no source loan.
    fn exact_source_expression_site<'input>(
        &self, _input: &Self::ExprInput<'input>,
    ) -> Result<Option<crate::mir::resolved_semantics::OwnedExprSiteV1>, String>
    where Self: 'input { Ok(None) }

    /// Select the existing field-index Set requirement before child descent.
    /// Physical emission independently reborrows the same original source.
    fn exact_source_borrowed_array_set<'input>(
        &self, _input: &Self::ExprInput<'input>, _method: &str, _arity: usize,
    ) -> Result<bool, String>
    where Self: 'input { Ok(false) }

    /// Retain one real variable read rather than reconstructing its provenance
    /// from the eventual ValueId. The returned read is consumed exactly once.
    fn exact_source_variable_read<'input>(
        &self, _input: &Self::ExprInput<'input>,
    ) -> Result<Option<crate::mir::builder::normal_callable_semantic_lowering_state::ExactLexicalReadV1>, String>
    where Self: 'input { Ok(None) }

    /// Optional exact source binding projection.  Raw/legacy ports return
    /// `None` and retain their existing variable-map behavior.  A source-aware
    /// port returns `Some` or an error, so the normalizer cannot fall back to
    /// a name after an exact site has been admitted.
    fn exact_source_variable_value<'input>(
        &self,
        _input: &Self::ExprInput<'input>,
        _physical_bindings: &std::collections::BTreeMap<String, ValueId>,
    ) -> Result<Option<ValueId>, String>
    where
        Self: 'input,
    {
        Ok(None)
    }

    /// Optional exact source assignment commit.  `true` means the source
    /// ledger consumed the rebind; `false` preserves raw/legacy behavior.
    fn exact_source_assignment_rebind<'input>(
        &self,
        _target: &Self::ExprInput<'input>,
        _value: ValueId,
    ) -> Result<bool, String>
    where
        Self: 'input,
    {
        Ok(false)
    }

    /// Optional exact source local completion. A source-aware port consumes
    /// the completed values at the exact statement site; raw ports retain
    /// their existing map-only lowering.
    fn exact_source_local_completion<'input>(
        &self,
        _statement: &Self::StmtInput<'input>,
        _values: &[ValueId],
    ) -> Result<bool, String>
    where
        Self: 'input,
    {
        Ok(false)
    }

    /// NoValue source calls are consumed separately from value demand.
    fn exact_source_statement_call<'input>(
        &self,
        _input: &Self::ExprInput<'input>,
        _method: &str,
        _arity: u32,
    ) -> Result<Option<ExactSourceStatementCallV1>, String>
    where
        Self: 'input,
    {
        Ok(None)
    }

    /// Consume one exact source-bound CoreMethod call before raw receiver
    /// lookup.  Raw ports return `None` and retain compatibility behavior.
    fn exact_source_method_call<'input>(
        &self,
        _input: &Self::ExprInput<'input>,
        _method: &str,
        _arity: u32,
    ) -> Result<Option<ExactSourceMethodCallV1>, String>
    where
        Self: 'input,
    {
        Ok(None)
    }

    /// Consume one package-issued declared-instance (`me.method`) locator row
    /// for this exact call site and return the canonical target plus its
    /// already-bound receiver value.  The receiver source site is consumed
    /// inside this take, so a `Some` result must not be followed by
    /// `exact_source_receiver_value` for the same site.  Unarmed ports return
    /// `None`; an armed port without a matching row reports the locator's
    /// typed error rather than falling back to dynamic dispatch.
    fn exact_source_declared_instance_call_v1<'input>(
        &self,
        _input: &Self::ExprInput<'input>,
        _method: &str,
        _arity: u32,
    ) -> Result<Option<ExactSourceDeclaredInstanceCallV1>, String>
    where
        Self: 'input,
    {
        Ok(None)
    }

    /// Consume one exact `Me`/`This` receiver source site and return the
    /// materialized receiver value.  `Ok(None)` means the receiver carries
    /// no registered variable site (e.g. `this` in a static box) and the
    /// caller keeps its existing resolution.  Raw ports return `None` and
    /// retain name-based receiver lookup.
    fn exact_source_receiver_value<'input>(
        &self,
        _input: &Self::ExprInput<'input>,
    ) -> Result<Option<ValueId>, String>
    where
        Self: 'input,
    {
        Ok(None)
    }
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

pub(in crate::mir::builder) fn raw_child_expr(
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

pub(in crate::mir::builder) fn raw_child_body(
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
