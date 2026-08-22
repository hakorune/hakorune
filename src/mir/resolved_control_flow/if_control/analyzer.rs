use std::num::NonZeroU32;

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedExprV1, LocatedStmtV1};
use crate::mir::compiler::source_view::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::resolved_semantics::{
    ResolvedIfRegionBundleV1, SourceExprSiteV1, SourceStmtSiteV1,
};

use super::super::function_control::VerifiedFunctionCompletionV1;
use super::super::source_coverage::{
    verify_located_source_coverage_v1, CoveredSourceSiteV1, SourceCoverageVerificationErrorV1,
};
use super::product::{
    IfControlCoverageClaimV1, ResolvedIfElsePortV1, ResolvedIfFallthroughPortV1,
    VerifiedLocatedIfControlV1, VerifiedResolvedFunctionIfControlV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::resolved_control_flow) enum ResolvedIfControlErrorV1 {
    OwnerClosureMismatch,
    CompletionOwnerMismatch,
    SourceNavigation(String),
    MissingIfBundle(SourceStmtSiteV1),
    ElseCardinalityMismatch(SourceStmtSiteV1),
    ForeignBundleOwner(SourceStmtSiteV1),
    UnsupportedStatement(SourceStmtSiteV1),
    UnsupportedExpression(SourceExprSiteV1),
    UnauthorizedReturn(SourceStmtSiteV1),
    InvalidRowSlot(usize),
    MissingRow(usize),
    DuplicateIfSite(SourceStmtSiteV1),
    BundleCardinalityMismatch { expected: usize, actual: usize },
    Coverage(SourceCoverageVerificationErrorV1),
    CoveragePartitionOverlap,
    CoveragePartitionMismatch,
    CoverageIndexOverflow,
}

impl From<SourceCoverageVerificationErrorV1> for ResolvedIfControlErrorV1 {
    fn from(error: SourceCoverageVerificationErrorV1) -> Self {
        Self::Coverage(error)
    }
}

pub(in crate::mir::resolved_control_flow) fn analyze_resolved_if_control_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<VerifiedResolvedFunctionIfControlV1, ResolvedIfControlErrorV1> {
    IfControlAnalyzerV1::new(input, completion, ExpressionPolicyV1::Closed)?.analyze()
}

/// Typed production boundary for the sealed If-control analyzer.
///
/// Internal verifier vocabulary stays encapsulated in this module; callers
/// may report the detail but cannot reinterpret a contract failure as profile
/// non-admission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedFunctionIfControlContractErrorV1 {
    detail: String,
}

pub(crate) fn verify_resolved_function_if_control_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<VerifiedResolvedFunctionIfControlV1, ResolvedFunctionIfControlContractErrorV1> {
    analyze_resolved_if_control_v1(input, completion).map_err(|error| {
        ResolvedFunctionIfControlContractErrorV1 {
            detail: format!("{error:?}"),
        }
    })
}

/// Disconnected P0c-S0b coverage ingress. The ordinary production verifier
/// remains call-closed until the atomic P0c-I1 route activation.
pub(crate) fn verify_resolved_function_if_control_with_direct_call_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<VerifiedResolvedFunctionIfControlV1, ResolvedFunctionIfControlContractErrorV1> {
    IfControlAnalyzerV1::new(input, completion, ExpressionPolicyV1::DirectCall)
        .and_then(IfControlAnalyzerV1::analyze)
        .map_err(|error| ResolvedFunctionIfControlContractErrorV1 {
            detail: format!("{error:?}"),
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExpressionPolicyV1 {
    Closed,
    DirectCall,
}

struct IfControlRowDraftV1<'source> {
    site: SourceStmtSiteV1,
    body: LocatedBodyV1<'source>,
    index: usize,
    regions: ResolvedIfRegionBundleV1,
    else_port: ResolvedIfElsePortV1,
    coverage: Vec<CoveredSourceSiteV1>,
}

struct IfControlAnalyzerV1<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    authorized_return_sites: Box<[SourceStmtSiteV1]>,
    rows: Vec<Option<IfControlRowDraftV1<'source>>>,
    partition: Vec<IfControlCoverageClaimV1>,
    expression_policy: ExpressionPolicyV1,
}

impl<'source> IfControlAnalyzerV1<'source> {
    fn new(
        input: ResolvedFunctionLoweringInputV1<'source>,
        completion: &VerifiedFunctionCompletionV1,
        expression_policy: ExpressionPolicyV1,
    ) -> Result<Self, ResolvedIfControlErrorV1> {
        if input.owner() != input.source().owner()
            || input.owner() != input.function().owner()
            || input.forest().owner(input.owner()).is_none()
        {
            return Err(ResolvedIfControlErrorV1::OwnerClosureMismatch);
        }
        if completion.owner() != input.owner() {
            return Err(ResolvedIfControlErrorV1::CompletionOwnerMismatch);
        }
        Ok(Self {
            input,
            authorized_return_sites: completion.explicit_sites().to_vec().into_boxed_slice(),
            rows: Vec::new(),
            partition: Vec::new(),
            expression_policy,
        })
    }

    fn analyze(mut self) -> Result<VerifiedResolvedFunctionIfControlV1, ResolvedIfControlErrorV1> {
        let body = self.input.source().root_body().map_err(source_navigation)?;
        self.visit_body(&body, None)?;
        self.seal()
    }

    fn visit_body(
        &mut self,
        body: &LocatedBodyV1<'source>,
        owner_row: Option<usize>,
    ) -> Result<(), ResolvedIfControlErrorV1> {
        for index in 0..body.statements().len() {
            let statement = self
                .input
                .source()
                .body_stmt(body, index)
                .map_err(source_navigation)?;
            self.visit_statement(body, index, &statement, owner_row)?;
        }
        Ok(())
    }

    fn visit_statement(
        &mut self,
        body: &LocatedBodyV1<'source>,
        index: usize,
        statement: &LocatedStmtV1<'source>,
        owner_row: Option<usize>,
    ) -> Result<(), ResolvedIfControlErrorV1> {
        if matches!(statement.node(), ASTNode::If { .. }) {
            return self.visit_if(body, index, statement);
        }
        self.claim(owner_row, CoveredSourceSiteV1::statement(statement))?;
        match statement.node() {
            ASTNode::Local { initial_values, .. } => {
                for (index, value) in initial_values.iter().enumerate() {
                    if value.is_some() {
                        let index = checked_index(index)?;
                        let value = self
                            .input
                            .source()
                            .child_expr_from_stmt(
                                statement,
                                ExprChildRoleV1::LocalInitializer(index),
                            )
                            .map_err(source_navigation)?;
                        self.visit_expression(&value, owner_row)?;
                    }
                }
            }
            ASTNode::Outbox {
                variables,
                initial_values,
                ..
            } if !variables.is_empty()
                && variables.len() == initial_values.len()
                && initial_values.iter().all(Option::is_none) => {}
            ASTNode::Assignment { .. } => {
                let target = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentTarget)
                    .map_err(source_navigation)?;
                let value = self
                    .input
                    .source()
                    .child_expr_from_stmt(statement, ExprChildRoleV1::AssignmentValue)
                    .map_err(source_navigation)?;
                self.visit_expression(&target, owner_row)?;
                self.visit_expression(&value, owner_row)?;
            }
            ASTNode::Return { value, .. } => {
                if !self.authorized_return_sites.contains(statement.site()) {
                    return Err(ResolvedIfControlErrorV1::UnauthorizedReturn(
                        statement.site().clone(),
                    ));
                }
                if value.is_some() {
                    let value = self
                        .input
                        .source()
                        .child_expr_from_stmt(statement, ExprChildRoleV1::ReturnValue)
                        .map_err(source_navigation)?;
                    self.visit_expression(&value, owner_row)?;
                }
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
                self.visit_expression(&expression, owner_row)?;
            }
            _ => {
                return Err(ResolvedIfControlErrorV1::UnsupportedStatement(
                    statement.site().clone(),
                ));
            }
        }
        Ok(())
    }

    fn visit_if(
        &mut self,
        body: &LocatedBodyV1<'source>,
        index: usize,
        statement: &LocatedStmtV1<'source>,
    ) -> Result<(), ResolvedIfControlErrorV1> {
        let ASTNode::If { else_body, .. } = statement.node() else {
            unreachable!("visit_if requires statement If")
        };
        let regions = *self
            .input
            .function()
            .if_region_bundle(statement.site())
            .map_err(|_| ResolvedIfControlErrorV1::MissingIfBundle(statement.site().clone()))?;
        if regions.control().owner() != self.input.owner()
            || regions.then_pair().region().owner() != self.input.owner()
            || regions.then_pair().scope().owner() != self.input.owner()
            || regions.else_pair().is_some_and(|pair| {
                pair.region().owner() != self.input.owner()
                    || pair.scope().owner() != self.input.owner()
            })
        {
            return Err(ResolvedIfControlErrorV1::ForeignBundleOwner(
                statement.site().clone(),
            ));
        }
        if else_body.is_some() != regions.else_pair().is_some() {
            return Err(ResolvedIfControlErrorV1::ElseCardinalityMismatch(
                statement.site().clone(),
            ));
        }
        let row = self.rows.len();
        self.rows.push(Some(IfControlRowDraftV1 {
            site: statement.site().clone(),
            body: body.clone(),
            index,
            regions,
            else_port: if else_body.is_some() {
                ResolvedIfElsePortV1::Explicit(ResolvedIfFallthroughPortV1::verified())
            } else {
                ResolvedIfElsePortV1::ImplicitIdentity
            },
            coverage: Vec::new(),
        }));

        self.claim(Some(row), CoveredSourceSiteV1::statement(statement))?;
        let condition = self
            .input
            .source()
            .child_expr_from_stmt(statement, ExprChildRoleV1::IfCondition)
            .map_err(source_navigation)?;
        self.visit_expression(&condition, Some(row))?;

        let then_body = self
            .input
            .source()
            .child_body_from_stmt(statement, BodyChildRoleV1::IfThen)
            .map_err(source_navigation)?;
        self.claim(Some(row), CoveredSourceSiteV1::body(&then_body))?;
        self.visit_body(&then_body, Some(row))?;

        if else_body.is_some() {
            let else_body = self
                .input
                .source()
                .child_body_from_stmt(statement, BodyChildRoleV1::IfElse)
                .map_err(source_navigation)?;
            self.claim(Some(row), CoveredSourceSiteV1::body(&else_body))?;
            self.visit_body(&else_body, Some(row))?;
        }
        Ok(())
    }

    fn visit_expression(
        &mut self,
        expression: &LocatedExprV1<'source>,
        owner_row: Option<usize>,
    ) -> Result<(), ResolvedIfControlErrorV1> {
        self.claim(owner_row, CoveredSourceSiteV1::expression(expression))?;
        match expression.node() {
            ASTNode::Literal { .. } | ASTNode::Variable { .. } => Ok(()),
            ASTNode::BinaryOp { .. } => {
                for role in [ExprChildRoleV1::BinaryLeft, ExprChildRoleV1::BinaryRight] {
                    let child = self
                        .input
                        .source()
                        .child_expr_from_expr(expression, role)
                        .map_err(source_navigation)?;
                    self.visit_expression(&child, owner_row)?;
                }
                Ok(())
            }
            ASTNode::BlockExpr { .. } => {
                let prelude = self
                    .input
                    .source()
                    .child_body_from_expr(expression, BodyChildRoleV1::BlockExprPrelude)
                    .map_err(source_navigation)?;
                self.claim(owner_row, CoveredSourceSiteV1::body(&prelude))?;
                self.visit_body(&prelude, owner_row)?;
                let tail = self
                    .input
                    .source()
                    .child_expr_from_expr(expression, ExprChildRoleV1::BlockExprTail)
                    .map_err(source_navigation)?;
                self.visit_expression(&tail, owner_row)
            }
            ASTNode::FunctionCall { arguments, .. }
                if self.expression_policy == ExpressionPolicyV1::DirectCall =>
            {
                for index in 0..arguments.len() {
                    let child = self
                        .input
                        .source()
                        .child_expr_from_expr(
                            expression,
                            ExprChildRoleV1::CallArgument(checked_index(index)?),
                        )
                        .map_err(source_navigation)?;
                    self.visit_expression(&child, owner_row)?;
                }
                Ok(())
            }
            _ => Err(ResolvedIfControlErrorV1::UnsupportedExpression(
                expression.site().clone(),
            )),
        }
    }

    fn claim(
        &mut self,
        owner_row: Option<usize>,
        site: CoveredSourceSiteV1,
    ) -> Result<(), ResolvedIfControlErrorV1> {
        let Some(row) = owner_row else {
            return Ok(());
        };
        let row_id = checked_index(row)?;
        let draft = self
            .rows
            .get_mut(row)
            .and_then(Option::as_mut)
            .ok_or(ResolvedIfControlErrorV1::InvalidRowSlot(row))?;
        draft.coverage.push(site.clone());
        self.partition
            .push(IfControlCoverageClaimV1 { row: row_id, site });
        Ok(())
    }

    fn seal(self) -> Result<VerifiedResolvedFunctionIfControlV1, ResolvedIfControlErrorV1> {
        let expected = self.input.function().if_region_bundle_count();
        if self.rows.len() != expected {
            return Err(ResolvedIfControlErrorV1::BundleCardinalityMismatch {
                expected,
                actual: self.rows.len(),
            });
        }
        let mut seen_sites = Vec::new();
        let mut seen_coverage = Vec::new();
        let mut rows = Vec::with_capacity(self.rows.len());
        for (index, draft) in self.rows.into_iter().enumerate() {
            let draft = draft.ok_or(ResolvedIfControlErrorV1::MissingRow(index))?;
            if seen_sites.contains(&draft.site) {
                return Err(ResolvedIfControlErrorV1::DuplicateIfSite(draft.site));
            }
            seen_sites.push(draft.site.clone());
            if draft
                .coverage
                .iter()
                .any(|site| seen_coverage.contains(site))
            {
                return Err(ResolvedIfControlErrorV1::CoveragePartitionOverlap);
            }
            seen_coverage.extend(draft.coverage.iter().cloned());
            let suffix = self
                .input
                .source()
                .body_suffix(draft.body, draft.index)
                .map_err(source_navigation)?;
            let range = self
                .input
                .source()
                .consumed_prefix(&suffix, NonZeroU32::new(1).expect("one is nonzero"))
                .map_err(source_navigation)?;
            let coverage =
                verify_located_source_coverage_v1(self.input.owner(), range, draft.coverage)?;
            if coverage.preorder().first()
                != Some(&CoveredSourceSiteV1::Statement {
                    owner: self.input.owner(),
                    site: draft.site.clone(),
                })
            {
                return Err(ResolvedIfControlErrorV1::CoveragePartitionMismatch);
            }
            let row_index = checked_index(index)?;
            let projected = self
                .partition
                .iter()
                .filter(|claim| claim.row == row_index)
                .map(|claim| &claim.site)
                .collect::<Vec<_>>();
            let expected = coverage.preorder().iter().collect::<Vec<_>>();
            if projected != expected {
                return Err(ResolvedIfControlErrorV1::CoveragePartitionMismatch);
            }
            rows.push(VerifiedLocatedIfControlV1 {
                site: draft.site,
                regions: draft.regions,
                then_port: ResolvedIfFallthroughPortV1::verified(),
                else_port: draft.else_port,
                coverage,
            });
        }
        Ok(VerifiedResolvedFunctionIfControlV1 {
            owner: self.input.owner(),
            rows: rows.into_boxed_slice(),
            coverage_partition: self.partition.into_boxed_slice(),
        })
    }
}

fn checked_index(index: usize) -> Result<u32, ResolvedIfControlErrorV1> {
    u32::try_from(index).map_err(|_| ResolvedIfControlErrorV1::CoverageIndexOverflow)
}

fn source_navigation(error: impl ToString) -> ResolvedIfControlErrorV1 {
    ResolvedIfControlErrorV1::SourceNavigation(error.to_string())
}
