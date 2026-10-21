//! The retained original scalar operation law, shared across expression purposes.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExpressionScalarOperationLoanV1 {
    pub(super) plan: Rc<ExpressionSourcePlanV1>,
    pub(super) node: usize,
    pub(super) positive_divisor: Option<i64>,
}
impl ExpressionScalarOperationLoanV1 {
    /// The existing exact Local/root law; value-class qualification stays with
    /// its caller. This issues no physical append or derived reuse permission.
    pub(super) fn issue_root(source: Rc<ExpressionSourcePlanV1>,
        binary: &ResolvedBinaryExpressionSourceV1, positive_divisor: Option<i64>,
    ) -> Option<Rc<Self>> {
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
        let loan = Rc::new(Self {
            plan: Rc::clone(&source),
            node: 0,
            positive_divisor,
        });
        Some(loan)
    }
    /// One source operator law shared by original scalar and expression users.
    pub(crate) fn integer_operator_v1(&self) -> Option<crate::mir::BinaryOp> {
        match self.binary().operator() {
            ResolvedBinaryOperatorV1::Subtract if self.positive_divisor.is_none() => Some(crate::mir::BinaryOp::Sub),
            ResolvedBinaryOperatorV1::Divide if self.positive_divisor.is_some_and(|n| n > 0) => Some(crate::mir::BinaryOp::Div),
            _ => None,
        }
    }

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
