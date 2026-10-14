//! Whole-expression Home qualification and stable scalar loans of one source plan.
//! Source structure remains owned by resolved_value_profile; physical rows live later.
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedBinaryExpressionSourceV1, ResolvedBinaryOperatorV1,
    ResolvedInitializerRelationV1, ResolvedLiteralSourceV1, SourceStmtSiteV1,
};
use crate::mir::resolved_value_profile::expression_plan::{
    ExpressionNodeSourceV1, ExpressionSourcePlanV1,
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionScalarOperationLoanV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    node: usize,
    positive_divisor: Option<i64>,
}
impl ExpressionScalarOperationLoanV1 {
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.plan.owner()
    }
    pub(crate) fn statement(&self) -> &SourceStmtSiteV1 {
        self.plan.root().statement()
    }
    pub(crate) fn initializer(&self) -> Option<&ResolvedInitializerRelationV1> {
        self.plan.root().initializer()
    }
    pub(crate) fn binary(&self) -> &ResolvedBinaryExpressionSourceV1 {
        self.plan.nodes()[self.node]
            .binary()
            .expect("qualified original binary node")
    }
    pub(crate) fn positive_divisor(&self) -> Option<i64> {
        self.positive_divisor
    }
}

// Like the previous flow payload, equality compares immutable original facts.
// It never replaces the physical consumer's explicit Rc membership checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HomeExpressionPlanV1 {
    source: Rc<ExpressionSourcePlanV1>,
    scalar_operations: Vec<Rc<ExpressionScalarOperationLoanV1>>,
}
impl HomeExpressionPlanV1 {
    /// Called only after the original scalar preflight proves both Integer
    /// children. The parent still must close its complete field/call batch
    /// before installing this wrapper in the persistent Home flow.
    pub(super) fn scalar_root(
        source: Rc<ExpressionSourcePlanV1>,
        binary: &ResolvedBinaryExpressionSourceV1,
        positive_divisor: Option<i64>,
    ) -> Option<Self> {
        if source.has_opaque_obligations() || source.root().initializer().is_none() {
            return None;
        }
        let root = source.nodes().first()?;
        if root.binary()? != binary
            || source.root().initializer()?.initializer_site() != Some(root.site())
        {
            return None;
        }
        match binary.operator() {
            ResolvedBinaryOperatorV1::Subtract if positive_divisor.is_none() => {}
            ResolvedBinaryOperatorV1::Divide
                if positive_divisor.is_some_and(|n| n > 0)
                    && source.nodes().iter().any(|node| {
                        node.site() == binary.rhs()
                            && matches!(node.source(), ExpressionNodeSourceV1::Literal(
                        ResolvedLiteralSourceV1::Integer(n)) if Some(*n) == positive_divisor)
                    }) => {}
            _ => return None,
        }
        let loan = Rc::new(ExpressionScalarOperationLoanV1 {
            plan: Rc::clone(&source),
            node: 0,
            positive_divisor,
        });
        Some(Self {
            source,
            scalar_operations: vec![loan],
        })
    }
    pub(crate) fn source(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.source
    }
    pub(crate) fn scalar_operations(
        &self,
    ) -> impl Iterator<Item = &Rc<ExpressionScalarOperationLoanV1>> {
        self.scalar_operations.iter()
    }
    pub(super) fn is_consistent(&self) -> bool {
        let mut seen = std::collections::BTreeSet::new();
        self.scalar_operations.iter().all(|loan| {
            Rc::ptr_eq(&self.source, loan.plan())
                && self
                    .source
                    .nodes()
                    .get(loan.node)
                    .is_some_and(|node| node.binary().is_some())
                && seen.insert(loan.node)
        })
    }
}
