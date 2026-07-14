//! Located-source traversal for disconnected resolved statement-`If` flow.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedAssignmentTargetV1, ScopeId, SourceExprSiteV1,
    SourceStmtSiteV1,
};

use super::coverage::IfFlowCoverageDraftV1;
use super::if_flow::{
    ResolvedFunctionFlowDraftV1, ResolvedIfFlowDraftV1, VerifiedResolvedFunctionFlowV1,
};
use super::verifier::{verify_if_flow_draft, ResolvedRegionFlowVerificationErrorV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedRegionFlowErrorV1 {
    OwnerClosureMismatch {
        input: FunctionOwnerIdV1,
        source: FunctionOwnerIdV1,
        function: FunctionOwnerIdV1,
    },
    CompletionOwnerMismatch {
        input: FunctionOwnerIdV1,
        completion: FunctionOwnerIdV1,
    },
    SourceNavigation {
        detail: String,
    },
    UnsupportedStatement {
        site: String,
        actual: &'static str,
        reason: &'static str,
    },
    UnsupportedExpression {
        site: String,
        actual: &'static str,
        reason: &'static str,
    },
    MissingAssignmentTarget(SourceExprSiteV1),
    UnsupportedAssignmentTarget(SourceExprSiteV1),
    MissingBlockExprScope(SourceExprSiteV1),
    Verification(ResolvedRegionFlowVerificationErrorV1),
}

impl From<ResolvedRegionFlowVerificationErrorV1> for ResolvedRegionFlowErrorV1 {
    fn from(error: ResolvedRegionFlowVerificationErrorV1) -> Self {
        Self::Verification(error)
    }
}

pub(crate) fn analyze_resolved_function_flow_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<VerifiedResolvedFunctionFlowV1, ResolvedRegionFlowErrorV1> {
    AnalyzerV1::new(input, completion)?.analyze()
}

#[derive(Debug, Default)]
struct AnalysisSummaryV1 {
    effects: Vec<BindingRefV1>,
    direct_assignments: Vec<SourceExprSiteV1>,
}

struct AnalyzerV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    draft: ResolvedFunctionFlowDraftV1,
    authorized_return_site: Option<SourceStmtSiteV1>,
}

impl<'source> AnalyzerV1<'source> {
    fn new(
        input: ResolvedFunctionLoweringInputV1<'source>,
        completion: &VerifiedFunctionCompletionV1,
    ) -> Result<Self, ResolvedRegionFlowErrorV1> {
        let owner = input.owner();
        let source = input.source().owner();
        let function = input.function().owner();
        if owner != source || owner != function || input.forest().owner(owner).is_none() {
            return Err(ResolvedRegionFlowErrorV1::OwnerClosureMismatch {
                input: owner,
                source,
                function,
            });
        }
        if completion.owner() != owner {
            return Err(ResolvedRegionFlowErrorV1::CompletionOwnerMismatch {
                input: owner,
                completion: completion.owner(),
            });
        }
        Ok(Self {
            input,
            draft: ResolvedFunctionFlowDraftV1::new(owner),
            authorized_return_site: completion.explicit_site().cloned(),
        })
    }

    fn analyze(mut self) -> Result<VerifiedResolvedFunctionFlowV1, ResolvedRegionFlowErrorV1> {
        let body = self.input.source().root_body().map_err(source_navigation)?;
        let summary = self.analyze_body(&body)?;
        for site in summary.direct_assignments {
            self.draft.coverage_mut().record_direct(site);
        }
        self.draft.seal(self.input.function()).map_err(Into::into)
    }

    fn analyze_body(
        &mut self,
        body: &LocatedBodyV1<'source>,
    ) -> Result<AnalysisSummaryV1, ResolvedRegionFlowErrorV1> {
        let mut summary = AnalysisSummaryV1::default();
        for index in 0..body.statements().len() {
            let statement = self
                .input
                .source()
                .body_stmt(body, index)
                .map_err(source_navigation)?;
            summary.merge(self.analyze_statement(&statement)?);
        }
        Ok(summary)
    }

    fn analyze_statement(
        &mut self,
        statement: &LocatedStmtV1<'source>,
    ) -> Result<AnalysisSummaryV1, ResolvedRegionFlowErrorV1> {
        match statement.node() {
            ASTNode::Local {
                variables,
                initial_values,
                declared_type_names,
                ..
            } => {
                if variables.is_empty()
                    || initial_values.len() != variables.len()
                    || declared_type_names.len() != variables.len()
                    || declared_type_names.iter().any(Option::is_some)
                {
                    return Err(self.unsupported_statement(statement, "local_shape_not_closed"));
                }
                let mut summary = AnalysisSummaryV1::default();
                for (index, value) in initial_values.iter().enumerate() {
                    if value.is_none() {
                        continue;
                    }
                    let value = self
                        .input
                        .source()
                        .child_expr_from_stmt(
                            statement,
                            ExprChildRoleV1::LocalInitializer(index as u32),
                        )
                        .map_err(source_navigation)?;
                    summary.merge(self.analyze_expression(&value)?);
                }
                Ok(summary)
            }
            ASTNode::Outbox {
                variables,
                initial_values,
                ..
            } => {
                if variables.is_empty()
                    || initial_values.len() != variables.len()
                    || initial_values.iter().any(Option::is_some)
                {
                    return Err(self.unsupported_statement(statement, "outbox_shape_not_closed"));
                }
                Ok(AnalysisSummaryV1::default())
            }
            ASTNode::Assignment { .. } => self.analyze_assignment(statement),
            ASTNode::If { .. } => self.analyze_if(statement),
            ASTNode::Return { value, .. } => {
                if self.authorized_return_site.as_ref() != Some(statement.site()) {
                    return Err(self.unsupported_statement(
                        statement,
                        "return_not_fallthrough_or_not_root_final",
                    ));
                }
                if value.is_none() {
                    return Ok(AnalysisSummaryV1::default());
                }
                let value = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                    .map_err(source_navigation)?;
                self.analyze_expression(&value)
            }
            ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::BlockExpr { .. } => {
                let expression = self
                    .input
                    .source()
                    .statement_expression(statement)
                    .map_err(source_navigation)?;
                self.analyze_expression(&expression)
            }
            _ => Err(self.unsupported_statement(statement, "statement_not_in_s2_family")),
        }
    }

    fn analyze_expression(
        &mut self,
        expression: &LocatedExprV1<'source>,
    ) -> Result<AnalysisSummaryV1, ResolvedRegionFlowErrorV1> {
        match expression.node() {
            ASTNode::Literal { .. } | ASTNode::Variable { .. } => Ok(AnalysisSummaryV1::default()),
            ASTNode::BinaryOp { operator, .. }
                if !matches!(operator, BinaryOperator::And | BinaryOperator::Or) =>
            {
                let left = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BinaryLeft)
                    .map_err(source_navigation)?;
                let right = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BinaryRight)
                    .map_err(source_navigation)?;
                let mut summary = self.analyze_expression(&left)?;
                summary.merge(self.analyze_expression(&right)?);
                Ok(summary)
            }
            ASTNode::BlockExpr { .. } => {
                let pair = self
                    .input
                    .function()
                    .block_expr_scope_region_pair(expression.owner(), expression.site())
                    .map_err(|_| {
                        ResolvedRegionFlowErrorV1::MissingBlockExprScope(expression.site().clone())
                    })?;
                let prelude = self
                    .input
                    .source()
                    .child_body_from_expr(expression, BodyChildRoleV1::BlockExprPrelude)
                    .map_err(source_navigation)?;
                let tail = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BlockExprTail)
                    .map_err(source_navigation)?;
                let mut summary = self.analyze_body(&prelude)?;
                summary.merge(self.analyze_expression(&tail)?);
                summary.effects =
                    self.effects_visible_outside_scope(summary.effects, pair.scope())?;
                Ok(summary)
            }
            _ => Err(ResolvedRegionFlowErrorV1::UnsupportedExpression {
                site: format!("{:?}", expression.site()),
                actual: expression.node().node_type(),
                reason: "expression_not_in_s2_family",
            }),
        }
    }

    /// Analyze the RHS fully before claiming the exact assignment target.
    fn analyze_assignment(
        &mut self,
        statement: &LocatedStmtV1<'source>,
    ) -> Result<AnalysisSummaryV1, ResolvedRegionFlowErrorV1> {
        let value = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
            .map_err(source_navigation)?;
        let mut summary = self.analyze_expression(&value)?;

        let target = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
            .map_err(source_navigation)?;
        if !matches!(target.node(), ASTNode::Variable { .. }) {
            return Err(ResolvedRegionFlowErrorV1::UnsupportedAssignmentTarget(
                target.site().clone(),
            ));
        }
        let resolved = self
            .input
            .function()
            .assignment_target(target.site())
            .ok_or_else(|| {
                ResolvedRegionFlowErrorV1::MissingAssignmentTarget(target.site().clone())
            })?;
        let ResolvedAssignmentTargetV1::BindingRebind(binding) = resolved else {
            return Err(ResolvedRegionFlowErrorV1::UnsupportedAssignmentTarget(
                target.site().clone(),
            ));
        };
        summary.record_effect(*binding);
        summary.direct_assignments.push(target.site().clone());
        Ok(summary)
    }

    fn analyze_if(
        &mut self,
        statement: &LocatedStmtV1<'source>,
    ) -> Result<AnalysisSummaryV1, ResolvedRegionFlowErrorV1> {
        let ASTNode::If { else_body, .. } = statement.node() else {
            unreachable!("analyze_if is called only for ASTNode::If")
        };
        let site = statement.site().clone();
        // Reservation precedes recursion; installation follows it. This keeps
        // publication in source preorder while completing children postorder.
        let slot = self.draft.reserve_if(site.clone());

        let condition = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
            .map_err(source_navigation)?;
        let condition = self.analyze_expression(&condition)?;

        let then_body = self
            .input
            .source()
            .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
            .map_err(source_navigation)?;
        let then_summary = self.analyze_body(&then_body)?;

        let else_summary = if else_body.is_some() {
            let else_body = self
                .input
                .source()
                .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                .map_err(source_navigation)?;
            Some(self.analyze_body(&else_body)?)
        } else {
            None
        };

        let mut coverage = IfFlowCoverageDraftV1::default();
        for site in condition.direct_assignments {
            coverage.record_condition(site);
        }
        for site in then_summary.direct_assignments {
            coverage.record_then(site);
        }
        let (else_effects, else_direct) = match else_summary {
            Some(summary) => (Some(summary.effects), summary.direct_assignments),
            None => (None, Vec::new()),
        };
        for site in else_direct {
            coverage.record_else(site);
        }

        let row = verify_if_flow_draft(
            self.input.function(),
            ResolvedIfFlowDraftV1::new(
                site,
                else_body.is_some(),
                condition.effects,
                then_summary.effects,
                else_effects,
                coverage,
            ),
        )?;
        let whole_effects = row.whole_effects().may_rebind_outer().to_vec();
        self.draft.install_if(slot, row)?;
        Ok(AnalysisSummaryV1 {
            effects: whole_effects,
            // Nested flow owns its direct sites; parents receive effects only.
            direct_assignments: Vec::new(),
        })
    }

    fn effects_visible_outside_scope(
        &self,
        effects: Vec<BindingRefV1>,
        closed_scope: ScopeId,
    ) -> Result<Vec<BindingRefV1>, ResolvedRegionFlowErrorV1> {
        let function = self.input.function();
        let mut outer = Vec::new();
        for binding in effects {
            if binding.owner() != function.owner() {
                return Err(ResolvedRegionFlowVerificationErrorV1::ForeignBinding(binding).into());
            }
            let owner_scope = function
                .binding(binding)
                .ok_or(ResolvedRegionFlowVerificationErrorV1::MissingBinding(
                    binding,
                ))?
                .owner_scope();
            if self.scope_is_ancestor(closed_scope, owner_scope)? {
                continue;
            }
            if !self.scope_is_ancestor(owner_scope, closed_scope)? {
                return Err(
                    ResolvedRegionFlowVerificationErrorV1::UnrelatedBindingScope {
                        binding,
                        entry_scope: closed_scope,
                        owner_scope,
                    }
                    .into(),
                );
            }
            push_unique(&mut outer, binding);
        }
        Ok(outer)
    }

    fn scope_is_ancestor(
        &self,
        ancestor: ScopeId,
        mut scope: ScopeId,
    ) -> Result<bool, ResolvedRegionFlowErrorV1> {
        let function = self.input.function();
        for _ in 0..=function.scope_count() {
            if scope == ancestor {
                return Ok(true);
            }
            let record = function
                .scope(scope)
                .ok_or(ResolvedRegionFlowVerificationErrorV1::MissingScope(scope))?;
            let Some(parent) = record.parent() else {
                return Ok(false);
            };
            scope = parent;
        }
        Err(ResolvedRegionFlowVerificationErrorV1::ScopeParentCycle(scope).into())
    }

    fn unsupported_statement(
        &self,
        statement: &LocatedStmtV1<'source>,
        reason: &'static str,
    ) -> ResolvedRegionFlowErrorV1 {
        ResolvedRegionFlowErrorV1::UnsupportedStatement {
            site: format!("{:?}", statement.site()),
            actual: statement.node().node_type(),
            reason,
        }
    }
}

impl AnalysisSummaryV1 {
    fn merge(&mut self, other: Self) {
        for binding in other.effects {
            self.record_effect(binding);
        }
        self.direct_assignments.extend(other.direct_assignments);
    }

    fn record_effect(&mut self, binding: BindingRefV1) {
        if !self.effects.contains(&binding) {
            self.effects.push(binding);
        }
    }
}

fn source_navigation(error: impl ToString) -> ResolvedRegionFlowErrorV1 {
    ResolvedRegionFlowErrorV1::SourceNavigation {
        detail: error.to_string(),
    }
}

fn push_unique(bindings: &mut Vec<BindingRefV1>, binding: BindingRefV1) {
    if !bindings.contains(&binding) {
        bindings.push(binding);
    }
}
