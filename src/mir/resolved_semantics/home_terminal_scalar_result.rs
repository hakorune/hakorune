//! Scalar Return source identity and its retained original evaluation.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind;

/// Exact source relation for a Completion-backed `return <expr>` whose
/// scalar classifier proves an i64 or Bool value (R16) that is neither a literal nor a
/// field read: a local bound to a proven-i64 source (literal store, exact
/// formal, field-read result, or classified call result) or a trivial
/// integer expression. Ordinary scalar proof stays in the source inventory and
/// current flow; a direct finite entry read retains its original source loan.
/// Neither form grants a physical result ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TerminalScalarReturnV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    value_site: SourceExprSiteV1,
    /// R16: the exit's scalar kind, from the stored local's kind or the evaluation's root class.
    kind: SourceScalarKind,
    entry_read: Option<crate::mir::normal_callable_semantic_package::FiniteEntryReadV1>,
    original_read: Option<crate::mir::resolved_semantics::home_new_prefix::CurrentScalarReadV1>,
    evaluation: Option<
        std::rc::Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    >,
}

impl TerminalScalarReturnV1 {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn issue(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
    ) -> Self {
        Self {
            owner,
            return_site,
            value_site,
            kind: SourceScalarKind::Integer,
            entry_read: None,
            original_read: None,
            evaluation: None,
        }
    }
    /// A returned local whose stored kind is Bool: the same relation, its kind taken from the local.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn issue_with_kind(
        owner: FunctionOwnerIdV1,
        return_site: SourceStmtSiteV1,
        value_site: SourceExprSiteV1,
        kind: SourceScalarKind,
    ) -> Self {
        Self { kind, ..Self::issue(owner, return_site, value_site) }
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn with_entry_read_v1(
        mut self,
        read: crate::mir::normal_callable_semantic_package::FiniteEntryReadV1,
    ) -> Option<Self> {
        let required = read.requirement();
        if self.evaluation.is_some()
            || !read.signed_only()
            || required.node() != 0
            || required.read() != &self.value_site
            || required.source_plan().owner() != self.owner
            || required.source_plan().root().statement() != &self.return_site
        {
            return None;
        }
        self.entry_read = Some(read);
        Some(self)
    }
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn with_original_read_v1(
        mut self, read: Option<crate::mir::resolved_semantics::home_new_prefix::CurrentScalarReadV1>,
    ) -> Self {
        self.original_read = read;
        self
    }
    pub(crate) fn original_read_v1(&self) -> Option<&crate::mir::resolved_semantics::home_new_prefix::CurrentScalarReadV1> {
        self.original_read.as_ref()
    }
    pub(crate) fn evaluation(
        &self,
    ) -> Option<
        &std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
    > {
        self.evaluation.as_ref()
    }
    pub(super) fn with_evaluation_v1(
        mut self,
        evaluation: std::rc::Rc<
            crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1,
        >,
    ) -> Option<Self> {
        use crate::mir::resolved_value_profile::{
            expression_class::ExpressionValueClassV1, expression_plan::ExpressionPlanRootV1,
            expression_projection::ExpressionLeafClassV1,
            expression_root::StatementExpressionRoleV1,
        };
        if self.entry_read.is_some()
            || evaluation.source().owner() != self.owner
            || !matches!(evaluation.source().root(), ExpressionPlanRootV1::Statement {
                statement, role: StatementExpressionRoleV1::ReturnValue, value,
            } if statement == &self.return_site && value == &self.value_site)
            || !matches!(
                evaluation.root_class(),
                ExpressionLeafClassV1::Value(
                    ExpressionValueClassV1::I64 | ExpressionValueClassV1::Bool
                )
            )
            || !evaluation.reference_coverage_is_exact()
        {
            return None;
        }
        self.kind = match evaluation.root_class() {
            ExpressionLeafClassV1::Value(ExpressionValueClassV1::Bool) => SourceScalarKind::Bool,
            _ => SourceScalarKind::Integer,
        };
        self.evaluation = Some(evaluation);
        Some(self)
    }
    pub(crate) const fn scalar_kind(&self) -> SourceScalarKind {
        self.kind
    }
    pub(crate) fn entry_read(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::FiniteEntryReadV1> {
        self.entry_read.as_ref()
    }
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }
    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }
}
