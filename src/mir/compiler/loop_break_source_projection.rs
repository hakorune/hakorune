//! Caller-zero source projection for the bounded direct LoopBreak shape.
//!
//! This observer keeps only resolver-issued source sites and the paired exit
//! record.  It does not issue a Recipe, select a route, or touch Builder state.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, ResolvedControlTransferV1, ResolvedExitOriginV1,
    ResolvedExitRecordV1, ResolvedExitSiteV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedFunctionV1, VerifiedResolvedLoopSourceV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoopBreakSourceProjectionRejectV1 {
    ForeignOwner,
    SourceLookup,
    SourceNavigation,
    ScopeBox,
    BodyArity,
    BreakIfShape,
    BreakIfBodyArity,
    BreakIfElseBody,
    CarrierUpdateShape,
    StepShape,
    ExitResolution,
    ExitTargetMismatch,
}

/// Source-only proof for the direct three-statement LoopBreak profile.
///
/// The three body sites are retained separately so a later Facts owner can
/// co-seal them with its existing `LoopBreakSourceTopologyV1` instead of
/// rebuilding source identity from flattened AST coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedLoopBreakSourceProjectionV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    loop_condition_site: SourceExprSiteV1,
    break_if_site: SourceStmtSiteV1,
    break_condition_site: SourceExprSiteV1,
    break_exit_site: SourceStmtSiteV1,
    break_exit: ResolvedExitRecordV1,
    carrier_update_site: SourceStmtSiteV1,
    step_site: SourceStmtSiteV1,
    root_frame_key: crate::mir::resolved_semantics::LoopExecutionFrameKeyV1,
}

impl VerifiedLoopBreakSourceProjectionV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn loop_condition_site(&self) -> &SourceExprSiteV1 {
        &self.loop_condition_site
    }

    pub(crate) fn break_if_site(&self) -> &SourceStmtSiteV1 {
        &self.break_if_site
    }

    pub(crate) fn break_condition_site(&self) -> &SourceExprSiteV1 {
        &self.break_condition_site
    }

    pub(crate) fn break_exit_site(&self) -> &SourceStmtSiteV1 {
        &self.break_exit_site
    }

    pub(crate) const fn break_exit(&self) -> ResolvedExitRecordV1 {
        self.break_exit
    }

    pub(crate) fn carrier_update_site(&self) -> &SourceStmtSiteV1 {
        &self.carrier_update_site
    }

    pub(crate) fn step_site(&self) -> &SourceStmtSiteV1 {
        &self.step_site
    }

    pub(crate) const fn root_frame_key(
        &self,
    ) -> &crate::mir::resolved_semantics::LoopExecutionFrameKeyV1 {
        &self.root_frame_key
    }

    pub(crate) fn matches_source_identity(
        &self,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        site: &SourceStmtSiteV1,
    ) -> bool {
        self.function_origin == function_origin
            && self.source_kind == source_kind
            && &self.loop_site == site
    }
}

pub(crate) fn issue_loop_break_source_projection_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
    resolved_source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedLoopBreakSourceProjectionV1, LoopBreakSourceProjectionRejectV1> {
    if input.owner() != loop_stmt.owner() {
        return Err(LoopBreakSourceProjectionRejectV1::ForeignOwner);
    }
    let function = input.function();
    verify_source_identity(function, loop_stmt, &resolved_source)?;
    let source = input.source();
    let loop_condition = source
        .child_expr_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::ExprChildRoleV1::LoopCondition,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    let loop_body = source
        .child_body_from_stmt(
            loop_stmt,
            crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if loop_body
        .statements()
        .iter()
        .any(|stmt| matches!(stmt, ASTNode::ScopeBox { .. }))
    {
        return Err(LoopBreakSourceProjectionRejectV1::ScopeBox);
    }
    if loop_body.statements().len() != 3 {
        return Err(LoopBreakSourceProjectionRejectV1::BodyArity);
    }

    let break_if = source
        .body_stmt(&loop_body, 0)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    let ASTNode::If { else_body, .. } = break_if.node() else {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfShape);
    };
    if else_body.is_some() {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfElseBody);
    }
    let then_body = source
        .child_body_from_stmt(
            &break_if,
            crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if then_body.statements().len() != 1 {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfBodyArity);
    }
    let break_exit = source
        .body_stmt(&then_body, 0)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if !matches!(break_exit.node(), ASTNode::Break { .. }) {
        return Err(LoopBreakSourceProjectionRejectV1::BreakIfShape);
    }
    function
        .if_region_bundle(break_if.site())
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceLookup)?;
    let break_condition = source
        .child_expr_from_stmt(
            &break_if,
            crate::mir::resolved_semantics::ExprChildRoleV1::IfCondition,
        )
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;

    let carrier_update = source
        .body_stmt(&loop_body, 1)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if !matches!(carrier_update.node(), ASTNode::Assignment { .. }) {
        return Err(LoopBreakSourceProjectionRejectV1::CarrierUpdateShape);
    }
    let step = source
        .body_stmt(&loop_body, 2)
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceNavigation)?;
    if !matches!(step.node(), ASTNode::Assignment { .. }) {
        return Err(LoopBreakSourceProjectionRejectV1::StepShape);
    }

    let loop_region = function
        .loop_region_bundle(loop_stmt.site())
        .map_err(|_| LoopBreakSourceProjectionRejectV1::SourceLookup)?
        .loop_pair()
        .region();
    let exit_record = function
        .resolved_exit(&ResolvedExitSiteV1::Statement(break_exit.site().clone()))
        .ok_or(LoopBreakSourceProjectionRejectV1::ExitResolution)?;
    if exit_record.origin() != ResolvedExitOriginV1::ExplicitBreak
        || exit_record.transfer()
            != (ResolvedControlTransferV1::Break {
                target_loop: loop_region,
            })
    {
        return Err(LoopBreakSourceProjectionRejectV1::ExitTargetMismatch);
    }

    Ok(VerifiedLoopBreakSourceProjectionV1 {
        owner: input.owner(),
        function_origin: function.function_origin(),
        source_kind: function.source_kind(),
        loop_site: loop_stmt.site().clone(),
        loop_condition_site: loop_condition.site().clone(),
        break_if_site: break_if.site().clone(),
        break_condition_site: break_condition.site().clone(),
        break_exit_site: break_exit.site().clone(),
        break_exit: *exit_record,
        carrier_update_site: carrier_update.site().clone(),
        step_site: step.site().clone(),
        root_frame_key: resolved_source.frame_key(),
    })
}

fn verify_source_identity(
    function: &VerifiedResolvedFunctionV1,
    loop_stmt: &LocatedStmtV1<'_>,
    source: &VerifiedResolvedLoopSourceV1,
) -> Result<(), LoopBreakSourceProjectionRejectV1> {
    if source.matches_identity(
        function.function_origin(),
        function.source_kind(),
        loop_stmt.site(),
    ) {
        Ok(())
    } else {
        Err(LoopBreakSourceProjectionRejectV1::SourceLookup)
    }
}

#[cfg(test)]
mod tests {
    use super::{issue_loop_break_source_projection_v1, LoopBreakSourceProjectionRejectV1};
    use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

    fn variable(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_owned(),
            span: Span::unknown(),
        }
    }

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn assignment(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(variable(name)),
            value: Box::new(value),
            span: Span::unknown(),
        }
    }

    fn direct_loop(scope_boxed: bool, explicit_else: bool) -> ASTNode {
        let statements = vec![
            ASTNode::If {
                condition: Box::new(variable("stop")),
                then_body: vec![ASTNode::Break {
                    span: Span::unknown(),
                }],
                else_body: explicit_else.then_some(Vec::new()),
                span: Span::unknown(),
            },
            assignment(
                "sum",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("sum")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                },
            ),
            assignment(
                "i",
                ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(variable("i")),
                    right: Box::new(integer(1)),
                    span: Span::unknown(),
                },
            ),
        ];
        let body = if scope_boxed {
            vec![ASTNode::ScopeBox {
                body: statements,
                span: Span::unknown(),
            }]
        } else {
            statements
        };
        ASTNode::FunctionDeclaration {
            name: "loop_break_projection".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![
                ASTNode::Local {
                    variables: vec!["i".into(), "sum".into(), "stop".into()],
                    initial_values: vec![
                        Some(Box::new(integer(0))),
                        Some(Box::new(integer(0))),
                        Some(Box::new(ASTNode::Literal {
                            value: LiteralValue::Bool(true),
                            span: Span::unknown(),
                        })),
                    ],
                    declared_type_names: vec![None, None, None],
                    span: Span::unknown(),
                },
                ASTNode::Loop {
                    condition: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Less,
                        left: Box::new(variable("i")),
                        right: Box::new(integer(3)),
                        span: Span::unknown(),
                    }),
                    body,
                    span: Span::unknown(),
                },
            ],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn direct_three_statement_loop_break_projection_is_source_bound() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(false, false))
            .expect("resolved fixture");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        let projection = issue_loop_break_source_projection_v1(input, &loop_stmt, resolved)
            .expect("direct LoopBreak projection");
        assert_eq!(projection.owner(), input.owner());
        assert_eq!(projection.break_if_site().node().segments().len(), 2);
        assert_eq!(projection.carrier_update_site().node().segments().len(), 2);
        assert_eq!(projection.step_site().node().segments().len(), 2);
        assert_eq!(
            projection.break_exit().origin(),
            crate::mir::resolved_semantics::ResolvedExitOriginV1::ExplicitBreak
        );
    }

    #[test]
    fn scope_box_loop_break_is_rejected_before_projection() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(true, false))
            .expect("resolved fixture");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        assert_eq!(
            issue_loop_break_source_projection_v1(input, &loop_stmt, resolved),
            Err(LoopBreakSourceProjectionRejectV1::ScopeBox)
        );
    }

    #[test]
    fn explicit_else_is_rejected_for_direct_break_if() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(direct_loop(false, true))
            .expect("resolved fixture");
        let input = unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let resolved = input
            .function()
            .resolved_loop_source(loop_stmt.site())
            .expect("loop source");
        assert_eq!(
            issue_loop_break_source_projection_v1(input, &loop_stmt, resolved),
            Err(LoopBreakSourceProjectionRejectV1::BreakIfElseBody)
        );
    }
}
