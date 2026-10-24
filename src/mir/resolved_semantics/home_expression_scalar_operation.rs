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
    pub(super) fn issue_root(
        source: Rc<ExpressionSourcePlanV1>,
        binary: &ResolvedBinaryExpressionSourceV1,
        positive_divisor: Option<i64>,
    ) -> Option<Rc<Self>> {
        Self::issue_node_v1(source, 0, binary, positive_divisor)
    }

    /// Retain the existing scalar integer law for one exact operation node in
    /// a complete expression. The expression root remains anchored to its
    /// original purpose; nested nodes never create a new source root.
    pub(super) fn issue_node_v1(
        source: Rc<ExpressionSourcePlanV1>,
        node_index: usize,
        binary: &ResolvedBinaryExpressionSourceV1,
        positive_divisor: Option<i64>,
    ) -> Option<Rc<Self>> {
        if source.has_opaque_obligations() {
            return None;
        }
        let root = source.nodes().first()?;
        let node = source.nodes().get(node_index)?;
        let original_value = match source.root() {
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                statement, call, ordinal,
            } if positive_divisor.is_none()
                && call.owner() == source.owner()
                && call.arguments().get(*ordinal as usize).is_some_and(|argument|
                    argument.ordinal() == *ordinal && argument.site() == root.site())
                && call.site().node().segments().starts_with(statement.node().segments()) => root.site(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::LocalInitializer { initializer, .. } => initializer.initializer_site()?,
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment }
                if assignment.form() == crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain => assignment.value_site(),
            crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Statement {
                role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::ReturnValue, value, ..
            } if positive_divisor.is_none() => value,
            _ => return None,
        };
        if node.binary()? != binary || original_value != root.site() {
            return None;
        }
        match binary.operator() {
            ResolvedBinaryOperatorV1::Subtract if positive_divisor.is_none() => {}
            ResolvedBinaryOperatorV1::Divide | ResolvedBinaryOperatorV1::Modulo
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
            node: node_index,
            positive_divisor,
        });
        Some(loan)
    }
    /// One source operator law shared by original scalar and expression users.
    pub(crate) fn integer_operator_v1(&self) -> Option<crate::mir::BinaryOp> {
        match self.binary().operator() {
            ResolvedBinaryOperatorV1::Subtract if self.positive_divisor.is_none() => {
                Some(crate::mir::BinaryOp::Sub)
            }
            ResolvedBinaryOperatorV1::Divide if self.positive_divisor.is_some_and(|n| n > 0) => {
                Some(crate::mir::BinaryOp::Div)
            }
            ResolvedBinaryOperatorV1::Modulo if self.positive_divisor.is_some_and(|n| n > 0) => {
                Some(crate::mir::BinaryOp::Mod)
            }
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

impl CheckedExpressionOperationLoanV1 {
    /// Lend the SAME scalar law to ordinary append/SSA/final correspondence.
    pub(crate) fn scalar_source_v1(&self) -> Option<&Rc<ExpressionScalarOperationLoanV1>> {
        let CheckedExpressionOperationProofV1::Scalar(source) = &self.proof else {
            return None;
        };
        (Rc::ptr_eq(source.plan(), &self.plan)
            && self
                .plan
                .nodes()
                .get(self.node)
                .and_then(|node| node.binary())
                == Some(source.binary()))
        .then_some(source)
    }
}
